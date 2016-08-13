
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian};
use rustc_serialize::base64::ToBase64;
use rustc_serialize::base64::FromBase64;
use rustc_serialize::base64::Newline::*;
use rustc_serialize::base64::CharacterSet::*;
use rustc_serialize::base64::Config;
use std::io::Cursor;

const C : Config = Config {
        char_set: UrlSafe,
        newline: LF,
        pad: false,
        line_length: None
};

pub fn encode(link_id: u64) -> String {
    let mut bytes = Vec::new();

    // Convert u64 -> [u8; 8]
    bytes.write_u64::<BigEndian>(link_id).unwrap();
    // Strip leading zeroes from [u8] and output Base64
    bytes.iter().cloned().skip_while(|b| *b==0).collect::<Vec<u8>>().to_base64(C)
}

pub fn decode(path: String) -> u64 {
    // Try to Base64 decode a string into a u64
    let mut bytes = path.from_base64().unwrap();
    // Prepend zeroes
    bytes.reverse();
    bytes.resize(8,0);
    bytes.reverse();
    // TODO: catch errors
    Cursor::new(bytes).read_u64::<BigEndian>().unwrap()
}
