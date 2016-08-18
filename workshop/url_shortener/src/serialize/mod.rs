extern crate rustc_serialize;

/// A trait for converting a value to base64 encoding.
pub trait ToShortBase64 {
    fn to_short_base64(&self, config: Config) -> String;
}

