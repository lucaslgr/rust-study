const LUCAS_CONSTANT: i64 = 13022103129;
static mut MY_STATIC_VARIABLE: i32 = 3128;

#[allow(unused_doc_comments)]
fn main() {
    //Casting example
    println!("Example of cast: {}", example_of_cast(10, 10.1));

    /**
     * Shadowing example:
     * Is when you have a variable inside a sacope with the same name of another varible outside its scope
     *
     */
    println!("Example of shadowing...");
    example_of_shadowing();

    //Example of constants
    example_of_constants();

    //Static variables (Global Variables)
    example_of_static_variable();
}

fn example_of_cast(some_i32: i32, some_f64: f64) -> i32 {
    some_i32 + some_f64 as i32
}

fn example_of_shadowing() {
    let var_a: i32 = 10;

    println!("var_a before the scope {}", var_a);
    {
        println!("var_a inside the scope wiwhout shadowing yet {}", var_a);
        let var_a: f64 = 18.321783;
        println!("var_a inside the scope using shadowing {}", var_a);
    }

    println!(
        "var_a outside the scope again with its same value as before {}",
        var_a
    );
}

fn example_of_constants() {
    println!("Printing my constant: {}", LUCAS_CONSTANT);
    println!(
        "Printing some RUST global constant PI: {}",
        std::f32::consts::PI
    )
}

fn example_of_static_variable() {
    unsafe {
        println!("Printing a static variable: {}", MY_STATIC_VARIABLE);
    }
}
