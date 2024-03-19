fn main() {
    println!("Hello, world!");
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s);
   let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    /* срезы */
   let mut string=String::from("Omurbek");
   let index=first_word(&string);
   println!("Last word is {}",index);
    string.clear();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}


//срезы
/*
напишите функцию, которая принимает строку слов, 
разделённых пробелами, и возвращает первое слово,
 которое она находит в этой строке. Если функция не находит пробела в строке, 
 вся строка должна состоять из одного слова, поэтому должна быть возвращена вся строка.
 */
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}