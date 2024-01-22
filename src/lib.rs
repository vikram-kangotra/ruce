use ruce_macro::js_function;

#[js_function]
fn console_log(x: i32) { 
    console.log(x);
}

#[js_function]
fn alert() { 
}

#[no_mangle]
fn run() {
    console_log(0);
}
