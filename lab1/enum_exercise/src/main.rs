use std::time::{Duration, SystemTime};
use std::thread::sleep;

pub enum Error {
    Simple(SystemTime),
    Complex(SystemTime, String),
    // print_error(e),
}

fn main() {
    let now = SystemTime::now();
    sleep(Duration::new(1, 0));
    let error = Error::Simple(now);
    let print_error = |e: Error| -> () {
        match e {
            Error::Simple(sys) => {
                match now.elapsed() {
                    Ok(elapsed) => {
                        // it prints '2'
                        println!("sono nel simple",);
                        println!("{:#?}", elapsed.as_secs());
                    }
                    Err(e) => {
                        // an error occurred!
                        println!("Error: {e:?}");
                    }
                }
            }
            Error::Complex(sys, s) => {
                match now.elapsed() {
                    Ok(elapsed) => {
                        // it prints '2'
                        println!("sono nel compelx",);
                        sleep(Duration::new(2, 0));
                        println!("{} {}", elapsed.as_secs(),s);
                    }
                    Err(e) => {
                        // an error occurred!
                        println!("Error: {e:?}");
                    }
                }
            }
        }
    };
    print_error(error);
}
