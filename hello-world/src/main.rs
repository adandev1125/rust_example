extern crate persons_lib;

use std::{thread, time::Duration};

use persons_lib::persons::*;

fn main() {
    let mut adan = Person::create_person("Adan", Gender::Male, 32);
    
    adan.display();
    
    adan.walk();

    let thread_handle = thread::spawn(move || {
        for i in 1..10 {
            println!("Hello year, {}", 2000 + i);
            adan.increase_age();
            thread::sleep(Duration::from_millis(1000))
        }
        println!("adan's age is now {}.", adan.age());
    });

    thread_handle.join().unwrap();
}
