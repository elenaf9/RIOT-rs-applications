#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{debug::println, thread::yield_same};

fn count() {
    let mut counter = 0;
    for _ in 0..1_000 {
        counter = core::hint::black_box(counter);
    }
    core::hint::black_box(counter);
}

#[riot_rs::thread(autostart)]
fn thread0() {
    match riot_rs::bench::benchmark(1000, || {
        count();
        yield_same();
    }) {
        Ok(ticks) => println!("took {} ticks per iteration", ticks),

        Err(_) => println!("benchmark returned error"),
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
