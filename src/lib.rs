pub fn hex_to_base64(input: &[u8]) -> &[u8] {
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_base64() {
        let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d
".as_bytes();
        let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".as_bytes();
        assert_eq!(hex_to_base64(input), output)
    }
}
