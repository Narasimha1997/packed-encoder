pub mod decoder;
pub mod encoder;

#[test]
fn test_encode_numbers_little() {
    let to_encode_numbers = &[
        encoder::EncodeType::Int8(-10),
        encoder::EncodeType::Int16(-2212),
        encoder::EncodeType::Int32(-2123453),
        encoder::EncodeType::Int64(-23334232333),
        encoder::EncodeType::Int128(-456784564567848),
        encoder::EncodeType::Uint8(122),
        encoder::EncodeType::Int16(2212),
        encoder::EncodeType::Int32(2123453),
        encoder::EncodeType::Int64(23334232333),
        encoder::EncodeType::Uint128(456784564567848),
    ];

    let decoded_expected_numbers = &[
        decoder::DecodedData::Int8(-10),
        decoder::DecodedData::Int16(-2212),
        decoder::DecodedData::Int32(-2123453),
        decoder::DecodedData::Int64(-23334232333),
        decoder::DecodedData::Int128(-456784564567848),
        decoder::DecodedData::Uint8(122),
        decoder::DecodedData::Int16(2212),
        decoder::DecodedData::Int32(2123453),
        decoder::DecodedData::Int64(23334232333),
        decoder::DecodedData::Uint128(456784564567848),
    ];

    let decoded_expected_types = &[
        decoder::DecodeType::Int8,
        decoder::DecodeType::Int16,
        decoder::DecodeType::Int32,
        decoder::DecodeType::Int64,
        decoder::DecodeType::Int128,
        decoder::DecodeType::Uint8,
        decoder::DecodeType::Int16,
        decoder::DecodeType::Int32,
        decoder::DecodeType::Int64,
        decoder::DecodeType::Uint128,
    ];

    // test numbers
    let encoded_data_result =
        encoder::encode_packed(to_encode_numbers, encoder::EncodeOrder::Little);

    assert_eq!(encoded_data_result.is_ok(), true);

    // verify the length
    let encoded_data = encoded_data_result.unwrap();
    assert_eq!(encoded_data.len(), 31 * 2);

    // decode
    let decoded_data_result = decoder::decode_packed(
        decoded_expected_types,
        &encoded_data,
        decoder::DecodeOrder::Little,
    );

    assert_eq!(decoded_data_result.is_ok(), true);

    // verify the result
    let decoded_data = decoded_data_result.unwrap();
    for (idx, entry) in decoded_data.iter().enumerate() {
        assert_eq!(*entry == decoded_expected_numbers[idx], true);
    }
}

#[test]
fn test_encode_numbers_big() {
    let to_encode_numbers = &[
        encoder::EncodeType::Int8(-10),
        encoder::EncodeType::Int16(-2212),
        encoder::EncodeType::Int32(-2123453),
        encoder::EncodeType::Int64(-23334232333),
        encoder::EncodeType::Int128(-456784564567848),
        encoder::EncodeType::Uint8(122),
        encoder::EncodeType::Int16(2212),
        encoder::EncodeType::Int32(2123453),
        encoder::EncodeType::Int64(23334232333),
        encoder::EncodeType::Uint128(456784564567848),
    ];

    let decoded_expected_numbers = &[
        decoder::DecodedData::Int8(-10),
        decoder::DecodedData::Int16(-2212),
        decoder::DecodedData::Int32(-2123453),
        decoder::DecodedData::Int64(-23334232333),
        decoder::DecodedData::Int128(-456784564567848),
        decoder::DecodedData::Uint8(122),
        decoder::DecodedData::Int16(2212),
        decoder::DecodedData::Int32(2123453),
        decoder::DecodedData::Int64(23334232333),
        decoder::DecodedData::Uint128(456784564567848),
    ];

    let decoded_expected_types = &[
        decoder::DecodeType::Int8,
        decoder::DecodeType::Int16,
        decoder::DecodeType::Int32,
        decoder::DecodeType::Int64,
        decoder::DecodeType::Int128,
        decoder::DecodeType::Uint8,
        decoder::DecodeType::Int16,
        decoder::DecodeType::Int32,
        decoder::DecodeType::Int64,
        decoder::DecodeType::Uint128,
    ];

    // test numbers
    let encoded_data_result = encoder::encode_packed(to_encode_numbers, encoder::EncodeOrder::Big);

    assert_eq!(encoded_data_result.is_ok(), true);

    // verify the length
    let encoded_data = encoded_data_result.unwrap();
    assert_eq!(encoded_data.len(), 31 * 2);

    // decode
    let decoded_data_result = decoder::decode_packed(
        decoded_expected_types,
        &encoded_data,
        decoder::DecodeOrder::Big,
    );

    assert_eq!(decoded_data_result.is_ok(), true);

    // verify the result
    let decoded_data = decoded_data_result.unwrap();
    for (idx, entry) in decoded_data.iter().enumerate() {
        assert_eq!(*entry == decoded_expected_numbers[idx], true);
    }
}

