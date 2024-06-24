#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{debug::println, thread};

#[riot_rs::thread(autostart)]
fn thread0() {
    match riot_rs::bench::benchmark(1_000, || {
        thread::current_pid().unwrap();
    }) {
        Ok(ticks) => println!("took {} ticks per iteration", ticks),
        Err(_) => println!("benchmark returned error"),
    }
    loop {}
}

#[riot_rs::thread(autostart)]
fn thread1() {
    loop {
        critical_section::with(|_| {
            // Some dummy computation.
            let mut counter = 0;
            for _ in 0..1_000 {
                counter = core::hint::black_box(counter + 1);
            }
            core::hint::black_box(counter);
        })
    }
}
