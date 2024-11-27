#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

use riot_rs::{debug::log::*, thread::{sync::Lock, thread_flags, ThreadId}};

static LOCK: Lock = Lock::new_locked();

#[riot_rs::task(autostart)]
async fn start() {
    thread_flags::set(ThreadId::new(0), 1);
}

#[riot_rs::thread(autostart)]
fn thread0() {
    thread_flags::wait_any(1);

    match bench_multicore::benchmark(10000, || LOCK.release()) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
}

#[riot_rs::thread(autostart, priority = 2)]
fn thread1() {
    loop {
        LOCK.acquire();
    }
}
