use ruce_macro::js_code;

#[js_code]
fn console_log() {
    console.log("Hello, world!");
}

#[js_code]
fn alert() {
    alert("Success!");
}

#[no_mangle]
fn run() {

    unsafe {
        for _ in 0..10 {
            console_log();
        }

        alert();
    }
}
