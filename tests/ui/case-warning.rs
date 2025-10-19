#![deny(warnings)]

use paste::paste;

macro_rules! m {
    ($i:ident) => {
        paste! {
            fn [<foo $i>]() {}
        }
    };
}

m!(bar);

fn main() {}
