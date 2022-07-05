use ::std::convert::TryInto;
use ::std::error::Error as ErrorTrait;
use ::std::fmt;
use ::std::time::Duration;

use ::regex::Regex;

#[derive(Debug, PartialEq, Eq, Clone)]
/// An enumeration of the possible errors while parsing.
pub enum Error {
    // Failed to parse the number, including too large numbers.
    ParseInt(String),
    /// An unrecognized unit was found.
    UnknownUnit(String),
    /// A `i64` was out of range for conversion into a smaller or unsigned type.
    OutOfBounds(i64),
    /// There was an overflow in the calculation. Usually this happens at 2^63 or 2^64.
    Overflow,
    /// Unlike zeta12ti's original parse_duration, this fork does
    /// not support exponential notation in durations.
    ExpNotSupported,
    /// A value without a unit was found.
    NoUnitFound(String),
    /// No value at all was found.
    NoValueFound(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ParseInt(ref s) => {
                write!(f, "ParseIntError: Failed to parse \"{}\" as an integer", s)
            }
            Error::UnknownUnit(ref s) => {
                write!(f, "UnknownUnitError: \"{}\" is not a known unit", s)
            }
            Error::OutOfBounds(ref b) => {
                write!(f, "OutOfBoundsError: \"{}\" cannot be converted to u64", b)
            }
            Error::NoUnitFound(ref s) => {
                write!(f, "NoUnitFoundError: no unit found for the value \"{}\"", s)
            }
            Error::NoValueFound(ref s) => write!(
                f,
                "NoValueFoundError: no value found in the string \"{}\"",
                s
            ),
            Error::Overflow => {
                write!(f, "Value too high or too low (maximum is around ±9.2e18)")
            }
            Error::ExpNotSupported => {
                write!(f, "Exponential notation not supported (i.e. not 2.3e4)")
            }
        }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ParseInt(_) => "Failed to parse a string into an integer",
            Error::UnknownUnit(_) => "An unknown unit was used",
            Error::OutOfBounds(_) => "An integer was too large to convert into a u64",
            Error::NoUnitFound(_) => "A value without a unit was found",
            Error::NoValueFound(_) => "No value was found",
            Error::Overflow => "Value too high or too low",
            Error::ExpNotSupported => "Exponential notation not supported (i.e. not 2.3e4)",
        }
    }
}

/// A `ProtoDuration` is a duration with arbitrarily large fields.
/// It can be conditionally converted into a normal Duration, if the fields are small enough.
#[derive(Default)]
struct ProtoDuration {
    /// The number of nanoseconds in the `ProtoDuration`. May be negative.
    nanoseconds: i64,
    /// The number of microseconds in the `ProtoDuration`. May be negative.
    microseconds: i64,
    /// The number of milliseconds in the `ProtoDuration`. May be negative.
    milliseconds: i64,
    /// The number of seconds in the `ProtoDuration`. May be negative.
    seconds: i64,
    /// The number of minutes in the `ProtoDuration`. May be negative.
    minutes: i64,
    /// The number of hours in the `ProtoDuration`. May be negative.
    hours: i64,
    /// The number of days in the `ProtoDuration`. May be negative.
    days: i64,
    /// The number of weeks in the `ProtoDuration`. May be negative.
    weeks: i64,
    /// The number of months in the `ProtoDuration`. May be negative.
    months: i64,
    /// The number of years in the `ProtoDuration`. May be negative.
    years: i64,
}

