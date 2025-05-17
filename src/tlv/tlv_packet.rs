use crate::tlv::tlv_record::{tlv_to_var, var_to_tlv, TlvRecord};
use bincode::{error, Decode, Encode};

#[derive(Debug, Decode, Encode, PartialEq, Clone)]
pub struct TlvPacket {
    body: Vec<TlvRecord>,
}

impl TlvPacket {
    pub fn new() -> TlvPacket {
        TlvPacket { body: Vec::new() }
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<TlvPacket, error::DecodeError> {
        match bincode::decode_from_slice(bytes, bincode::config::standard()) {
            Ok((packet, _)) => Ok(packet),
            Err(e) => Err(e),
        }
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        bincode::encode_to_vec(self, bincode::config::standard()).unwrap()
    }
    pub fn push(&mut self, tag: u16, value: &impl bincode::Encode) {
        self.body.push(var_to_tlv(tag, value));
    }
    pub fn get<T: bincode::Decode<()>>(&self, tag: u16) -> Result<T, error::DecodeError> {
        for tlv in &self.body {
            if tlv.tag == tag {
                return match tlv_to_var::<T>(&tlv) {
                    Ok((res, _)) => Ok(res),
                    Err(e) => Err(e),
                };
            }
        }
        Err(error::DecodeError::Other("tag not found"))
    }
}
