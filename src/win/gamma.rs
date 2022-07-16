use std::time::Duration;

use windows::Win32::{
    Foundation::HWND,
    Graphics::{
        Direct3D9::{IDirect3D9, IDirect3DDevice9, D3DGAMMARAMP},
        Gdi::{GetDC, ReleaseDC},
    },
    UI::ColorSystem::{GetDeviceGammaRamp, SetDeviceGammaRamp},
};

use crate::cli::error::Result;

macro_rules! clamp {
    ($in:expr, $min:expr, $max:expr) => {
        if $in > $max {
            $max
        } else if $in < $min {
            $min
        } else {
            $in
        }
    };
}

pub fn calculate_gamma_ramp(color: &Color) -> [u16; 256] {
    let gamma = color.gamma();
    let contrast = (color.contrast() - 0.5) * 2.0;
    let brightness = (color.brightness() - 0.5) * 2.0;
    let mut offset = if contrast > 0.0 {
        contrast * -25.4
    } else {
        contrast * -32.0
    };
    let range = 255.0 + offset * 2.0;
    offset += brightness * (range / 5.0);
    let mut result: [u16; 256] = [0; 256];
    for i in 0..256 {
        let i = i as f32;
        let mut factor = (i + offset) / range;
        factor = factor.powf(1.0 / gamma);
        factor = clamp!(factor, 0.0, 1.0);
        result[i as usize] = ((factor * u16::MAX as f32).round() as u16) << 8;
    }
    result
}

#[derive(Debug, Clone)]
pub struct Color {
    brightness: f32,
    contrast: f32,
    gamma: f32,
}

impl Color {
    pub fn new() -> Self {
        Self {
            brightness: 0.5,
            contrast: 0.5,
            gamma: 1.0,
        }
    }
    fn brightness(&self) -> f32 {
        self.brightness
    }
    fn contrast(&self) -> f32 {
        self.contrast
    }
    fn gamma(&self) -> f32 {
        self.gamma
    }
    pub fn set_brightness(&mut self, brightness: &f32) {
        self.brightness = clamp!(*brightness, 0.0, 1.0);
    }
    pub fn set_contrast(&mut self, contrast: &f32) {
        self.contrast = clamp!(*contrast, 0.0, 1.0);
    }
    pub fn set_gamma(&mut self, gamma: &f32) {
        self.gamma = clamp!(*gamma, 0.3, 2.8);
    }
}

pub fn set_color_config(color: &Color) -> Result<()> {
    // let device_context;
    // unsafe {
    //     device_context = GetDC(HWND(0));
    // }
    // let ramps = vec![calculate_gamma_ramp(color); 3];
    // println!("context: {:?}", &device_context);
    // println!("ramps: {:?}", &ramps);
    // unsafe {
    //     let result = SetDeviceGammaRamp(&device_context, ramps.as_ptr() as *const std::ffi::c_void);
    //     std::thread::sleep(Duration::from_millis(1000));
    //     if !result.as_bool() {
    //         return Err("Failed to set color config".to_string());
    //     }
    // }
    // Ok(())

    Ok(())
}

pub fn get_gamma_ramp() -> Result<()> {
    let device_context;
    unsafe {
        device_context = GetDC(HWND(0));
    }
    let mut ramp: [[u16; 256]; 3] = [[0; 256]; 3];
    // let mut ramp: [u16; 768] = [0; 768];
    unsafe {
        let result =
            GetDeviceGammaRamp(&device_context, ramp.as_mut_ptr() as *mut std::ffi::c_void);
        ReleaseDC(HWND(0), &device_context);
        println!("{:?}", &ramp);
        if !result.as_bool() {
            return Err("Failed to get color config".to_string());
        }
    }
    // Ok(ramp)
    Ok(())
}
