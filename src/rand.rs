use std::{convert::TryInto, ptr, time::SystemTime, u64};

use num::{Bounded, Integer};
use sdl2::{libc::RAND_MAX, sys::{rand, srand}};

/// Somewhat dumb wrapper over libc's rand.
/// We really don't need cryptographically secure rands for this
/// application
pub struct Rand {}

impl Rand {
    pub fn new() -> Self {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("system time is before UNIX_EPOCH")
            .as_secs();
        unsafe {
            srand(timestamp as u32);
        }

        Self {}
    }

    // Leaving this here in case I ever get it to work with u8
    //
    // pub fn rand<T: Bounded + From<i32> + From<f64> + Into<f64>>() -> T {
    //     // The biggest value that T can represent
    //     let scalar = T::max_value().into() * (RAND_MAX as f64 + 1.0);
    //     unsafe { rand() as f64 / scalar }.into()
    // }

    pub fn rand_u8(&self) -> u8 {
        // The biggest value that T can represent
        let scalar = RAND_MAX/255;
        let r = unsafe { rand() / scalar };
        r as u8
    }
}
