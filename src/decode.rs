use crate::{HIGH, LOW};
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq)]
pub struct DecodeError {
    pos: usize,
    char: char,
}

impl DecodeError {
    pub fn new(pos: usize, char: char) -> Self {
        DecodeError { pos, char }
    }
}

impl Display for DecodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid character '{}' at position {}",
            self.char, self.pos
        )
    }
}

impl std::error::Error for DecodeError {}

/// Decodes the given whitespace.
///
/// ## Errors
///
/// Returns `DecodeError` if the input string contains invalid characters.
///
/// ## Examples
///
/// ```rust
/// use whitespace::decode::decode;
///
/// let input = "    \t \t       \t ";
/// let expected = vec![10, 2];
///
/// assert_eq!(Ok(expected), decode(input))
/// ```
///
pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
    Ok(input
        .chars()
        .enumerate()
        .map(|(pos, c)| {
            Ok(match c {
                LOW => 0,
                HIGH => 1,
                _ => return Err(DecodeError::new(pos, c)),
            })
        })
        .collect::<Result<Vec<u8>, DecodeError>>()?
        .chunks_exact(8)
        .map(|chunk| {
            chunk
                .iter()
                .rev()
                .enumerate()
                .fold(0u8, |byte, (pos, bit)| byte | bit << pos)
        })
        .collect())
}

#[cfg(test)]
mod test {
    use crate::decode::{decode, DecodeError};

    #[test]
    pub fn test_decode_valid() {
        let input = "    \t \t       \t ";
        let expected = vec![10, 2];

        assert_eq!(Ok(expected), decode(input))
    }

    #[test]
    pub fn test_decode_invalid() {
        let input = "\t\t yeet";
        let expected = DecodeError::new(3, 'y');

        assert_eq!(Err(expected), decode(input))
    }

    #[test]
    pub fn test_format_decode_error() {
        let input = DecodeError::new(3, 'y');
        let expected = "Invalid character 'y' at position 3";

        assert_eq!(expected, &input.to_string())
    }
}
