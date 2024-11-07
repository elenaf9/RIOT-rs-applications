# Benchmarks for RIOT-rs

Benchmarks for the [RIOT-rs](https://github.com/future-proof-iot/RIOT-rs) embedded operating system.
The RIOT-rs dependency is patched to exact commit revisions based on the set `REV` below (see also [benchmarks/laze.yml](benchmarks/laze.yml)).

## Execute individual Benchmarks

Individual benchmarks in the `benchmarks/` folder can be executed with the following command:

```sh
laze build -C benchmarks/<BENCHMARK> -b <BOARD> [-s <FEAT>] -s <REV> run
```

- \<BOARD> may be any of the supported boards in RIOT-rs. However, we only tested the following boards:
    - rpi-pico
    - espressif-esp32-s3-wroom-1 
- \<FEAT>: single-core | dual-core (dual-core only supported for the above two boards)
    - **Must be specified before \<REV> due to laze internals.**
- \<REV>: main | multicore-v1 | multicore-v2 |multicore-v2-cs | multicore-v2-locking
    - when `REV=main`, `-s <FEAT>` must be omitted!

Note that some benchmarks require additional configuration through their own laze modules.
See the individual benchmark README's.  
Not all benchmarks are supported on all revisions or for all features.
For invalid configurations, laze will print a conflict and "laze: error: no matching target for task "run" found".

### Examples

```sh
laze build -C benchmarks/bench_leibnitz_pi -b espressif-esp32-s3-wroom-1 -s main run

laze build -C benchmarks/runqueue/bench_runqueue_add -b rpi-pico -s dual-core -s multicore-v1 run
```

## Execute all Benchmarks

The [run.sh](run.sh) script atomizes the execution of all benchmarks for all feature- and revision combination.  
**Only tested on Arch Linux. Might work for other Linux distributions as well.**
The `BOARD` env must be set to one of the supported boards, e.g. `BOARD=rpi-pico`.
Then, the script can be executed with:

```sh
./run.sh
```

It creates a folder `data/` if it doesn't exist yet and prints the benchmark results in a markdown table format into a file `data/$BOARD.md`.

The executed configuration can be customized by setting the following environment variables:
- **REVS**: list of revisions that should be tested, e.g. `REVS="main multicore-v1"`
- **FEAT**: feature that should be tested, e.g. `FEAT="dual-core"`

## Prerequisites

See [RIOT-rs](https://github.com/future-proof-iot/RIOT-rs).
