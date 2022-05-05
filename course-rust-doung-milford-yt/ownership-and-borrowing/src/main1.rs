/**
 * Ownership and Borrowing
 *  First you need to understand the difference between STACK and HEAP
 *
 * # STACK
 * - Fast memory creation and retrieval... speed, speed and SPEED!
 * - Memory is automatically recaptured by the program after variables go out of scope
 * - Is the default in Rust
 * - Fixed size variables... Collections CAN NOT be stack based (and String is a collection of u8's)
 * - By default STACK allocated memory is copied when you pass a variable by argument to a function or procedure
 *
 * # HEAP
 * - Flexibility
 * - Memory that can grow in size (Vector, HashMap, String, etc)
 * - Runtime performance cost (speed)
 * - Memory that can live beyond the scope that created it
 * - Memory is automatically recaptured when the last OWNER goes out of scope
 * - Heap allocated memory is cleaned up automatically when the last "Owner" of the memory goes out of scope
 * - Heap allocated memory has always one memory owner
 */

#[allow(unused_variables)]
fn main() {
    //Example of stack alocated variables
    let stack_i8: i8 = 10;
    let stack_f32: f32 = 10.1;
    let stack_bool: bool = true;
    let stack_char: char = 'a';

    //Example of heap alocated variables
    let heap_vector: Vec<i8> = Vec::new(); //vec![1,2,3,45]
    let heap_string: String = String::from("Lucas");
    let heap_i8: Box<i8> = Box::new(30);

    //No problem here with STACK alocated variables
    let stack_i8_2 = stack_i8;
    println!("{}", stack_i8_2);
    println!("{}", stack_i8);

    //Problem here with HEAP alocated variables
    let heap_i8_2 = heap_i8.clone(); //The only solution is clone the memory to a new place
                                     //OBS: Copy memory is very expensive to performance
    println!("{}", heap_i8_2);
    println!("{}", heap_i8);
}
