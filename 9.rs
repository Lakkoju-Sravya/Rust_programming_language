/*
Write a program to create different types of constants print it in the output
*/

fn main() {
    const INTEGER_CONST: i32 = 42;
    const FLOAT_CONST: f64 = 3.14159;
    const CHAR_CONST: char = 'A';
    const BOOL_CONST: bool = true;
    const STRING_CONST: &str = "Hello, Rust!";
    const ARRAY_CONST: [i32; 3] = [10, 20, 30];
    const TUPLE_CONST: (i32, f64, char) = (42, 3.14, 'R');
    println!("Integer constant: {}", INTEGER_CONST);
    println!("Floating point constant: {}", FLOAT_CONST);
    println!("Character constant: {}", CHAR_CONST);
    println!("Boolean constant: {}", BOOL_CONST);
    println!("String constant: {}", STRING_CONST);
    println!("Array constant: {:?}", ARRAY_CONST);
    println!("Tuple constant: {:?}", TUPLE_CONST);
}
