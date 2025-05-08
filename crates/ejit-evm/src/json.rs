//! 
//! Very simple JSON deserialiser
//!
//! See https://www.json.org/json-en.html 

use std::{collections::BTreeMap, io::Write};

use crate::ethereum::{ethereum_types::numeric::{Uint, U64}, utils::hexadecimal::hex_to_slice};

#[derive(Debug)]
pub enum JsonError {
    UnexpectedEof,
    UnexpectedChar,
    UnterminatedString,
    Expected(char),
    MissingKey,
    ExpectedDigit,
    ExpectedIdentifier,
    ExpectedBool,
    NumericOverflow,
    BadString,
    ExpectedHexString,
}

#[derive(Debug)]
pub struct Decoder<'b, 'de> {
    pub buffer: &'b mut &'de [u8],
    orignal: &'de [u8],
}

pub trait JsonDecode<'de> : where Self : 'de {
    fn decode_json(&mut self, buffer: & mut &'de [u8]) -> Result<(), JsonError>;
}

impl<'de> JsonDecode<'de> for &'de str {
    fn decode_json(&mut self, buffer: & mut &'de [u8]) -> Result<(), JsonError> {
        let mut s = parse_string(buffer)?;
        *self = std::str::from_utf8(s)
            .map_err(|_| JsonError::BadString)?;
        Ok(())
    }
}

impl<'de, T : JsonDecode<'de> + Default> JsonDecode<'de> for Vec<T> {
    fn decode_json(&mut self, buffer: &mut &'de [u8]) -> Result<(), JsonError> {
        skip_whitespace(buffer);
        expect(buffer, b'[')?;
        skip_whitespace(buffer);
        if buffer.first() != Some(&b']') {
            loop {
                let mut t : T = Default::default();
                t.decode_json(buffer)?;
                self.push(t);
                skip_whitespace(buffer);
                if buffer.first() == Some(&b']') {
                    break
                };
                expect(buffer, b',')?;
            }
        }
        *buffer = &buffer[1..];
        Ok(())
    }
}

impl<'de, K : JsonDecode<'de> + Default + Ord, V : JsonDecode<'de> + Default> JsonDecode<'de> for BTreeMap<K, V> {
    fn decode_json(&mut self, buffer: &mut &'de [u8]) -> Result<(), JsonError> {
        let mut p = ObjectParser::new(buffer);
        loop {
            let Some(key) = p.next_map_key::<K>()? else { break; };
            let mut value = V::default();
            value.decode_json(p.buffer)?;
            self.insert(key, value);
        }
        Ok(())
    }
}

impl<'de> JsonDecode<'de> for bool {
    fn decode_json(&mut self, buffer: &mut &'de [u8]) -> Result<(), JsonError> {
        skip_whitespace(buffer);
        *self = match parse_indent(buffer)? {
            b"true" => true,
            b"false" => false,
            _ => return Err(JsonError::ExpectedBool),
        };
        Ok(())
    }
}

impl<'de> JsonDecode<'de> for String {
    fn decode_json(&mut self, buffer: &mut &'de [u8]) -> Result<(), JsonError> {
        let mut s = parse_string(buffer)?;
        if s.iter().any(|b| b.is_ascii_control()) {
            return Err(JsonError::BadString);
        }
        if !s.contains(&b'\\') {
            *self = String::from_utf8(s.to_vec())
                .map_err(|_| JsonError::BadString)?;
        } else {
            let mut i = 0;
            *self = String::with_capacity(s.len());
            while i != s.len() {
                if s[i] != b'\\' {
                    self.push(s[i].into());
                    i += 1;
                } else {
                    match s[i+1] {
                        b'"' => self.push('"'),
                        b'\\' => self.push('\\'),
                        b'b' => self.push('\x08'),
                        b'f' => self.push('\x0c'),
                        b'n' => self.push('\n'),
                        b'r' => self.push('\r'),
                        b'u' => {
                            if i + 6 > s.len() {
                                return Err(JsonError::BadString);
                            }
                            let x = std::str::from_utf8(&s[i+2..i+6])
                                .map_err(|_| JsonError::BadString)?;
                            let c = u32::from_str_radix(x, 16)
                                .map_err(|_| JsonError::BadString)?
                                .try_into()
                                .map_err(|_| JsonError::BadString)?;
                            self.push(c);
                            i += 4;
                        }
                        _ => return Err(JsonError::BadString),
                    }
                    i += 2;
                }
            }
        }
        Ok(())
    }
}

