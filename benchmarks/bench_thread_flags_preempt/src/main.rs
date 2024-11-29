#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use ariel_os::{
    debug::log::*,
    thread::{current_pid, sync::Channel, thread_flags, ThreadId},
};

static ID_EXCHANGE: Channel<ThreadId> = Channel::new();

#[ariel_os::thread(autostart, priority = 1)]
fn thread0() {
    let target_pid = ID_EXCHANGE.recv();

    match bench_multicore::benchmark(10000, || {
        thread_flags::set(target_pid, 1);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}

#[ariel_os::thread(autostart, priority = 2)]
fn thread1() {
    ID_EXCHANGE.send(&current_pid().unwrap());
    loop {
        thread_flags::wait_all(1);
    }
}
