# bench_thread_flags

## About

This benchmark tests two pairs of threads setting flags for each other. The purpose is to benchmark how parallel but independent calls in the kernel perform with different locking primitives.

## How to run

In this folder, run

    laze build -b rpi-pico  -s <FEAT> -s <REV> run
