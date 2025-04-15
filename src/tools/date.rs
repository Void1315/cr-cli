
use chrono::{DateTime, Local};
use std::time::SystemTime;
use lettre::message::header::{Header, HeaderName, HeaderValue};
use std::error::Error as StdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Date(DateTime<Local>);

impl Date {
    /// Build a `Date` from [`SystemTime`]
    pub fn new(st: DateTime<Local>) -> Self {
        Self(st.into())
    }

    /// Get the current date
    ///
    /// Shortcut for `Date::new(SystemTime::now())`
    pub fn now() -> Self {
        Self::new(Local::now())
    }
}

impl Header for Date {
    fn name() -> HeaderName {
        HeaderName::new_from_ascii_str("Date")
    }

    fn parse(s: &str) -> Result<Self, Box<dyn StdError + Send + Sync>> {
        let s = String::from(s);
        Ok(Self(s.parse::<DateTime<Local>>()?))
    }

    fn display(&self) -> HeaderValue {
        let val = self.0.format("%a, %d %b %Y %H:%M:%S %z").to_string();
        HeaderValue::dangerous_new_pre_encoded(Self::name(), val.clone(), val)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use lettre::message::header::{HeaderName, HeaderValue, Headers};
    #[test]
    fn format_date() {
        let mut headers = Headers::new();
        headers.set(Date::now());
        dbg!(headers.to_string());
    }
}
