use crate::nvapi_sys::NvAPI_Initialize;

pub fn initialize() {
    unsafe {
        NvAPI_Initialize();
    }
}
