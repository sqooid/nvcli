use nvapi_sys_new::{NvAPI_EnumPhysicalGPUs, NvPhysicalGpuHandle, NvPhysicalGpuHandle__};

pub fn get_gpu_handles() -> Vec<NvPhysicalGpuHandle> {
    let mut gpus: Vec<NvPhysicalGpuHandle> = vec![];
    let mut gpu_handles: [NvPhysicalGpuHandle__; 64] = [NvPhysicalGpuHandle__ { unused: 0 }; 64];
    let mut gpu_handle_ptrs: [NvPhysicalGpuHandle; 64] = [0 as NvPhysicalGpuHandle; 64];
    for i in 0..64 {
        gpu_handle_ptrs[i] = &mut gpu_handles[i];
    }
    let mut gpu_count_raw: u32 = 0;
    unsafe {
        NvAPI_EnumPhysicalGPUs(gpu_handle_ptrs.as_mut_ptr(), &mut gpu_count_raw);
    }
    for i in 0..gpu_count_raw {
        gpus.push(gpu_handle_ptrs[i as usize]);
    }
    gpus
}