impl ProtoDuration {
    /// Try to convert a `ProtoDuration` into a `Duration`.
    /// This may fail if the `ProtoDuration` is too long or it ends up having a negative total duration.
    fn into_duration(self) -> Result<Duration, Error> {
        let mut nanoseconds =
            self.nanoseconds + 1_000_i64 * self.microseconds + 1_000_000_i64 * self.milliseconds;
        let mut seconds = self.seconds
            + 60_i64 * self.minutes
            + 3_600_i64 * self.hours
            + 86_400_i64 * self.days
            + 604_800_i64 * self.weeks
            + 2_629_746_i64 * self.months
            + 31_556_952_i64 * self.years;

        seconds += &nanoseconds / 1_000_000_000;
        nanoseconds %= 1_000_000_000;

        let seconds: u64 = seconds
            .try_into()
            .map_err(|_| Error::OutOfBounds(seconds))?;
        let nanoseconds: u32 = nanoseconds
            .try_into()
            .map_err(|_| Error::OutOfBounds(nanoseconds))?;

        Ok(Duration::new(seconds, nanoseconds))
    }
}

lazy_static! {
    static ref NUMBER_RE: Regex = Regex::new(
        r"(?x)
        ^
        [^0-9a-zA-Z_-]*     # any non-word characters, except '-' (for negatives - may add '.' for decimals)
        (-?\d+)             # a possible negative sign and some positive number of digits
        [^0-9a-zA-Z_-]*     # more non-word characters
        $"
    )
    .expect("Compiling a regex went wrong");
}

lazy_static! {
    static ref DURATION_RE: Regex = Regex::new(
        r"(?x)(?i)
        (?P<int>-?\d+)              # the integer part
        \.?(?:(?P<dec>\d+))?        # an optional decimal part
                                    # note: the previous part will eat any decimals
                                    # if there's no decimal point.
                                    # This means we'll always have the decimal point if this
                                    # section matches at all.
        (?:e(?P<exp>[-+]?\d+))?     # an optional exponent
        (?:
            [^0-9a-zA-Z_]*          # some amount of junk (non word characters)
            (?P<unit>[0-9a-zA-Z_&&[^\d]]+)  # a word with no digits
        )?
        ",
    )
    .expect("Compiling a regex went wrong");
}

/// Convert some unit abbreviations to their full form.
/// See the [module level documentation](index.html) for more information about which abbreviations are accepted.
// TODO: return an `enum`.
fn parse_unit(unit: &str) -> &str {
    let unit_casefold = unit.to_lowercase();

    if unit_casefold.starts_with('n')
        && ("nanoseconds".starts_with(&unit_casefold) || "nsecs".starts_with(&unit_casefold))
    {
        "nanoseconds"
    } else if unit_casefold.starts_with("mic") && "microseconds".starts_with(&unit_casefold)
        || unit_casefold.starts_with('u') && "usecs".starts_with(&unit_casefold)
        || unit_casefold.starts_with('μ') && "\u{3bc}secs".starts_with(&unit_casefold)
    {
        "microseconds"
    } else if unit_casefold.starts_with("mil") && "milliseconds".starts_with(&unit_casefold)
        || unit_casefold.starts_with("ms") && "msecs".starts_with(&unit_casefold)
    {
        "milliseconds"
    } else if unit_casefold.starts_with('s')
        && ("seconds".starts_with(&unit_casefold) || "secs".starts_with(&unit_casefold))
    {
        "seconds"
    } else if (unit_casefold.starts_with("min") || unit.starts_with('m'))
        && ("minutes".starts_with(&unit_casefold) || "mins".starts_with(&unit_casefold))
    {
        "minutes"
    } else if unit_casefold.starts_with('h')
        && ("hours".starts_with(&unit_casefold) || "hrs".starts_with(&unit_casefold))
    {
        "hours"
    } else if unit_casefold.starts_with('d') && "days".starts_with(&unit_casefold) {
        "days"
    } else if unit_casefold.starts_with('w') && "weeks".starts_with(&unit_casefold) {
        "weeks"
    } else if (unit_casefold.starts_with("mo") || unit.starts_with('M'))
        && "months".starts_with(&unit_casefold)
    {
        "months"
    } else if unit_casefold.starts_with('y')
        && ("years".starts_with(&unit_casefold) || "yrs".starts_with(&unit_casefold))
    {
        "years"
    } else {
        unit
    }
}

