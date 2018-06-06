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

## Overview

There are 4 main pieces in the `aobench` benchmark:

* ray-plane intersection algorithm: [source](https://github.com/gnzlbg/aobench/blob/master/src/intersection/ray_plane.rs)
* ray-sphere intersection algorithm: [source](https://github.com/gnzlbg/aobench/blob/master/src/intersection/ray_sphere.rs)
* ambient occlusion algorithm: [source](https://github.com/gnzlbg/aobench/blob/master/src/ambient_occlusion.rs)
* ray-casting the pixels:
  * scalar serial: [source](https://github.com/gnzlbg/aobench/blob/master/src/scalar.rs)
  * scalar parallel: [source](https://github.com/gnzlbg/aobench/blob/master/src/scalar_parallel.rs)
  * vector serial: [source](https://github.com/gnzlbg/aobench/blob/master/src/vector.rs)
  * vector parallel: [source](https://github.com/gnzlbg/aobench/blob/master/src/vector_parallel.rs)
