fn main() {
    println!("Hello, world!");

    function_1();
    function_2(5);
    function_3();

    let x = five();
    println!("The value of x is: {x}");
}

fn function_1() {
    println!("Another function.");
}

fn function_2(x: i32) {
    println!("The value of x is: {x}");
}

// Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
fn function_3() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

//In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value
fn five() -> i32 {
    5
}