/// Parse a string into a duration object.
///
/// See the [module level documentation](index.html) for more.
pub fn parse(input: &str) -> Result<Duration, Error> {
    if let Some(int) = NUMBER_RE.captures(input) {
        // This means it's just a value
        // Since the regex matched, the first group exists, so we can unwrap.
        let txt = int.get(1).unwrap().as_str();
        let seconds = txt
            .parse::<i64>()
            .map_err(|_| Error::ParseInt(txt.to_owned()))?;
        Ok(Duration::new(
            seconds.try_into().map_err(|_| Error::Overflow)?,
            0,
        ))
    } else if DURATION_RE.is_match(input) {
        // This means we have at least one "unit" (or plain word) and one value.
        let mut duration = ProtoDuration::default();
        for capture in DURATION_RE.captures_iter(input) {
            if capture.name("exp").is_some() {
                return Err(Error::ExpNotSupported);
            }
            match (
                capture.name("int"),
                capture.name("dec"),
                capture.name("unit"),
            ) {
                // capture.get(0) is *always* the actual match, so unwrapping causes no problems
                (.., None) => {
                    return Err(Error::NoUnitFound(
                        capture.get(0).unwrap().as_str().to_owned(),
                    ))
                }
                (None, ..) => {
                    return Err(Error::NoValueFound(
                        capture.get(0).unwrap().as_str().to_owned(),
                    ))
                }
                (Some(int), None, Some(unit)) => {
                    let txt = int.as_str();
                    let int = txt
                        .parse::<i64>()
                        .map_err(|_| Error::ParseInt(txt.to_owned()))?;

                    match parse_unit(unit.as_str()) {
                        "nanoseconds" => duration.nanoseconds += int,
                        "microseconds" => duration.microseconds += int,
                        "milliseconds" => duration.milliseconds += int,
                        "seconds" => duration.seconds += int,
                        "minutes" => duration.minutes += int,
                        "hours" => duration.hours += int,
                        "days" => duration.days += int,
                        "weeks" => duration.weeks += int,
                        "months" => duration.months += int,
                        "years" => duration.years += int,
                        s => return Err(Error::UnknownUnit(s.to_owned())),
                    }
                }
                (Some(int), Some(dec), Some(unit)) => {
                    let txt = int.as_str();
                    let int = txt
                        .parse::<i64>()
                        .map_err(|_| Error::ParseInt(txt.to_owned()))?;

                    let exp: u32 = dec
                        .as_str()
                        .len()
                        .try_into()
                        .expect("number of decimals too large");

                    let txt = dec.as_str();
                    let dec = txt
                        .parse::<i64>()
                        .map_err(|_| Error::ParseInt(txt.to_owned()))?;

                    // boosted_int is value * 10^exp * unit
                    let mut boosted_int = int * 10_i64.pow(exp) + dec;

                    // boosted_int is now value * 10^exp * nanoseconds
                    match parse_unit(unit.as_str()) {
                        "nanoseconds" => {}
                        "microseconds" => boosted_int *= 1_000_i64,
                        "milliseconds" => boosted_int *= 1_000_000_i64,
                        "seconds" => boosted_int *= 1_000_000_000_i64,
                        "minutes" => boosted_int *= 60_000_000_000_i64,
                        "hours" => boosted_int *= 3_600_000_000_000_i64,
                        "days" => boosted_int *= 86_400_000_000_000_i64,
                        "weeks" => boosted_int *= 604_800_000_000_000_i64,
                        "months" => boosted_int *= 2_629_746_000_000_000_i64,
                        "years" => boosted_int *= 31_556_952_000_000_000_i64,
                        s => return Err(Error::UnknownUnit(s.to_owned())),
                    }

                    // boosted_int is now value * nanoseconds (rounding down)
                    boosted_int /= 10_i64.pow(exp);
                    duration.nanoseconds += boosted_int;
                }
            }
        }
        duration.into_duration()
    } else {
        // Just a unit or nothing at all
        Err(Error::NoValueFound(input.to_owned()))
    }
}
