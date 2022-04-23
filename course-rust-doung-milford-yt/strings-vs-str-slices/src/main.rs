#[allow(unused_variables)] //This is so that I don't get warnings throughout the demostration
#[allow(unused_doc_comments)]

/**
 * String and &Str
 * - Both String and &str are a grouping of characters (u8's)
 *
 * #Differences:
 * - How they are stored in memory
 * - How programmer uses
 *
 * #Strings:
 * - Heap
 * - Mutable
 *
 * #&str is more complex:
 * - Immutable (exceptions)
 * - Often allocated on the stack
 * - Sometimes a heap reference
 * - Sometimes embedded in the code
 *
 * #Can translate between String and str
 *
 * #Conclusion
 * - String is great for mutading and holding data longer than the stack is able to
 * - &str is great for runtime speed
 */
fn main() {
    //There are two types of strings in Rust

    //1º Type
    let example_str: &str = "Howdy";

    //2º Type
    let example_string: String = String::from("Partner");

    //Getting a String from &str
    let string_from_str: String = example_str.to_string();
    let string_from_str2: String = "Some hardcoded str".to_string();
    let string_from_str3: String = String::from("Some hardcoded");
    let string_from_str4: String = String::from(example_str);

    //Getting a &str from String
    let str_from_string: &str = &example_string;

    //Concatenating &str and String
    //                                        &str  +  &str
    let combine_string_literals: String = ["first", "second"].concat();
    //                                        &str  +  &str
    let combine_with_format_macro: String = format!("{} {}", "first", "second");
    /**
     * Note that to do the concatenating trick below the String type always
     * should be first and after the &str
     */
    let combine_with_plus_operator: String = String::from("example of String") + example_str;

    //Another tricks to concatenate &str or literal &str into a String
    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("some hardcoded &str");
    mut_string.push('m'); //Note the single quote for char and the method is just push, not push_str

    //Putting two Strings together
    let a = String::from('a');
    let b = String::from('b');
    let combined = a + &b + &mut_string;

    /**
     * Getting a substrings from &str
     *
     * To get the substring from 0 position just ommit the first number as the example [..5]
     * To get the substring until last position just ommit the last number as the example [3..]
     * To include the last character index you need to write like this [2..=3]
     */
    let str_from_substring1: &str = &example_str[2..3];
    let str_from_substring2: &str = &example_str[2..=3];
    let str_from_substring3: &str = &example_str[..3];
    let str_from_substring4: &str = &example_str[2..];
    println!(
        "{} {} {} {}",
        str_from_substring1, str_from_substring2, str_from_substring3, str_from_substring4
    );
    //Forcing a error of panic compiler when you overflow the index of the string
    let str_from_substring_error: &str = &example_str[0..2000];

    //Getting a specific char from &str
    let char_by_index = &example_str.chars().nth(2); //nth method returns an Option type

    //Handling the Option type in two ways
    //1º
    match char_by_index {
        Some(c) => println!("Found a char {}", c),
        None => println!("None was found"),
    }
    //2º Short form
    if let Some(c) = example_str.chars().nth(2) {
        println!("Found a char {}", c);
    }
}
