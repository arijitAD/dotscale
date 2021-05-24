extern crate libc;
use libc::{c_char, c_int, c_schar, c_uint, c_ushort};
use parity_scale_codec::{Decode, Encode};
use std::ffi::{CStr, CString};

#[link(
    name = "encodeString",
    "decodeString",
    "encodeI8",
    "decodeI8",
    // "encodeU16",
    // "decodeU16",
    "encodeU32",
    "decodeU32"
)]
extern "C" {
    pub fn EncodeString(s: *const c_char) -> *const c_char;
    pub fn DecodeString(s: *const c_char) -> *const c_char;

    pub fn EncodeI8(a: c_schar) -> *const c_char;
    pub fn DecodeI8(s: *const c_char) -> c_char;

    // pub fn EncodeU16(a: c_ushort) -> *const c_char;
    // pub fn DecodeU16(s: *const c_char) -> c_ushort;

    pub fn EncodeU32(a: c_uint) -> *const c_char;
    pub fn DecodeU32(s: *const c_char) -> c_uint;
}

fn main() {
    // enc_dec_string();
    // enc_dec_i8();
    // enc_dec_u16();
    enc_dec_u32();
}

#[allow(dead_code)]
fn enc_dec_string() {
    let val = String::from("Hello, World!");

    let exp_enc = val.encode();
    let exp_dec = <String>::decode(&mut &exp_enc[..]).unwrap();

    let c_str = CString::new(val).expect("CString::new failed");
    let enc_result = unsafe { EncodeString(c_str.as_ptr()) };
    let enc_buf_name = unsafe { CStr::from_ptr(enc_result).to_bytes() };
    assert_eq!(exp_enc, enc_buf_name);

    let dec_resp = unsafe { DecodeString(enc_result) };
    let resp = char_to_string(dec_resp);
    assert_eq!(exp_dec, resp);
}

#[allow(dead_code)]
fn enc_dec_i8() {
    let expected: i8 = 69;

    let exp_enc = expected.encode();
    let exp_dec: i8 = Decode::decode(&mut &exp_enc[..]).unwrap();

    let enc_result = unsafe { EncodeI8(expected) };
    let enc_buf_name = unsafe { CStr::from_ptr(enc_result).to_bytes() };
    println!("enc_buf_name {:?}", enc_buf_name);
    assert_eq!(exp_enc, enc_buf_name);

    let dec_resp = unsafe { DecodeI8(enc_result) };

    assert_eq!(exp_dec, dec_resp);
}

fn enc_dec_u32() {
    let expected: u32 = 16777215;

    let exp_enc = expected.encode();
    let exp_dec: u32 = Decode::decode(&mut &exp_enc[..]).unwrap();

    let enc_result = unsafe { EncodeU32(expected) };

    let enc_buf_name = unsafe { CStr::from_ptr(enc_result).to_bytes() };
    println!("enc_buf_name {:?}", enc_buf_name);
    assert_eq!(exp_enc, enc_buf_name);

    let dec_resp = unsafe { DecodeU32(enc_result) };
    assert_eq!(exp_dec, dec_resp);
}

// #[allow(dead_code)]
// fn enc_dec_u16() {
//     let expected: u16 = 42;

//     let exp_enc = expected.encode();
//     let exp_dec: u16 = Decode::decode(&mut &exp_enc[..]).unwrap();

//     let enc_result = unsafe { EncodeU16(expected) };

//     let enc_buf_name = unsafe { CStr::from_ptr(enc_result).to_bytes() };
//     assert_eq!(exp_enc, enc_buf_name);

//     let dec_resp = unsafe { DecodeU16(enc_result) };
//     assert_eq!(exp_dec, dec_resp);
// }

fn char_to_string(resp: *const c_char) -> String {
    let buf_name = unsafe { CStr::from_ptr(resp).to_bytes() };
    let str_name = String::from_utf8(buf_name.to_vec()).unwrap();
    return str_name;
}
