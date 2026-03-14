#![allow(clippy::let_underscore_untyped)]

use macro_paste::macro_paste;
use paste_test_suite::paste_test;

#[test]
fn test_attr() {
    macro_paste! {
        #[paste_test(k = "val" "ue")]
        struct A;

        #[paste_test_suite::paste_test(k = "val" "ue")]
        struct B;

        #[::paste_test_suite::paste_test(k = "val" "ue")]
        struct C;

        #[paste_test(k = "va" [<l u>] e)]
        struct D;
    }

    let _ = A;
    let _ = B;
    let _ = C;
    let _ = D;
}

#[test]
fn test_paste_cfg() {
    macro_rules! m {
        ($ret:ident, $width:expr) => {
            macro_paste! {
                #[cfg(any(feature = "protocol_feature_" $ret:snake, target_pointer_width = "" $width))]
                fn new() -> $ret { todo!() }
            }
        };
    }

    struct MacroPaste;

    #[cfg(target_pointer_width = "64")]
    m!(MacroPaste, 64);
    #[cfg(target_pointer_width = "32")]
    m!(macro_paste, 32);

    let _ = new;
}

#[test]
fn test_path_in_attr() {
    macro_rules! m {
        (#[x = $x:ty]) => {
            stringify!($x)
        };
    }

    let ty = macro_paste! {
        m!(#[x = foo::Bar])
    };

    assert_eq!("foo::Bar", ty);
}
