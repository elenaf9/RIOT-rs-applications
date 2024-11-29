#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]
#![feature(array_chunks)]

use core::usize;

#[cfg(feature = "dual-core")]
use ariel_os::thread::sync::Channel;
use ariel_os::{debug::log::*, thread};

#[cfg(feature = "dual-core")]
static INPUT_CHANNEL: Channel<([[u16; N]; N / 2], [[u16; N]; N])> = Channel::new();
#[cfg(feature = "dual-core")]
static RESULT_CHANNEL: Channel<[[u16; N]; N / 2]> = Channel::new();

#[cfg(feature = "n10")]
const N: usize = 10;
#[cfg(feature = "n20")]
const N: usize = 20;
#[cfg(feature = "n30")]
const N: usize = 30;
#[cfg(feature = "n40")]
const N: usize = 40;

fn matrix_mult(matrix_a: &[[u16; N]], matrix_b: &[[u16; N]], matrix_c: &mut [[u16; N]]) {
    for i in 0..N / 2 {
        for j in 0..N {
            for k in 0..N {
                matrix_c[i][j] += matrix_a[i][k] * matrix_b[k][j]
            }
        }
    }
}

#[ariel_os::task(autostart)]
async fn start() {
    thread::thread_flags::set(thread::ThreadId::new(0), 1);
}

#[ariel_os::thread(autostart, stacksize = 32768)]
fn thread0() {
    // while thread::thread_flags::get() == 0 {}
    thread::thread_flags::wait_any(1);

    let matrix_a = core::hint::black_box([[3; N]; N]);
    let matrix_b = core::hint::black_box([[7; N]; N]);

    match bench_multicore::benchmark(10, || {
        let mut matrix_c = core::hint::black_box([[0; N]; N]);

        let mut matrix_a_iter = matrix_a.array_chunks::<{ N / 2 }>();
        let mut matrix_c_iter = matrix_c.array_chunks_mut::<{ N / 2 }>();

        let matrix_a1 = matrix_a_iter.next().unwrap();
        let matrix_a2 = matrix_a_iter.next().unwrap();

        #[cfg(feature = "dual-core")]
        INPUT_CHANNEL.send(&(*matrix_a2, matrix_b));

        matrix_mult(matrix_a1, &matrix_b, matrix_c_iter.next().unwrap());

        #[cfg(not(feature = "dual-core"))]
        {
            matrix_mult(matrix_a2, &matrix_b, matrix_c_iter.next().unwrap());
        }

        #[cfg(feature = "dual-core")]
        {
            *matrix_c_iter.next().unwrap() = RESULT_CHANNEL.recv();
        }

        core::hint::black_box(matrix_c);
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),
        Err(err) => error!("benchmark error: {}", err),
    }
}

#[cfg(feature = "dual-core")]
#[ariel_os::thread(autostart, stacksize = 32768)]
fn thread1() {
    loop {
        let (matrix_a, matrix_b) = INPUT_CHANNEL.recv();
        let mut matrix_c = [[0; N]; N / 2];
        matrix_mult(&matrix_a, &matrix_b, &mut matrix_c);
        RESULT_CHANNEL.send(&matrix_c);
    }
}
