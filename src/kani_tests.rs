use crate::*;

fn assert_convert_roundtrip(s: &str, strkey: &Strkey) {
    let strkey_result = Strkey::from_string(s).unwrap();
    assert!(&strkey_result == strkey);
    let str_result = strkey.to_string();
    assert!(s == str_result)
}

#[kani::proof]
pub fn kani_test() {
    // Valid account.
    assert_convert_roundtrip(
        "GA3D5KRYM6CB7OWQ6TWYRR3Z4T7GNZLKERYNZGGA5SOAOPIFY6YQHES5",
        &Strkey::PublicKeyEd25519(StrkeyPublicKeyEd25519([
            0x36, 0x3e, 0xaa, 0x38, 0x67, 0x84, 0x1f, 0xba, 0xd0, 0xf4, 0xed, 0x88, 0xc7, 0x79,
            0xe4, 0xfe, 0x66, 0xe5, 0x6a, 0x24, 0x70, 0xdc, 0x98, 0xc0, 0xec, 0x9c, 0x07, 0x3d,
            0x05, 0xc7, 0xb1, 0x03,
        ])),
    );
}
