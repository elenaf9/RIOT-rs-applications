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

#[ariel_os::thread(autostart)]
fn thread0() {
    let mut total = 0;
    let iterations = 10000;
    for _ in 0..iterations {
        let mut rq = RunQueue::new();
        match bench_multicore::benchmark(1, || {
            rq.add(ThreadId::new(0), RunqueueId::new(5));
            core::hint::black_box(&mut rq);
        }) {
            Ok(ticks) => total += ticks,
            Err(err) => error!("benchmark error: {}", err),
        }
        core::hint::black_box(rq);
    }
    info!("took {} ticks per iteration ", total / iterations);
}
