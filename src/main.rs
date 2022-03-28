use std::thread;

fn main() {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(std::time::Duration::from_secs(2));
        num
    };
    println!("closure test: {}",expensive_closure(6));



    
}