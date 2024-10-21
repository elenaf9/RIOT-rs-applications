#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

use riot_rs::{debug::log::*, thread};

fn count() {
    let mut counter = 0;
    for _ in 0..1_000 {
        counter = core::hint::black_box(counter + 1);
    }
    core::hint::black_box(counter);
}


#[riot_rs::task(autostart)]
async fn start() {
    thread::thread_flags::set(thread::ThreadId::new(0), 1);
}

#[riot_rs::thread(autostart)]
fn thread0() {
    thread::thread_flags::wait_any(1);

    match bench_multicore::benchmark(100, || {
        count();
        thread::yield_same();
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
        thread::yield_same();
    }
}
