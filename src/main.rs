use crate::tlv::tlv_packet::TlvPacket;
use bincode::{Decode, Encode};

mod tlv;

// 自定义结构体，只需添加以下derive即可。
#[derive(Encode, Decode, Debug, PartialEq)]
struct MyStruct {
    a: u32,
    b: Vec<u8>,
    c: String,
}

fn main() {
    let s = String::from("hello");
    let u32_num = 8_u32;
    let u8_num = 1_u8;
    let struct_input: MyStruct = MyStruct {
        a: 122,
        b: vec![1, 2, 3],
        c: "message".to_string(),
    };
    let mut tlv_packet: TlvPacket = TlvPacket::new();
    tlv_packet.push(1, &s);
    tlv_packet.push(3, &u32_num);
    tlv_packet.push(2, &u8_num);
    tlv_packet.push(10, &struct_input);
    match tlv_packet.get::<String>(1) {
        Ok(s_output) => assert_eq!(s_output, s),
        Err(e) => panic!("{}", e),
    }
    match tlv_packet.get::<u8>(2) {
        Ok(u_output) => assert_eq!(u_output, u8_num),
        Err(e) => panic!("{}", e),
    }
    match tlv_packet.get::<MyStruct>(10) {
        Ok(struct_output) => assert_eq!(struct_output, struct_input),
        Err(e) => panic!("{}", e),
    }
    println!("{:?}", tlv_packet);
    // tlv 序列化
    let tlv_byte = tlv_packet.as_bytes();
    println!("{:?}", tlv_byte);
    // tlv 反序列化
    let tlv_packet_trans = TlvPacket::from_bytes(&tlv_byte).unwrap();
    assert_eq!(tlv_packet_trans, tlv_packet);
}
