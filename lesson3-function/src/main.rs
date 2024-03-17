fn main() {
    println!("Hello, world!");
    another_function();
    function_with_params(10, 20);
    let result = function_with_return_value(10, 20);
    println!("result: {}", result);
}

fn another_function(){
    println!("Another function.");
}
//Function with params
fn function_with_params(x:i32, y:i32){
    println!("x: {}", x);
    println!("y: {}", y);
}

//Function with return value
fn function_with_return_value(x:i32, y:i32) -> i32{
    x + y
}

