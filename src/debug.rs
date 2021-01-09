pub mod debug {
    use crate::utils::str_to_ptr;

    pub fn log_info(message: &str) {
        unsafe {
            let (ptr, len) = str_to_ptr(message);
            debug_log_info(ptr, len);
        }
    }

    extern "C" {
        fn debug_log_info(ptr: usize, len: usize);
    }
}
