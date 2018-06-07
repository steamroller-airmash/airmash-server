

#[cfg(test)]
mod test {
    use protocol::field::*;
    use protocol::serde_am::{Serializer, Deserializer};
    use std::vec::Vec;
    use protocol::from_bytes;

    fn text_to_vec(val: &str) -> Vec<u8> {
        let mut bytes = vec![ val.len() as u8 ];

        for byte in val.bytes() {
            bytes.push(byte);
        }

        bytes
    }

    #[test]
    fn text_serialize() {
        let mut ser = Serializer{ output: vec![] };
        let val = "test string";
        let bytes = text_to_vec(val);

        text::serialize(val, &mut ser).unwrap();

        for i in 0..bytes.len() {
            println!("{}", i);
            assert_eq!(ser.output[i], bytes[i]);
        }
    }

    #[test]
    fn text_deserialize() {
        let text = "hi there";
        let bytes = text_to_vec(text);

        let mut de = Deserializer{ bytes: &bytes[..] };
        let val = text::deserialize(&mut de).unwrap();

        assert_eq!(text, &val);
    }

    #[test]
    fn u16_little_endian() {
        let bytes = [ 0xAA, 0xBB ];
        let val = from_bytes::<u16>(&bytes).unwrap();

        assert_eq!(val, 0xBBAA);
    }
    
    #[test]
    fn u32_little_endian() {
        let bytes = [ 0xAA, 0xBB, 0xCC, 0xDD ];
        let val = from_bytes::<u32>(&bytes).unwrap();

        assert_eq!(val, 0xDDCCBBAA);
    }
    
    #[test]
    fn u64_little_endian() {
        let bytes = [ 0xAA, 0xBB, 0xCC, 0xDD, 0x11, 0x22, 0x33, 0x44 ];
        let val = from_bytes::<u64>(&bytes).unwrap();

        assert_eq!(val, 0x44332211DDCCBBAA);
    }

    #[test]
    fn healthnergy_is_u8() {
        let bytes = [ 0x52 ];
        let mut de = Deserializer::from_bytes(&bytes);
        let val = healthnergy::deserialize(&mut de);

        assert!(val.is_ok());
    }
}
