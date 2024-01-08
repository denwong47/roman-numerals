//! # Roman Numerals Parser
//!
//! This crate provides a function to parse a string of roman numerals into a u64.
//!
//! Done as a 15 minutes challenge. Not guaranteed to work correctly.

/// Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
///
/// Negative numbers are not supported.
#[derive(Debug, PartialEq, Eq)]
pub enum RomanNumeral {
    I,
    V,
    X,
    L,
    C,
    D,
    M,
}

impl RomanNumeral {
    /// The value of a roman numeral is the number it represents.
    pub fn value(&self) -> u64 {
        match self {
            RomanNumeral::I => 1,
            RomanNumeral::V => 5,
            RomanNumeral::X => 10,
            RomanNumeral::L => 50,
            RomanNumeral::C => 100,
            RomanNumeral::D => 500,
            RomanNumeral::M => 1000,
        }
    }

    /// The base of a roman numeral is the power of 10 that it represents.
    pub fn base(&self) -> i8 {
        (self.value() as f64).log10().floor() as i8
    }

    /// Whether the roman numeral is a five, e.g. V, L, D.
    pub fn is_five(&self) -> bool {
        matches!(self, RomanNumeral::V | RomanNumeral::L | RomanNumeral::D)
    }
}

impl TryFrom<char> for RomanNumeral {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'I' => Ok(RomanNumeral::I),
            'V' => Ok(RomanNumeral::V),
            'X' => Ok(RomanNumeral::X),
            'L' => Ok(RomanNumeral::L),
            'C' => Ok(RomanNumeral::C),
            'D' => Ok(RomanNumeral::D),
            'M' => Ok(RomanNumeral::M),
            _ => Err(format!("{} is not a valid roman numeral", value)),
        }
    }
}

/// Parse a string of roman numerals into a u64.
pub fn parse_roman_numerals(value: &str) -> Result<u64, String> {
    value
        .chars()
        .try_fold(
            (0_u64, (i8::MAX, 0), Option::<RomanNumeral>::None),
            |(acc, (base, count), staged), c| {
                let current = RomanNumeral::try_from(c)?;

                match (acc, staged) {
                    // If no last numeral is recorded, then we just store the current numeral as staging.
                    (_, None) if base >= current.base() && count < 4 => {
                        Ok((acc, (current.base(), 1), Some(current)))
                    },
                    // If the current numeral is greater than the last numeral, e.g. "IX", "CD",
                    // then we subtract the last numeral from the accumulator and store the current numeral as staging.
                    // These should be the only numerals of that base in the sequence; e.g. "IV" is valid, but "IIV" or "VIV" is not.
                    // No more numerals of the same base can be added after this.
                    (acc, Some(staged)) if current.value() > staged.value() => {
                        if count > 1 {
                            return Err(format!("{current:?} cannot follow {staged:?} after {count}No. of base {base} numerals."));
                        }

                        let base_diff = current.base() - staged.base();
                        if !(0..=1).contains(&base_diff) {
                            Err(format!("{:?} cannot be subtracted from {:?}", current, staged))
                        } else {
                            Ok((acc + current.value() - staged.value(), (staged.base()-1, 0), None)) // if IV, then I is the last numeral, so we can't have another I
                        }
                    },
                    // If the current numeral is less than the last numeral, e.g. "VI", "DC",
                    // then we add the last numeral to the accumulator and store the current numeral as staging.
                    // Increment the counter if the current numeral is the same as the last numeral.
                    (acc, Some(staged)) if base >= current.base() && count < 4 => Ok((acc + staged.value(), {
                        if current == staged {
                            if current.is_five() && staged.is_five() {
                                return Err(format!("Consecutive {current:?}s are not allowed in {value:?}"));
                            }
                            (base, count+1)
                        } else {
                            (current.base(), 1)
                        }
                    }, Some(current))),
                    // Unparseable sequence, report error.
                    _ => Err(format!("Invalid roman numeral sequence: {value:?}")),
                }
            }
        )
        .map(
            // Add the last staged numeral to the accumulator.
            |(acc, _, last)| acc + last.map(|n| n.value()).unwrap_or(0)
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    macro_rules! expand_tests {
        ($((
            $name:ident,
            $value:expr,
            $expected:expr
        ),)*) => {
            $(
                #[test]
                fn $name() {
                    assert_eq!(parse_roman_numerals($value), $expected);
                }
            )*
        };
    }

    expand_tests!(
        (test_0, "", Ok(0)),
        (test_1, "I", Ok(1)),
        (test_4x1, "IIII", Ok(4)),
        (test_1_before_5, "IV", Ok(4)),
        (test_1_before_10, "IX", Ok(9)),
        (test_508, "DVIII", Ok(508)),
        (test_9, "VIV", Ok(9)),
        (test_1989, "MCMLXXXIX", Ok(1989)),
        (test_2019, "MMXIX", Ok(2019)),
        (
            test_9x2,
            "VIVV",
            Err("Invalid roman numeral sequence: \"VIVV\"".to_string())
        ),
        (
            test_2x5,
            "VV",
            Err("Consecutive Vs are not allowed in \"VV\"".to_string())
        ),
        (
            test_3x1_before_5,
            "IIIV",
            Err("V cannot follow I after 3No. of base 0 numerals.".to_string())
        ),
    );
}
