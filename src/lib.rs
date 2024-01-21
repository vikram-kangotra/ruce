use ruce_macro::js_code;

#[js_code]
fn console_log(x: i32) { 
    console.log(x);
}

#[js_code]
fn alert() { 
    alert("Success!"); 
}

#[no_mangle]
fn run() {

    for _ in 0..10 {
        console_log(10);
    }

    alert();
}
