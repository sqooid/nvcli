use crate::{cli::error::Result, nvapi::display::get_display_config};

pub fn find_primary_display_id() -> Result<u32> {
    let result = get_display_config();

    let display_configs = match result {
        Ok(configs) => configs,
        Err(e) => {
            return Err(e);
        }
    };
    let mut display_id: u32 = 0;
    if display_configs
        .iter()
        .find(|x| {
            if x.source_mode_info.bGDIPrimary() == 1 {
                display_id = x.target_info[0].display_id;
                true
            } else {
                false
            }
        })
        .is_none()
    {
        Err("No primary display found".to_owned())
    } else {
        Ok(display_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::nvapi::general::{initialize, unload};

    use super::*;

    #[test]
    fn test_find_primary_display() {
        initialize();
        let primary_id = find_primary_display_id();
        match primary_id {
            Ok(id) => println!("{id}"),
            Err(e) => println!("{e}"),
        }
        unload();
    }
}
