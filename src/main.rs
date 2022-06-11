mod rustyframe;
mod rustytest;

use std::fs;
use std::path::Path;
use std::time::Instant;
use crate::rustyframe::Rustyframe;
use crate::rustytest::RustyTest;

fn main() {
    let path = Path::new("test_files\\BRK-B.csv");

    let now = Instant::now();

    let big = RustyTest::from_csv(path);
    
    println!("{} ms", now.elapsed().as_millis()); // 436 read
    big.print();
}

