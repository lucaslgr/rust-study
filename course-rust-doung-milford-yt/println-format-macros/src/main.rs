#[derive(Debug)]
struct LucasData {
    pub a: i32,
    pub b: i32,
}

fn main() {
    let data: i32 = 312093;
    let more_data: i32 = 11111;
    println!("My data is {}", data);
    println!("My data 2: {1} my data 1: {0}", data, more_data);
    println!(
        "My lastname is {last_name} and my first name is {first_name}",
        first_name = "Lucas",
        last_name = "Guimaraes"
    );

    //Printing using display trait
    let lucas_data1 = LucasData { a: 32, b: 31231 };
    let lucas_data2 = LucasData { a: 1515, b: 3128 };
    println!("Lucas data is {:?}", lucas_data1);
    println!("Printing Lucas data formated {:#?}", lucas_data1);
    println!(
        "Printing both lucas data {:#?} and {:#?}",
        lucas_data1, lucas_data2
    );
    println!(
        "Printing both lucas data specifying the positions {1:#?} and {0:#?}",
        lucas_data1, lucas_data2
    );

    //Format statement
    let some_formatted_string = format!(
        "Printing both lucas data specifying the positions {1:#?} and {0:#?}",
        lucas_data1, lucas_data2
    );
    println!("{}", some_formatted_string);
}
