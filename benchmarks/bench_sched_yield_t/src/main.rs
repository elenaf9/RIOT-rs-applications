#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]

#[cfg(any(feature = "use-affinity"))]
use ariel_os::thread::{CoreAffinity, CoreId};
use ariel_os::{debug::log::*, thread};

#[ariel_os::task(autostart)]
async fn start() {
    thread::thread_flags::set(thread::ThreadId::new(0), 1);
}

#[cfg_attr(not(feature = "use-affinity"), ariel_os::thread(autostart))]
#[cfg_attr(feature = "affinity-0", ariel_os::thread(autostart, affinity = CoreAffinity::one(CoreId::new(0))))]
#[cfg_attr(feature = "affinity-1", ariel_os::thread(autostart, affinity = CoreAffinity::one(CoreId::new(1))))]
fn thread0() {
    // Unavoidable race condition that the benchmarking thread might migrate to another
    // core when core affinities aren't enabled.
    // Experimental trying showed that these different wait mechanism work best for the
    // two different boards.
    #[cfg(context = "rp2040")]
    thread::thread_flags::wait_any(1);
    #[cfg(context = "esp32s3")]
    while thread::thread_flags::get() == 0 {}

    match bench_multicore::benchmark(1000, || thread::yield_same()) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks,),
        Err(err) => error!("benchmark error: {}", err),
    }
}

#[cfg(not(feature = "t1"))]
#[ariel_os::thread(autostart)]
fn thread1() {
    loop {
        thread::yield_same()
    }
}

#[cfg(any(feature = "t3", feature = "t4"))]
#[ariel_os::thread(autostart)]
fn thread2() {
    loop {
        thread::yield_same()
    }
}

#[cfg(feature = "t4")]
#[ariel_os::thread(autostart)]
fn thread3() {
    loop {
        thread::yield_same()
    }
}
