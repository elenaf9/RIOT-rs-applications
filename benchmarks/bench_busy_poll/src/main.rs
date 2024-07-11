#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use embassy_time::{Duration, Timer};
use riot_rs::{
    debug::log::*,
    embassy::thread_executor::Executor,
    static_cell::make_static,
    thread::{thread_flags, ThreadId},
};

const ITERATIONS: usize = 100;

#[riot_rs::task(autostart)]
async fn start_other_tasks() {
    thread_flags::set(ThreadId::new(0), 0b10);
    thread_flags::set(ThreadId::new(1), 0b10);
    thread_flags::set(ThreadId::new(2), 0b10);
}

#[cfg(feature = "poll")]
fn now() -> u64 {
    loop {
        let hi = rp_pac::TIMER.timerawh().read();
        let lo = rp_pac::TIMER.timerawl().read();
        let hi2 = rp_pac::TIMER.timerawh().read();
        if hi == hi2 {
            return (hi as u64) << 32 | (lo as u64);
        }
    }
}

#[riot_rs::task(pool_size = 1)]
async fn network_task() {
    thread_flags::wait_one(0b10);
    let delay = Duration::from_millis(10);
    loop {
        Timer::after(delay).await;

        // Some dummy computation.
        let mut counter = 0;
        for _ in 0..10_000 {
            counter = core::hint::black_box(counter + 1);
        }
        core::hint::black_box(counter);
    }
}

#[riot_rs::task(pool_size = 1)]
async fn critical_task() {
    thread_flags::wait_one(0b10);
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

#[riot_rs::thread(autostart, priority = 3)]
fn thread0() {
    thread_flags::wait_one(0b10);
    match riot_rs::bench::benchmark(1, || {
        thread_flags::wait_all(1);
    }) {
        Ok(ticks) => info!("took {} ticks", ticks),

        Err(_) => error!("benchmark returned error"),
    }
}

// This thread has a higher priority than the `thread2` because
// otherwise it would be blocked by the busy poll in `critical_task`.
#[riot_rs::thread(autostart, priority = 2, stacksize = 4096)]
fn thread1() {
    let executor = make_static!(Executor::new());
    executor.run(|spawner| {
        spawner.must_spawn(network_task());
    });
}

#[riot_rs::thread(autostart, priority = 1, stacksize = 4096)]
fn thread2() {
    let executor = make_static!(Executor::new());
    executor.run(|spawner| {
        spawner.must_spawn(critical_task());
    });
}
