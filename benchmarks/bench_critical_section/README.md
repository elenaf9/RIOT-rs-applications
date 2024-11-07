# bench_critical_section

## About

This benchmark tests if/ how usage of `critical_section` in a user thread can block kernel functions.
Compared to most of the other benchmarks, this benchmark is multicore-only because it expects that the two threads run in parallel.

## How to run

In this folder, run

    laze build -b rpi-pico  -s <FEAT> -s <REV> run
