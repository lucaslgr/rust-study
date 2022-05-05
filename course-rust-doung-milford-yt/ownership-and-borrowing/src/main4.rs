/**
 * Borrowing and Owner exceptions:
 *
 * - You can keep the reference of a memory in many others variables (pointers) but only the owner can change this memory data
 * - AND the owner just can change the memory data after the use of other pointers
 */

fn main() {
  let mut var_a = String::from("Lucas");

  var_a.push('c'); // HERE is before the var_b and var_c exists

  let var_b = &var_a; //Just refering the data of var_a
  let var_c = &var_a; //Just refering the data of var_a

  //RUST can't allow change the var_a memory while the var_b or var_c will be used in the future by println!
  //var_a.push('b'); //HERE we have a COMPILE ERROR

  println!("{} {} {}", var_a, var_b, var_c);
  var_a.push('b'); // HERE is after the use of var_b and var_c to read the memory
}
