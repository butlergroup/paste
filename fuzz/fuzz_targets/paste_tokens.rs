#![no_main]

use libfuzzer_sys::fuzz_target;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        let code = format!(
            r#"
            use macro_paste::paste;
            paste! {{
                {}
            }}
        "#,
            s
        );
        let _ = std::panic::catch_unwind(|| {
            let _ = syn::parse_file(&code);
        });
    }
});
