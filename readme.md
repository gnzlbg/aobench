# Ambient Occlusion Benchmark

> Originally written by Syoyo Fujita: https://github.com/syoyo/aobench

`aoench` is a small ambient occlusion renderer for benchmarking realworld
floating point performance in various languages.

![image_vector_par](https://user-images.githubusercontent.com/904614/41043073-653aa5be-69a3-11e8-8a9d-007def8516cc.png)

## Instructions


To run it with the default target options (replace `${NAME}` with an algorithm name):

```
> cargo run --release -- 800 600 --algo ${NAME}
```

On a dual core AVX1 i5 @1.8 GHz:

| `${NAME}` (800x600) | time [ms] | speedup [-] |
|---------------------|-----------|-------------|
| `scalar`            |      6266 |        1.0x |
| `vector`            |      2386 |        2.6x |
| `scalar_par`        |      2443 |        2.5x |
| `vector_par`        |       983 |        6.4x |

On a 28 core Xeon E5-2690 v4 @ 2.60GHz:

| `${NAME}` (800x600) | time [ms] | speedup [-] |
|---------------------|-----------|-------------|
| `scalar`            |      3234 |        1.0x |
| `vector`            |      1096 |        3.0x |
| `scalar_par`        |       132 |       24.5x |
| `vector_par`        |        76 |       42.6x |

| `${NAME}` (4096x4096) | time [ms] | speedup [-] |
|-----------------------|-----------|-------------|
| `scalar`              |    116121 |        1.0x |
| `vector`              |     40076 |        2.9x |
| `scalar_par`          |      3273 |       35.5x |
| `vector_par`          |      1398 |       83.1x |

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

As a comparison, the ISPC sources of the same benchmark are [here](https://github.com/ispc/ispc/tree/master/examples/aobench).
