extern crate byteorder;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

#[derive(Debug, Clone, PartialEq)]
/// `DecodedData` enum is used to wrap the decoded content into one of the supported data-type.
/// Example: `DecodedData::Str("hello")`, contains the string `hello` decoded back from the encoded bytes.
pub enum DecodedData {

    /// Int8 data representation
    Int8(i8),
    /// Int16 data representation
    Int16(i16),
    /// Int32 data representation
    Int32(i32),
    /// Int64 data representation
    Int64(i64),
    /// Int128 data representation
    Int128(i128),

    /// Uint8 data representation
    Uint8(u8),
    /// Uint16 data representation
    Uint16(u16),
    /// Uint32 data representation
    Uint32(u32),
    /// Uint64 data representation
    Uint64(u64),
    /// Uint128 data representation
    Uint128(u128),

    /// Str data representation
    Str(String),

    /// Bytes data representation
    Bytes(Vec<u8>),
}

#[derive(Debug, Clone)]
/// `DecodeType` enum can be used to tell the decoder who a sequence of bytes at a given offset must be decoded back.
/// Example: `DecodeType::Uint16` can be used to tell the decoder to interpret the next two bytes as `uint16`.
pub enum DecodeType {
    /// Int8 tells the decoder to decode next 1 byte as signed 8-bit integer
    Int8,
    /// Int16 tells the decoder to decode next 2 bytes as signed 16-bit integer
    Int16,
    /// Int32 tells the decoder to decode next 4 bytes as signed 32-bit integer
    Int32,
    /// Int64 tells the decoder to decode next 8 bytes as signed 64-bit integer
    Int64,
    /// Int128 tells the decoder to decode next 16 bytes as signed 128-bit integer
    Int128,

    /// Uint8 tells the decoder to decode next 1 byte as unsigned 8-bit integer
    Uint8,
    /// Uint16 tells the decoder to decode next 2 bytes as unsigned 16-bit integer
    Uint16,
    /// Uint32 tells the decoder to decode next 4 bytes as unsigned 32-bit integer
    Uint32,
    /// Uint64 tells the decoder to decode next 8 bytes as unsigned 64-bit integer
    Uint64,
    /// Uint128 tells the decoder to decode next 16 bytes as unsigned 128-bit integer
    Uint128,

    /// Str(usize) tells the decoded to decode next `x` bytes as a string
    Str(usize),

    /// Str(usize) tells the decoded to decode next `x` bytes as a byte-array
    Bytes(usize),
}

#[derive(Debug, Clone)]
/// `DecodeOrder` is used to specify how signed and unsigned integers encoded as bytes must be assumed w.r.t byte-order for decoding.
/// Example `DecodeOrder::Little` assumes all the bytes to be decoded are in little endian byte order.
pub enum DecodeOrder {
    /// Big endian byte ordering
    Big,
    /// Little endian byte ordering
    Little,
}

#[derive(Debug, Clone)]
/// `DecodeError` wraps the error that occurred during during
pub enum DecodeError {
    /// InvalidData represents an error that happens when given sequency of bytes at given offset cannot be decoded into the required data-type.
    /// Example `Err(DecodeErr::InvalidData(1))` says that the given bytes cannot be converted into the data-type specified at index 1.
    InvalidData(usize),
    /// IndexOutOfBounds occurs when offset > size of the byte array.
    IndexOutOfBounds,
}

