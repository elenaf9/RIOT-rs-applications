#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{
    debug::log::*,
    thread::{SCHED_PRIO_LEVELS, THREADS_NUMOF},
};
use riot_rs_runqueue::{GlobalRunqueue, RunQueue, RunqueueId, ThreadId};

#[riot_rs::thread(autostart)]
fn thread0() {
    let mut rq = RunQueue::<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>::new();
    rq.add(ThreadId::new(0), RunqueueId::new(5));
    rq.add(ThreadId::new(1), RunqueueId::new(4));
    match riot_rs::bench::benchmark(10000, || {
        let changed_core = rq.reallocate();
        core::hint::black_box(changed_core);
        core::hint::black_box(&mut rq);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration ", ticks),
        Err(_) => error!("benchmark returned error"),
    }
    loop {}
}

#[cfg(feature = "multicore")]
#[riot_rs::thread(autostart)]
fn thread1() {
    cortex_m::asm::wfi();
}
