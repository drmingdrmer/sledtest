use std::thread;
use tempfile::tempdir;
use tokio::runtime::Builder;
use tokio::runtime::Runtime;

fn rt(i: i32) -> Runtime {
    Builder::new_multi_thread()
        .worker_threads(1)
        .thread_name(format!("rt-{}", i))
        .thread_stack_size(3 * 1024 * 1024)
        .build()
        .unwrap()
}

async fn insert_flush_async(db: sled::Tree) {
    db.insert(b"foo1", b"bar").unwrap();
    db.flush_async().await.unwrap();
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
                    tokio::spawn({
                        let t = t.clone();
                        async move {
                            insert_flush_async(t).await;
                            println!("{} spawned done!", i);
                        }
                    });
                    insert_flush_async(t).await;
                    println!("{}, insert_flush_async() done!", i);
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
