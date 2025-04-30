//! Defines the serialization and deserialization format used throughout Ethereum.

use crate::ethereum::{cancun::fork_types::{Address, VersionedHash}, ethereum_types::{bytes::{Bytes, Bytes32}, numeric::{Uint, U256, U64}}};

use super::exceptions::RLPException;


pub trait Extended {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException>;
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException>;
}

//
// RLP Encode
//

/// Encodes `raw_data` into a sequence of bytes using RLP.
///
pub fn encode<T : Extended>(t: &T) -> Result<Bytes, RLPException> {
    let mut res = Bytes::default();
    t.encode(&mut res)?;
    Ok(res)
}

impl Extended for bool {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
    // fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
    //     if *self {
    //         Ok(encode_bytes(b"\x01"))
    //     } else {
    //         Ok(encode_bytes(b""))
    //     }
    // }
    // fn decode<'a, 'b>(buffer: &'a mut &'b [u8]) -> Result<Self, RLPException>;

    // fn encode(&self) -> Result<Bytes, RLPException> {
    //     if *self {
    //         Ok(encode_bytes(b"\x01"))
    //     } else {
    //         Ok(encode_bytes(b""))
    //     }
    // }
    
    // fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
    //     todo!()
    // }
}

impl Extended for Uint {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

impl Extended for U256 {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

impl Extended for Option<Address> {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

impl Extended for Address {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

impl Extended for Bytes {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

impl Extended for U64 {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}


impl Extended for Vec<(Address, Vec<Bytes32>)> {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

impl Extended for Vec<VersionedHash> {
    fn encode<'a, 'b>(&self, buffer: &'a mut Bytes) -> Result<(), RLPException> {
        todo!()
    }
    
