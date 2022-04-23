#[allow(unused_doc_comments)]
#[allow(unused_variables)]

fn main() {
    let some_bool = true;
    let some_int1 = 11;
    let some_int2 = 200;

    /**
     * Simple case of use
     */
    if some_bool == true || (some_int1 > 10 && some_int2 == 200) {
        println!("Hit if branch");
    } else if some_int1 == 11 {
        println!("Hit else if branch");
    } else {
        println!("Hit the else branch");
    }

    /**
     * Inline use
     * - You need to set some value in each statement without ";" to indicates that this value need be returned to the variable
     */
    let var_from_inline = if some_int1 == 10 {
        300
    } else if some_int1 == 20 {
        10
    } else {
        200
    };

    /**
     * using Match
     */
    match some_bool {
        true => {
            println!("Hit true branch");
        }
        false => {
            println!("Hit false branch");
        }
    }

    /**
     * Using match to more than 2 cases
     */
    match some_int1 {
        0 => println!("Hit 0 branch"),
        1..=100 => println!("Between 1 and 100 branch"),
        _ => println!("Else branch"),
    }

    /**
     * Using match to more than 2 cases
     */
    match some_int1 {
        0 => println!("Hit 0 branch"),
        1..=200 => {
            println!("Between 1 and 100 branch")
        }
        _ => println!("Else branch"),
    }

    /**
     * Using match inline
     */
    let var_from_match_inline = match some_bool {
        true => 10,
        false => 20,
    };

    /**
     * Using match and or logical operator
     */
    let var_from_match = match some_int1 {
        0 => 0,
        1 | 200 => 100, //The integer is equals 1 or 200
        _ => 20,
    };
}
