pub mod cli;
pub mod nvapi;

use colored::*;
use std::str::FromStr;

use clap::Parser;
use nvapi::{
    display::get_display_config,
    general::{initialize, unload},
};

use crate::{
    cli::clap::Cli,
    nvapi::display::Output,
    nvapi::{display::set_display_config, scaling::Scaling},
};

fn main() {
    if std::env::args().len() < 2 {
        Cli::parse_from(["", "--help"]);
    }

    let config = Cli::parse();

    initialize();
    let result = get_display_config();
    let mut display_configs = match result {
        Ok(configs) => configs,
        Err(e) => {
            println!("{}: {}", "Failed to get current display config".red(), e);
            unload();
            return;
        }
    };

    if config.list {
        for config in display_configs.iter() {
            println!("{}\n", config.short_display());
        }

        unload();
        return;
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

    if let Some(width) = &config.width {
        display_configs[display_idx[0]]
            .source_mode_info
            .resolution
            .width = width.to_owned();
    }

    if let Some(height) = &config.height {
        display_configs[display_idx[0]]
            .source_mode_info
            .resolution
            .height = height.to_owned();
    }

    if let Some(scaling) = &config.scaling {
        display_configs[display_idx[0]].target_info[display_idx[1]]
            .details
            .scaling = match Scaling::from_str(scaling) {
            Ok(scaling) => scaling,
            Err(_) => {
                println!("{}", "Invalid scaling option".red());
                unload();
                return;
            }
        } as i32;
    }

    if let Some(refresh) = &config.refresh {
        display_configs[display_idx[0]].target_info[display_idx[1]]
            .details
            .refreshRate1K = refresh * 1000;
    }

    if config.display_config_needed() {
        let result = set_display_config(display_configs);
        match result {
            Ok(_) => println!("{}", "Successfully applied display settings".green()),
            Err(e) => {
                println!("{}: {}", "Failed to apply display config".red(), e);
            }
        };
    }

    unload();
}
