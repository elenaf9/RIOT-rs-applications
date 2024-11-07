# bench_spinlocks

## About

Benchmark for testing different spinlock Backends.

## How to run

In this folder, run

    laze build -b rpi-pico -s <noop|cs|atomic|hardware> -s <FEAT> -s <REV> run
