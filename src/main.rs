pub mod cli;
pub mod nvapi;

use std::str::FromStr;

use clap::Parser;
use nvapi::{
    display::get_display_config,
    general::{initialize, unload},
    rotation::Rotation,
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
            bunt::println!("{$red}Failed to get current display config: {}{/$}", e);
            unload();
            return;
        }
    };

    if config.list {
        for config in display_configs.iter() {
            config.print_short();
        }

        unload();
        return;
    }

    let mut display_idx: [usize; 2] = [0, 0];
    match config.display {
        Some(id) => {
            if display_configs
                .iter()
                .find(|info| {
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
                })
                .is_none()
            {
                bunt::println!("{$red}Display with specified ID not found{/$}");
                unload();
                return;
            };
        }
        None => {
            if display_configs
                .iter()
                .find(|x| {
                    if x.source_mode_info.bGDIPrimary() == 1 {
                        true
                    } else {
                        display_idx[0] += 1;
                        false
                    }
                })
                .is_none()
            {
                bunt::println!("{$red}No primary display found{/$}");
                unload();
                return;
            };
        }
    };

    if let Some(width) = &config.resolution_x {
        display_configs[display_idx[0]]
            .source_mode_info
            .resolution
            .width = width.to_owned();
    }

    if let Some(height) = &config.resolution_y {
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
                bunt::println!("{$red}Invalid scaling option{/$}");
                unload();
                return;
            }
        } as i32;
    }

    if let Some(position_x) = &config.position_x {
        display_configs[display_idx[0]].source_mode_info.position.x = position_x.clone();
    }

    if let Some(position_y) = &config.position_y {
        display_configs[display_idx[0]].source_mode_info.position.y = position_y.clone();
    }

    if let Some(refresh) = &config.refresh {
        display_configs[display_idx[0]].target_info[display_idx[1]]
            .details
            .refreshRate1K = refresh * 1000;
    }

    if let Some(rotation) = &config.rotation {
        let rotation = match Rotation::try_from(rotation) {
            Ok(rot) => rot.0,
            Err(e) => {
                bunt::println!("{[red]}", e);
                unload();
                return;
            }
        };
        display_configs[display_idx[0]].target_info[display_idx[1]]
            .details
            .rotation = rotation;
    }

    if config.display_config_needed() {
        let result = set_display_config(display_configs);
        match result {
            Ok(_) => bunt::println!("{$green}Successfully applied display settings{/$}"),
            Err(e) => {
                bunt::println!("{$red}Failed to apply display config: {}{/$}", e);
            }
        };
    }

    unload();
}
