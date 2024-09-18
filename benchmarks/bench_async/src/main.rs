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

#[cfg(feature = "multicore-v1")]
use core::cell::RefCell;
#[cfg(feature = "multicore-v1")]
use critical_section::{with, Mutex};

#[cfg(feature = "affinity")]
use riot_rs::thread::{CoreAffinity, CoreId};

const ITERATIONS: usize = 100;

#[cfg(feature = "multicore-v1")]
static BENCHMARK_CORE: Mutex<RefCell<usize>> = Mutex::new(RefCell::new(0xff));

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

    // Blocks other core so that the benchmark has to continue running on its original core
    // FIXME: implement core affinity masks instead.
    #[cfg(feature = "multicore-v1")]
    if with(|cs| *BENCHMARK_CORE.borrow(cs).borrow() != usize::from(riot_rs::thread::core_id())) {
        loop {}
    }
}

#[cfg_attr(not(feature = "affinity"), riot_rs::thread(autostart))]
#[cfg_attr(feature = "affinity", riot_rs::thread(autostart, affinity = CoreAffinity::one(CoreId::new(0))))]
fn thread0() {
    thread_flags::wait_one(0b1);
    #[cfg(feature = "multicore-v1")]
    with(|cs| *BENCHMARK_CORE.borrow(cs).borrow_mut() = usize::from(riot_rs::thread::core_id()));
    match riot_rs::bench::benchmark(1, || {
        thread_flags::wait_all(0b11);
    }) {
        Ok(ticks) => info!("took {} ticks", ticks),
        Err(_) => error!("benchmark returned error"),
    }
}

#[riot_rs::thread(autostart)]
fn thread1() {
    static EXECUTOR: StaticCell<Executor> = StaticCell::new();
    EXECUTOR.init_with(|| Executor::new()).run(|spawner| {
        spawner.must_spawn(task(0));
        #[cfg(feature = "single-core")]
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
