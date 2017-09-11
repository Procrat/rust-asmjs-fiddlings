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
