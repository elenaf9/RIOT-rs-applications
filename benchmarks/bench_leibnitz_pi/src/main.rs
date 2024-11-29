#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

use ariel_os::{debug::log::*, thread};

#[cfg(feature = "dual-core")]
use ariel_os::thread::sync::Channel;

#[cfg(feature = "dual-core")]
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

#[ariel_os::task(autostart)]
async fn start() {
    thread::thread_flags::set(thread::ThreadId::new(0), 1);
}

#[ariel_os::thread(autostart)]
fn thread0() {
    thread::thread_flags::wait_any(1);

    match bench_multicore::benchmark(10, || {
        let res;
        #[cfg(not(feature = "dual-core"))]
        {
            res = leibniz_formula(1, ROUNDS * 2);
        }
        #[cfg(feature = "dual-core")]
        {
            res = leibniz_formula(1, ROUNDS) + RESULT_CHANNEL.recv();
        }
        core::hint::black_box(res);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}

#[cfg(feature = "dual-core")]
#[ariel_os::thread(autostart)]
fn thread1() {
    loop {
        let start = ROUNDS + 1;
        let res = leibniz_formula(start, ROUNDS * 2);
        RESULT_CHANNEL.send(&res);
    }
}
