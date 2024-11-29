#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use ariel_os::{debug::log::*, thread};

#[ariel_os::thread(autostart)]
fn thread0() {
    match bench_multicore::benchmark(10_000, || {
        // Immutable access to `THREADS`.
        let pid = thread::current_pid().unwrap();
        core::hint::black_box(pid);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}
