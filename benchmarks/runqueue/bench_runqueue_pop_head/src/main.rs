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

#[riot_rs::thread(autostart, priority = 2)]
fn thread0() {
    let thread0 = ThreadId::new(0);
    let rq_id = RunqueueId::new(5);
    let mut total = 0;
    let iterations = 10000;
    for _ in 0..iterations {
        let mut rq = RunQueue::new();
        rq.add(thread0, rq_id);
        match bench_multicore::benchmark(1, || {
            let thread = rq.pop_head(thread0, rq_id);
            core::hint::black_box(thread);
            core::hint::black_box(&mut rq);
        }) {
            Ok(ticks) => total += ticks,
            Err(err) => error!("benchmark error: {}", err),
        }
        core::hint::black_box(rq);
    }
    info!("took {} ticks per iteration ", total/ iterations);
}
