use macro_paste::macro_paste;

macro_paste! {
    fn [<env!("VAR" "VAR")>]() {}
}

fn main() {}
