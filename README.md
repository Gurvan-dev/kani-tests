# Verification of RISC-V Vector inline assembly in Rust

```
git submodule update --init --recursive
cd kani-dev && cargo build-dev
```

# Benchmarks

TODO: Give a small note for each memset about its specific implementation
details

| Benchmark                             | Time (s)  | Time (m)  | Time (h)  |
|---------------------------------------|-----------|-----------|-----------|
| memset1 (len=8)                       | 1282.98   | 21.38     | 0.35      |
| memset2 (len=8)                       | 169.58    | 2.82      | 0.04      |
|---------------------------------------|-----------|-----------|-----------|
| memset1 (len=16)                      | 2105.44   | 35        | 0.58      |
| memset2 (len=16)                      | 190.47    | 3.17      | 0.05      |
|---------------------------------------|-----------|-----------|-----------|
| memset8 (len=32)                      | 0.77      | 0         | 0         |
| memset9 (len=32)                      | 194.97    | 3.24      |           |
| memset10 (len=32)                     | 194.26    | 3.23      |           |
| memset11 (len=32)                     | 192.90    | 3.21      |           |
| memset12 (len=32)                     | 106.91    | 1.78      |           |
| memset13 (len=32)                     | 3584.53   | 59.74     |           |
| memset14 (len=32)                     | 3573.991  | 59.56     |           |
| memset15 (len=32)                     | 3496.30   | 58.27     |           |
| memset16 (len=32)                     | 3458.78   | 57.64     |           |
| memset17 (len=32)                     |           |           |           |
| memset18 (len=32)                     | 1181.14   |           |           |
| memset19 (len=32)                     |           |           |           |
|---------------------------------------|-----------|-----------|-----------|
| memset1 (len=64)                      | 7425.88s  | 123.76    | 2.06      |
| memset3 (len=64)                      | 1.21      | 0         | 0         |
| memset4 (len=64)                      | 0.54      | 0         | 0         |
| memset5 (len=64)                      | 324.28s   | 0         | 0         |
| memset6 (len=64)                      | 320.28s   | 0         | 0         |
| memset7 (len=64)                      | 7457.27s  | 124.28    | 2.13      |
| memset8 (len=64)                      | 1.47      | 0         | 0         |
| memset9 (len=64)                      | 254.37    |           |           |
| memset10 (len=64)                     | 252.59    |           |           |
| memset11 (len=64)                     | 258.32    |           |           |
| memset12 (len=64)                     | 117.69    |           |           |
| memset_broken (len=8)                 | 1290.93   | 21.5      | 0.35      |
| add1 (len=4)                          | 939.68    | 15.66     | 0.26      |
| add1 (len=1)                          | 921.48    | 15.52     | 0.25      |
| add_full (len=4)                      | 1196.38   | 19.93     | 0.33      |
| xor_cipher (inp_len=64, key_len=64)   | 13403.12  | 223.38    | 3.72      |
