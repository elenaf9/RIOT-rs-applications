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
    let thread1 = ThreadId::new(1);
    let rq_id = RunqueueId::new(5);
    match bench_multicore::benchmark(10000, || {
        let mut rq = RunQueue::new();
        rq.add(thread0, rq_id);
        rq.add(thread1, rq_id);
        rq = core::hint::black_box(rq);
        // #[cfg(not(any(feature = "multicore-v1", feature = "multicore-v2")))]
        // {
        //     rq.del(thread1);
        //     rq.del(thread0);
        // }
        #[cfg(not(feature = "multicore-v1"))]
        {
            // if riot_rs::thread::CORES_NUMOF > 1 {
            rq.del(thread1);
            rq.del(thread0);
            // } else {
            //     rq.del(thread0, rq_id);
            //     rq.del(thread1, rq_id);
            // }
        }
        #[cfg(feature = "multicore-v1")]
        {
            // if riot_rs::thread::CORES_NUMOF > 1 {
            rq.del(thread1, rq_id);
            rq.del(thread0, rq_id);
            // } else {
            //     rq.del(thread0, rq_id);
            //     rq.del(thread1, rq_id);
            // }
        }
        // #[cfg(feature = "multicore-v2")]
        // {
        //     rq.del(thread1);
        //     rq.del(thread0);
        // }
        core::hint::black_box(rq);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration ", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}
