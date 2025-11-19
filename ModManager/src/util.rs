#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unnecessary_transmutes)]
#![allow(unsafe_op_in_unsafe_fn)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
use std::{env, ffi::CString};
use directories::BaseDirs;

pub fn local_path(path: &str) -> String {
    let mut dir = env::current_exe().expect("Couldn't locate executable directory");
    dir.pop();
    dir.push(path);
    return String::from(dir.to_str().unwrap_or(path));
}

pub fn appdata_path(path: &str) -> String {
    let dir = BaseDirs::new().unwrap().data_dir().join(path);
    return String::from(dir.to_str().unwrap_or(path));
}

pub fn run_program(path: &str) -> bool {
    unsafe {
        return ex_run_program(CString::new(path).unwrap().into_raw()) > 0;
    };
}
