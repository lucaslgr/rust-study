#[allow(unused_variables)]
fn main() {
  let stack_f64: f64 = 1.;
  let heap_f64: Box<f64> = Box::new(2.);

  stack_procedure(stack_f64);
  println!("In main stack {}", stack_f64);

  #[allow(unused_doc_comments)]
  /**
   * By default Rust pass the reference of heap allocated memory, chaging the owner of the refernece.
   * If you want to retrieve this memory in the main scope you need to just borrow the reference of the memory forwhile and get back the owner after the scope of the procedure ends, using &: &heap_f64
   */
  heap_procedure(&heap_f64);
  println!("In main heap {}", heap_f64);
}

//By default Rust make a copy of stack allocated memory to another variables
fn stack_procedure(mut param: f64) {
  param += 9.;
  println!("In stack_procedure with param {}", param);
}

//By default Rust only allows one owner of the ref to heap allocated memory
fn heap_procedure(param: &Box<f64>) {
  println!("In heap_procedure with param {}", param);
}
