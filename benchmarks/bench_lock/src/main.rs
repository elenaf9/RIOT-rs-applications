#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

use ariel_os::{debug::log::*, thread::{sync::Lock, thread_flags, ThreadId}};

static LOCK: Lock = Lock::new_locked();

#[ariel_os::task(autostart)]
async fn start() {
    thread_flags::set(ThreadId::new(0), 1);
}

#[ariel_os::thread(autostart)]
fn thread0() {
    thread_flags::wait_any(1);

    match bench_multicore::benchmark(10000, || LOCK.release()) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
}

#[ariel_os::thread(autostart, priority = 2)]
fn thread1() {
    loop {
        LOCK.acquire();
    }
}
