extern crate byteorder;

use byteorder::{BigEndian, LittleEndian, WriteBytesExt};

#[derive(Debug, Clone, PartialEq)]
/// `EncodeType` contains various data-types that are supported by packed-encoder.
/// This enum can be used to tell the encoder how a specific data needs to be encoded.
/// Example: `EncodeType::Int16(2422)` tells the encoder to encode the value `2422` as a 16-bit signed integer.
pub enum EncodeType {
    /// Int8 type is a 8-bit signed integer
    Int8(i8),
    /// Int8 type is a 16-bit signed integer
    Int16(i16),
    /// Int8 type is a 32-bit signed integer
    Int32(i32),
    /// Int8 type is a 64-bit signed integer
    Int64(i64),
    /// Int8 type is a 128-bit signed integer
    Int128(i128),

    /// Uint8 type is a 8-bit unsigned integer
    Uint8(u8),
    /// Uint16 type is a 16-bit unsigned integer
    Uint16(u16),
    /// Uint32 type is a 32-bit unsigned integer
    Uint32(u32),
    /// Uint64 type is a 64-bit unsigned integer
    Uint64(u64),
    /// Uint128 type is a 128-bit unsigned integer
    Uint128(u128),

    // Str type represents a finite string
    Str(String),

    // Bytes represents a sequence of finite bytes
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
/// `EncodeError` wraps the value that caused an error during encoding and returns it.
/// Example `Err(EncoderErr::Int16(2422))` is used to depict that value 2422 which was of type `int16`
/// caused an error during encoding.  
pub enum EncodeError {
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
/// `EncodeOrder` is used to specify the endian order signed and unsigned integers while encoding.
/// Example: `EncodeOrder::Big` is used to specify that all the integers should be ordered according to Big-Endian byte ordering.
pub enum EncodeOrder {
    Big,
    Little,
}

// signed integer

#[inline]
fn encode_i8(array: &mut [u8], value: &i8) -> Result<(), EncodeError> {
    array[0] = *value as u8;
    Ok(())
}

#[inline]
fn encode_i16(
    mut array: &mut [u8],
    value: &i16,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_i16::<BigEndian>(*value),
        EncodeOrder::Little => array.write_i16::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Int16(*value)), |_| Ok(()))
}

#[inline]
fn encode_i32(
    mut array: &mut [u8],
    value: &i32,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_i32::<BigEndian>(*value),
        EncodeOrder::Little => array.write_i32::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Int32(*value)), |_| Ok(()))
}

#[inline]
fn encode_i64(
    mut array: &mut [u8],
    value: &i64,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_i64::<BigEndian>(*value),
        EncodeOrder::Little => array.write_i64::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Int64(*value)), |_| Ok(()))
}

#[inline]
fn encode_i128(
    mut array: &mut [u8],
    value: &i128,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_i128::<BigEndian>(*value),
        EncodeOrder::Little => array.write_i128::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Int128(*value)), |_| Ok(()))
}

// unsigned integer

#[inline]
fn encode_u8(array: &mut [u8], value: &u8) -> Result<(), EncodeError> {
    array[0] = *value as u8;
    Ok(())
}

#[inline]
fn encode_u16(
    mut array: &mut [u8],
    value: &u16,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_u16::<BigEndian>(*value),
        EncodeOrder::Little => array.write_u16::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Uint16(*value)), |_| Ok(()))
}

#[inline]
fn encode_u32(
    mut array: &mut [u8],
    value: &u32,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_u32::<BigEndian>(*value),
        EncodeOrder::Little => array.write_u32::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Uint32(*value)), |_| Ok(()))
}

#[inline]
fn encode_u64(
    mut array: &mut [u8],
    value: &u64,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_u64::<BigEndian>(*value),
        EncodeOrder::Little => array.write_u64::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Uint64(*value)), |_| Ok(()))
}

#[inline]
fn encode_u128(
    mut array: &mut [u8],
    value: &u128,
    encode_order: EncodeOrder,
) -> Result<(), EncodeError> {
    match encode_order {
        EncodeOrder::Big => array.write_u128::<BigEndian>(*value),
        EncodeOrder::Little => array.write_u128::<LittleEndian>(*value),
    }
    .map_or_else(|_| Err(EncodeError::Uint128(*value)), |_| Ok(()))
}

