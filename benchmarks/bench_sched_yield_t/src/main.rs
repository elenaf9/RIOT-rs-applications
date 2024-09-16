#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{debug::log::*, thread};

#[riot_rs::thread(autostart)]
fn thread0() {
    match riot_rs::bench::benchmark(1000, || thread::yield_same()) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks,),
        Err(_) => error!("benchmark returned error"),
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
