use macro_paste::macro_paste;

macro_paste! {
    fn [<a env!("PASTE_UNKNOWN") b>]() {}
}

fn main() {}
