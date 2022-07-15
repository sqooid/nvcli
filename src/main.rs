pub mod cli;
pub mod nvapi;
pub mod windows;

use std::str::FromStr;

use clap::Parser;
use nvapi::{
    general::{initialize, unload},
    vio::get_topologies,
};

use crate::{
    cli::clap::Cli,
    nvapi::display::Output,
    nvapi::{display::set_display_config, scaling::Scaling},
};

fn main() -> crate::cli::error::Result<()> {
    if std::env::args().len() < 2 {
        Cli::parse_from(["", "--help"]);
    }

    let config = Cli::parse();

    initialize();
    let mut display_configs = nvapi::display::get_display_config()?;

    if config.list {
        for config in display_configs.iter() {
            println!("{}\n", config.short_display());
        }

        unload();
        return Ok(());
    }

    let mut display_idx: [usize; 2] = [0, 0];
    match config.display {
        Some(id) => {
            display_configs.iter().find(|info| {
                if info
                    .target_info
                    .iter()
                    .find(|target| {
                        if target.display_id == id {
                            true
                        } else {
                            display_idx[1] += 1;
                            false
                        }
                    })
                    .is_none()
                {
                    display_idx[0] += 1;
                    display_idx[1] = 0;
                    false
                } else {
                    true
                }
            });
        }
        None => {
            display_configs.iter().find(|x| {
                if x.source_mode_info.bGDIPrimary() == 1 {
                    true
                } else {
                    display_idx[0] += 1;
                    false
                }
            });
        }
    };

    if let Some(width) = config.width {
        display_configs[display_idx[0]]
            .source_mode_info
            .resolution
            .width = width;
    }

    if let Some(height) = config.height {
        display_configs[display_idx[0]]
            .source_mode_info
            .resolution
            .height = height;
    }

    if let Some(scaling) = config.scaling {
        display_configs[display_idx[0]].target_info[display_idx[1]]
            .details
            .scaling = Scaling::from_str(&scaling)? as i32;
    }

    if let Some(refresh) = config.refresh {
        display_configs[display_idx[0]].target_info[display_idx[1]]
            .details
            .refreshRate1K = refresh * 1000;
    }

    set_display_config(display_configs)?;

    unload();
    Ok(())
}
