use clap::Parser;
use clap::builder::styling::{AnsiColor, Effects, Styles};

fn styles() -> Styles {
    Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default())
}

#[derive(Parser, Debug, Clone)]
#[command(version, author, about, long_about = None, styles = styles())]
pub struct Args {
    /// The path of input bam file
    #[arg(short, long)]
    pub input: String,
    /// output directory
	#[arg(short, long, default_value = "bamslim_out")]
	pub outdir: String,
    /// slim depth under this value
	#[arg(short, long, default_value = "200")]
	pub slim_depth: u32,
    /// process record log interval
    #[arg(short, long, default_value = "100000")]
    pub log_record: u32,
}



#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Args::command().debug_assert();
}