pub fn skip_whitespace(buffer: &mut &[u8]) {
    while buffer.first().map(u8::is_ascii_whitespace) == Some(true) {
        *buffer = &buffer[1..];
    }
}

pub fn expect(buffer: &mut &[u8], chr: u8) -> Result<(), JsonError> {
    skip_whitespace(buffer);
    if !buffer.first().is_some_and(|c| *c == chr) { return Err(JsonError::Expected(chr.into())); };
    *buffer = &buffer[1..];
    Ok(())
}

pub fn parse_string<'b, 'c>(buffer: &'c mut &'b [u8]) -> Result<&'b [u8], JsonError> {
    skip_whitespace(buffer);
    match buffer.first() {
        Some(b'"') => {
            if let Some(nbytes) = buffer.windows(2).position(|w| w[1] == b'"' && w[0] != b'\\') {
                let bytes = &buffer[1..nbytes+1];
                *buffer = &buffer[nbytes+2..];
                Ok(bytes)
            } else {
                Err(JsonError::UnterminatedString)
            }
        }
        Some(_) => Err(JsonError::UnexpectedChar),
        None => Err(JsonError::UnexpectedEof),
    }
}

pub fn parse_indent<'b, 'c>(buffer: &'c mut &'b [u8]) -> Result<&'b [u8], JsonError> {
    skip_whitespace(buffer);
    let start = *buffer;
    if !matches!(buffer.first(), Some(x) if x.is_ascii_alphabetic()) {
        return Err(JsonError::ExpectedIdentifier);
    }
    *buffer = &buffer[1..];
    let mut n = 1;
    while matches!(buffer.first(), Some(x) if x.is_ascii_alphabetic()) {
        *buffer = &buffer[1..];
        n += 1;
    }
    Ok(&start[0..n])
}

pub fn decode_object<'de>(dest: &mut [(&mut dyn JsonDecode<'de>, &str)], buffer: &mut &'de [u8]) -> Result<(), JsonError> {
    expect(buffer, b'{')?;
    if let Some(b'}') = buffer.first() {
        *buffer = &buffer[1..];
        return Ok(());
    }
    loop {
        let key = parse_string(buffer)?;

        println!("{key:02x?} {buffer:02x?}");
        expect(buffer, b':')?;

        let Some((obj, _)) = dest.iter_mut().find(|(_, k)| k.as_bytes() == key) else {
            return Err(JsonError::MissingKey);
        };

        obj.decode_json(buffer)?;

        skip_whitespace(buffer);

        match buffer.first() {
            Some(b'}') => { *buffer = &buffer[1..]; break; }
            Some(b',') => { *buffer = &buffer[1..]; }
            Some(_) => return Err(JsonError::UnexpectedChar),
            None => return Err(JsonError::UnexpectedEof),
        }
    }

    skip_whitespace(buffer);
    Ok(())
}

pub struct ObjectParser<'b, 'de> {
    pub buffer: &'b mut &'de [u8],
    started: bool,
}

impl<'b, 'de> ObjectParser<'b, 'de> {
    pub fn new(buffer: &'b mut &'de [u8]) -> Self {
        Self { buffer, started: false }
    }