#[inline]
fn encode_string(array: &mut [u8], value: &str) -> Result<(), EncodeError> {
    let u8_repr = value.as_bytes();
    array.clone_from_slice(u8_repr);
    Ok(())
}

/// `encode_packed` encodes an array of values of any `EncodeType` enum into a packed byte-array. Returns the byte vector representing
/// the packed byte-array or `EncodeErr` enum.
///
/// # Arguments
///
/// * `elements`: List of elements to encode, example: `&[ EncodeType::Int8(10), EncodeType::Str("hello".to_owned()) ]`
/// * `endian`: The byte-ordering to use while encoding
///
/// # Examples:
/// ```rust
/// extern crate packed_encoder;
///
/// use packed_encoder::encoder;
///
/// fn main() {
///     // list of values to encode
///     let to_encode = &[
///         encoder::EncodeType::Int128(-234984564544),
///         encoder::EncodeType::Str("this-is-good".to_owned()),
///         encoder::EncodeType::Uint64(837477899),
///         encoder::EncodeType::Int8(10),
///         encoder::EncodeType::Bytes(vec![0xff, 0xab, 0x12, 0x33]),
///    ];
///    // encode the values the result will be of type `Result<Vec<u8>, EncodeError>`
///    let encoded_result = encoder::encode_packed(to_encode, encoder::EncodeOrder::Little);
///    assert_eq!(encoded_result.is_ok(), true);
///    println!("bytes={:?}", encoded_result.unwrap());
/// }
///
/// ```
pub fn encode_packed(elements: &[EncodeType], endian: EncodeOrder) -> Result<Vec<u8>, EncodeError> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut last_read = 0;

    for symbol in elements {
        let (result, size_offset) = match symbol {
            EncodeType::Int8(value) => {
                buffer.extend_from_slice(&[0]);
                (encode_i8(&mut buffer[last_read..], value), 1)
            }
            EncodeType::Int16(value) => {
                buffer.extend_from_slice(&[0, 0]);
                (
                    encode_i16(&mut buffer[last_read..], value, endian.clone()),
                    2,
                )
            }
            EncodeType::Int32(value) => {
                buffer.extend_from_slice(&[0, 0, 0, 0]);
                (
                    encode_i32(&mut buffer[last_read..], value, endian.clone()),
                    4,
                )
            }
            EncodeType::Int64(value) => {
                buffer.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
                (
                    encode_i64(&mut buffer[last_read..], value, endian.clone()),
                    8,
                )
            }
            EncodeType::Int128(value) => {
                buffer.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
                (
                    encode_i128(&mut buffer[last_read..], value, endian.clone()),
                    16,
                )
            }
            EncodeType::Uint8(value) => {
                buffer.extend_from_slice(&[0]);
                (encode_u8(&mut buffer[last_read..], value), 1)
            }
            EncodeType::Uint16(value) => {
                buffer.extend_from_slice(&[0, 0]);
                (
                    encode_u16(&mut buffer[last_read..], value, endian.clone()),
                    2,
                )
            }
            EncodeType::Uint32(value) => {
                buffer.extend_from_slice(&[0, 0, 0, 0]);
                (
                    encode_u32(&mut buffer[last_read..], value, endian.clone()),
                    4,
                )
            }
            EncodeType::Uint64(value) => {
                buffer.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0]);
                (
                    encode_u64(&mut buffer[last_read..], value, endian.clone()),
                    8,
                )
            }
            EncodeType::Uint128(value) => {
                buffer.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
                (
                    encode_u128(&mut buffer[last_read..], value, endian.clone()),
                    16,
                )
            }
            EncodeType::Str(string) => {
                let mut temp = Vec::new();
                temp.resize(string.len(), 0);
                buffer.extend_from_slice(&temp);
                (
                    encode_string(&mut buffer[last_read..], &string),
                    string.len(),
                )
            }
            EncodeType::Bytes(bytes) => {
                buffer.extend_from_slice(&bytes);
                (Ok(()), bytes.len())
            }
        };

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        last_read += size_offset
    }

    Ok(buffer)
}
