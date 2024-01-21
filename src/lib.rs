use ruce_macro::js_function;

#[js_function]
fn console_log(x: i32) { 
}

#[js_function]
fn alert() { 
}

#[no_mangle]
fn run() {

    alert();
}
