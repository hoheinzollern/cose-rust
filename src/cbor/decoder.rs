use std::io::{Cursor, Read, Seek, SeekFrom};
use std::string::String;

/// Struct holding a cursor and additional information for decoding.
#[derive(Debug)]
pub struct DecoderCursor {
    pub cursor: Cursor<Vec<u8>>,
    pub decoded: CBORObject,
}

impl DecoderCursor {
    /// Convert num bytes to a u64
    fn read_int_from_bytes(&mut self, num: usize) -> Result<u64, &'static str> {
        let mut bytes = &mut self.cursor;
        let mut x: Vec<u8> = vec![0; num];
        if bytes.read(&mut x).unwrap() != num {
            return Err("Couldn't read all bytes");
        }
        let mut result: u64 = 0;
        for i in (0..num).rev() {
            result += (x[num - 1 - i] as u64) << (i * 8);
        }
        Ok(result)
    }

    /// Read an integer and return it as u64.
    fn read_int(&mut self) -> Result<u64, &'static str> {
        let pos = self.cursor.position() as usize;
        let first_value = self.cursor.get_ref()[pos] & 0x1F;
        self.cursor.seek(SeekFrom::Current(1)).unwrap();
        let mut val: u64 = 0;
        match first_value {
            24 => {
                // Manually advance cursor.
                let pos = self.cursor.position() as usize;
                val = self.cursor.get_ref()[pos] as u64;
                self.cursor.seek(SeekFrom::Current(1)).unwrap();
            }
            25 => val = self.read_int_from_bytes(2).unwrap(),
            26 => val = self.read_int_from_bytes(4).unwrap(),
            27 => val = self.read_int_from_bytes(8).unwrap(),
            28...31 => return Err("Not well formed and indefinite len isn't supported"),
            _ => val = first_value as u64,
        }
        Ok(val)
    }

    /// Read an integer and add it to the decoded values.
    fn parse_int(&mut self, tag: bool) -> Result<(), &'static str> {
        let val = self.read_int().unwrap();
        if tag {
            self.decoded.values.push(CBORType::Tag(val));
        } else {
            self.decoded.values.push(CBORType::Integer(val));
        }
        Ok(())
    }

    fn read_signed_int(&mut self) -> Result<CBORType, &'static str> {
        let uint = self.read_int().unwrap();
        if uint > i64::max_value() as u64 {
            return Err("Signed integer doesn't fit in a i64 (too large)");
        }
        let result: i64 = -1 - uint as i64;
        Ok(CBORType::SignedInteger(result))
    }

    /// Read an integer and add it to the decoded values.
    fn parse_signed_int(&mut self) -> Result<(), &'static str> {
        let val = self.read_signed_int().unwrap();
        self.decoded.values.push(val);
        Ok(())
    }

    /// Read an array of data items and return it.
    fn read_array(&mut self) -> Result<CBORType, &'static str> {
        // Create a new array.
        let mut array: Vec<CBORType> = Vec::new();
        // Read the length of the array.
        let num_items = self.read_int().unwrap();
        // Decode each of the num_items data items.
        for item_num in 0..num_items {
            array.push(self.decode_item().unwrap());
        }
        Ok(CBORType::Array(array))
    }

    /// Read an array of data items.
    fn parse_array(&mut self) -> Result<(), &'static str> {
        let array = self.read_array().unwrap();
        self.decoded.values.push(array);
        Ok(())
    }

    /// Read a byte string and return it as hex string.
    fn read_byte_string(&mut self) -> Result<CBORType, &'static str> {
        let length = self.read_int().unwrap();
        if length > usize::max_value() as u64 {
            return Err("Byte array is too large to allocate.");
        }
        let length = length as usize;
        let mut byte_string: Vec<u8> = Vec::with_capacity(length);
        // XXX: rewrite without unsafe.
        unsafe {
            byte_string.set_len(length);
        }
        if self.cursor.read(&mut byte_string).unwrap() != length {
            return Err("Couldn't read enough data for this byte string");
        }
        Ok(CBORType::Bytes(byte_string))
    }

    /// Read a byte string and return it as hex string.
    fn parse_byte_string(&mut self) -> Result<(), &'static str> {
        let bytes = self.read_byte_string().unwrap();
        self.decoded.values.push(bytes);
        Ok(())
    }

    /// Read a map.
    fn read_map(&mut self) -> Result<CBORType, &'static str> {
        // XXX: check for duplicate keys.
        let num_items = self.read_int().unwrap();
        // Create a new array.
        let mut map: Vec<CBORMap> = Vec::new();
        // Decode each of the num_items (key, data item) pairs.
        for item_num in 0..num_items {
            let key_val = self.decode_item().unwrap();
            let item_value = self.decode_item().unwrap();
            let item = CBORMap {
                key: key_val,
                value: item_value
            };
            map.push(item);
        }
        Ok(CBORType::Map(map))
    }

    /// Read a map.
    fn parse_map(&mut self) -> Result<(), &'static str> {
        let map = self.read_map().unwrap();
        self.decoded.values.push(map);
        Ok(())
    }

    /// Decodes the next item in DecoderCursor.
    fn decode_item(&mut self) -> Result<CBORType, &'static str> {
        let pos = self.cursor.position() as usize;
        let major_type = self.cursor.get_ref()[pos] >> 5;
        match major_type {
            0 => return Ok(CBORType::Integer(self.read_int().unwrap())),
            1 => return Ok(self.read_signed_int().unwrap()),
            2 => return Ok(self.read_byte_string().unwrap()),
            4 => return Ok(self.read_array().unwrap()),
            5 => return Ok(self.read_map().unwrap()),
            6 => return Ok(CBORType::Tag(self.read_int().unwrap())),
            _ => return Err("Malformed first byte"),
        }
    }
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub struct CBORMap {
    pub key: CBORType,
    pub value: CBORType,
}

