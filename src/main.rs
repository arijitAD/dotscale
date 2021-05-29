extern crate libc;
use libc::{c_char, c_int, c_schar, c_uchar, c_uint, c_ushort};
use parity_scale_codec::{Decode, Encode, OptionBool};
use std::slice;

#[link(
    name = "encodeString",
    "decodeString",
    "encodeI8",
    "decodeI8",
    "encodeU16",
    "decodeU16",
    "encodeU32",
    "decodeU32",
    "encodeVecU8",
    "decodeVecU8",
    "encodeOptionBool",
    "encodeOptional"
)]
#[allow(improper_ctypes)]
extern "C" {
    pub fn EncodeString(data: *const c_char) -> (*const c_char, c_int);
    pub fn DecodeString(data: *const c_char, len: c_int) -> (*const c_char, c_int);

    pub fn EncodeI8(data: c_schar) -> (*const c_char, c_int);
    pub fn DecodeI8(data: *const c_char, len: c_int) -> c_char;

    pub fn EncodeU16(data: c_ushort) -> (*const c_char, c_int);
    pub fn DecodeU16(data: *const c_char, len: c_int) -> c_ushort;

    pub fn EncodeU32(data: c_uint) -> (*const c_char, c_int);
    pub fn DecodeU32(data: *const c_char, len: c_int) -> c_uint;

    pub fn EncodeVecU8(data: Vec<u8>, len: c_int) -> (*const c_char, c_uint);
    pub fn DecodeVecU8(data: *const c_char, len: c_int) -> (*const c_char, c_int);

    pub fn EncodeResult(data: Result<u8, bool>) -> (*const c_char, c_uint);

    pub fn EncodeOptionBool(hasValue: c_uchar, value: c_uchar) -> (*const c_char, c_uchar);
    pub fn DecodeOptionBool(data: *const c_char, len: c_int) -> (c_uchar, c_uchar);

    pub fn EncodeOptional(hasValue: c_uchar, value: c_uchar) -> (*const c_char, c_uchar);
    pub fn DecodeOptional(data: *const c_char, len: c_int) -> (c_uchar, c_uchar);
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optional_some() {
        let exp_val: u8 = 6;
        let expected: Option<u8> = Some(exp_val);
        let exp_enc = expected.encode();

        let has_value: u8 = 1;
        let (resp, len) = unsafe { EncodeOptional(has_value, exp_val) };
        let enc_result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, enc_result);

        let (dec_has_val, val) = unsafe {
            DecodeOptional(
                enc_result.as_ptr() as *const c_char,
                enc_result.len() as c_int,
            )
        };
        assert_eq!(has_value, dec_has_val);
        assert_eq!(exp_val, val);
    }

    #[test]
    fn optional_none() {
        let expected: Option<u8> = None;
        let exp_enc = expected.encode();

        let exp_has_value: u8 = 0;
        let exp_val: u8 = 0;
        let (resp, len) = unsafe { EncodeOptional(exp_has_value, exp_val) };
        let enc_result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, enc_result);

        let (dec_has_val, val) = unsafe {
            DecodeOptional(
                enc_result.as_ptr() as *const c_char,
                enc_result.len() as c_int,
            )
        };
        assert_eq!(exp_has_value, dec_has_val);
        assert_eq!(exp_val, val)
    }

    #[test]
    fn option_bool_false() {
        let expected = OptionBool(Some(false));
        let exp_enc = expected.encode();

        let has_value: u8 = 1;
        let value: u8 = 2;

        let (resp, len) = unsafe { EncodeOptionBool(has_value, value) };
        let enc_result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, enc_result);

        let (dec_has_val, val) = unsafe {
            DecodeOptionBool(
                enc_result.as_ptr() as *const c_char,
                enc_result.len() as c_int,
            )
        };
        assert_eq!(has_value, dec_has_val);
        assert_eq!(value, val)
    }

    #[test]
    fn option_bool_true() {
        let expected = OptionBool(Some(true));
        let exp_enc = expected.encode();

        let has_value: u8 = 1;
        let value: u8 = 1;
        let (resp, len) = unsafe { EncodeOptionBool(has_value, value) };

        let result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, result);

        let (dec_has_val, val) = unsafe { DecodeOptionBool(resp, result.len() as c_int) };
        assert_eq!(has_value, dec_has_val);
        assert_eq!(value, val);
    }

    #[test]
    fn option_bool_none() {
        let expected = OptionBool(None);
        let exp_enc = expected.encode();

        let has_value: u8 = 0;
        let value: u8 = 0;
        let (resp, len) = unsafe { EncodeOptionBool(has_value, value) };

        let result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, result);

        let (dec_has_val, val) = unsafe { DecodeOptionBool(resp, result.len() as c_int) };
        assert_eq!(has_value, dec_has_val);
        assert_eq!(value, val);
    }

    #[test]
    fn vec_u8_works() {
        let expected: Vec<u8> = vec![4, 8, 15, 16, 23, 42];

        let exp_enc = expected.encode();
        let exp_dec: Vec<u8> = Decode::decode(&mut &exp_enc[..]).unwrap();

        let len = expected.len();
        let (resp, len) = unsafe { EncodeVecU8(expected, len as c_int) };

        let result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, result);

        let (dec_resp, len) =
            unsafe { DecodeVecU8(result.as_ptr() as *const c_char, result.len() as c_int) };

        let result = unsafe { slice::from_raw_parts(dec_resp as *const u8, len as usize) };

        let result_vec = String::from_utf8(result.to_vec()).unwrap();
        assert_eq!(exp_dec, result_vec.as_bytes());
    }

    #[test]
    fn string_works() {
        let val = String::from("Hello, World!");

        let exp_enc = val.encode();
        let exp_dec = <String>::decode(&mut &exp_enc[..]).unwrap();

        let (enc_result, len) = unsafe { EncodeString(val.as_ptr() as *const c_char) };

        let result = unsafe { slice::from_raw_parts(enc_result as *const u8, len as usize) };
        assert_eq!(exp_enc, result);

        let (dec_resp, len) = unsafe { DecodeString(enc_result as *const c_char, len as c_int) };

        let buf_name = unsafe { slice::from_raw_parts(dec_resp as *const u8, len as usize) };
        let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
        assert_eq!(exp_dec, str_name);
    }

    #[test]
    fn i8_works() {
        let expected: i8 = 69;

        let exp_enc = expected.encode();
        let exp_dec: i8 = Decode::decode(&mut &exp_enc[..]).unwrap();

        let (enc_result, len) = unsafe { EncodeI8(expected) };

        let result = unsafe { slice::from_raw_parts(enc_result as *const u8, len as usize) };
        assert_eq!(exp_enc, result);

        let dec_resp = unsafe { DecodeI8(enc_result, result.len() as c_int) };

        assert_eq!(exp_dec, dec_resp);
    }

    #[test]
    fn u16_works() {
        let expected: u16 = 42;

        let exp_enc = expected.encode();
        let exp_dec: u16 = Decode::decode(&mut &exp_enc[..]).unwrap();

        let (resp, len) = unsafe { EncodeU16(expected) };

        let result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, result);

        let dec_resp =
            unsafe { DecodeU16(result.as_ptr() as *const c_char, result.len() as c_int) };
        assert_eq!(exp_dec, dec_resp);
    }

    #[test]
    fn u32_works() {
        let expected: u32 = 16777215;

        let exp_enc = expected.encode();
        let exp_dec: u32 = Decode::decode(&mut &exp_enc[..]).unwrap();

        let (resp, len) = unsafe { EncodeU32(expected) };

        let result = unsafe { slice::from_raw_parts(resp as *const u8, len as usize) };
        assert_eq!(exp_enc, result);

        let dec_resp =
            unsafe { DecodeU32(result.as_ptr() as *const c_char, result.len() as c_int) };
        assert_eq!(exp_dec, dec_resp);
    }
}
