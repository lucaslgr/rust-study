/**
 * Using Generics in FUNCTIONS
 */

fn main() {
  let a = testing_generics(4, 5);
  println!("Return of the function {}", a);

  let b = testing_two_generics(41, 5, "Ula ula");
  println!("Return of the function {}", b);
}

#[allow(unused_variables)]
//Specifying the Traits that the generic Type implements
fn testing_generics<T: std::ops::Add<Output = T> + std::fmt::Debug>(input_a: T, input_b: T) -> T {
  println!("Adding a:{:?} + b{:?}:", input_a, input_b);
  input_a + input_b
}

//Another way to specify Traits that the T generic type implements
#[allow(unused_variables)]
fn testing_two_generics<T, U>(input_a: T, input_b: T, input_c: U) -> T
where
  T: std::ops::Add<Output = T> + std::ops::Sub<Output = T> + std::fmt::Debug,
  U: std::fmt::Debug,
{
  println!("input a {:?}", input_a);
  println!("input b {:?}", input_b);
  println!("input c {:?}", input_c);
  input_a + input_b
}
