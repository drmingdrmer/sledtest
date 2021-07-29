
import time

import os, sys


path = "./xp.txt"
fcont = "foo"
with open(path, 'w') as f:
    t0 = time.time()
    for i in range(10*1000):
        f.write(fcont)
        f.flush()
        os.fsync(f.fileno())

        d = time.time() - t0

        if i % 100 == 1:
            print("avg: ", d*1000 /i,  "ms")
