use std::{sync::Once, fs};
use std::io::Write;

static INIT: Once = Once::new();

pub fn initialize_file(path: &str) -> String {

    const OUTPUT_FOLDER: &str = "ruce_out";

    INIT.call_once(|| {


        if !fs::metadata(OUTPUT_FOLDER).is_ok() {
            fs::create_dir(OUTPUT_FOLDER).expect("Failed to create output folder");
        }


        let file_path = format!("{}/{}", OUTPUT_FOLDER, path);

        let mut file = fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(file_path)
            .unwrap();

        let content = r#"// This file is generated by ruce_macro
// Do not edit this file manually

#include <emscripten.h>

"#;

        write!(file, "{}", content).expect("Failed to write to file");
    });

    return format!("{}/{}", OUTPUT_FOLDER, path);
}