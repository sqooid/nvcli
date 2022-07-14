use nvapi_sys_new::NvAPI_Initialize;

pub fn initialize() {
    unsafe {
        NvAPI_Initialize();
    }
}
