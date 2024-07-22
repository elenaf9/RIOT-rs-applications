#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use embassy_time::{Duration, Timer};
#[cfg(feature = "multicore-v1")]
use riot_rs::thread::CoreId;
use riot_rs::{
    debug::log::*,
    embassy::thread_executor::Executor,
    static_cell::make_static,
    thread::{thread_flags, ThreadId},
};

const ITERATIONS: usize = 100;

#[riot_rs::task(autostart)]
async fn start_other_tasks() {
    thread_flags::set(ThreadId::new(0), 0b1);
    thread_flags::set(ThreadId::new(1), 0b110);
    #[cfg(feature = "multicore")]
    thread_flags::set(ThreadId::new(2), 0b110);
}

#[riot_rs::task(pool_size = 2)]
async fn task(id: usize) {
    thread_flags::wait_one(0b110);
    for _ in 0..ITERATIONS {
        Timer::after(Duration::from_millis(1)).await;
    }
    thread_flags::set(ThreadId::new(0), 1 << id);

    // Blocks Core 1 so that the benchmark has to continue running on Core 0;
    // FIXME: implement core affinity masks instead.
    #[cfg(feature = "multicore")]
    if usize::from(riot_rs::thread::core_id()) == 1 {
        loop {}
    }
}

#[riot_rs::thread(autostart, priority = 1)]
fn thread0() {
    thread_flags::wait_one(0b1);
    match riot_rs::bench::benchmark(1, || {
        thread_flags::wait_all(0b11);
    }) {
        Ok(ticks) => info!("took {} ticks", ticks),

        Err(_) => error!("benchmark returned error"),
    }
}

#[riot_rs::thread(autostart, priority = 1)]
fn thread1() {
    let executor = make_static!(Executor::new());
    executor.run(|spawner| {
        spawner.must_spawn(task(0));
        #[cfg(feature = "single-core")]
        spawner.must_spawn(task(1));
    });
}

#[cfg(feature = "multicore")]
#[riot_rs::thread(autostart, priority = 1)]
fn thread2() {
    let executor = make_static!(Executor::new());
    executor.run(|spawner| {
        spawner.must_spawn(task(1));
    });
}
