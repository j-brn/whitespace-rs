## 2.0.0

### breaking 

- low bytes are represented as zero width whitespace (U+200B)
- high bytes are represented as whitespace (U+0020)
- `encode()` and `decode()` are now in the crate root