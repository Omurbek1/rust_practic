fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);
   let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
