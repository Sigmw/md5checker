use md5;
use std::env;
use std::fs;
fn main() {
    let mut args = env::args().into_iter();
    args.next();
    for i in args.next() {
        match fs::read(&i) {
            Ok(x) => println!("{:?} {}", md5::compute(x), i),
            Err(x) => println!("md5checker: {}: {}", i, x),
        }
    }
}
