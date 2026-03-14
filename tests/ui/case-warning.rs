#![deny(warnings)]

use macro_paste::macro_paste;

macro_rules! m {
    ($i:ident) => {
        macro_paste! {
            pub fn [<foo $i>]() {}
        }
    };
}

m!(Bar);

fn main() {}