    pub fn next_key(&mut self) -> Result<Option<&'de str>, JsonError> {
        if !self.started {
            expect(self.buffer, b'{')?;
            if expect(self.buffer, b'}').is_ok() {
                return Ok(None)
            }
            self.started = true;
        } else {
            if expect(self.buffer, b'}').is_ok() {
                return Ok(None);
            } else {
                expect(self.buffer, b',')?;
            }
        }
        let mut key = "";
        key.decode_json(self.buffer)?;
        expect(self.buffer, b':')?;
        return Ok(Some(key));
    }

    /// When decoding maps, we do accept non-strings as keys.
    /// 
    /// Also many types have string encodings.
    pub fn next_map_key<T : JsonDecode<'de> + Default>(&mut self) -> Result<Option<T>, JsonError> {
        if !self.started {
            expect(self.buffer, b'{')?;
            if expect(self.buffer, b'}').is_ok() {
                return Ok(None)
            }
            self.started = true;
        } else {
            if expect(self.buffer, b'}').is_ok() {
                return Ok(None);
            } else {
                expect(self.buffer, b',')?;
            }
        }
        let mut key = T::default();
        key.decode_json(self.buffer)?;
        expect(self.buffer, b':')?;
        return Ok(Some(key));
    }

    pub fn decode_one(&mut self, a: &mut dyn JsonDecode<'de>, ka: &str) -> Result<(), JsonError> {
        // Note that even with one target, the JSON may repeat the key.
        loop {
            match self.next_key()? {
                Some(k) if k == ka => a.decode_json(self.buffer)?,
                None => return Ok(()),
                _ => return Err(crate::json::JsonError::MissingKey),
            }
        }
    }

    pub fn decode_two(&mut self, a: &mut dyn JsonDecode<'de>, ka: &str, b: &mut dyn JsonDecode<'de>, kb: &str) -> Result<(), JsonError> {
        loop {
            match self.next_key()? {
                Some(k) if k == ka => a.decode_json(self.buffer)?,
                Some(k) if k == kb => b.decode_json(self.buffer)?,
                None => return Ok(()),
                _ => return Err(crate::json::JsonError::MissingKey),
            }
        }
    }

    pub fn decode_three(&mut self, a: &mut dyn JsonDecode<'de>, ka: &str, b: &mut dyn JsonDecode<'de>, kb: &str, c: &mut dyn JsonDecode<'de>, kc: &str) -> Result<(), JsonError> {
        loop {
            match self.next_key()? {
                Some(k) if k == ka => a.decode_json(self.buffer)?,
                Some(k) if k == kb => b.decode_json(self.buffer)?,
                Some(k) if k == kc => c.decode_json(self.buffer)?,
                None => return Ok(()),
                _ => return Err(crate::json::JsonError::MissingKey),
            }
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    use crate::json::{decode_object, expect, skip_whitespace, ObjectParser};

    use super::JsonDecode;

    #[test]
    fn test_bool() {
        let mut cursor = b"true".as_slice(); 
        let mut b = false;
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, true);
        assert!(cursor.is_empty());

        let mut cursor = b"false".as_slice(); 
        let mut b = false;
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, false);
        assert!(cursor.is_empty());

        let mut cursor = b"null".as_slice(); 
        let mut b = false;
        assert!(b.decode_json(&mut cursor).is_err());
    }

    #[test]
    fn test_int() {
        let mut cursor = b"1234".as_slice(); 
        let mut b : u128 = 0;
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, 1234);
        assert!(cursor.is_empty());

        let mut cursor = b"340282366920938463463374607431768211455".as_slice(); 
        let mut b : u128 = 0;
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, 340282366920938463463374607431768211455);
        assert!(cursor.is_empty());

        let mut cursor = b"340282366920938463463374607431768211456".as_slice(); 
        let mut b : u128 = 0;
        assert!(b.decode_json(&mut cursor).is_err());
    }

    #[test]
    fn test_struct() {
        #[derive(Debug, PartialEq, Default)]
        struct ABC {
            a: u128,
            b: bool,
            c: u128,
        }

        impl<'de> JsonDecode<'de> for ABC {
            fn decode_json(&mut self, buffer: &mut &'de [u8]) -> Result<(), super::JsonError> {
                let mut p = ObjectParser::new(buffer);
                p.decode_three(&mut self.a, "a", &mut self.b, "b", &mut self.c, "c")
            }
        }

        let mut cursor = b"{}".as_slice();
        let mut b : ABC = Default::default();
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, ABC{..Default::default()});

        let mut cursor = br#"{"a":1,"b":true,"c":2}"#.as_slice();
        let mut b : ABC = Default::default();
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, ABC{a:1, b:true, c:2});

    }

    #[test]
    fn test_string() {
        let mut cursor = br#""abc""#.as_slice();
        let mut b : String = Default::default();
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, "abc");
        let mut cursor = br#""abc\\\"\b\f\n\r\u4f60def""#.as_slice();
        let mut b : String = Default::default();
        b.decode_json(&mut cursor).unwrap();
        assert_eq!(b, "abc\\\"\u{8}\u{c}\n\r你def");
    }
}
