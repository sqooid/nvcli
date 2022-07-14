use std::str::FromStr;

use clap::Parser;

use crate::{
    cli::clap::Cli,
    nvapi::{display::set_display_config, handle::get_display_name, scaling::Scaling},
};

pub mod cli;
pub mod nvapi;

fn main() -> crate::cli::error::Result<()> {
    let config = Cli::parse();

    nvapi::nvapi::initialize();
    let mut display_configs = nvapi::display::get_display_config()?;

    if config.list {
        for config in display_configs.iter() {
            println!("{}", config);
        }
        return Ok(());
    }

    let display_index: usize = match config.display {
        Some(id) => id as usize,
        None => {
            let mut index = 0;
            display_configs.iter().find(|x| {
                if x.source_mode_info.bGDIPrimary() == 1 {
                    true
                } else {
                    index += 1;
                    false
                }
            });
            index
        }
    };

    if let Some(width) = config.width {
        display_configs[display_index]
            .source_mode_info
            .resolution
            .width = width;
    }

    if let Some(height) = config.height {
        display_configs[display_index]
            .source_mode_info
            .resolution
            .height = height;
    }

    if let Some(scaling) = config.scaling {
        display_configs[display_index].target_info.details.scaling =
            Scaling::from_str(&scaling)? as i32;
    }

    set_display_config(&mut display_configs);
    let handles = nvapi::handle::get_display_handles();
    println!("{:?}", handles);
    println!("{}", get_display_name(handles[0]));

    Ok(())
}
