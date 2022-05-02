/**
 * Using Generics in FUNCTIONS and implementing my own Trait on them
 * - We can implementate uncountable traits in a generic type but only one Trait can implemented directly in a function
 */

fn main() {}

trait SomeTrait {
  fn blah_blah(&self, input_a: &str, input_b: &str) -> String;
}

//Implementing the trait throught a Generic Type
fn do_this1<T>(some_var: &T) -> String
where
  T: SomeTrait + std::fmt::Debug,
{
  println!("{:?}", some_var);
  some_var.blah_blah("first", "second")
}

//Implementating the trait directly in a function's argument
fn do_this2(some_var: &dyn SomeTrait) -> String {
  some_var.blah_blah("first", "second")
}
