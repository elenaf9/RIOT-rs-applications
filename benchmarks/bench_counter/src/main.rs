#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{debug::log::*, thread::yield_same};

fn count() {
    let mut counter = 0;
    for _ in 0..1_000 {
        counter = core::hint::black_box(counter + 1);
    }
    core::hint::black_box(counter);
}

#[riot_rs::thread(autostart)]
fn thread0() {
    match bench_multicore::benchmark(100, || {
        count();
        yield_same();
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),

        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}

#[riot_rs::thread(autostart)]
fn thread1() {
    loop {
        count();
        yield_same();
    }
}
