fn main() {
    println!("Hello, world!");
    printing::say_goodbye();
    printing::announce_time()
}
mod printing {
    pub mod timing;
    pub fn say_goodbye() {
        println!("Goodbye");
    }
    pub use timing::tell_time as announce_time;
}

// Output:
// Hello, world!
// Goodbye
// The time is 2023-05-10 21:09:16.096791400 +05:30
