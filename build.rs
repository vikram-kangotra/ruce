use std::fs;
use syn::{Item, parse_str};

fn main() {

    let files = fs::read_dir("src").expect("Unable to read directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().map(|ft| ft.is_file()).unwrap_or(false))
        .filter(|entry| entry.path().extension().map(|ext| ext == "rs").unwrap_or(false))
        .map(|entry| entry.path())
        .collect::<Vec<_>>();

    let mut macro_calls = Vec::new();

    for file in files {
        let content = fs::read_to_string(&file).expect("Unable to read file");
        let content = format!("mod my_mod {{ {} }}", content);
        if content.contains("js!") {
            let tokens: Item = parse_str(&content).unwrap();
            visit_macro_calls(&tokens, &mut macro_calls);
        }
    }

    let out_dir = std::env::var("OUT_DIR").unwrap();
    let file = format!("{}/output.c", out_dir);
    let mut content = "#include <emscripten.h>\n".to_string();
    let macro_calls = macro_calls.iter().enumerate().map(|(index, call)| {
        format!("const char* js_code_{}() {{ return CODE_EXPR(\"{}\"); }}", index, call)
    }).collect::<Vec<_>>().join("\n");
    content += &macro_calls;
    fs::write(&file, content).expect("Unable to write file");
}

fn visit_macro_calls(item: &Item, macro_calls: &mut Vec<String>) {

    match item {
        Item::Fn(item_fn) => {
            for stmt in &item_fn.block.stmts {
                match stmt {
                    syn::Stmt::Macro(macro_stmt) => {
                        let macro_tokens = &macro_stmt.mac.tokens;
                        macro_calls.push(macro_tokens.to_string().escape_default().to_string());
                    }
                    _ => {
                        println!("Not a macro");
                    }
                }
            }
        },
        Item::Mod(item_mod) => {
            item_mod.content.as_ref().map(|(_, items)| {
                for item in items {
                    visit_macro_calls(item, macro_calls);
                }
            });
        },
        _ => {}
    }
}
