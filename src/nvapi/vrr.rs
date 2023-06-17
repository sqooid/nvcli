use crate::cli::error::Result;
use nvapi_sys_new::{
    NvAPI_Disp_GetVRRInfo, _NvAPI_Status_NVAPI_OK, make_nvapi_version, NV_GET_VRR_INFO,
};

use super::general::get_status_message;

#[derive(Debug)]
pub struct VvrData {
    version: u32,
    vrr_enabled: u32,
    reserved: u32,
    reserved_ex: [u32; 4],
}

pub fn get_vvr_data(display_id: u32) -> Result<VvrData> {
    let mut data = NV_GET_VRR_INFO::default();
    data.version = make_nvapi_version::<NV_GET_VRR_INFO>(1);
    let result: i32;
    unsafe {
        result = NvAPI_Disp_GetVRRInfo(display_id, &mut data);
    }
    if result != _NvAPI_Status_NVAPI_OK {
        Err(get_status_message(&result))
    } else {
        Ok(VvrData {
            version: data.version,
            vrr_enabled: data.bIsVRREnabled(),
            reserved: data.reserved(),
            reserved_ex: data.reservedEx,
        })
    }
}

// pub fn set_vvr_data(display_id: u32, data: &VvrData) -> Result<()> {
//     let mut input = NV_SET_VIRTUAL_REFRESH_RATE_DATA {
//         version: data.version,
//         frameIntervalUs: data.frame_interval,
//         reservedEx: data.reserved,
//     };
//     let result: i32;
//     unsafe {
//         result = NvAPI_DISP_SetVirtualRefreshRateData(display_id, &mut input);
//     }
//     if result != _NvAPI_Status_NVAPI_OK {
//         Err(get_status_message(&result))
//     } else {
//         Ok(())
//     }
// }

#[cfg(test)]
mod tests {
    use crate::nvapi::{
        general::{initialize, unload},
        monitor::find_primary_display_id,
    };

    use super::*;

    #[test]
    fn test_get_vrr_data() {
        initialize();
        if let Ok(primary_id) = find_primary_display_id() {
            match get_vvr_data(primary_id) {
                Ok(data) => println!("{:?}", data),
                Err(e) => println!("{e}"),
            }
        }
        unload();
    }
}
