# bench_thread_flags_preempt

## About

This benchmark tests the context switching performance in case of a preemption.
Two threads are running: one higher priority one that is just waiting for it's flag to be set, and a lower priority one that sets the flag and is then being preempted

## How to run

In this folder, run

    laze build -b rpi-pico run
