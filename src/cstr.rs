//! A wrapper for C strings.
//! Probably won't work well if you use non-ASCII characters.

use ::libc::c_char;

pub fn from_cstr(str_in: *const c_char) -> String {
    let mut str_out = String::new();
    let mut pos: isize = 0;
    while unsafe { *str_in.offset(pos) } != 0 {
        str_out.push(unsafe { *str_in.offset(pos) } as u8 as char);
        pos += 1;
    }
    str_out
}
pub fn to_cstr(str_in: &str) -> Vec<c_char> {
    let mut v: Vec<c_char> = str_in.chars().map(|c| c as c_char).collect();
    v.push(0);
    v
}

