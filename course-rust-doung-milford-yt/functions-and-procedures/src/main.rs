#[allow(unused_variables)]
/**
 * Functions and Procedures
 * - The only diference is that functions return a value and procedures not
 * - Is usually preferable to define your parameters as string slices &str
 */
fn main() {
    let returned_data = some_function(2.2, 10);
    println!("returned_data = {}", returned_data);
    some_procedure(10.2, 8);
    some_str_procedure("bababibibibo");
    let string_slice_var: &str = "Howdy";
    some_str_procedure(string_slice_var);
    let string_var: String = String::from("I'm a REAL String :D");
    some_str_procedure(&string_var);
    some_string_procedure(string_var);
}

fn some_string_procedure(param: String) {
    println!("I'm in some_string_procedure with param {}", param);
}

fn some_str_procedure(param: &str) {
    println!("I'm in some_str_procedure with param {}", param);
}

fn some_procedure(param_a: f32, param_b: i8) {
    println!("I'm in some_procedure with a {} b {}", param_a, param_b);
}

fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I'm in some_function");

    // # 3 ways to change the type of variable
    // 10f32 //Specifying the type of the number method #1
    // 10_f32 //Specifying the type of the number method #1
    // 10 as f32 //Specifying the type of the number method #2

    param_a + param_b as f32
}
