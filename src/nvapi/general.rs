use std::fmt::Display;

use nvapi_sys_new::{NvAPI_GetErrorMessage, NvAPI_Initialize, NvAPI_Unload, _NvAPI_Status};

pub fn initialize() {
    unsafe {
        NvAPI_Initialize();
    }
}

pub fn unload() {
    unsafe {
        NvAPI_Unload();
    }
}

pub fn get_status_message(status: &_NvAPI_Status) -> String {
    let mut buffer: [i8; 64] = [0; 64];
    let str_buffer;
    unsafe {
        NvAPI_GetErrorMessage(*status, buffer.as_mut_ptr());
        str_buffer = std::mem::transmute::<[i8; 64], [u8; 64]>(buffer);
        let mut message = std::str::from_utf8_unchecked(&str_buffer).to_owned();
        message.truncate(message.find("\0").unwrap_or(64));
        message
    }
}