#[inline]
fn decode_i16(mut array: &[u8], decode_order: DecodeOrder) -> Option<i16> {
    match decode_order {
        DecodeOrder::Big => array.read_i16::<BigEndian>(),
        DecodeOrder::Little => array.read_i16::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_i32(mut array: &[u8], decode_order: DecodeOrder) -> Option<i32> {
    match decode_order {
        DecodeOrder::Big => array.read_i32::<BigEndian>(),
        DecodeOrder::Little => array.read_i32::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_i64(mut array: &[u8], decode_order: DecodeOrder) -> Option<i64> {
    match decode_order {
        DecodeOrder::Big => array.read_i64::<BigEndian>(),
        DecodeOrder::Little => array.read_i64::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_i128(mut array: &[u8], decode_order: DecodeOrder) -> Option<i128> {
    match decode_order {
        DecodeOrder::Big => array.read_i128::<BigEndian>(),
        DecodeOrder::Little => array.read_i128::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_u16(mut array: &[u8], decode_order: DecodeOrder) -> Option<u16> {
    match decode_order {
        DecodeOrder::Big => array.read_u16::<BigEndian>(),
        DecodeOrder::Little => array.read_u16::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_u32(mut array: &[u8], decode_order: DecodeOrder) -> Option<u32> {
    match decode_order {
        DecodeOrder::Big => array.read_u32::<BigEndian>(),
        DecodeOrder::Little => array.read_u32::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_u64(mut array: &[u8], decode_order: DecodeOrder) -> Option<u64> {
    match decode_order {
        DecodeOrder::Big => array.read_u64::<BigEndian>(),
        DecodeOrder::Little => array.read_u64::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_u128(mut array: &[u8], decode_order: DecodeOrder) -> Option<u128> {
    match decode_order {
        DecodeOrder::Big => array.read_u128::<BigEndian>(),
        DecodeOrder::Little => array.read_u128::<LittleEndian>(),
    }
    .ok()
}

#[inline]
fn decode_string(array: &[u8]) -> Option<String> {
    String::from_utf8(array.to_vec()).ok()
}

pub fn decode_packed(
    types: &[DecodeType],
    buffer: &[u8],
    decode_order: DecodeOrder,
) -> Result<Vec<DecodedData>, DecodeError> {
    let mut decoded_data = vec![];
    let mut last_read = 0;

    for (idx, entry) in types.iter().enumerate() {
        let (result, size_offset) = match entry {
            DecodeType::Int8 => {
                if buffer.len() < last_read + 1 {
                    (Err(DecodeError::IndexOutOfBounds), 1)
                } else {
                    (Ok(DecodedData::Int8(buffer[last_read] as i8)), 1)
                }
            }

            DecodeType::Int16 => {
                if buffer.len() < last_read + 2 {
                    (Err(DecodeError::IndexOutOfBounds), 2)
                } else {
                    let decoded_result =
                        decode_i16(&buffer[last_read..last_read + 2], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Int16(decoded)),
                        ),
                        2,
                    )
                }
            }

            DecodeType::Int32 => {
                if buffer.len() < last_read + 4 {
                    (Err(DecodeError::IndexOutOfBounds), 4)
                } else {
                    let decoded_result =
                        decode_i32(&buffer[last_read..last_read + 4], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Int32(decoded)),
                        ),
                        4,
                    )
                }
            }

            DecodeType::Int64 => {
                if buffer.len() < last_read + 8 {
                    (Err(DecodeError::IndexOutOfBounds), 8)
                } else {
                    let decoded_result =
                        decode_i64(&buffer[last_read..last_read + 8], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Int64(decoded)),
                        ),
                        8,
                    )
                }
            }

            DecodeType::Int128 => {
                if buffer.len() < last_read + 16 {
                    (Err(DecodeError::IndexOutOfBounds), 16)
                } else {
                    let decoded_result =
                        decode_i128(&buffer[last_read..last_read + 16], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Int128(decoded)),
                        ),
                        16,
                    )
                }
            }

            DecodeType::Uint8 => {
                if buffer.len() < last_read + 1 {
                    (Err(DecodeError::IndexOutOfBounds), 1)
                } else {
                    (Ok(DecodedData::Uint8(buffer[last_read])), 1)
                }
            }

            DecodeType::Uint16 => {
                if buffer.len() < last_read + 2 {
                    (Err(DecodeError::IndexOutOfBounds), 2)
                } else {
                    let decoded_result =
                        decode_u16(&buffer[last_read..last_read + 2], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Uint16(decoded)),
                        ),
                        2,
                    )
                }
            }

            DecodeType::Uint32 => {
                if buffer.len() < last_read + 4 {
                    (Err(DecodeError::IndexOutOfBounds), 4)
                } else {
                    let decoded_result =
                        decode_u32(&buffer[last_read..last_read + 4], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Uint32(decoded)),
                        ),
                        4,
                    )
                }
            }

            DecodeType::Uint64 => {
                if buffer.len() < last_read + 8 {
                    (Err(DecodeError::IndexOutOfBounds), 8)
                } else {
                    let decoded_result =
                        decode_u64(&buffer[last_read..last_read + 8], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Uint64(decoded)),
                        ),
                        8,
                    )
                }
            }

            DecodeType::Uint128 => {
                if buffer.len() < last_read + 16 {
                    (Err(DecodeError::IndexOutOfBounds), 16)
                } else {
                    let decoded_result =
                        decode_u128(&buffer[last_read..last_read + 16], decode_order.clone());
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Uint128(decoded)),
                        ),
                        16,
                    )
                }
            }

            DecodeType::Str(size) => {
                if buffer.len() < last_read + *size {
                    (Err(DecodeError::IndexOutOfBounds), *size)
                } else {
                    let decoded_result = decode_string(&buffer[last_read..last_read + *size]);
                    (
                        decoded_result.map_or_else(
                            || Err(DecodeError::InvalidData(idx)),
                            |decoded| Ok(DecodedData::Str(decoded)),
                        ),
                        *size,
                    )
                }
            }

            DecodeType::Bytes(size) => {
                if buffer.len() < last_read + *size {
                    (Err(DecodeError::IndexOutOfBounds), *size)
                } else {
                    let vec_repr = buffer[last_read..last_read + *size].to_vec();
                    (Ok(DecodedData::Bytes(vec_repr)), *size)
                }
            }
        };

        if result.is_err() {
            return Err(result.unwrap_err());
        }

        last_read += size_offset;
        decoded_data.push(result.unwrap());
    }

    Ok(decoded_data)
}
