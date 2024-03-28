fn main() {
    let mut s1=String::from("hello");
    let s2=String::from("world");
    s1.push_str(&s2);
    println!("{}",s1);
    println!("{}",s2);

    let s3=s1+&s2;
    println!("{}",s3);
   
}
