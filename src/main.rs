pub mod nvapi;
pub mod nvapi_sys;

fn main() {
    nvapi::initialize();
    let gpu_handles = nvapi::gpu::get_gpu_handles();
    let display_ids = nvapi::display::get_display_ids(gpu_handles[0]);
    println!("{:?}", &gpu_handles);
    println!("{:?}", &display_ids);
}
