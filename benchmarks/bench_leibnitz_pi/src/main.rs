#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::log::*;
#[cfg(feature = "multicore")]
use riot_rs::thread::channel::Channel;
#[cfg(feature = "multicore-v2")]
use riot_rs::thread::{CoreAffinity, CoreId};

#[cfg(feature = "multicore")]
static RESULT_CHANNEL: Channel<f32> = Channel::new();

const ROUNDS: usize = 1_000; // Must be a multiple of 4.

fn leibniz_formula(start: usize, end: usize) -> f32 {
    let mut res = 0f32;
    let mut factor: i8 = 4;
    for i in (start..end).step_by(2) {
        res += factor as f32 * 1f32 / (i as f32);
        factor *= -1
    }
    res
}

fn benchmark_fn() {
    match riot_rs::bench::benchmark(10, || {
        let res;
        #[cfg(feature = "single-core")]
        {
            res = leibniz_formula(1, ROUNDS * 2);
        }
        #[cfg(feature = "multicore")]
        {
            res = leibniz_formula(1, ROUNDS) + RESULT_CHANNEL.recv();
        }
        core::hint::black_box(res);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(_) => error!("benchmark returned error"),
    }
    loop {}
}

#[cfg(not(feature = "multicore-v2"))]
#[riot_rs::thread(autostart)]
fn thread0() {
    benchmark_fn()
}

#[cfg(feature = "multicore-v2")]
#[riot_rs::thread(autostart, affinity = CoreAffinity::one(CoreId::new(1)))]
fn thread0() {
    benchmark_fn()
}

#[cfg(feature = "multicore")]
#[riot_rs::thread(autostart)]
fn thread1() {
    loop {
        let start = ROUNDS + 1;
        let res = leibniz_formula(start, ROUNDS * 2);
        RESULT_CHANNEL.send(&res);
    }
}
