#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::log::*;

fn fib(n: usize) -> usize {
    if n <= 1 {
        return n;
    }
    return fib(n - 1) + fib(n - 2);
}

#[riot_rs::thread(autostart)]
fn thread0() {
    match riot_rs::bench::benchmark(1000, || {
        core::hint::black_box(fib(25));
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),

        Err(_) => error!("benchmark returned error"),
    }
    loop {}
}

#[riot_rs::thread(autostart)]
fn thread1() {
    #[cfg(feature = "none")]
    cortex_m::asm::wfi();
    #[cfg(any(feature = "fib", feature = "loop"))]
    loop {
        #[cfg(feature = "fib")]
        core::hint::black_box(fib(25));
    }
}
