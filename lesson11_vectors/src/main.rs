fn main() {
    let v=vec![1,2,3];
    let v2=vec![0;10];
    let v3=vec![1,2,3,4,5];
    let v4=&v3[1..4];
    
    println!("{:?}",v4,);
    println!("{:?}",v2);
    println!("Hello, world!");
    vectors();
}

fn vectors(){
    #[derive(Debug)]
    enum SpreadSheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row=vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Text(String::from("blue")),
        SpreadSheetCell::Float(10.12),
    ];
    println!("{:?}",row);
}
