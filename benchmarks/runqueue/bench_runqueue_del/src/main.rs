#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{
    debug::println,
    thread::{SCHED_PRIO_LEVELS, THREADS_NUMOF},
};
#[cfg(feature = "multicore-v1")]
use riot_rs_runqueue::GlobalRunqueue;
use riot_rs_runqueue::{RunQueue, RunqueueId, ThreadId};

#[riot_rs::thread(autostart, priority = 2)]
fn thread0() {
    let thread_id = ThreadId::new(0);
    let rq_id = RunqueueId::new(5);
    match riot_rs::bench::benchmark(10000, || {
        let mut rq = RunQueue::<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>::new();
        rq.add(thread_id, rq_id);
        rq = core::hint::black_box(rq);
        rq.del(thread_id, rq_id);
        core::hint::black_box(rq);
    }) {
        Ok(ticks) => println!("took {} ticks per iteration ", ticks),
        Err(_) => println!("benchmark returned error"),
    }
    loop {}
}

#[cfg(feature = "multicore")]
#[riot_rs::thread(autostart)]
fn thread1() {
    cortex_m::asm::wfi();
}
