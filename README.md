# Benchmarking Rust bitvec crates with Conway's Game of Life

It's hard to benchmark bitvecs: we effectively want to measure the overhead of a couple of very fast
instructions. Perhaps slightly offset by some gain in cache locality, when compared to `Vec<bool>`.

To complicate things further, some existing benchmarks focus just on random accesses (and maybe end
up mostly measuring the random number generator) or bit operations between two entire sets (e.g.
`BitAnd`). And those cases aren't representative of all applications.

So this is where Conway's Game of Life comes in: it's still very cache friendly, but counting the
live neighbors, results in an access pattern that's slightly more complex than just a linear scan.

However, this is in no way the definitive bitvec benchmark. It probably isn't even a very good one.
You should still measure your real-world application.

## TL; DR

Summary output with an i7-8700K @ 4800 MHz:

```
Game of Life/Vec<bool>/8×8
                        time:   [281.93 ns 282.25 ns 282.58 ns]
                        thrpt:  [226.48 Melem/s 226.75 Melem/s 227.01 Melem/s]
Game of Life/Vec<bool>/64×64
                        time:   [18.257 µs 18.260 µs 18.262 µs]
                        thrpt:  [224.29 Melem/s 224.32 Melem/s 224.35 Melem/s]
Game of Life/Vec<bool>/256×256
                        time:   [287.25 µs 287.66 µs 288.01 µs]
                        thrpt:  [227.55 Melem/s 227.83 Melem/s 228.15 Melem/s]
Game of Life/Vec<bool>/1024×1024
                        time:   [4.5818 ms 4.5866 ms 4.5910 ms]
                        thrpt:  [228.40 Melem/s 228.62 Melem/s 228.86 Melem/s]
Game of Life/Vec<bool>/4096×4096
                        time:   [84.690 ms 85.399 ms 86.097 ms]
                        thrpt:  [194.86 Melem/s 196.46 Melem/s 198.10 Melem/s]
Game of Life/Vec<bool>/16384×16384
                        time:   [1.2862 s 1.2867 s 1.2874 s]
                        thrpt:  [208.50 Melem/s 208.62 Melem/s 208.71 Melem/s]
Game of Life/bitvec::vec::BitVec/8×8
                        time:   [694.03 ns 694.39 ns 694.86 ns]
                        thrpt:  [92.105 Melem/s 92.167 Melem/s 92.215 Melem/s]
Game of Life/bitvec::vec::BitVec/64×64
                        time:   [47.336 µs 47.354 µs 47.383 µs]
                        thrpt:  [86.445 Melem/s 86.497 Melem/s 86.531 Melem/s]
Game of Life/bitvec::vec::BitVec/256×256
                        time:   [763.85 µs 764.96 µs 765.83 µs]
                        thrpt:  [85.575 Melem/s 85.673 Melem/s 85.797 Melem/s]
Game of Life/bitvec::vec::BitVec/1024×1024
                        time:   [12.204 ms 12.212 ms 12.219 ms]
                        thrpt:  [85.813 Melem/s 85.863 Melem/s 85.917 Melem/s]
Game of Life/bitvec::vec::BitVec/4096×4096
                        time:   [194.89 ms 195.00 ms 195.12 ms]
                        thrpt:  [85.985 Melem/s 86.036 Melem/s 86.086 Melem/s]
Game of Life/bitvec::vec::BitVec/16384×16384
                        time:   [3.1208 s 3.1224 s 3.1238 s]
                        thrpt:  [85.932 Melem/s 85.972 Melem/s 86.014 Melem/s]
Game of Life/bit_vec::BitVec/8×8
                        time:   [919.26 ns 919.41 ns 919.56 ns]
                        thrpt:  [69.599 Melem/s 69.610 Melem/s 69.621 Melem/s]
Game of Life/bit_vec::BitVec/64×64
                        time:   [60.329 µs 60.332 µs 60.337 µs]
                        thrpt:  [67.885 Melem/s 67.891 Melem/s 67.894 Melem/s]
Game of Life/bit_vec::BitVec/256×256
                        time:   [973.14 µs 973.81 µs 974.64 µs]
                        thrpt:  [67.241 Melem/s 67.298 Melem/s 67.345 Melem/s]
Game of Life/bit_vec::BitVec/1024×1024
                        time:   [15.622 ms 15.636 ms 15.649 ms]
                        thrpt:  [67.007 Melem/s 67.064 Melem/s 67.123 Melem/s]
Game of Life/bit_vec::BitVec/4096×4096
                        time:   [248.63 ms 248.89 ms 249.21 ms]
                        thrpt:  [67.320 Melem/s 67.409 Melem/s 67.480 Melem/s]
Game of Life/bit_vec::BitVec/16384×16384
                        time:   [4.0044 s 4.0061 s 4.0077 s]
                        thrpt:  [66.980 Melem/s 67.007 Melem/s 67.035 Melem/s]
```
