use std::thread;
use tempfile::tempdir;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

/// create a tokio runtime
fn rt(i: i32) -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name(format!("rt-{}", i))
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap()
}

/// open a db and call flush_async()
async fn do_async_flush() {
    let t = tempdir().expect("create temp dir to store meta");
    let tmpdir = t.path().to_str().unwrap().to_string();

    let db = sled::open(tmpdir).expect("open sled db");

    db.insert(b"foo1", b"bar").unwrap();
    db.flush_async().await.unwrap();
    // db.flush().unwrap(); // blocking flush has no problem
    println!("do_async_flush returning");
}

fn main() -> anyhow::Result<()> {
    let mut handles = vec![];
    for i in 0..10 {
        let th = thread::spawn(move || {
            rt(i).block_on(async {
                do_async_flush().await;
                println!("do_async_flush() done!");
            });
        });

        handles.push(th);
    }

    for h in handles {
        h.join().expect("joining");
        println!("joined");
    }

    println!("all done");

    Ok(())
}
