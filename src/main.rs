fn main() {
    println!("Hello, world!");
    if true {
        let mut test_variable: String = String::from("test");
        print!("{}", test_variable.as_mut().len())
    }
}
