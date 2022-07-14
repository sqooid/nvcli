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
}
