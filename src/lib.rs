use thiserror::Error;

const HIGH: char = '\u{0020}';
const LOW: char = '\u{200b}';

/// Encodes the given binary data as whitespace.
///
/// Each byte is with one character for each bit.
///  - \u{0020} (whitespace) represents a high bit
///  - \u{200b} (zero width whitespace) represents a low bit
///
/// ## Examples
///
/// ```rust
/// use whitespace::encode;
///
/// let data = vec![10, 10];
/// let encoded = encode(&data);
///
/// assert_eq!(
///     "\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}",
///     encoded
/// );
/// ```
pub fn encode(data: &[u8]) -> String {
    data.iter()
        .copied()
        .fold(String::with_capacity(data.len() * 8), |buffer, byte| {
            (0..8).rev().fold(buffer, |mut buffer, bit| {
                buffer.push(match byte & (1u8 << bit) {
                    0 => LOW,
                    _ => HIGH,
                });

                buffer
            })
        })
}

/// Decodes the given whitespace string back into binary.
///
/// - \u{0020} (whitespace) represents a high bit
/// - \u{200b} (zero width whitespace) represents a low bit
///
/// ## Errors
///
/// The function returns a `DecodeError` under the following circumstances:
///
/// - `DecodeError::InvalidLength` if the input length is not divisible through 8
/// - `DecodeError::InvalidCharacter` if the input contains invalid characters
///
/// ## Examples
///
/// ```rust
/// use whitespace::decode;
///
/// let encoded = "\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}";
/// let decoded = decode(encoded);
///
/// assert_eq!(Ok(vec![10, 10]), decoded);
/// ```
pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
    let bits = input
        .chars()
        .enumerate()
        .map(|(pos, char)| match char {
            LOW => Ok(0),
            HIGH => Ok(1),
            _ => Err(DecodeError::InvalidCharacter { pos, char }),
        })
        .collect::<Result<Vec<u8>, DecodeError>>()?;

    if bits.len() % 8 != 0 {
        return Err(DecodeError::InvalidLength { length: bits.len() });
    }

    let bytes =
        bits.chunks_exact(8)
            .fold(Vec::with_capacity(input.len() / 8), |mut buffer, chunk| {
                buffer.push(
                    chunk
                        .iter()
                        .rev()
                        .enumerate()
                        .fold(0u8, |byte, (pos, bit)| byte | bit << pos),
                );

                buffer
            });

    Ok(bytes)
}

#[derive(Error, Debug, Eq, PartialEq)]
pub enum DecodeError {
    #[error("Invalid input length {length}. Must be divisible through 8")]
    InvalidLength { length: usize },
    #[error("Invalid character {char} at position {pos}")]
    InvalidCharacter { pos: usize, char: char },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let data = vec![10, 10];
        let encoded = encode(&data);

        assert_eq!(
            "\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}",
            encoded
        );
    }

    #[test]
    fn test_successful_decode() {
        let encoded = "\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}";
        let decoded = decode(encoded);

        assert_eq!(Ok(vec![10, 10]), decoded);
    }

    #[test]
    fn test_decode_with_invalid_input_length() {
        let encoded = "\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}";
        let decoded = decode(encoded);

        assert_eq!(Err(DecodeError::InvalidLength { length: 15 }), decoded);
    }

    #[test]
    fn test_decode_with_invalid_character() {
        let encoded = "\u{200b}?\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}\u{200b}\u{200b}\u{200b}\u{200b}\u{200b}\u{0020}\u{200b}\u{0020}";
        let decoded = decode(encoded);

        assert_eq!(
            Err(DecodeError::InvalidCharacter { pos: 1, char: '?' }),
            decoded
        );
    }
}
