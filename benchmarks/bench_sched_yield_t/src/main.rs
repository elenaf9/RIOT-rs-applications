#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

#[cfg(any(feature = "use-affinity"))]
use riot_rs::thread::{CoreAffinity, CoreId};
use riot_rs::{debug::log::*, thread};

#[riot_rs::task(autostart)]
async fn start() {
    thread::thread_flags::set(thread::ThreadId::new(0), 1);
}

#[cfg_attr(not(feature = "use-affinity"), riot_rs::thread(autostart))]
#[cfg_attr(feature = "affinity-0", riot_rs::thread(autostart, affinity = CoreAffinity::one(CoreId::new(0))))]
#[cfg_attr(feature = "affinity-1", riot_rs::thread(autostart, affinity = CoreAffinity::one(CoreId::new(1))))]
fn thread0() {
    thread::thread_flags::wait_any(1);
    match bench_multicore::benchmark(1000, || thread::yield_same()) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks,),
        Err(err) => error!("benchmark error: {}", err),
    }
}

#[cfg(not(feature = "t1"))]
#[riot_rs::thread(autostart)]
fn thread1() {
    loop {
        thread::yield_same()
    }
}

#[cfg(any(feature = "t3", feature = "t4"))]
#[riot_rs::thread(autostart)]
fn thread2() {
    loop {
        thread::yield_same()
    }
}

#[cfg(feature = "t4")]
#[riot_rs::thread(autostart)]
fn thread3() {
    loop {
        thread::yield_same()
    }
}
