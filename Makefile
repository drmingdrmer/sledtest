# benchmark fsync with rust and c
all:
	cargo run --bin ben_sync
	# gcc ben_fsync.c -o ben_fsync_c
	# time ./ben_fsync_c
	# rm c.dump
