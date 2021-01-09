pub mod time {
    pub fn get_time() -> f32 {
        unsafe {
            return time_get_time();
        }
    }

    pub fn get_delta_time() -> f32 {
        unsafe {
            return time_get_delta_time();
        }
    }

    extern "C" {
        fn time_get_time() -> f32;
        fn time_get_delta_time() -> f32;
    }
}
