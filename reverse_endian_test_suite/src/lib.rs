extern crate reverse_endian;

#[cfg(test)]
mod tests {
    use reverse_endian::ReverseEndian;

    #[derive(Debug, PartialEq, ReverseEndian)]
    struct TestStruct {
        four_bytes: u32,
        two_bytes: u16,
    }

    #[test]
    fn test_struct_named() {
        let original = TestStruct {
            four_bytes: 0xd4c3b2a1,
            two_bytes: 0xa1b2,
        };

        let reversed = TestStruct {
            four_bytes: 0xa1b2c3d4,
            two_bytes: 0xb2a1,
        };
        assert_eq!(reversed, original.reverse_endian());
    }
}
