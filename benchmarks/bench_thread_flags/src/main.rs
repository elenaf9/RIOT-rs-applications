#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

#[cfg(feature = "affinity")]
use riot_rs::thread::{CoreAffinity, CoreId};
use riot_rs::{
    debug::log::*,
    thread::{thread_flags, ThreadId},
};

#[riot_rs::task(autostart)]
async fn start() {
    thread_flags::set(ThreadId::new(1), 0b10);
    thread_flags::set(ThreadId::new(0), 0b10);
}

#[cfg_attr(
    not(feature = "affinity"),
    riot_rs::thread(autostart, stacksize = 4094)
)]
#[cfg_attr(feature = "affinity", riot_rs::thread(autostart, stacksize = 4094, affinity = CoreAffinity::one(CoreId::new(0))))]
fn thread0() {
    while thread_flags::get() != 0b10 { }
    match bench_multicore::benchmark(1000, || {
        thread_flags::set(ThreadId::new(2), 1);
        thread_flags::wait_all(1);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}

#[riot_rs::thread(autostart)]
fn thread1() {
    thread_flags::wait_all(0b10);
    loop {
        thread_flags::set(ThreadId::new(3), 1);
        thread_flags::wait_all(1);
    }
}

#[riot_rs::thread(autostart)]
fn thread2() {
    loop {
        thread_flags::wait_all(1);
        thread_flags::set(ThreadId::new(0), 1);
    }
}

#[riot_rs::thread(autostart)]
fn thread3() {
    loop {
        thread_flags::wait_all(1);
        thread_flags::set(ThreadId::new(1), 1);
    }
}
