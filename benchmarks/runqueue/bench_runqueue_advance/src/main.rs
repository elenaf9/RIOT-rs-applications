#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::{
    debug::log::*,
    thread::{SCHED_PRIO_LEVELS, THREADS_NUMOF},
};
#[cfg(feature = "multicore-v1")]
use riot_rs_runqueue::GlobalRunqueue;
use riot_rs_runqueue::{RunQueue, RunqueueId, ThreadId};

#[riot_rs::thread(autostart, priority = 2)]
fn thread0() {
    let mut rq = RunQueue::<{ SCHED_PRIO_LEVELS }, { THREADS_NUMOF }>::new();
    let rq_id = RunqueueId::new(5);
    rq.add(ThreadId::new(0), rq_id);
    rq.add(ThreadId::new(1), rq_id);
    match riot_rs::bench::benchmark(10000, || {
        #[cfg(not(feature = "multicore-v1"))]
        {
            rq.advance(rq_id);
            rq.advance(rq_id);
        }
        #[cfg(feature = "multicore-v1")]
        {
            rq.advance(ThreadId::new(0), rq_id);
            rq.advance(ThreadId::new(1), rq_id);
        }
        core::hint::black_box(&mut rq);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration ", ticks),
        Err(_) => error!("benchmark returned error"),
    }
    loop {}
}