    fn decode<'a, 'b>(&mut self, buffer: &'a mut &'b [u8]) -> Result<(), RLPException> {
        todo!()
    }
}

// def encode(raw_data: Extended) -> Bytes:
//     """
//     Encodes `raw_data` into a sequence of bytes using RLP.
//     """
//     if isinstance(raw_data, Sequence):
//         if isinstance(raw_data, (bytearray, bytes)):
//             return encode_bytes(raw_data)
//         elif isinstance(raw_data, str):
//             return encode_bytes(raw_data.encode())
//         else:
//             return encode_sequence(raw_data)
//     elif isinstance(raw_data, (Uint, FixedUnsigned)):
//         return encode(raw_data.to_be_bytes())
//     elif isinstance(raw_data, bool):
//         if raw_data:
//             return encode_bytes(b"\x01")
//         else:
//             return encode_bytes(b"")
//     elif is_dataclass(raw_data):
//         return encode(astuple(raw_data))
//     else:
//         raise EncodingError(
//             "RLP Encoding of type {} is not supported".format(type(raw_data))
//         )


/// Encodes `raw_bytes`, a sequence of bytes, using RLP.
pub fn encode_bytes(dest: &mut Bytes, raw_bytes: &[u8]) {
    let len_raw_data = raw_bytes.len();

    if len_raw_data == 1 && raw_bytes[0] < 0x80 {
        dest.push(raw_bytes[0]);
    } else if len_raw_data < 0x38 {
        dest.push(0x80 + (len_raw_data as u8));
        dest.extend(raw_bytes.iter().copied());
    } else {
        // length of raw data represented as big endian bytes
        let len_raw_data_as_be = len_raw_data.to_be_bytes();
        let lz = len_raw_data_as_be.iter()
            .position(|b| *b != 0)
            .unwrap(); // len_raw_data not zero.
        let len_raw_data_as_be = &len_raw_data_as_be[lz..];
        dest.push(0xB7 + len_raw_data_as_be.len() as u8);
        dest.extend(len_raw_data_as_be.iter().copied());
        dest.extend(raw_bytes.iter().copied());
    }
}


/// Encodes a list of RLP encodable objects (`raw_sequence`) using RLP.
pub fn encode_sequence(dest: &mut Bytes, raw_sequence: &[&dyn Extended]) -> Result<Bytes, RLPException> {
    let joined_encodings = join_encodings(raw_sequence)?;
    let len_joined_encodings = joined_encodings.len();

    if len_joined_encodings < 0x38 {
        Ok(Bytes([&[0xC0 + len_joined_encodings as u8], &*joined_encodings].concat()))
    } else {
        let len_joined_encodings_as_be = len_joined_encodings.to_be_bytes();
        let lz = len_joined_encodings_as_be.iter()
            .position(|b| *b != 0)
            .unwrap(); // len_joined_encodings not zero.
        let len_joined_encodings_as_be = &len_joined_encodings_as_be[lz..];
        Ok(Bytes([
            &[0xF7 + len_joined_encodings_as_be.len() as u8],
            len_joined_encodings_as_be,
            &joined_encodings
        ].concat()))
    }
}

/// Obtain concatenation of rlp encoding for each item in the sequence
/// raw_sequence.
fn join_encodings(raw_sequence: &[&dyn Extended]) -> Result<Bytes, RLPException> {
    let mut res = Bytes::default();
    for e in raw_sequence {
        e.encode(&mut res)?;
    }
    Ok(res)
}


//
// RLP Decode
//


/// Decodes an integer, byte sequence, or list of RLP encodable objects
/// from the byte sequence `encoded_data`, using RLP.
pub fn decode_to<T : Extended  + Default>(mut encoded_data: &[u8]) -> Result<T, RLPException> {
    if encoded_data.len() <= 0 {
        return Err(RLPException::DecodingError("Cannot decode empty bytestring"));
    }
    let mut res = T::default();
    T::decode(&mut res, &mut encoded_data)?;
    Ok(res)

    // if encoded_data[0] <= 0xBF {
    //     // This means that the raw data is of type bytes
    //     decode_to_bytes(encoded_data)
    // } else {
    //     // This means that the raw data is of type sequence
    //     decode_to_sequence(encoded_data)
    // }
}



// def _deserialize_to_dataclass(cls: Type[U], decoded: Simple) -> U:
//     assert is_dataclass(cls)
//     hints = get_type_hints(cls, include_extras=True)
//     target_fields = fields(cls)

//     if isinstance(decoded, bytes):
//         raise DecodingError(f"got `bytes` while decoding `{cls.__name__}`")

//     if len(target_fields) != len(decoded):
//         name = cls.__name__
//         actual = len(decoded)
//         expected = len(target_fields)
//         raise DecodingError(
//             f"`{name}` needs {expected} field(s), but got {actual} instead"
//         )

//     values: Dict[str, Any] = {}

//     for value, target_field in zip(decoded, target_fields):
//         resolved_type = hints[target_field.name]
//         try:
//             values[target_field.name] = deserialize_to(resolved_type, value)
//         except Exception as e:
//             msg = f"cannot decode field `{cls.__name__}.{target_field.name}`"
//             raise DecodingError(msg) from e

//     result = cls(**values)
//     assert isinstance(result, cls)
//     return cast(U, result)


// def _deserialize_to_bool(value: Simple) -> bool:
//     if value == b"":
//         return False
//     elif value == b"\x01":
//         return True
//     else:
//         raise DecodingError("invalid boolean")


// def _deserialize_to_bytes(
//     class_: Union[Type[Bytes], Type[FixedBytes]], value: Simple
// ) -> Union[Bytes, FixedBytes]:
//     if not isinstance(value, bytes):
//         raise DecodingError("invalid bytes")
//     try:
//         return class_(value)
//     except ValueError as e:
//         raise DecodingError from e


// def _deserialize_to_uint(
//     class_: Union[Type[Uint], Type[FixedUnsigned]], decoded: Simple
// ) -> Union[Uint, FixedUnsigned]:
//     if not isinstance(decoded, bytes):
//         raise DecodingError("invalid uint")
//     try:
//         return class_.from_be_bytes(decoded)
//     except ValueError as e:
//         raise DecodingError from e


// @runtime_checkable
// class _Annotation(Protocol):
//     __metadata__: Sequence[object]
//     __origin__: object


// def _deserialize_annotated(
//     annotation: _Annotation, value: Simple
// ) -> Union[Tuple[Extended, None], Tuple[None, object]]:
//     codecs = [x for x in annotation.__metadata__ if isinstance(x, With)]
//     if not codecs:
//         return (None, annotation.__origin__)

//     if len(codecs) > 1:
//         raise Exception(
//             "multiple rlp.With annotations applied to the same type"
//         )

//     codec = codecs[0]
//     result = codec._decoder(value)

//     try:
//         assert isinstance(
//             result, annotation.__origin__  # type: ignore[arg-type]
//         ), "annotated returned wrong type"
//     except TypeError as e:
//         # TODO: Check annotation types that don't work with `isinstance`.
//         msg = f"annotation {annotation.__origin__} doesn't support isinstance"
//         raise NotImplementedError(msg) from e

//     return (codec._decoder(value), None)


// def _deserialize_to_annotation(annotation: object, value: Simple) -> Extended:
//     origin = get_origin(annotation)
//     if origin is Union:
//         return _deserialize_to_union(annotation, value)
//     elif origin in (Tuple, tuple):
//         return _deserialize_to_tuple(annotation, value)
//     elif origin in (List, Sequence, list):
//         return _deserialize_to_list(annotation, value)
//     elif origin is None:
//         raise Exception(annotation)
//     else:
//         raise NotImplementedError(f"RLP non-type {origin!r}")


// def _deserialize_to_union(annotation: object, value: Simple) -> Extended:
//     arguments = get_args(annotation)
//     successes: List[Extended] = []
//     failures = []
//     for argument in arguments:
//         try:
//             success = deserialize_to(argument, value)
//         except Exception as e:
//             failures.append(e)
//             continue

//         successes.append(success)

//     if len(successes) == 1:
//         return successes[0]
//     elif not successes:
//         raise DecodingError(f"no matching union variant\n{failures!r}")
//     else:
//         raise DecodingError("multiple matching union variants")


// def _deserialize_to_tuple(
//     annotation: object, values: Simple
// ) -> Sequence[Extended]:
//     if isinstance(values, bytes):
//         raise DecodingError("invalid tuple")
//     arguments = list(get_args(annotation))

//     if arguments[-1] is Ellipsis:
//         arguments.pop()
//         fill_count = len(values) - len(arguments)
//         arguments = list(arguments) + [arguments[-1]] * fill_count

//     decoded = []
//     for index, (argument, value) in enumerate(zip(arguments, values)):
//         try:
//             deserialized = deserialize_to(argument, value)
//         except Exception as e:
//             msg = f"cannot decode tuple element {index} of type `{argument}`"
//             raise DecodingError(msg) from e
//         decoded.append(deserialized)

//     return tuple(decoded)


// def _deserialize_to_list(
//     annotation: object, values: Simple
// ) -> Sequence[Extended]:
//     if isinstance(values, bytes):
//         raise DecodingError("invalid list")
//     argument = get_args(annotation)[0]
//     results = []
//     for index, value in enumerate(values):
//         try:
//             deserialized = deserialize_to(argument, value)
//         except Exception as e:
//             msg = f"cannot decode list item {index} of type `{annotation}`"
//             raise DecodingError(msg) from e
//         results.append(deserialized)
//     return results


// def decode_to_bytes(encoded_bytes: Bytes) -> Bytes:
//     """
//     Decodes a rlp encoded byte stream assuming that the decoded data
//     should be of type `bytes`.
//     """
//     if len(encoded_bytes) == 1 and encoded_bytes[0] < 0x80:
//         return encoded_bytes
//     elif encoded_bytes[0] <= 0xB7:
//         len_raw_data = encoded_bytes[0] - 0x80
//         if len_raw_data < 0:
//             raise DecodingError("negative length")
//         if len_raw_data >= len(encoded_bytes):
//             raise DecodingError("truncated")
//         raw_data = encoded_bytes[1 : 1 + len_raw_data]
//         if len_raw_data == 1 and raw_data[0] < 0x80:
//             raise DecodingError
//         return raw_data
//     else:
//         # This is the index in the encoded data at which decoded data
//         # starts from.
//         decoded_data_start_idx = 1 + encoded_bytes[0] - 0xB7
//         if decoded_data_start_idx - 1 >= len(encoded_bytes):
//             raise DecodingError
//         if encoded_bytes[1] == 0:
//             raise DecodingError
//         len_decoded_data = int(
//             Uint.from_be_bytes(encoded_bytes[1:decoded_data_start_idx])
//         )
//         if len_decoded_data < 0x38:
//             raise DecodingError
//         decoded_data_end_idx = decoded_data_start_idx + int(len_decoded_data)
//         if decoded_data_end_idx - 1 >= len(encoded_bytes):
//             raise DecodingError
//         return encoded_bytes[decoded_data_start_idx:decoded_data_end_idx]


// def decode_to_sequence(encoded_sequence: Bytes) -> Sequence[Simple]:
//     """
//     Decodes a rlp encoded byte stream assuming that the decoded data
//     should be of type `Sequence` of objects.
//     """
//     if encoded_sequence[0] <= 0xF7:
//         len_joined_encodings = encoded_sequence[0] - 0xC0
//         if len_joined_encodings >= len(encoded_sequence):
//             raise DecodingError
//         joined_encodings = encoded_sequence[1 : 1 + len_joined_encodings]
//     else:
//         joined_encodings_start_idx = 1 + encoded_sequence[0] - 0xF7
//         if joined_encodings_start_idx - 1 >= len(encoded_sequence):
//             raise DecodingError
//         if encoded_sequence[1] == 0:
//             raise DecodingError
//         len_joined_encodings = int(
//             Uint.from_be_bytes(encoded_sequence[1:joined_encodings_start_idx])
//         )
//         if len_joined_encodings < 0x38:
//             raise DecodingError
//         joined_encodings_end_idx = (
//             joined_encodings_start_idx + len_joined_encodings
//         )
//         if joined_encodings_end_idx - 1 >= len(encoded_sequence):
//             raise DecodingError
//         joined_encodings = encoded_sequence[
//             joined_encodings_start_idx:joined_encodings_end_idx
//         ]

//     return decode_joined_encodings(joined_encodings)


// def decode_joined_encodings(joined_encodings: Bytes) -> Sequence[Simple]:
//     """
//     Decodes `joined_encodings`, which is a concatenation of RLP encoded
//     objects.
//     """
//     decoded_sequence = []

//     item_start_idx = 0
//     while item_start_idx < len(joined_encodings):
//         encoded_item_length = decode_item_length(
//             joined_encodings[item_start_idx:]
//         )
//         if item_start_idx + encoded_item_length - 1 >= len(joined_encodings):
//             raise DecodingError
//         encoded_item = joined_encodings[
//             item_start_idx : item_start_idx + encoded_item_length
//         ]
//         decoded_sequence.append(decode(encoded_item))
//         item_start_idx += encoded_item_length

//     return decoded_sequence


// def decode_item_length(encoded_data: Bytes) -> int:
//     """
//     Find the length of the rlp encoding for the first object in the
//     encoded sequence.

//     Here `encoded_data` refers to concatenation of rlp encoding for each
//     item in a sequence.
//     """
//     if len(encoded_data) <= 0:
//         raise DecodingError

//     first_rlp_byte = encoded_data[0]

//     # This is the length of the big endian representation of the length of
//     # rlp encoded object byte stream.
//     length_length = 0
//     decoded_data_length = 0

//     # This occurs only when the raw_data is a single byte whose value < 128
//     if first_rlp_byte < 0x80:
//         # We return 1 here, as the end formula
//         # 1 + length_length + decoded_data_length would be invalid for
//         # this case.
//         return 1
//     # This occurs only when the raw_data is a byte stream with length < 56
//     # and doesn't fall into the above cases
//     elif first_rlp_byte <= 0xB7:
//         decoded_data_length = first_rlp_byte - 0x80
//     # This occurs only when the raw_data is a byte stream and doesn't fall
//     # into the above cases
//     elif first_rlp_byte <= 0xBF:
//         length_length = first_rlp_byte - 0xB7
//         if length_length >= len(encoded_data):
//             raise DecodingError
//         if encoded_data[1] == 0:
//             raise DecodingError
//         decoded_data_length = int(
//             Uint.from_be_bytes(encoded_data[1 : 1 + length_length])
//         )
//     # This occurs only when the raw_data is a sequence of objects with
//     # length(concatenation of encoding of each object) < 56
//     elif first_rlp_byte <= 0xF7:
//         decoded_data_length = first_rlp_byte - 0xC0
//     # This occurs only when the raw_data is a sequence of objects and
//     # doesn't fall into the above cases.
//     elif first_rlp_byte <= 0xFF:
//         length_length = first_rlp_byte - 0xF7
//         if length_length >= len(encoded_data):
//             raise DecodingError
//         if encoded_data[1] == 0:
//             raise DecodingError
//         decoded_data_length = int(
//             Uint.from_be_bytes(encoded_data[1 : 1 + length_length])
//         )

//     return 1 + length_length + decoded_data_length


// Decoder: TypeAlias = Callable[[Simple], Extended]


// class With:
//     """
//     When used with [`Annotated`][0], indicates that a value needs to be
//     encoded/decoded using a custom function.

//     [0]: https://docs.python.org/3/library/typing.html#typing.Annotated
//     """

//     def __init__(self, decoder: Decoder) -> None:
//         self._decoder = decoder
