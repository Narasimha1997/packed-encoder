# packed-encoder
A tiny rust crate that can be used to encode data of different types into a packed byte array which can be passed over network, system calls or FFI. The byte array produced by this library can be casted into C-Like packed structs.

## Installation
The crate is published on crates.io, check out:

1. [crates.io crate](https://crates.io/crates/packed-encoder)
2. [docs.rs documentation](https://docs.rs/packed-encoder/0.1.1/packed_encoder/)

To add `packed-encoder` to crates.io, add the following entry into `dependencies` section of `Cargo.toml` file:
```
[dependencies]
packed-encoder = "0.1.1"
```

## Encoding
You can pass set of values that needs to be encoded into a byte array. As shown below:
```rust
extern crate packed_encoder;

use packed_encoder::encoder;

fn main() {
    // list of values to encode
    let to_encode = &[
        encoder::EncodeType::Int128(-234984564544),
        encoder::EncodeType::Str("this-is-good".to_owned()),
        encoder::EncodeType::Uint64(837477899),
        encoder::EncodeType::Int8(10),
        encoder::EncodeType::Bytes(vec![0xff, 0xab, 0x12, 0x33]),
    ];
    // encode the values the result will be of type `Result<Vec<u8>, EncodeError>`
    let encoded_result = encoder::encode_packed(to_encode, encoder::EncodeOrder::Little);
    assert_eq!(encoded_result.is_ok(), true);
    println!("bytes={:?}", encoded_result.unwrap());
}
```

## Decoding
You can pass the byte array into the decoder and pass the required data-types to obtain the decoded data, as shown below:
```rust
 extern crate packed_encoder;
 use packed_encoder::decoder;

 fn main() {

     // byte data to decode
     let bytes = vec![192, 24, 212, 73, 201, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 116, 104, 105, 115, 45, 105, 115, 45, 103, 111, 111, 100, 11, 230, 234, 49, 0, 0, 0, 0, 10, 255, 171, 18, 51];
     // required types to decode from the given byte array
     let required_types = &[
            decoder::DecodeType::Int128,
            decoder::DecodeType::Str(12),
            decoder::DecodeType::Uint64,
            decoder::DecodeType::Int8,
            decoder::DecodeType::Bytes(4),
        ];
     
     // decode
     let result = decoder::decode_packed(required_types, &bytes, decoder::DecodeOrder::Little);
     assert_eq!(result.is_ok(), true);

     // check values
     let decoded_data = result.unwrap();
     match &decoded_data[1] {
         decoder::DecodedData::Str(content) => {println!("decoded string at position 1: {}", content)},
         _ => {}
     }

 }
```

### Struct interoperability
The byte array obtained from decoder can be casted into any packed struct. Look at the example below.
```rust
extern crate packed_encoder;

use packed_encoder::encoder;

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
```

### Running tests and docs
To validate the functionalities of this crate, run tests using `cargo`:
```
cargo test
```

To view the docs of this crate locally:
```
cargo doc --open
```

### Contributing
Please feel free to raise issues, make PRs and suggest improvements, alternative libraries.