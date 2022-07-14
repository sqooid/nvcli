use crate::nvapi::{display::set_display_config, handle::get_display_name};

pub mod cli;
pub mod nvapi;

fn main() -> crate::cli::error::Result<()> {
    nvapi::nvapi::initialize();
    let mut display_configs = nvapi::display::get_display_config()?;
    println!("{:#?}", &display_configs);
    set_display_config(&mut display_configs);
    let handles = nvapi::handle::get_display_handles();
    println!("{:?}", handles);
    println!("{}", get_display_name(handles[0]));

    Ok(())
}
