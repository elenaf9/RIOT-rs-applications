#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::log::*;
#[cfg(not(feature = "noop"))]
use riot_rs::thread::sync;

#[cfg(feature = "cs")]
type Spinlock = sync::GenericSpinlock<usize, sync::Cs, 10>;
#[cfg(feature = "atomic")]
type Spinlock = sync::GenericSpinlock<usize, sync::Atomic, 10>;
#[cfg(feature = "hardware")]
type Spinlock = sync::GenericSpinlock<usize, sync::Hardware<10>, 10>;
#[cfg(feature = "noop")]
type Spinlock = noop::Noop<usize>;

#[cfg(feature = "noop")]
mod noop {
    use core::cell::UnsafeCell;

    pub struct Noop<T> {
        inner: UnsafeCell<T>,
    }

    impl<T> Noop<T> {
        pub fn new(inner: T) -> Self {
            Self {
                inner: UnsafeCell::new(inner),
            }
        }
        pub fn lock(&self) -> &mut T {
            unsafe { &mut *self.inner.get() }
        }
    }
}

#[riot_rs::thread(autostart)]
fn thread0() {
    let counter = Spinlock::new(0);
    match bench_multicore::benchmark(100_000, || {
        let mut _c = core::hint::black_box(counter.lock());
        *_c += core::hint::black_box(1);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),

        Err(err) => error!("benchmark error: {}", err),
    }
    loop {}
}
