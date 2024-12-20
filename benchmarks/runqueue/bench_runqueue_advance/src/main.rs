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
    let mut rq = RunQueue::new();
    let rq_id = RunqueueId::new(5);
    rq.add(ThreadId::new(0), rq_id);
    rq.add(ThreadId::new(1), rq_id);
    match bench_multicore::benchmark(10000, || {
        #[cfg(not(feature = "multicore-v1"))]
        {
            rq.advance(rq_id);
        }
        #[cfg(feature = "multicore-v1")]
        {
            rq.advance(ThreadId::new(0), rq_id);
        }
        core::hint::black_box(&mut rq);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration ", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}
