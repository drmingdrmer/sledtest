use std::{panic, thread};
use tempfile::tempdir;
use tempfile::TempDir;
use tokio::runtime::Builder;

use std::fmt;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    // export has a snapshot view?

    // let try_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
    //     crate::sys_common::backtrace::__rust_begin_short_backtrace(f)
    // }));

    let th2 = thread::spawn(move || {
        let rt2 = Builder::new_multi_thread()
            .worker_threads(4)
            .thread_name("rt2")
            .thread_stack_size(3 * 1024 * 1024)
            .build()
            .unwrap();

        let h = rt2.block_on(async {
            let t = tempdir().expect("create temp dir to store meta");
            let tmpdir = t.path().to_str().unwrap().to_string();

            let db = sled::open(tmpdir).expect("open sled db");

            let mut i = 0;

            loop {
                db.insert(b"foo1", b"bar").unwrap();
                db.flush_async().await.unwrap();
                if i % 100 == 0 {
                    println!("rt2: flushed :{}", i);
                }
                i += 1;
            }
        });
    });

    loop {
        let th1 = thread::spawn(move || {
            let rt1 = Builder::new_multi_thread()
                .worker_threads(4)
                .thread_name("rt1")
                .thread_stack_size(3 * 1024 * 1024)
                .build()
                .unwrap();

            let h = rt1.block_on(async {
                let h1 = tokio::spawn(async move {
                    let t = tempdir().expect("create temp dir to store meta");
                    let tmpdir = t.path().to_str().unwrap().to_string();

                    let db = sled::open(tmpdir).expect("open sled db");

                    db.insert(b"foo1", b"bar").unwrap();
                    let x = db.flush_async().await.unwrap();
                    println!("rt1 flushed");
                });
                let h2 = tokio::spawn(async move {
                    println!("-------------------------------panic1 rt1");
                    panic!("wow");
                });
            });
        });
        let try_result = panic::catch_unwind(panic::AssertUnwindSafe(|| {
            th1.join();
        }));
    }

    // th1.join().expect("joint th1");
    th2.join().expect("joint th2");

    println!("all done");

    Ok(())
}
