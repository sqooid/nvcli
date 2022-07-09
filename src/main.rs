use nvapi_hi::sys::{gpu::NvAPI_EnumPhysicalGPUs, handles::NvPhysicalGpuHandle};

fn main() {
    get_display_ids();
}

fn get_display_ids() {
    let gpu_handles: *mut [NvPhysicalGpuHandle; 64] = &mut [NvPhysicalGpuHandle::default(); 64];
    let gpu_count: *mut u32 = &mut 0;
    unsafe {
        NvAPI_EnumPhysicalGPUs(gpu_handles, gpu_count);
        println!("handles {:?}", *gpu_handles);
        println!("count {:?}", *gpu_count);
    }
}
