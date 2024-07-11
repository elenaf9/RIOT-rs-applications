#![no_main]
#![no_std]
#![feature(type_alias_impl_trait)]
#![feature(used_with_arg)]

use riot_rs::debug::log::*;
#[cfg(feature = "multicore")]
use riot_rs::thread::channel::Channel;

#[cfg(feature = "multicore")]
static INPUT_CHANNEL: Channel<([[u16; N]; N / 2], [[u16; N]; N])> = Channel::new();
#[cfg(feature = "multicore")]
static RESULT_CHANNEL: Channel<[[u16; N]; N / 2]> = Channel::new();

const N: usize = 20;

fn matrix_mult<const M: usize>(
    matrix_a: &[[u16; N]; M],
    matrix_b: &[[u16; N]; N],
) -> [[u16; N]; M] {
    let mut matrix_c = [[0; N]; M];
    for i in 0..M {
        for j in 0..N {
            for k in 0..N {
                matrix_c[i][j] += matrix_a[i][k] * matrix_b[k][j]
            }
        }
    }
    matrix_c
}

#[riot_rs::thread(autostart, stacksize = 16384)]
fn thread0() {
    let matrix_a = core::hint::black_box([[3; N]; N]);
    let matrix_b = core::hint::black_box([[7; N]; N]);
    match riot_rs::bench::benchmark(10, || {
        #[cfg(feature = "single-core")]
        {
            let matrix_c = matrix_mult(&matrix_a, &matrix_b);
            core::hint::black_box(matrix_c);
        }
        #[cfg(feature = "multicore")]
        {
            let (matrix_a1, matrix_a2) = matrix_a.split_at(N / 2);
            let matrix_a1: [_; N / 2] = matrix_a1.try_into().unwrap();
            let matrix_a2: [_; N / 2] = matrix_a2.try_into().unwrap();

            INPUT_CHANNEL.send(&(matrix_a2, matrix_b));

            let matrix_c1 = matrix_mult(&matrix_a1, &matrix_b);
            let matrix_c2 = RESULT_CHANNEL.recv();

            core::hint::black_box((matrix_c1, matrix_c2));
        }
    }) {
        Ok(ticks) => info!("took {} ticks per iteration", ticks),

        Err(_) => error!("benchmark returned error"),
    }
}
#[cfg(feature = "multicore")]
#[riot_rs::thread(autostart, stacksize = 16384)]
fn thread1() {
    loop {
        let (matrix_a, matrix_b) = INPUT_CHANNEL.recv();
        let matrix_c = matrix_mult(&matrix_a, &matrix_b);
        RESULT_CHANNEL.send(&matrix_c);
    }
}
