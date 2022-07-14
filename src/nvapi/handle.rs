use nvapi_sys_new::{
    NvAPI_EnumNvidiaDisplayHandle, NvAPI_GetAssociatedNvidiaDisplayName, NvDisplayHandle,
    _NvAPI_Status_NVAPI_OK,
};

use crate::cli::error::Result;

pub fn get_display_handles() -> Vec<NvDisplayHandle> {
    let mut display_handles: Vec<NvDisplayHandle> = vec![];
    let mut display_handle_buffer: NvDisplayHandle = std::ptr::null_mut();
    let mut index = 0;
    while unsafe {
        NvAPI_EnumNvidiaDisplayHandle(index, &mut display_handle_buffer) == _NvAPI_Status_NVAPI_OK
    } {
        index += 1;
        display_handles.push(display_handle_buffer);
    }
    display_handles
}

pub unsafe fn get_display_name(handle: NvDisplayHandle) -> String {
    let mut buffer: [i8; 64] = [0; 64];
    unsafe {
        NvAPI_GetAssociatedNvidiaDisplayName(handle, buffer.as_mut_ptr());
    }
    let string_buffer: [u8; 64];
    unsafe {
        string_buffer = std::mem::transmute::<[i8; 64], [u8; 64]>(buffer);
    }
    std::str::from_utf8(&string_buffer)
        .unwrap_or("")
        .to_string()
}
