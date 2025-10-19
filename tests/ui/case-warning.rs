#![deny(warnings)]

use paste::paste;

macro_rules! m {
    ($i:ident) => {
        paste! {
            pub fn [<foo $i>]() {}
        }
    };
}

m!(_bar);

fn main() {}
