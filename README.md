# Benchmarks for RIOT-rs

Benchmarks for the [RIOT-rs](https://github.com/future-proof-iot/RIOT-rust) embedded operating system.
The RIOT-rs dependency is patched to exact commit revisions based on the set `REV` below (see also [benchmarks/laze.yml](benchmarks/laze.yml)).

## Execute individual Benchmarks

Individual benchmarks in the `benchmarks/` folder can be executed with the following command:

```sh
laze build -C benchmarks/<BENCHMARK> -b <BOARD> run
```

- \<BOARD> may be any of the supported boards in RIOT-rs. However, we only tested the following boards:
    - rpi-pico
    - espressif-esp32-s3-wroom-1
    - nrf52840dk
    - ai-c3 (esp32c3)  

Note that some benchmarks require additional configuration through their own laze modules.
See the individual benchmark README's.  

### Examples

```sh
laze build -C benchmarks/bench_thread_flags -b espressif-esp32-s3-wroom-1 run
```

## Prerequisites

See [RIOT-rs](https://github.com/future-proof-iot/RIOT-rust).
