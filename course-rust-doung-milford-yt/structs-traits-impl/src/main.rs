#[allow(dead_code)]
/**
 * Structs
 * - Structs are similar classes in another languages
 * - Represent complex data types that you define
 * - Structs are like "objects", but have differences
 *  - Rust doesn't have inheritance
 *  - Rust does have methods
 *  - Rust has "Traits" - Similar to polymorphism for object oriented
 * - Derived traits can be done using macros
 */
mod random_info;
use random_info::*;

#[derive(Debug)] //This macro implements the debug Trait on the struct
struct DougsData {
    some_int: i32,
    some_float: f64,
    some_bool: bool,
    random: RandomInfo,
}

//Implementing the Trait on DougsData struct
impl SomeTrait for DougsData {
    fn is_valid(&self) -> bool {
        true
    }
}

//Any struct the implements the trait "SomeTrait" can be passed into the parameter
fn print_if_is_valid(check_me: &dyn SomeTrait) {
    if check_me.is_valid() {
        println!("Yeahhh!");
    }
}

//Implementing the Default trait in DougData struct
impl Default for DougsData {
    fn default() -> Self {
        Self {
            some_bool: true,
            some_float: 10.3,
            some_int: 2,
            random: RandomInfo::new(true),
        }
    }
}

/**
 * Implementing new functionalities on RandomInfo
 */
impl RandomInfo {
    pub fn is_larger(&self, some_int: i32) -> bool {
        self.some_int < some_int
    }
}

#[allow(unused_variables)]
fn main() {
    let mut doug_data_var = DougsData {
        some_int: 38,
        some_float: 10.9,
        some_bool: false,
        random: RandomInfo {
            call_count: 0,
            some_bool: false,
            some_int: 2,
        },
    };

    let doug_data_var0 = DougsData::default();
    println!("{:?}", doug_data_var0);

    let is_this_smaller = doug_data_var.random.is_smaller(2);
    let is_this_larger = doug_data_var.random.is_larger(12);
    //Calling a function signed on the Trait
    let is_valid = doug_data_var.random.is_valid();

    let doug_data_var1 = DougsData {
        some_int: 38,
        some_float: 10.9,
        some_bool: false,
        random: RandomInfo::new(true),
    };

    doug_data_var.some_int = 22;

    let doug_data_var2 = DougsData {
        some_int: 12,
        ..doug_data_var
    };

    let random_info_struct_var = RandomInfo {
        call_count: 0,
        some_bool: false,
        some_int: 2,
    };

    print_if_is_valid(&random_info_struct_var);
    print_if_is_valid(&doug_data_var1);
}
