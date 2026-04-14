use crate::protocol::src::packet::{Packet, PacketType};
use crate::protocol::src::encoder::Encoder;
use crate::protocol::src::decoder::Decoder;

#[test]
fn test_packet_encode_decode() {
    let payload = b"hello world".to_vec();

    let packet = Packet::new(
        PacketType::Scene,
        123,
        456,
        payload.clone(),
    );

    let encoded = Encoder::encode(&packet);
    let decoded = Decoder::decode(&encoded).unwrap();

    assert_eq!(decoded.payload, payload);
    assert_eq!(decoded.source, 123);
    assert_eq!(decoded.destination, 456);
}
