/**
 * Difference between String and &str in Owner and Borrowing angle
 *
 * String is always allocated in HEAP memory
 * &str is always just a pointer to either stack or heap
 */

/**
 * OBS: If you need to change the value (turns mutable) of a String in a function or procedure, chose use &str
 */

#[allow(unused_variables)]
fn main() {
  let some_string: String = String::from("Lucas");
  let some_str: &str = "Lucas Amigo";

  some_procedure(some_str, &some_string);
  println!("{} {}", some_string, some_str);
}

fn some_procedure(param_a: &str, param_b: &String) {
  println!("{} {}", param_a, param_b);
}
