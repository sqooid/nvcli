use nvapi_sys_new::{make_nvapi_version, NvAPI_VIO_GetGamma, NVVIOGAMMACORRECTION};

// pub fn get_gamma() -> NVVIOGAMMACORRECTION {
//     let mut gamma = NVVIOGAMMACORRECTION {
//         version: make_nvapi_version::<NVVIOGAMMACORRECTION>(1),
//         ..Default::default()
//     };
//     unsafe { NvAPI_VIO_GetGamma(hVioHandle, pGamma) }
// }
