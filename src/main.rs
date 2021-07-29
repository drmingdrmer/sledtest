use tempfile::tempdir;
use tempfile::TempDir;
use tokio::runtime::Builder;

use log::{debug, error, info, log_enabled, Level};

use std::fmt;
use std::time::{Duration, Instant};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    let t = tempdir().expect("create temp dir to store meta");
    let tmpdir = t.path().to_str().unwrap().to_string();

    let db = sled::Config::default()
        .path(tmpdir)
        .cache_capacity(100_000_000)
        .print_profile_on_drop(true)
        .mode(sled::Mode::HighThroughput)
        .open()
        .expect("open sled");

    // sled::open(tmpdir).expect("open sled db");

    let mut total: Duration = Default::default();
    let n = 10_000;

    for i in 0..n {
        let foo = format!("foo{}", i);
        let k = sled::IVec::from(foo.as_str());
        let v = sled::IVec::from(foo.as_str());

        let t0 = Instant::now();
        db.insert(&k, v).unwrap();
        db.flush().unwrap();
        // db.flush_async().await.unwrap();

        let d = t0.elapsed();

        total += d;
        if i % 100 == 1 {
            println!("avg flush time: {:?}", total / i);
        }
    }
    println!("avg flush time: {:?}", total / n);

    Ok(())
}
