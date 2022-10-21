pub mod decoder;
pub mod encoder;

#[test]
pub fn test_encode_numbers_little() {
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
        decoder::DecodedData::Uint128(456784564567848)
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
pub fn test_encode_numbers_big() {
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
        decoder::DecodedData::Uint128(456784564567848)
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
        encoder::encode_packed(to_encode_numbers, encoder::EncodeOrder::Big);

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