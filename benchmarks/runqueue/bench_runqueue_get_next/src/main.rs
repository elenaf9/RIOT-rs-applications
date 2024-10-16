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
use riot_rs_runqueue::{CoreId, GlobalRunqueue};

#[cfg(not(all(feature = "dual-core", feature = "multicore-v1")))]
type RunQueue = GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>;
#[cfg(all(feature = "dual-core", feature = "multicore-v1"))]
type RunQueue =
    GenericRunqueue<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }, { riot_rs::thread::CORES_NUMOF }>;

#[riot_rs::thread(autostart)]
fn thread0() {
    let mut rq = RunQueue::new();
    rq.add(ThreadId::new(0), RunqueueId::new(5));
    match riot_rs::bench::benchmark(10000, || {
        #[cfg(not(feature = "multicore-v1"))]
        let next = rq.get_next();
        #[cfg(feature = "multicore-v1")]
        let next = rq.get_next(CoreId::new(0));

        core::hint::black_box(next);
        core::hint::black_box(&mut rq);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration ", ticks),
        Err(_) => error!("benchmark returned error"),
    }
    loop {}
}
