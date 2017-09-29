use cbor::decoder::*;

// First test all the basic types
#[allow(dead_code)]
fn test_decoder(bytes: Vec<u8>, expected: CBORObject) {
    assert_eq!(decode(bytes).unwrap(), expected);
}

#[allow(dead_code)]
fn test_integer(bytes: Vec<u8>, expected: u64) {
    let decoded = decode(bytes).unwrap();
    for val in decoded.values {
        match val {
            CBORType::Integer(val) => assert_eq!(val, expected),
            _ => assert_eq!(1, 0),
        }
    }
}

#[allow(dead_code)]
fn test_integer_all(bytes: Vec<u8>, expected_value: u64) {
    let expected = CBORObject { values: vec![CBORType::Integer(expected_value)] };
    test_decoder(bytes.clone(), expected);
    test_integer(bytes, expected_value);
}

#[test]
fn test_integer_objects() {
    let bytes: Vec<u8> = vec![0x00];
    test_integer_all(bytes, 0);

    let bytes = vec![0x01];
    test_integer_all(bytes, 1);

    let bytes = vec![0x0A];
    test_integer_all(bytes, 10);

    let bytes = vec![0x17];
    test_integer_all(bytes, 23);

    let bytes = vec![0x18, 0x18];
    test_integer_all(bytes, 24);

    let bytes = vec![0x18, 0x19];
    test_integer_all(bytes, 25);

    let bytes = vec![0x18, 0x64];
    test_integer_all(bytes, 100);

    let bytes = vec![0x19, 0x03, 0xe8];
    test_integer_all(bytes, 1000);

    let bytes = vec![0x1a, 0x00, 0x0f, 0x42, 0x40];
    test_integer_all(bytes, 1000000);

    let bytes = vec![0x1b, 0x00, 0x00, 0x00, 0xe8, 0xd4, 0xa5, 0x10, 0x00];
    test_integer_all(bytes, 1000000000000);

    let bytes = vec![0x1b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];
    test_integer_all(bytes, 18446744073709551615);
}

#[allow(dead_code)]
fn test_tag(bytes: Vec<u8>, expected: u64) {
    let decoded = decode(bytes).unwrap();
    for val in decoded.values {
        match val {
            CBORType::Tag(val) => assert_eq!(val, expected),
            _ => assert_eq!(1, 0),
        }
    }
}

#[test]
fn test_tagged_objects() {
    let bytes: Vec<u8> = vec![0xD8, 0x62];
    let expected_value: u64 = 0x62;
    let expected = CBORObject { values: vec![CBORType::Tag(expected_value)] };
    test_decoder(bytes.clone(), expected);
    test_tag(bytes, expected_value);
}

#[test]
fn test_arrays() {
    // []
    let bytes: Vec<u8> = vec![0x80];
    let expected = CBORObject { values: vec![CBORType::Array(vec![])] };
    test_decoder(bytes, expected);

    // [1, 2, 3]
    let bytes: Vec<u8> = vec![0x83, 0x01, 0x02, 0x03];
    let tmp = vec![
        CBORType::Integer(1),
        CBORType::Integer(2),
        CBORType::Integer(3),
    ];
    let expected = CBORObject { values: vec![CBORType::Array(tmp)] };
    test_decoder(bytes, expected);

    // [1, [2, 3], [4, 5]]
    let bytes: Vec<u8> = vec![0x83, 0x01, 0x82, 0x02, 0x03, 0x82, 0x04, 0x05];
    let tmp1 = vec![CBORType::Integer(2), CBORType::Integer(3)];
    let tmp2 = vec![CBORType::Integer(4), CBORType::Integer(5)];
    let tmp = vec![
        CBORType::Integer(1),
        CBORType::Array(tmp1),
        CBORType::Array(tmp2),
    ];
    let expected = CBORObject { values: vec![CBORType::Array(tmp)] };
    test_decoder(bytes, expected);

    // [1, [[[[1]]]], [1]]
    let bytes: Vec<u8> = vec![0x83, 0x01, 0x81, 0x81, 0x81, 0x81, 0x01, 0x81, 0x02];
    let tmp = vec![
        CBORType::Integer(1),
        CBORType::Array(vec![
            CBORType::Array(vec![
                CBORType::Array(vec![
                    CBORType::Array(vec![
                        CBORType::Integer(1)])])])]),
        CBORType::Array(vec![CBORType::Integer(2)]),
    ];
    let expected = CBORObject { values: vec![CBORType::Array(tmp)] };
    test_decoder(bytes, expected);

    let bytes: Vec<u8> = vec![0x98, 0x1A, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06,
                              0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
                              0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16,
                              0x17, 0x18, 0x18, 0x18, 0x19, 0x82, 0x81, 0x81,
                              0x81, 0x05, 0x81, 0x1A, 0x49, 0x96, 0x02, 0xD2];
    // [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    //  21, 22, 23, 24, 25, [[[[5]]], [1234567890]]]
    let tmp = vec![
        CBORType::Integer(1),
        CBORType::Integer(2),
        CBORType::Integer(3),
        CBORType::Integer(4),
        CBORType::Integer(5),
        CBORType::Integer(6),
        CBORType::Integer(7),
        CBORType::Integer(8),
        CBORType::Integer(9),
        CBORType::Integer(10),
        CBORType::Integer(11),
        CBORType::Integer(12),
        CBORType::Integer(13),
        CBORType::Integer(14),
        CBORType::Integer(15),
        CBORType::Integer(16),
        CBORType::Integer(17),
        CBORType::Integer(18),
        CBORType::Integer(19),
        CBORType::Integer(20),
        CBORType::Integer(21),
        CBORType::Integer(22),
        CBORType::Integer(23),
        CBORType::Integer(24),
        CBORType::Integer(25),
        CBORType::Array(vec![
            CBORType::Array(vec![
                CBORType::Array(vec![
                    CBORType::Array(vec![
                        CBORType::Integer(5)])])]),
            CBORType::Array(vec![CBORType::Integer(1234567890)])])
    ];
    let expected = CBORObject { values: vec![CBORType::Array(tmp)] };
    test_decoder(bytes, expected);
}

