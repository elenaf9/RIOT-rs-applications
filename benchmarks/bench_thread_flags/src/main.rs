#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{
    debug::log::*,
    thread::{thread_flags, ThreadId},
};

#[riot_rs::thread(autostart)]
fn thread0() {
    loop {
        thread_flags::wait_all(1);
        thread_flags::set(ThreadId::new(1), 1);
    }
}

#[riot_rs::thread(autostart)]
fn thread1() {
    loop {
        thread_flags::set(ThreadId::new(0), 1);
        thread_flags::wait_all(1);
    }
}

#[riot_rs::thread(autostart)]
fn thread2() {
    loop {
        thread_flags::wait_all(1);
        thread_flags::set(ThreadId::new(3), 1);
    }
}

#[riot_rs::thread(autostart)]
fn thread3() {
    match riot_rs::bench::benchmark(1000, || {
        thread_flags::set(ThreadId::new(2), 1);
        thread_flags::wait_all(1);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(_) => error!("benchmark returned error"),
    }
    loop {}
}
