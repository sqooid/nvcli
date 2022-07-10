use crate::nvapi_sys::NvAPI_Initialize;

pub fn initialize() {
    unsafe {
        let status = NvAPI_Initialize();
    }
}
