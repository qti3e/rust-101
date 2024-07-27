use std::fmt::Display;

trait MyToString {
    fn to_string(&self) -> String;
}

impl MyToString for u32 {
    fn to_string(&self) -> String {
        String::from("the type is u32")
    }
}

impl MyToString for u64 {
    fn to_string(&self) -> String {
        String::from("the type is u64")
    }
}

fn use_something_with_my_to_string<T: MyToString + Display>(s: T) -> T {
    println!("{s}: {}", s.to_string());
    s
}

fn main() {
    let x = use_something_with_my_to_string(12u64);
    let x = use_something_with_my_to_string(14u32);
}
