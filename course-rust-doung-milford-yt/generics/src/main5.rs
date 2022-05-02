/**
 * Using Generics in FUNCTIONS and implementing my own Trait on them
 * - Using a custom Struct to pass as an argument to a function using Generic Types
 */

trait SomeTrait {
  fn blah_blah(&self, a: &str, b: &str) -> String;
}

#[derive(Debug)]
struct LucasStruct {
  something: i32,
}

impl SomeTrait for LucasStruct {
  fn blah_blah(&self, a: &str, b: &str) -> String {
    self.something.to_string() + " - " + a + " - " + b
  }
}

impl SomeTrait for i32 {
  fn blah_blah(&self, _a: &str, _b: &str) -> String {
    self.to_string() + " method blah_blah implemented by SomeTrait into i32 primitive type"
  }
}

fn main() {
  //Testing our own type
  let some_var = LucasStruct { something: 10320 };
  let result = do_this(&some_var);
  println!("Result of the function do_this: {:?}", result);

  //Testing extend a primitive type with an
  let some_i32_var: i32 = 10;
  let result1 = do_this(&some_i32_var);
  println!("Result of the function do_this: {:?}", result1);
}

fn do_this<T>(some_var: &T) -> String
where
  T: std::fmt::Debug + SomeTrait,
{
  println!("{:?}", some_var);
  some_var.blah_blah("first", "second")
}
