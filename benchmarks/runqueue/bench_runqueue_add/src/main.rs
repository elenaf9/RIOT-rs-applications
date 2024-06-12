#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{
    debug::println,
    thread::{SCHED_PRIO_LEVELS, THREADS_NUMOF},
};
use riot_rs_runqueue::{RunQueue, RunqueueId, ThreadId};

#[riot_rs::thread(autostart)]
fn thread0() {
    match riot_rs::bench::benchmark(10000, || {
        let mut rq = RunQueue::<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>::new();
        rq.add(ThreadId::new(0), RunqueueId::new(5));
        core::hint::black_box(rq);
    }) {
        Ok(ticks) => println!("took {} ticks per iteration ", ticks),
        Err(_) => println!("benchmark returned error"),
    }
}
