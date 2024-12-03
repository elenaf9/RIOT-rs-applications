# Benchmarks for Ariel OS

Benchmarks for the [Ariel](https://github.com/ariel-os/ariel-os) embedded operating system.
The Ariel OS dependency is patched to exact commit revisions based on the set `REV` below (see also [benchmarks/laze.yml](benchmarks/laze.yml)).

## Execute individual Benchmarks

Individual benchmarks in the `benchmarks/` folder can be executed with the following command:

```sh
laze build -C benchmarks/<BENCHMARK> -b <BOARD> [-s <FEAT>] -s <REV> run
```

- \<BOARD> may be any of the supported boards in Ariel OS. However, we only tested the following boards:
    - rpi-pico (dual-core)
    - espressif-esp32-s3-wroom-1 (dual-core)
    - nrf52840dk (single-core)
    - ai-c3 (esp32c3) (single core)
- \<FEAT>: single-core | dual-core
    - **Must be specified before \<REV> due to laze internals.**
- \<REV>: ariel-reallocation | ariel

Note that some benchmarks require additional configuration through their own laze modules.
See the individual benchmark README's.  
Not all benchmarks are supported on all revisions or for all features.
For invalid configurations, laze will print a conflict and "laze: error: no matching target for task "run" found".

### Examples

```sh
laze build -C benchmarks/bench_thread_flags -b rpi-pico -s dual-core -s ariel-reallocation run
```

## Prerequisites

See [Ariel OS](https://github.com/ariel-os/ariel-os).
