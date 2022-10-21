extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

#[derive(Debug, Clone)]
pub enum EncodeType {
    // int types
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    // uint types
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),

    // string
    Str(String),

    // bytes
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum EncoderError {
    // int types
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),

    // uint types
    Uint8(u8),
    Uint16(u16),
    Uint32(u32),
    Uint64(u64),
    Uint128(u128),

    // string
    Str(String),

    // bytes
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
pub enum EncodeOrder {
    Big,
    Little,
}

#[inline]
fn encode_i8(array: &mut [u8], value: &i8) -> Result<(), EncoderError> {
    array[0] = *value as u8;
    Ok(())
}

#[inline]
fn encode_i16(
    mut array: &mut [u8],
    value: &i16,
    encode_order: EncodeOrder,
) -> Result<(), EncoderError> {
    match encode_order {
        EncodeOrder::Big => array.write_i16::<BigEndian>(*value),
        EncodeOrder::Little => array.write_i16::<LittleEndian>(*value),
    }
    .map_or_else(|_| Ok(()), |_| Err(EncoderError::Int16(*value)))
}

#[inline]
fn encode_i32(
    mut array: &mut [u8],
    value: i32,
    encode_order: EncodeOrder,
) -> Result<(), EncoderError> {
    match encode_order {
        EncodeOrder::Big => array.write_i32::<BigEndian>(value),
        EncodeOrder::Little => array.write_i32::<LittleEndian>(value),
    }
    .map_or_else(|_| Ok(()), |_| Err(EncoderError::Int32(value)))
}

#[inline]
fn encode_i64(
    mut array: &mut [u8],
    value: &i64,
    encode_order: EncodeOrder,
) -> Result<(), EncoderError> {
    match encode_order {
        EncodeOrder::Big => array.write_i64::<BigEndian>(*value),
        EncodeOrder::Little => array.write_i64::<LittleEndian>(*value),
    }
    .map_or_else(|_| Ok(()), |_| Err(EncoderError::Int64(*value)))
}

#[inline]
fn encode_i128(
    mut array: &mut [u8],
    value: &i128,
    encode_order: EncodeOrder,
) -> Result<(), EncoderError> {
    match encode_order {
        EncodeOrder::Big => array.write_i128::<BigEndian>(*value),
        EncodeOrder::Little => array.write_i128::<LittleEndian>(*value),
    }
    .map_or_else(|_| Ok(()), |_| Err(EncoderError::Int128(*value)))
}

