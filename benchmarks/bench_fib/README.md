# bench_fib

## About

Benchmark that tests if/how independent computations on core 1 can influence the performance of a fibonacci computation on core 0.
Three scenarios can be tested:
- no code running on core 1
- core 1 doing same fibonacci computation
- core 1 doing a busy wait.

## How to run

In this folder, run

    laze build -b rpi-pico run
