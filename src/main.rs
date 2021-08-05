use std::{panic, thread};
use tempfile::tempdir;
use tempfile::TempDir;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

use std::fmt;
use std::time::Duration;

fn rt(i: i32) -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name(format!("rt-{}", i))
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap()
}

async fn doit(db: sled::Tree) {
    db.insert(b"foo1", b"bar").unwrap();
    db.flush_async().await.unwrap();
    println!("doit returning");
}

fn main() -> anyhow::Result<()> {
    let t = tempdir().expect("create temp dir to store meta");
    let tmpdir = t.path().to_str().unwrap().to_string();

    let db = sled::open(tmpdir).expect("open sled db");

    let mut handles = vec![];
    for i in 0..10 {
        let th = {
            let t = db.open_tree(format!("tree-{}", i)).unwrap();

            thread::spawn(move || {
                rt(i).block_on(async {
                    doit(t).await;
                    println!("doit() done!");
                });
            })
        };
        handles.push(th);
    }

    for h in handles {
        h.join().expect("joining");
        println!("joined");
    }

    println!("all done");

    Ok(())
}
