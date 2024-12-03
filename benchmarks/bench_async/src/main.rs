#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]
#![feature(used_with_arg)]
use embassy_time::{Duration, Timer};
use riot_rs::{
    debug::log::*,
    thread::{thread_flags, ThreadId},
    StaticCell,
};
use riot_rs_embassy::thread_executor::Executor;

const ITERATIONS: usize = 100;

#[riot_rs::task(autostart)]
async fn start_other_tasks() {
    thread_flags::set(ThreadId::new(0), 0b1);
    thread_flags::set(ThreadId::new(1), 0b110);
    #[cfg(feature = "dual-core")]
    thread_flags::set(ThreadId::new(2), 0b110);
}

#[riot_rs::task(pool_size = 2)]
async fn task(id: usize) {
    thread_flags::wait_one(0b110);
    for _ in 0..ITERATIONS {
        Timer::after(Duration::from_millis(1)).await;
    }
    thread_flags::set(ThreadId::new(0), 1 << id);
}

#[riot_rs::thread(autostart)]
fn thread0() {
    thread_flags::wait_one(0b1);
    match bench_multicore::benchmark(1, || {
        thread_flags::wait_all(0b11);
    }) {
        Ok(ticks) => info!("took {} ticks", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
}

#[riot_rs::thread(autostart)]
fn thread1() {
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    EXECUTOR.init_with(|| Executor::new()).run(|spawner| {
        spawner.must_spawn(task(0));
        #[cfg(not(feature = "dual-core"))]
        spawner.must_spawn(task(1));
    });
}

#[cfg(feature = "dual-core")]
#[riot_rs::thread(autostart)]
fn thread2() {
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    EXECUTOR
        .init_with(|| Executor::new())
        .run(|spawner| spawner.must_spawn(task(1)));
}
