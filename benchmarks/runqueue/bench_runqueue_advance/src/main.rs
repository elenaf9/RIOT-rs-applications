#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use ariel_os::{
    debug::log::*,
    thread::{SCHED_PRIO_LEVELS, THREADS_NUMOF},
};
use ariel_os_runqueue::{RunQueue as GenericRunqueue, RunqueueId, ThreadId};

#[cfg(feature = "reallocation")]
use ariel_os_runqueue::GlobalRunqueue;

#[cfg(not(all(feature = "dual-core", feature = "reallocation")))]
type RunQueue = GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>;
#[cfg(all(feature = "dual-core", feature = "reallocation"))]
type RunQueue =
    GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }, { ariel_os::thread::CORES_NUMOF }>;

#[ariel_os::thread(autostart, priority = 2)]
fn thread0() {
    let mut rq = RunQueue::new();
    let rq_id = RunqueueId::new(5);
    rq.add(ThreadId::new(0), rq_id);
    rq.add(ThreadId::new(1), rq_id);
    match bench_multicore::benchmark(10000, || {
        #[cfg(not(feature = "reallocation"))]
        {
            rq.advance(rq_id);
        }
        #[cfg(feature = "reallocation")]
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
