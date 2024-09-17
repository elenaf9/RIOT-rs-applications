#![no_main]
#![no_std]
#![feature(impl_trait_in_assoc_type)]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use embassy_time::Duration;
#[cfg(feature = "await")]
use embassy_time::Timer;
use riot_rs::{
    debug::log::*,
    thread::{thread_flags, ThreadId},
};

#[cfg(feature = "affinity")]
use riot_rs::thread::{CoreAffinity, CoreId};

const ITERATIONS: usize = 100;

#[cfg(feature = "poll")]
fn now() -> u64 {
    embassy_time_driver::now()
}

#[riot_rs::task(autostart)]
async fn critical_task() {
    let delay = Duration::from_millis(1);

    // Start benchmark.
    thread_flags::set(ThreadId::new(0), 0b10);
    for _ in 0..ITERATIONS {
        #[cfg(feature = "await")]
        Timer::after(delay).await;
        #[cfg(feature = "poll")]
        {
            let expires = now() + delay.as_ticks();
            // Busy loop for timer.
            while now() < expires {}
        }
    }
    thread_flags::set(ThreadId::new(0), 0b100);
}

#[riot_rs::thread(autostart, priority = 10)]
fn thread0() {
    thread_flags::wait_all(0b10);
    match riot_rs::bench::benchmark(1, || {
        thread_flags::wait_all(0b100);
    }) {
        Ok(ticks) => info!("took {} ticks", ticks),
        Err(_) => error!("benchmark returned error"),
    }
}
