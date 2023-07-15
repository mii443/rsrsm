use bytes::{Buf, BufMut};
use encoding_rs::UTF_8;

#[derive(Debug, Clone)]
pub struct RconPacket {
    pub id: i32,
    pub packet_type: i32,
    pub body: String,
}

#[allow(dead_code)]
impl RconPacket {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut bytes = bytes;
        let id = bytes.get_i32_le();
        let packet_type = bytes.get_i32_le();
        let (string, _, _) = UTF_8.decode(&bytes[..bytes.len() - 2]);
        let body = string.to_string();

        Self {
            id,
            packet_type,
            body,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.put_i32_le(8 + self.body.as_bytes().len() as i32);
        buf.put_i32_le(self.id);
        buf.put_i32_le(self.packet_type);
        buf.put(self.body.as_bytes());
        buf.put_u8(0);

        buf
    }
}
