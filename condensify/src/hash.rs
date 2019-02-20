use crate::error::Error;
use crate::error::ErrorKind;
use std::str::FromStr;

const HASH_VALUES: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

pub struct Hash(String);

impl FromStr for Hash {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 64 {
            return Err(Error::from(ErrorKind::HashParseError));
        }

        let check_chars = s.chars().all(|c| {
            if !c.is_ascii() {
                return false;
            }

            let is_in_values = HASH_VALUES.iter().any(|v| c == v);

            if !is_in_values {
                return false;
            }

            true
        });

        Ok(Self(String::from(s)))
    }
}

pub enum HashType {
    File,
    Link,
}
