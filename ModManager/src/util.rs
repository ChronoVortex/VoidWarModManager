use std::{env, ffi::{c_char, c_double, CString}};
use directories::BaseDirs;
use libloading::{Library, Symbol};

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

// Janky solution to running an external executable without it becoming a child,
// but this library is needed for the GM extension anyway so we might as well use it
pub fn run_program(path: &str) -> bool {
    unsafe {
        let lib = Library::new("ExternalRunLib.dll").expect("Could not load ExternalRunLib.dll");
        let ex_run_program: Symbol<unsafe extern "C" fn(program_path: *const c_char) -> c_double> =
            lib.get(b"EX_RunProgram\0").expect("Could not load the function EX_RunProgram");
        let program_path = CString::new(path).unwrap();
        return ex_run_program(program_path.as_ptr()) > 0.;
    };
}
