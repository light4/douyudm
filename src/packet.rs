pub const HEADER_LEN_SIZE: usize = 4;
pub const HEADER_LEN_TYPECODE: usize = 2;
pub const HEADER_LEN_ENCRYPT: usize = 1;
pub const HEADER_LEN_PLACEHOLDER: usize = 1;
pub const HEADER_LEN_TOTAL: usize =
    HEADER_LEN_SIZE * 2 + HEADER_LEN_TYPECODE + HEADER_LEN_ENCRYPT + HEADER_LEN_PLACEHOLDER;

pub fn encode(data: &str) -> Vec<u8> {
    let mut msg = data.as_bytes().to_vec();
    msg.push(0);

    let msg_length = (msg.len() + HEADER_LEN_SIZE * 2) as u32;

    let mut result = Vec::with_capacity(HEADER_LEN_TOTAL + msg.len());
    result.extend(msg_length.to_le_bytes());
    result.extend(msg_length.to_le_bytes());
    result.extend(689_u16.to_le_bytes());
    result.extend(0_u16.to_le_bytes());
    result.extend(msg);

    result
}

pub fn decode(data: Vec<u8>) -> String {
    let length_data: [u8; 4] = data[..4].try_into().unwrap();
    let msg_length = u32::from_le_bytes(length_data) as usize;
    String::from_utf8_lossy(&data[HEADER_LEN_TOTAL..(data.len() - 1)]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        assert_eq!(
            encode("type@=test@=hello/roomid@=9999/"),
            vec![
                0x28, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0xb1, 0x02, 0x00, 0x00, 0x74, 0x79,
                0x70, 0x65, 0x40, 0x3d, 0x74, 0x65, 0x73, 0x74, 0x40, 0x3d, 0x68, 0x65, 0x6c, 0x6c,
                0x6f, 0x2f, 0x72, 0x6f, 0x6f, 0x6d, 0x69, 0x64, 0x40, 0x3d, 0x39, 0x39, 0x39, 0x39,
                0x2f, 0x00
            ]
        );
    }

    #[test]
    fn test_decode() {
        assert_eq!(
            decode(vec![
                0x28, 0x00, 0x00, 0x00, 0x28, 0x00, 0x00, 0x00, 0xb1, 0x02, 0x00, 0x00, 0x74, 0x79,
                0x70, 0x65, 0x40, 0x3d, 0x74, 0x65, 0x73, 0x74, 0x40, 0x3d, 0x68, 0x65, 0x6c, 0x6c,
                0x6f, 0x2f, 0x72, 0x6f, 0x6f, 0x6d, 0x69, 0x64, 0x40, 0x3d, 0x39, 0x39, 0x39, 0x39,
                0x2f, 0x00
            ]),
            "type@=test@=hello/roomid@=9999/",
        );
    }
}
