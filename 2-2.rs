 fn main() {
  let num = 20;
  let num_ref = &num;
  println!("original value: {}", num);
  println!("Borrowed value(refernce): {}", num_ref);
  println!("Dereferenced value: {}", * num_ref);
  let mut num2 = 50;
  let num2_ref = &mut num2;
  *num2_ref += 10;
  println!("modified value after mutable borrow: {}", num2);
}
