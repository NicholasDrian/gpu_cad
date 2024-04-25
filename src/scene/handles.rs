use std::sync::Mutex;

/// Handles are used on the JS side to uniquely describe any gpu_nurbs objects.
/// 0 is a null handle.
pub type Handle = u64;

static mut HANDLE_GENERATOR: Mutex<Handle> = Mutex::new(0u64);

// TODO: think about this
// unwrap is icky
pub fn new_handle() -> Handle {
    unsafe {
        let mut changer = HANDLE_GENERATOR.lock().unwrap();
        *changer += 1u64;
        *changer
    }
}
