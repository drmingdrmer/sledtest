use std::time::{Duration, Instant};

use std::fs::File;
use std::io::prelude::*;

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let mut f = File::create("foo.txt")?;

    let mut total: Duration = Default::default();
    let n = 1000;

    for i in 0..n {
        let t0 = Instant::now();

        f.write_all(b"bar")?;
        f.sync_all()?;

        let d = t0.elapsed();

        total += d;
        if i % 100 == 1 {
            println!("avg flush time: {:?}", total / i);
        }
    }
    println!("avg flush time: {:?}", total / n);

    Ok(())
}
