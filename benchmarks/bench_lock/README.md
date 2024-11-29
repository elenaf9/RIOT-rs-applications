# bench_lock

## About

This benchmark tests triggering the scheduler without a context switch performance.

**Unfortunately, the required `ariel_os_threads::schedule` function for this benchmark is private.
It must be changed locally to public in the checked-out `$HOME/.cargo/git/checkouts/<path-to-ariel-os-rev>`.**

## How to run

In this folder, run

    laze build -b rpi-pico  -s <FEAT> -s <REV> run
