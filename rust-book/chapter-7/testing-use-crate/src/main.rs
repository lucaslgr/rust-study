pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {}
        }
    }

    pub mod teste {

    }

    pub mod teste2 {

    }
}

use a::{series, teste};

fn main() {
    series::of::nested_modules();
}