use esp_hal::xtensa_lx;

pub fn benchmark<F: FnMut() -> ()>(iterations: usize, mut f: F) -> Result<usize, crate::Error> {
    let before = xtensa_lx::timer::get_cycle_count();
    for _ in 0..iterations {
        f();
    }
    let after = xtensa_lx::timer::get_cycle_count();
    after
        .checked_sub(before)
        .map(|total| total as usize / iterations)
        .ok_or(crate::Error::TimerWrapper)
}
