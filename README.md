# sledtest

This is a performance issue found in my recent work on [datafuse](https://github.com/datafuselabs/datafuse).

Snipppets in this repo shows that a **correct** fsync on Mac is quite slow:
10 ms, 20 ms or 30 ms per `File::sync_all()` and at worst 500 ms.

E.g. `make` run a bench of `File::sync_all()` on my iMac:

```
avg flush time: 48.289185ms
avg flush time: 19.535487ms
avg flush time: 19.228731ms
avg flush time: 19.345619ms
avg flush time: 18.989675ms
avg flush time: 19.042004ms
avg flush time: 19.159605ms
avg flush time: 19.265687ms
avg flush time: 19.848374ms
avg flush time: 20.031939ms
avg flush time: 19.917162ms
```

Source code: [ben_sync.rs](src/ben_sync.rs)

Reference:

- On Mac the correct fsync is a `fcntl()` followed by a `fsync()`:
  https://github.com/golang/go/issues/26650
