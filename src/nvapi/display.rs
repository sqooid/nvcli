use crate::nvapi_sys::{NvAPI_GPU_GetConnectedDisplayIds, NvPhysicalGpuHandle, NV_GPU_DISPLAYIDS};

pub fn get_display_ids(gpu_handle: NvPhysicalGpuHandle) -> Vec<NV_GPU_DISPLAYIDS> {
    let mut display_ids: Vec<NV_GPU_DISPLAYIDS>;
    let mut display_count: u32 = 0;
    unsafe {
        NvAPI_GPU_GetConnectedDisplayIds(
            gpu_handle,
            0 as *mut NV_GPU_DISPLAYIDS,
            &mut display_count,
            0,
        );
        display_ids = vec![
            NV_GPU_DISPLAYIDS {
                version: NV_GPU_DISPLAYIDS_VER2,
                ..NV_GPU_DISPLAYIDS::default()
            };
            display_count as usize
        ];
        NvAPI_GPU_GetConnectedDisplayIds(
            gpu_handle,
            display_ids.as_mut_ptr(),
            &mut display_count,
            0,
        );
    }
    display_ids
}

pub const NV_GPU_DISPLAYIDS_VER2: u32 = std::mem::size_of::<NV_GPU_DISPLAYIDS>() as u32 | 3 << 16;
