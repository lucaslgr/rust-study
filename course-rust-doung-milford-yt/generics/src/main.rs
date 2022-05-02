/**
 * Using Generics into Struct fields
 * Implementing generics in a block of function and dealing with the needs traits
 */

struct LucasStruct<T, U> {
  a: T,
  b: U,
}

impl<T, U> LucasStruct<T, U>
where
  T: std::fmt::Debug,
  U: std::fmt::Debug,
{
  fn log_something(&self) {
    println!("{:?} {:?}", self.a, self.b);
  }
}

fn main() {
  let test = LucasStruct {
    a: "Lucas",
    b: vec![1, 2, 3, 4],
  };
  test.log_something();
}
