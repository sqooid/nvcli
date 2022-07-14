use crate::cli::error::Result;
use nvapi_sys_new::{
    make_nvapi_version, NvAPI_VIO_QueryTopology, NVVIOTOPOLOGYTARGET, NV_VIO_TOPOLOGY,
};

use super::general::get_status_message;

pub fn get_topologies() -> Result<Vec<NVVIOTOPOLOGYTARGET>> {
    let mut targets = [NVVIOTOPOLOGYTARGET::default(); 8];
    let mut topology = NV_VIO_TOPOLOGY {
        version: make_nvapi_version::<NV_VIO_TOPOLOGY>(1),
        vioTarget: targets,
        ..Default::default()
    };

    unsafe {
        let status = NvAPI_VIO_QueryTopology(&mut topology);
        if status != 0 {
            return Err(format!(
                "Failed to get topologies: {}",
                get_status_message(&status)
            ));
        }
    }

    let mut targets_result: Vec<NVVIOTOPOLOGYTARGET> = vec![];
    for i in 0..topology.vioTotalDeviceCount {
        targets_result.push(topology.vioTarget[i as usize]);
    }
    Ok(targets_result)
}
