/**
 * Using Generics in ENUMs
 * - You can use Generics in Enum for diferente field types in the same option
 */

#[allow(dead_code)]
#[allow(unused_variables)]
enum SomeEnum<T> {
  OptionA(T),
  OptionB(T),
  OptionC,
}

fn main() {
  let some_option1 = SomeEnum::OptionA(20);

  match some_option1 {
    SomeEnum::OptionA(a) => println!("Option A contais {}", a),
    SomeEnum::OptionB(b) => println!("Option B contais {}", b),
    SomeEnum::OptionC => println!("Option C contains nothing!"),
  }

  let some_option2 = SomeEnum::OptionB(20.10);

  match some_option2 {
    SomeEnum::OptionA(a) => println!("Option A contais {}", a),
    SomeEnum::OptionB(b) => println!("Option B contais {}", b),
    SomeEnum::OptionC => println!("Option C contains nothing!"),
  }

  let some_option3 = SomeEnum::OptionA(vec![1, 2, 3, 4]);
}
