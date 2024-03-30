mod main2;

use std::fs;
use std::net::SocketAddr;
use std::error::Error;


fn main() {
    // --snip--
    println!("In file {}", "test.txt");

    let mut contents = fs::read_to_string("src/test.txt")
        .expect("Should have been able to read the file");

    let mut var=String::from("\n");

    let mut result=String::new();
    let mut var= String::new();
    for i in 0..10 {
         result = contents.clone() + "\n";
         var.push_str(result.as_str());
    }

    println!("{}", var);
    fs::write("src/test.txt", var).expect("Unable to write file");

}


