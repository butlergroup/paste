#![allow(clippy::let_underscore_untyped)]

use macro_paste::macro_paste;

#[test]
fn test_paste_doc() {
    macro_rules! m {
        ($ret:ident) => {
            macro_paste! {
                #[doc = "Create a new [`" $ret "`] object."]
                fn new() -> $ret { todo!() }
            }
        };
    }

    struct MacroPaste;
    m!(MacroPaste);

    let _ = new;
}

macro_rules! get_doc {
    (#[doc = $literal:tt]) => {
        $literal
    };
}

#[test]
fn test_escaping() {
    let doc = macro_paste! {
        get_doc!(#[doc = "s\"" r#"r#""#])
    };

    let expected = "s\"r#\"";
    assert_eq!(doc, expected);
}

#[test]
fn test_literals() {
    let doc = macro_paste! {
        get_doc!(#[doc = "int=" 0x1 " bool=" true " float=" 0.01])
    };

    let expected = "int=0x1 bool=true float=0.01";
    assert_eq!(doc, expected);
}

#[test]
fn test_case() {
    let doc = macro_paste! {
        get_doc!(#[doc = "HTTP " get:upper "!"])
    };

    let expected = "HTTP GET!";
    assert_eq!(doc, expected);
}

// https://github.com/butlergroup/macro_paste/issues/63
#[test]
fn test_stringify() {
    macro_rules! create {
        ($doc:expr) => {
            macro_paste! {
                #[doc = $doc]
                pub struct Struct;
            }
        };
    }

    macro_rules! forward {
        ($name:ident) => {
            create!(stringify!($name));
        };
    }

    forward!(documentation);

    let _ = Struct;
}
