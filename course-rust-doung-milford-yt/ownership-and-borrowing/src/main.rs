/**
 * Borrowing and Ownership in a Struct view point
 * - Structs can have thousands of fields inside, so by default all struct is passed as reference in functions and procedures.
 */

#[derive(Debug)]
struct LucasStruct {
  a: i32,
  b: f64,
}

#[allow(unused_variables)]
fn main() {
  let mut var_1 = LucasStruct { a: 19, b: 29.1 };
  some_procedure(&mut var_1);
  println!("{:?}", var_1);
}

fn some_procedure(param_a: &mut LucasStruct) {
  param_a.a += 29;
  println!("{:?}", param_a);
}
