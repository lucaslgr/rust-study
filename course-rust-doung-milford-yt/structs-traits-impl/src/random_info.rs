/**
 * Traits
 * It are like a inteface of POO, you can set some functions and its signatures and you can implement this Trait in this Struct
 */
pub trait SomeTrait {
  fn is_valid(&self) -> bool;
  // fn get_the_better_one(&self, some_other_dude: &Self) -> Self;
}

/**
 * If you not make explicit that the struct and what fields are publics Rust will assume that all is private by default
 */
#[derive(Debug, Copy, Clone)] //This macro implements the Debug Trait on the struct
pub struct RandomInfo {
  pub call_count: i64,
  pub some_bool: bool,
  pub some_int: i32,
}

/**
 * Implementing that trait on the RandomInfo struct
 */
impl SomeTrait for RandomInfo {
  fn is_valid(&self) -> bool {
    self.some_bool
  }
}

impl RandomInfo {
  pub fn new(param_a: bool) -> Self {
    Self {
      call_count: 0,
      some_bool: !param_a,
      some_int: 2,
    }
  }

  pub fn is_smaller(&mut self, compare_to: i64) -> bool {
    self.call_count += 1;

    self.some_int < compare_to as i32
  }
}
