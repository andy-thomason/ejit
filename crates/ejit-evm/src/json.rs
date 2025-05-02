//! 
//! Very simple JSON deserialiser
//! 

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
}

pub trait JsonDecode {
    fn decode(&mut self, buffer: &mut &[u8]) -> Result<(), JsonError>;
}

impl JsonDecode for u128 {
    fn decode(&mut self, buffer: &mut &[u8]) -> Result<(), JsonError> {
        skip_whitespace(buffer);
        if !matches!(buffer.first(), Some(x) if x.is_ascii_digit()) {
            return Err(JsonError::ExpectedDigit);
        }
        let mut value : u128 = (buffer[0] - b'0').into();
        *buffer = &buffer[1..];
        while matches!(buffer.first(), Some(x) if x.is_ascii_digit()) {
            let (v, e1) = value.overflowing_mul(10);
            let (v, e2) = v.overflowing_add((buffer[0] - b'0').into());
            if e1 || e2 {
                return Err(JsonError::NumericOverflow);
            }
            value = v;
            *buffer = &buffer[1..];
        }
        *self = value;
        Ok(())
    }
}

impl JsonDecode for bool {
    fn decode(&mut self, buffer: &mut &[u8]) -> Result<(), JsonError> {
        skip_whitespace(buffer);
        *self = match parse_indent(buffer)? {
            b"true" => true,
            b"false" => false,
            _ => return Err(JsonError::ExpectedBool),
        };
        Ok(())
    }
}

impl<T : JsonDecode + Default> JsonDecode for Vec<T> {
    fn decode(&mut self, buffer: &mut &[u8]) -> Result<(), JsonError> {
        skip_whitespace(buffer);
        expect(buffer, b'[')?;
        skip_whitespace(buffer);
        if buffer.first() != Some(&b']') {
            loop {
                let mut t : T = Default::default();
                t.decode(buffer)?;
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

pub fn parse_string<'a>(buffer: &mut &'a [u8]) -> Result<&'a [u8], JsonError> {
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

pub fn parse_indent<'a>(buffer: &mut &'a [u8]) -> Result<&'a [u8], JsonError> {
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

pub fn decode_object(dest: &mut [(&mut dyn JsonDecode, &[u8])], buffer: &mut &[u8]) -> Result<(), JsonError> {
    expect(buffer, b'{')?;
    if let Some(b'}') = buffer.first() {
        *buffer = &buffer[1..];
        return Ok(());
    }
    loop {
        let key = parse_string(buffer)?;

        println!("{key:02x?} {buffer:02x?}");
        expect(buffer, b':')?;

        let Some((obj, _)) = dest.iter_mut().find(|(_, k)| *k == key) else {
            return Err(JsonError::MissingKey);
        };

        obj.decode(buffer)?;

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


#[cfg(test)]
mod tests {
    use crate::json::decode_object;

    use super::JsonDecode;

    #[test]
    fn test_bool() {
        let mut cursor = b"true".as_slice(); 
        let mut b = false;
        b.decode(&mut cursor).unwrap();
        assert_eq!(b, true);
        assert!(cursor.is_empty());

        let mut cursor = b"false".as_slice(); 
        let mut b = false;
        b.decode(&mut cursor).unwrap();
        assert_eq!(b, false);
        assert!(cursor.is_empty());

        let mut cursor = b"null".as_slice(); 
        let mut b = false;
        assert!(b.decode(&mut cursor).is_err());
    }

    #[test]
    fn test_int() {
        let mut cursor = b"1234".as_slice(); 
        let mut b : u128 = 0;
        b.decode(&mut cursor).unwrap();
        assert_eq!(b, 1234);
        assert!(cursor.is_empty());

        let mut cursor = b"340282366920938463463374607431768211455".as_slice(); 
        let mut b : u128 = 0;
        b.decode(&mut cursor).unwrap();
        assert_eq!(b, 340282366920938463463374607431768211455);
        assert!(cursor.is_empty());

        let mut cursor = b"340282366920938463463374607431768211456".as_slice(); 
        let mut b : u128 = 0;
        assert!(b.decode(&mut cursor).is_err());
    }

    #[test]
    fn test_struct() {
        #[derive(Debug, PartialEq, Default)]
        struct ABC {
            a: u128,
            b: bool,
            c: u128,
        }

        impl JsonDecode for ABC {
            fn decode(&mut self, buffer: &mut &[u8]) -> Result<(), super::JsonError> {
                decode_object(&mut[
                    (&mut self.a, b"a"),
                    (&mut self.b, b"b"),
                    (&mut self.c, b"c"),
                ], buffer)
            }
        }

        let mut cursor = b"{}".as_slice();
        let mut b : ABC = Default::default();
        b.decode(&mut cursor).unwrap();
        assert_eq!(b, ABC{..Default::default()});

        let mut cursor = br#"{"a":1,"b":true,"c":2}"#.as_slice();
        let mut b : ABC = Default::default();
        b.decode(&mut cursor).unwrap();
        assert_eq!(b, ABC{a:1, b:true, c:2});

    }
}
