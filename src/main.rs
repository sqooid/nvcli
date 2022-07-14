use crate::nvapi::handle::get_display_name;

pub mod cli;
pub mod nvapi;

fn main() {
    nvapi::nvapi::initialize();
    let display_configs = nvapi::display::get_display_config();
    println!("{}", display_configs[0]);
    println!("{}", display_configs[1]);
    let handles = nvapi::handle::get_display_handles();
    println!("{:?}", handles);
    println!("{}", get_display_name(handles[1]));
}
