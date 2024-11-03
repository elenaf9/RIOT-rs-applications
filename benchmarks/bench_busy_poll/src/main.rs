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

/// Add second task to prevent that the thread
/// is suspended in the `await` case, which would
/// add extra cost for context switching.
#[riot_rs::task(autostart)]
async fn yielder() {
    embassy_futures::yield_now().await;
}

#[riot_rs::task(autostart)]
async fn critical_task() {
    let delay = Duration::from_millis(1);

    // Start benchmark.
    thread_flags::set(ThreadId::new(0), 0b1);
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
    thread_flags::set(ThreadId::new(0), 0b10);
}

#[cfg_attr(not(feature = "affinity"), riot_rs::thread(autostart, priority = 10))]
#[cfg_attr(feature = "affinity", 
    riot_rs::thread(autostart, priority = 10, affinity = CoreAffinity::one(CoreId::new(0)))
)]
fn thread0() {
    while thread_flags::get() & 0b1 == 0 {}
    match bench_multicore::benchmark(1, || while thread_flags::get() & 0b10 == 0 {}) {
        Ok(ticks) => info!("took {} ticks", ticks / ITERATIONS),
        Err(err) => error!("benchmark error: {}", err),
    }
}
