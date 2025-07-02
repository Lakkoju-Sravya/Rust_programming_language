/*
Write a program to implement the Scope and Shadowing
*/

fn main() {
    let  x = 5;
    println!("Outside block: x = {}", x);
    {
        let x = x + 10;
        println!("inside block: x = {}",x);
        let y = 20;
        println!("inside block: y = {}",y);
    }
    println!("Outside block after inner:x = {}",x);
    let x = x*2;
    println!("After shadowing in main scop:x = {}", x);
}
