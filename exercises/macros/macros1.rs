// macros1.rs
// Make me compile! Execute `rustmlings hint macros1` for hints :)



macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };
}

fn main() {
    my_macro!();
}