#[test]
fn test_strings_and_bytes() {
    let to_encode_data = &[
        encoder::EncodeType::Str("hello".to_string()),
        encoder::EncodeType::Bytes(vec![0, 1, 2, 3, 4]),
        encoder::EncodeType::Bytes(vec![0, 10, 20, 30]),
        encoder::EncodeType::Str("world!!".to_string()),
    ];

    let expected_decoded_data = &[
        decoder::DecodedData::Str("hello".to_string()),
        decoder::DecodedData::Bytes(vec![0, 1, 2, 3, 4]),
        decoder::DecodedData::Bytes(vec![0, 10, 20, 30]),
        decoder::DecodedData::Str("world!!".to_string()),
    ];

    let decoded_data_types = &[
        decoder::DecodeType::Str(5),
        decoder::DecodeType::Bytes(5),
        decoder::DecodeType::Bytes(4),
        decoder::DecodeType::Str(7),
    ];

    let encoded_result = encoder::encode_packed(to_encode_data, encoder::EncodeOrder::Little);

    assert_eq!(encoded_result.is_ok(), true);
    let encoded_data = encoded_result.unwrap();

    assert_eq!(encoded_data.len(), 21);

    // decode data back
    let decoded_result = decoder::decode_packed(
        decoded_data_types,
        &encoded_data,
        decoder::DecodeOrder::Little,
    );
    assert_eq!(decoded_result.is_ok(), true);

    let decoded_data = decoded_result.unwrap();

    for (idx, element) in decoded_data.iter().enumerate() {
        assert_eq!(*element == expected_decoded_data[idx], true);
    }
}

#[test]
fn encode_decode_mixed() {
    let to_encode = &[
        encoder::EncodeType::Int128(-234984564544),
        encoder::EncodeType::Str("this-is-good".to_owned()),
        encoder::EncodeType::Uint64(837477899),
        encoder::EncodeType::Int8(10),
        encoder::EncodeType::Bytes(vec![0xff, 0xab, 0x12, 0x33]),
    ];

    let encoded_result = encoder::encode_packed(to_encode, encoder::EncodeOrder::Little);
    assert_eq!(encoded_result.is_ok(), true);

    let encoded_data = encoded_result.unwrap();

    let to_decode = &[
        decoder::DecodeType::Int128,
        decoder::DecodeType::Str(12),
        decoder::DecodeType::Uint64,
        decoder::DecodeType::Int8,
        decoder::DecodeType::Bytes(4),
    ];

    let expected_decoded_data = &[
        decoder::DecodedData::Int128(-234984564544),
        decoder::DecodedData::Str("this-is-good".to_owned()),
        decoder::DecodedData::Uint64(837477899),
        decoder::DecodedData::Int8(10),
        decoder::DecodedData::Bytes(vec![0xff, 0xab, 0x12, 0x33]),
    ];

    let decoded_result =
        decoder::decode_packed(to_decode, &encoded_data, decoder::DecodeOrder::Little);
    assert_eq!(decoded_result.is_ok(), true);

    let decoded_data = decoded_result.unwrap();

    for (idx, element) in decoded_data.iter().enumerate() {
        assert_eq!(*element == expected_decoded_data[idx], true);
    }
}

#[test]
fn test_struct_interoperability() {
    #[repr(C, packed)]
    struct Sample {
        pub x: u8,
        pub y: [u8; 5],
        pub z: i64,
        pub a: [char; 5],
    }

    let to_encode = &[
        encoder::EncodeType::Uint8(100),
        encoder::EncodeType::Bytes(vec![0, 1, 2, 3, 4]),
        encoder::EncodeType::Int64(256),
        encoder::EncodeType::Str("hello".to_owned()),
    ];

    let encode_result = encoder::encode_packed(to_encode, encoder::EncodeOrder::Little);
    assert_eq!(encode_result.is_ok(), true);

    let encoded_data = encode_result.unwrap();

    unsafe {
        let sample: *const Sample = encoded_data.as_ptr() as *const Sample;
        assert_eq!((*sample).x == 100, true);
        assert_eq!((*sample).y == [0, 1, 2, 3, 4], true);
        assert_eq!((*sample).z == 256, true);
    }
}
