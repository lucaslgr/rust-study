/**
 * Tuples group together data
 * Data elements in tuples don't have names
 *
 * Why use tuples?
 *  - You can do certain abstract things with tuples
 *  - You can use tuples instead of explicit data structures
 */

#[allow(unused_variables)]

fn main() {
    let some_tuple = (2, 3.5, "a", "b".to_string(), 'c', (1.1, 2.2));

    println!("My data by index is {} {}", some_tuple.1, some_tuple.0);
    println!("My full tuple is {:?}", some_tuple);

    //Ways to catch a value inside a tuple that is inside another tuple
    let some_value1 = some_tuple.5 .1; //First way
    let some_value2 = (some_tuple.5).0; //Second way

    println!(
        "The value caught from the tuple declared inside the main tuple: {} {}",
        some_value1, some_value2
    );

    let rgb_color = get_some_rgb_color();
    println!("R {} G {} B {}", rgb_color.0, rgb_color.1, rgb_color.2);
    //OR
    let (my_red, my_green, my_blue) = get_some_rgb_color();
    println!("r {} g {} b {}", my_red, my_green, my_blue);

    //rgba
    let some_other_color_rgba: (u8, u8, u8, u8) = (0, 10, 20, 30);

    //empty tuples
    let empty_tuple = ();
    //example using empty tuple
    //Curiosity: The procedures always must return a empty tuple
    match some_other_color_rgba.2 {
        0..=200 => println!("Yeah!"),
        _ => (),
    }
}

fn get_some_rgb_color() -> (u8, u8, u8) {
    //Some logic
    (1, 2, 3)
}