#[derive(Debug)]
#[derive(Clone)]
#[derive(PartialEq)]
pub enum CBORType {
    Integer(u64),
    SignedInteger(i64),
    Tag(u64),
    Bytes(Vec<u8>),
    String(String),
    Array(Vec<CBORType>),
    Map(Vec<CBORMap>),
}

/// XXX: I really have to rethink the Tag value.
impl From<u64> for CBORType {
    fn from(x: u64) -> Self {
        CBORType::Integer(x)
    }
}

impl From<i64> for CBORType {
    fn from(x: i64) -> Self {
        CBORType::SignedInteger(x)
    }
}

impl From<Vec<u8>> for CBORType {
    fn from(x: Vec<u8>) -> Self {
        CBORType::Bytes(x)
    }
}

impl From<String> for CBORType {
    fn from(x: String) -> Self {
        CBORType::String(x)
    }
}

impl From<Vec<CBORType>> for CBORType {
    fn from(x: Vec<CBORType>) -> Self {
        CBORType::Array(x)
    }
}

impl From<Vec<CBORMap>> for CBORType {
    fn from(x: Vec<CBORMap>) -> Self {
        CBORType::Map(x)
    }
}

#[derive(Debug)]
pub struct CBORObject {
    pub values: Vec<CBORType>,
}

impl PartialEq for CBORObject {
    fn eq(&self, other: &CBORObject) -> bool {
        if self.values.len() != other.values.len() {
            return false;
        }
        self.values.eq(&other.values)
    }
}

/// Decodes the next item in DecoderCursor.
pub fn decode_item(decoder_cursor: &mut DecoderCursor) -> Result<(), &'static str> {
    let major_type = decoder_cursor.cursor.get_ref()[decoder_cursor.cursor.position() as usize] >>
        5;
    match major_type {
        0 => decoder_cursor.parse_int(false),
        1 => decoder_cursor.parse_signed_int(),
        2 => decoder_cursor.parse_byte_string(),
        4 => decoder_cursor.parse_array(),
        5 => decoder_cursor.parse_map(),
        6 => decoder_cursor.parse_int(true),
        _ => return Err("Malformed first byte"),
    }
}

/// Read the CBOR structure in bytes and return it as a CBOR object.
#[allow(dead_code)]
pub fn decode(bytes: Vec<u8>) -> Result<CBORObject, &'static str> {
    let mut decoder_cursor = DecoderCursor {
        cursor: Cursor::new(bytes),
        decoded: CBORObject { values: Vec::new() },
    };
    decode_item(&mut decoder_cursor).unwrap();
    Ok(decoder_cursor.decoded)
}
