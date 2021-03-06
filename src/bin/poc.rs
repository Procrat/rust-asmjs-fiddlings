#![feature(link_args)]

extern crate eva_poc;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;


use std::ffi::{CStr, CString};
use std::ops::Index;
use std::os::raw::c_char;

use eva_poc::SomeTrait;


fn main() {}

#[no_mangle]
pub extern fn print_something() {
    eva_poc::print_argument("Rust function called from JS: ⚡ ∑ ♥ ")
}

#[no_mangle]
pub unsafe fn print_argument(c_string: *mut c_char) {
    let string = cstr_to_string(&c_string);
    eva_poc::print_argument(&string)
}

#[no_mangle]
pub unsafe extern fn manipulate_and_print_array(array: *const Vec<String>) {
    let array = &*array;
    let array = array.into_iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    eva_poc::manipulate_and_print_array(&array)
}

#[no_mangle]
pub unsafe extern fn manipulate_and_return_array(array: *const Vec<String>) -> *mut Vec<String> {
    let array = &*array;
    let array = array.into_iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let manipulated_array = eva_poc::manipulate_and_return_array(&array)
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
    Box::into_raw(Box::new(manipulated_array))
}

#[cfg_attr(target_arch="asmjs", link_args="--js-library docs/lib.js -s INVOKE_RUN=0")]
extern {}

#[allow(improper_ctypes)]
extern {
    fn js_phone_home(arr: *mut Vec<String>) -> *mut Vec<String>;
    fn js_query_local_storage() -> *mut c_char;
}

struct X;

impl SomeTrait for X {
    #[allow(unused_variables)]
    fn manipulate_and_return_array<'a, 'b>(&self, array: &'b [&'a str]) -> Vec<&'a str> {
        unsafe {
            let arr = array.into_iter().map(|x| x.to_owned().to_owned()).collect::<Vec<String>>();
            let new_arr = &*js_phone_home(Box::into_raw(Box::new(arr)));
            new_arr.into_iter().map(|x| x.as_str()).collect()
        }
    }
}

#[no_mangle]
pub unsafe extern fn phone_home(array: *const Vec<String>) -> *mut Vec<String> {
    let array = &*array;
    let array = array.into_iter().map(|x| x.as_str()).collect::<Vec<&str>>();
    let manipulated_array = eva_poc::phone_home(&X {}, &array)
        .into_iter()
        .map(|x| x.to_owned())
        .collect();
    Box::into_raw(Box::new(manipulated_array))
}

#[no_mangle]
pub unsafe extern fn query_local_storage() -> *mut c_char {
    js_query_local_storage()
}

#[derive(Serialize, Deserialize, Debug)]
struct Y {
    key1: String,
    key2: String,
    a_list: Vec<String>,
}

#[no_mangle]
pub unsafe extern fn change_key2(string: *mut c_char) -> *mut c_char {
    let serialised = cstr_to_string(&string);
    let mut parsed: Y = serde_json::from_str(&serialised).unwrap();
    parsed.key2 = "changed value".to_owned();
    parsed.a_list.push("item2: ⚡ ∑ ♥ ".to_owned());
    let reserialised = serde_json::to_string(&parsed).unwrap();
    string_to_cstr(&reserialised)
}

#[no_mangle]
pub extern fn write_to_indexed_db() {
    eva_poc::write_to_indexed_db()
}



#[no_mangle]
pub extern fn make_vec() -> *mut Vec<String> {
    Box::into_raw(Box::new(Vec::new()))
}

#[no_mangle]
pub unsafe extern fn get_vec_length(vec: *mut Vec<String>) -> usize {
    let vec = &*vec;
    vec.len()
}

#[no_mangle]
pub unsafe extern fn get_vec_index(vec: *mut Vec<String>, index: usize) -> *mut c_char {
    let vec = &*vec;
    string_to_cstr(vec.index(index))
}

#[no_mangle]
pub unsafe extern fn vec_append(vec: *mut Vec<String>, string: *mut c_char) {
    let vec = &mut *vec;
    vec.push(cstr_to_string(&string));
}

#[no_mangle]
pub unsafe extern fn drop_vec(vec: *mut Vec<String>) {
    drop(Box::from_raw(vec))
}

unsafe fn cstr_to_string(c_string: &*mut c_char) -> String {
    CStr::from_ptr(*c_string).to_string_lossy().into_owned()
}

fn string_to_cstr(string: &str) -> *mut c_char {
    CString::new(string).unwrap().into_raw()
}
