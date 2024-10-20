#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{
    debug::log::*,
    thread::{SCHED_PRIO_LEVELS, THREADS_NUMOF},
};
use riot_rs_runqueue::{RunQueue as GenericRunqueue, RunqueueId, ThreadId};

#[cfg(feature = "multicore-v1")]
use riot_rs_runqueue::GlobalRunqueue;

#[cfg(not(all(feature = "dual-core", feature = "multicore-v1")))]
type RunQueue = GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>;
#[cfg(all(feature = "dual-core", feature = "multicore-v1"))]
type RunQueue =
    GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }, { riot_rs::thread::CORES_NUMOF }>;

#[riot_rs::thread(autostart)]
fn thread0() {
    match bench_multicore::benchmark(10000, || {
        let mut rq = RunQueue::new();
        rq.add(ThreadId::new(0), RunqueueId::new(5));
        core::hint::black_box(rq);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration ", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}
