use std::fmt::Display;

use nvapi_sys_new::{
    make_nvapi_version, NvAPI_DISP_GetDisplayConfig, NvAPI_GPU_GetConnectedDisplayIds,
    NvPhysicalGpuHandle, NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO,
    NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO_V1, NV_DISPLAYCONFIG_PATH_INFO,
    NV_DISPLAYCONFIG_PATH_TARGET_INFO_V2, NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1, NV_GPU_DISPLAYIDS,
};

use super::scaling::Scaling;

pub fn get_display_ids(gpu_handle: NvPhysicalGpuHandle) -> Vec<NV_GPU_DISPLAYIDS> {
    let mut display_ids: Vec<NV_GPU_DISPLAYIDS>;
    let mut display_count: u32 = 0;
    unsafe {
        NvAPI_GPU_GetConnectedDisplayIds(gpu_handle, std::ptr::null_mut(), &mut display_count, 0);
        display_ids = vec![
            NV_GPU_DISPLAYIDS {
                version: make_nvapi_version::<NV_GPU_DISPLAYIDS>(3),
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

pub fn get_display_config() -> Vec<NvDisplayConfigPathInfo> {
    let mut path_info_count: u32 = 0;
    // Get count
    unsafe {
        NvAPI_DISP_GetDisplayConfig(&mut path_info_count, std::ptr::null_mut());
    }
    // Allocate path info
    let mut path_info = vec![
        NV_DISPLAYCONFIG_PATH_INFO {
            version: make_nvapi_version::<NV_DISPLAYCONFIG_PATH_INFO>(2),
            ..Default::default()
        };
        path_info_count as usize
    ];
    let mut source_mode_info =
        vec![NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1::default(); path_info_count as usize];
    for (i, info) in path_info.iter_mut().enumerate() {
        info.sourceModeInfo = &mut source_mode_info[i];
    }
    unsafe {
        NvAPI_DISP_GetDisplayConfig(&mut path_info_count, path_info.as_mut_ptr());
    }
    let mut target_info_array = vec![];
    let mut advanced_target_info_array = vec![];
    for info in path_info.iter_mut() {
        advanced_target_info_array.push(NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO {
            version: make_nvapi_version::<NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO>(1),
            ..NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO::default()
        });
        target_info_array.push(vec![
            NV_DISPLAYCONFIG_PATH_TARGET_INFO_V2 {
                details: advanced_target_info_array
                    .last_mut()
                    .expect("Advanced target info array missing items"),
                ..NV_DISPLAYCONFIG_PATH_TARGET_INFO_V2::default()
            };
            info.targetInfoCount as usize
        ]);
        info.targetInfo = target_info_array
            .last_mut()
            .expect("Target info array missing items")
            .as_mut_ptr();
    }
    // Get target info
    unsafe {
        NvAPI_DISP_GetDisplayConfig(&mut path_info_count, path_info.as_mut_ptr());
    }

    // Collect outputs
    let mut output = vec![];
    for (i, info) in path_info.into_iter().enumerate() {
        output.push(NvDisplayConfigPathInfo {
            target_info_count: info.targetInfoCount,
            target_info: target_info_array[i]
                .iter()
                .map(|x| NvDisplayConfigPathTargetInfo {
                    display_id: x.displayId,
                    details: advanced_target_info_array[i],
                    target_id: x.targetId,
                })
                .collect(),
            source_mode_info: source_mode_info[i],
        });
    }
    output
}

#[derive(Debug, Clone)]
pub struct NvDisplayConfigPathInfo {
    pub target_info_count: u32,
    pub target_info: Vec<NvDisplayConfigPathTargetInfo>,
    pub source_mode_info: NV_DISPLAYCONFIG_SOURCE_MODE_INFO_V1,
}
#[derive(Debug, Clone)]
pub struct NvDisplayConfigPathTargetInfo {
    pub display_id: u32,
    pub details: NV_DISPLAYCONFIG_PATH_ADVANCED_TARGET_INFO,
    pub target_id: u32,
}

impl Display for NvDisplayConfigPathInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"
ID: {}
Primary: {}
Resolution: {} x {}
Refresh rate: {} Hz
Color depth: {} bits
Scaling: {}"#,
            // if self.target_info[0].details.primary() == 1 {
            //     "true"
            // } else {
            //     "false"
            // },
            self.target_info[0].display_id,
            if self.source_mode_info.bGDIPrimary() == 0 {
                "true"
            } else {
                "false"
            },
            self.source_mode_info.resolution.width,
            self.source_mode_info.resolution.height,
            self.target_info[0].details.refreshRate1K / 1000,
            self.source_mode_info.resolution.colorDepth,
            Scaling::from(self.target_info[0].details.scaling)
        )
    }
}
