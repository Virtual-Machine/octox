#![no_std]
use ulib::sys;
use ulib::{print, println};

fn main() {
    println!("Shutting down!...");
    sys::shutdown();
    ()
}