#[test]
fn test_signed_integer() {
    let bytes: Vec<u8> = vec![0x20];
    let expected = CBORObject { values: vec![CBORType::SignedInteger(-1)] };
    test_decoder(bytes, expected);

    let bytes = vec![0x29];
    let expected = CBORObject { values: vec![CBORType::SignedInteger(-10)] };
    test_decoder(bytes, expected);

    let bytes = vec![0x38, 0x63];
    let expected = CBORObject { values: vec![CBORType::SignedInteger(-100)] };
    test_decoder(bytes, expected);

    let bytes = vec![0x39, 0x03, 0xe7];
    let expected = CBORObject { values: vec![CBORType::SignedInteger(-1000)] };
    test_decoder(bytes, expected);

    let bytes = vec![0x39, 0x27, 0x0F];
    let expected = CBORObject { values: vec![CBORType::SignedInteger(-10000)] };
    test_decoder(bytes, expected);

    let bytes = vec![0x3A, 0x00, 0x01, 0x86, 0x9F];
    let expected = CBORObject { values: vec![CBORType::SignedInteger(-100000)] };
    test_decoder(bytes, expected);

    let bytes = vec![0x3B, 0x00, 0x00, 0x00, 0xE8, 0xD4, 0xA5, 0x0F, 0xFF];
    let expected = CBORObject { values: vec![CBORType::SignedInteger(-1000000000000)] };
    test_decoder(bytes, expected);
}

#[test]
fn test_byte_strings() {
    let bytes: Vec<u8> = vec![0x40];
    let expected = CBORObject { values: vec![CBORType::Bytes(vec![])] };
    test_decoder(bytes, expected);

    // 01020304
    let bytes: Vec<u8> = vec![0x44, 0x01, 0x02, 0x03, 0x04];
    let expected = CBORObject { values: vec![
        CBORType::Bytes(vec![0x01, 0x02, 0x03, 0x04])] };
    test_decoder(bytes, expected);

    // 0102030405060708090A0B0C0D0E0F10203040506070
    let bytes: Vec<u8> = vec![0x56, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                              0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                              0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70];
    let expected = CBORObject { values: vec![
        CBORType::Bytes(vec![0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
                             0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
                             0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70])] };
    test_decoder(bytes, expected);

    let bytes: Vec<u8> = vec![0x59, 0x01, 0x0E, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                              0xFF];
    let expected = CBORObject { values: vec![
        CBORType::Bytes(vec![0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF,
                             0xFF])] };
    test_decoder(bytes, expected);
}

#[test]
fn test_maps() {
    // {}
    let bytes: Vec<u8> = vec![0xa0];
    let expected = CBORObject { values: vec![CBORType::Map(vec![])] };
    test_decoder(bytes, expected);

    // {1: 2, 3: 4}
    let bytes: Vec<u8> = vec![0xa2, 0x01, 0x02, 0x03, 0x04];
    let expected = CBORObject { values: vec![
        CBORType::Map(vec![
            CBORMap{key: CBORType::Integer(1), value: CBORType::Integer(2)},
            CBORMap{key: CBORType::Integer(3), value: CBORType::Integer(4)}])] };
    test_decoder(bytes, expected);

    // {"a": 1, "b": [2, 3]}
    // let bytes: Vec<u8> = vec![0xa2, 0x61, 0x61, 0x01, 0x61, 0x62, 0x82, 0x02, 0x03];
    // let expected = CBORObject { values: vec![
    //     CBORType::Map(vec![
    //         CBORMap{key: CBORType::Integer(1), value: CBORType::Integer(2)},
    //         CBORMap{key: CBORType::Integer(3), value: CBORType::Integer(4)}])] };
    // test_decoder(bytes, expected);

    // let bytes: Vec<u8> = vec![0x82, 0x61, 0x61, 0xa1, 0x61, 0x62, 0x61, 0x63];
    // test_decoder(bytes, "[a, {b: c}]");

    // let bytes: Vec<u8> = vec![0xa5, 0x61, 0x61, 0x61, 0x41, 0x61, 0x62, 0x61,
    //                           0x42, 0x61, 0x63, 0x61, 0x43, 0x61, 0x64, 0x61,
    //                           0x44, 0x61, 0x65, 0x61, 0x45];
    // test_decoder(bytes, "{a: A, b: B, c: C, d: D, e: E}");
}
