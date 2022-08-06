#!/usr/bin/env python
import random
import vecstats

if __name__ == '__main__':
    vec = [random.randint(0, 1000) for _ in range(0, 1000)]
    print(repr(vecstats.vec_stats(vec)))
