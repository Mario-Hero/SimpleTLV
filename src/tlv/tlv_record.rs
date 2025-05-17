use bincode::{error, Decode, Encode};

#[derive(Debug, Decode, Encode, PartialEq, Clone)]
pub struct TlvRecord {
    pub tag: u16, // 标签，通常是enum类型
    pub len: u16, // 数据长度。使用bincode编解码，好像不需要len这个值了。
    pub value: Vec<u8>,
}

// 将变量转换成TLV格式
pub fn var_to_tlv(tag: u16, value: &impl bincode::Encode) -> TlvRecord {
    let value_bytes = bincode::encode_to_vec(value, bincode::config::standard()).unwrap();
    let len = value_bytes.len() as u16;
    TlvRecord {
        tag,
        len,
        value: value_bytes,
    }
}

// 从TLV格式还原出变量
pub fn tlv_to_var<T: bincode::Decode<()>>(
    tlv: &TlvRecord,
) -> Result<(T, usize), error::DecodeError> {
    bincode::decode_from_slice(&*tlv.value, bincode::config::standard())
}
