#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use ariel_os::{
    debug::log::*,
    thread::{SCHED_PRIO_LEVELS, THREADS_NUMOF},
};
use ariel_os_runqueue::{GlobalRunqueue, RunQueue as GenericRunqueue, RunqueueId, ThreadId};

#[cfg(not(feature = "dual-core"))]
type RunQueue = GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>;
#[cfg(feature = "dual-core")]
type RunQueue =
    GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }, { ariel_os::thread::CORES_NUMOF }>;

#[ariel_os::thread(autostart)]
fn thread0() {
    let mut rq = RunQueue::new();
    rq.add(ThreadId::new(0), RunqueueId::new(5));
    rq.add(ThreadId::new(1), RunqueueId::new(4));
    match bench_multicore::benchmark(10000, || {
        let changed_core = rq.reallocate();
        core::hint::black_box(changed_core);
        core::hint::black_box(&mut rq);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration ", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}
