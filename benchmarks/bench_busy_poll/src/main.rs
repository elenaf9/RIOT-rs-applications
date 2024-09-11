#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use embassy_time::Duration;
#[cfg(feature = "await")]
use embassy_time::Timer;
use riot_rs::{
    debug::log::*,
    static_cell::make_static,
    thread::{thread_flags, ThreadId},
    thread_executor::Executor,
};

#[cfg(feature = "affinity")]
use riot_rs::thread::{CoreAffinity, CoreId};

const ITERATIONS: usize = 100;

#[riot_rs::task(autostart)]
async fn start_other_tasks() {
    thread_flags::set(ThreadId::new(1), 0b10);
    thread_flags::set(ThreadId::new(0), 0b10);
}

#[cfg(feature = "poll")]
fn now() -> u64 {
    embassy_time_driver::now()
}

#[riot_rs::task(pool_size = 1)]
async fn critical_task() {
    let delay = Duration::from_millis(1);

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
    thread_flags::set(ThreadId::new(0), 1);
}

#[cfg_attr(not(feature = "affinity"), riot_rs::thread(autostart))]
#[cfg_attr(feature = "affinity", riot_rs::thread(autostart, affinity = CoreAffinity::one(CoreId::new(0))))]
fn thread0() {
    thread_flags::wait_one(0b10);
    thread_flags::wait_all(1);
    match riot_rs::bench::benchmark(1, || {
        thread_flags::wait_all(1);
    }) {
        Ok(ticks) => info!("took {} ticks", ticks),

        Err(_) => error!("benchmark returned error"),
    }
}

#[riot_rs::thread(autostart, stacksize = 4096)]
fn thread1() {
    let executor = make_static!(Executor::new());
    executor.run(|spawner| {
        spawner.must_spawn(critical_task());
    });
}
