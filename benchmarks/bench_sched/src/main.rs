#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{debug::log::*, thread};

#[riot_rs::thread(autostart)]
fn thread0() {
    match riot_rs::bench::benchmark(10000, || thread::schedule()) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),

        Err(_) => error!("benchmark returned error"),
    }
    loop {}
}
