use crate::{HIGH, LOW};

/// Encodes the given data into whitespace.
///
/// ## Examples
///
/// ```rust
/// use whitespace::encode::encode;
///
/// let input = vec![10, 2]; // arbitrary data
/// let expected = "    \t \t       \t ";
///
/// assert_eq!(expected, &encode(input))
/// ```
pub fn encode<I: IntoIterator<Item = u8>>(input: I) -> String {
    input
        .into_iter()
        .map(|byte| {
            (0..8).rev().fold(String::new(), |mut res, pos| {
                res.push(match byte & (1u8 << pos) {
                    0 => LOW,
                    _ => HIGH,
                });

                res
            })
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::encode::encode;

    #[test]
    pub fn test_encode() {
        let input = vec![10, 2];
        let expected = "    \t \t       \t ";

        assert_eq!(expected, &encode(input))
    }
}
