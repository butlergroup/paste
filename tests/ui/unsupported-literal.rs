use macro_paste::macro_paste;

macro_paste! {
    fn [<x 1e+100 z>]() {}
}

macro_paste! {
    // `xyz` is not correct. `xbyz` is certainly not correct. Maybe `x121z`
    // would be justifiable but for now don't accept this.
    fn [<x b'y' z>]() {}
}

macro_paste! {
    fn [<x b"y" z>]() {}
}

macro_paste! {
    fn [<x br"y" z>]() {}
}

fn main() {}
