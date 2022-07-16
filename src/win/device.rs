use std::time::Duration;

use windows::Win32::{
    Foundation::{BOOL, HWND, LPARAM, RECT},
    Graphics::Gdi::{
        EnumDisplayMonitors, GetDC, GetDeviceCaps, ReleaseDC, CM_GAMMA_RAMP, COLORMGMTCAPS, HDC,
        HMONITOR, MONITORENUMPROC,
    },
};

unsafe extern "system" fn enum_func(
    param0: HMONITOR,
    param1: HDC,
    param2: *mut RECT,
    param3: LPARAM,
) -> BOOL {
    println!("{:?}", param0);
    println!("{:?}", param1);
    println!("{:?}", *param2);
    println!("{:?}", param3);
    BOOL(1)
}

pub fn get_device_contexts() -> Vec<HDC> {
    let mut contexts: Vec<HDC> = vec![];
    unsafe {
        let result = EnumDisplayMonitors(HDC(0), std::ptr::null(), Some(enum_func), LPARAM(0));
        match result.as_bool() {
            true => println!("Success"),
            false => println!("Failed"),
        }
    }
    contexts
}

pub fn has_gamma_caps() -> bool {
    unsafe {
        let context = GetDC(HWND(0));
        let result = (GetDeviceCaps(&context, COLORMGMTCAPS) as u32 & CM_GAMMA_RAMP) != 0;
        ReleaseDC(HWND(0), &context);
        println!("{}", &result);
        result
    }
}
