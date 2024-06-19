#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::println;

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
        Ok(ticks) => println!("took {} ticks per iteration", ticks),

        Err(_) => println!("benchmark returned error"),
    }
    loop {}
}

#[cfg(any(feature = "fib", feature = "loop"))]
#[riot_rs::thread(autostart)]
fn thread1() {
    loop {
        #[cfg(feature = "fib")]
        core::hint::black_box(fib(25));
    }
}
