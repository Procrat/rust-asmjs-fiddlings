use std::fs::File;
use std::path::Path;
use std::io::prelude::*;


pub trait SomeTrait {
    fn manipulate_and_return_array<'a, 'b>(&self, array: &'b [&'a str]) -> Vec<&'a str>;
}

pub fn print_argument(string: &str) {
    println!("String: {}", string);
}

pub fn manipulate_and_print_array(array: &[&str]) {
    for string in array.iter().rev() {
        println!("String: {}", string)
    }
}

pub fn manipulate_and_return_array<'a, 'b>(array: &'b [&'a str]) -> Vec<&'a str> {
    array.iter().cloned().rev().collect()
}

pub fn phone_home<'a, 'b, T: SomeTrait>(t: &T, array: &'b [&'a str]) -> Vec<&'a str> {
    t.manipulate_and_return_array(array)
}

pub fn write_to_indexed_db() {
    let path = Path::new("/indexed_db/test.txt");
    match File::open(path) {
        Ok(mut file) => {
            let mut string_buffer = String::new();
            file.read_to_string(&mut string_buffer).unwrap();
            println!("{:?} existed with contents: {}", path, string_buffer);
        },
        Err(_) => {
            // File probably just doesn't exist, so let's create it
            let mut file = File::create(path).unwrap();
            file.write_all(b"Hello, world!").unwrap();
            file.sync_all().unwrap();
            println!("Done writing to {:?}!", path);
        }
    }
}
