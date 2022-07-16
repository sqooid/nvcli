use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Horizontal resolution in pixels. Defaults to current resolution.
    #[clap(short = 'x', long)]
    pub resolution_x: Option<u32>,

    /// Vertical resolution in pixels. Defaults to current resolution.
    #[clap(short = 'y', long)]
    pub resolution_y: Option<u32>,

    /// Scaling setting. Defaults to current scaling setting. Valid values are:
    /// bfs - balanced full screen,
    /// ffs - forced full screen,
    /// fc  - forced centered,
    /// far - forced aspect ratio,
    /// bar - balanced aspect ratio,
    /// bc  - balanced centered,
    /// fis - forced integer scaling
    #[clap(short, long)]
    pub scaling: Option<String>,

    /// Display ID to apply settings to. Defaults to primary display.
    #[clap(short, long)]
    pub display: Option<u32>,

    /// List connected display ID's and settings instead of applying settings. Defaults to false.
    #[clap(short, long)]
    pub list: bool,

    /// Refresh rate. Defaults to current refresh rate
    #[clap(short, long)]
    pub refresh: Option<u32>,

    /// x coordinate of monitor's top left corner.
    /// (0,0) is located at the top left corner of the primary monitor.
    /// Negative values must be specified using "=" e.g. `-X=-1080`
    /// Defaults to current value
    #[clap(short = 'X', long)]
    pub position_x: Option<i32>,

    /// y coordinate of monitor's top left corner.
    #[clap(short = 'Y', long)]
    pub position_y: Option<i32>,
}

impl Cli {
    pub fn display_config_needed(&self) -> bool {
        self.resolution_x.is_some()
            || self.resolution_y.is_some()
            || self.scaling.is_some()
            || self.refresh.is_some()
            || self.position_x.is_some()
            || self.position_y.is_some()
    }
}
