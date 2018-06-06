# Ambient Occlusion Benchmark

> Originally written by Syoyo Fujita: https://github.com/syoyo/aobench

`aoench` is a small ambient occlusion renderer for benchmarking realworld
floating point performance in various languages.

## Instructions


To run it with the default target options (replace `${NAME}` with an algorithm name):

```
> cargo run --release -- 800 600 --algo ${NAME}
```

On a dual core AVX1 i5 @1.8 GHz:

|  `${NAME}`   | time [ms] | speedup [-] |
|--------------|-----------|-------------|
| `scalar`     |      6266 |     1.0x    |
| `vector`     |      2386 |     2.6x   |
| `scalar_par` |      2443 |     2.5x    |
| `vector_par` |       983 |     6.4x   |

On a 28 core Xeon E5-2690 v4 @ 2.60GHz:

|  `${NAME}`   | time [ms] | speedup [-] |
|--------------|-----------|-------------|
| `scalar`     |      3628 |     1.0x    |
| `vector`     |      1943 |     1.9x    |
| `scalar_par` |       225 |    16.1x    |
| `vector_par` |       165 |    22.0x    |

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

The scalar and vectorized implementations of the intersection and ao algorithms
are in the same file so that they can be easily compared.
