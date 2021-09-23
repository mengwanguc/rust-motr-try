#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::ptr;
use std::mem::MaybeUninit;
use std::ffi::CString;
use std::os::raw::c_void;


fn main() {
    unsafe {
        println!("{}", object_read());
    }
}

