# bench_sched_yield_t

Copied from <https://github.com/future-proof-iot/RIOT-rs/tree/main/tests/benchmarks>.

## About

This benchmark tests basic context switch performance.

## How to run

In this folder, run

    laze build -b rpi-pico  -s <t1|t2|t3|t4> [-s <affinity|affinity-0|affinity-1>]  -s <FEAT> -s <REV> run
