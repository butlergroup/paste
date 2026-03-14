use macro_paste::macro_paste;

macro_paste! {
    fn [<0 f>]() {}
}

macro_paste! {
    fn [<f '"'>]() {}
}

macro_paste! {
    fn [<f "'">]() {}
}

fn main() {}
