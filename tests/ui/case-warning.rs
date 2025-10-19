#![deny(warnings)]

use paste::paste;

macro_rules! m {
    ($i:ident) => {
        paste! {
            fn [<foo $i>]() {
                let unused = 42;  // will trigger "unused variable" warning
            }
        }
    };
}

m!(bar);

fn main() {}
