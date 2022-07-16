use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Horizontal resolution in pixels. Defaults to current resolution.
    #[clap(short, long)]
    pub width: Option<u32>,

    /// Vertical resolution in pixels. Defaults to current resolution.
    #[clap(short, long)]
    pub height: Option<u32>,

    /// Scaling setting. Defaults to current scaling setting.
    #[clap(short, long)]
    pub scaling: Option<String>,

    /// Display ID to apply settings to. Defaults to primary display.
    #[clap(short, long)]
    pub display: Option<u32>,

    /// List connected display ID's and settings. Defaults to false.
    #[clap(short, long)]
    pub list: bool,

    /// Refresh rate. Defaults to current refresh rate
    #[clap(short, long)]
    pub refresh: Option<u32>,

    /// Gamma adjustment. Should be between 0.3 and 2.8
    #[clap(short, long)]
    pub gamma: Option<f32>,
}

impl Cli {
    pub fn display_config_needed(&self) -> bool {
        self.width.is_some()
            || self.height.is_some()
            || self.scaling.is_some()
            || self.refresh.is_some()
    }
    pub fn color_config_needed(&self) -> bool {
        self.gamma.is_some()
    }
}
