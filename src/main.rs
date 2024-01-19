use quote::quote;

macro_rules! js {
    ($($x:tt)*) => {
        quote! {
            $($x)*
        };
    }
}

fn main() {
    
    js! {
        console.log("Hello, world!");
    }
}
