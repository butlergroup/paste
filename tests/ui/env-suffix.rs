use macro_paste::macro_paste;

macro_paste! {
    fn [<env!("VAR"suffix)>]() {}
}

fn main() {}
