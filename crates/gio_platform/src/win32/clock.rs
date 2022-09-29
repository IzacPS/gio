use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};
static mut CLOCK_FREQUENCY: f64 = 0.0;
static mut is_platform_clock_initialized: bool = false;

#[inline(always)]
fn initialize_platform_clock_frequency() {
    unsafe {
        if !is_platform_clock_initialized {
            let mut frequency: i64 = 0;
            QueryPerformanceFrequency(&mut frequency as *mut _);
            CLOCK_FREQUENCY = frequency as f64 / 1000.0;
            is_platform_clock_initialized = true;
            //TODO: log error or warning?
        }
    }
}

#[inline(always)]
fn platform_get_time() -> f64 {
    unsafe {
        let mut cur: i64 = 0;
        QueryPerformanceCounter(&mut cur as *mut _);
        cur as f64 / CLOCK_FREQUENCY
    }
}

pub fn initialize() {
    initialize_platform_clock_frequency();
}

pub fn get_time() -> f64 {
    platform_get_time()
}
