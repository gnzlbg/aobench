# Ambient Occlusion Benchmark

> Originally written by Syoyo Fujita: https://github.com/syoyo/aobench

`aoench` is a small ambient occlusion renderer for benchmarking realworld
floating point performance in various languages.

## Instructions


To run it with the default target options (replace `${NAME}` with an algorithm name):

```
> cargo run --release -- 800 600 --algo ${NAME}
```

|  `${NAME}`   | time [ms] | speedup [-] |
|--------------|-----------|-------------|
| `scalar`     |      6705 |     1.0x    |
| `vector`     |      3174 |     2.0x    |
| `scalar_par` |      2638 |     2.5x    |
| `vector_par` |      1360 |     4.9x    |

And using `RUSTFLAGS` to set the target CPU:

```
> RUSTFLAGS="-C target-cpu=native" cargo run --release -- 800 600 --algo ${NAME}
```
