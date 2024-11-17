#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

use riot_rs::{
    debug::log::*,
    thread::{thread_flags, ThreadId},
};

#[riot_rs::task(autostart)]
async fn start() {
    thread_flags::set(ThreadId::new(0), 1);
    thread_flags::set(ThreadId::new(1), 1);
}

#[riot_rs::thread(autostart)]
fn thread0() {
    thread_flags::wait_all(1);
    thread_flags::set(ThreadId::new(0), 1);
    match bench_multicore::benchmark(1000, || {
        thread_flags::wait_all(1);
        thread_flags::set(ThreadId::new(2), 1);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}

#[riot_rs::thread(autostart)]
fn thread1() {
    loop {
        thread_flags::wait_all(1);
        thread_flags::set(ThreadId::new(3), 1);
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
