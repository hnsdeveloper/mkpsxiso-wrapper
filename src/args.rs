use crate::constants::MKPSXISO_EXEC_NAME;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long, required_unless_present = "xml")]
    pub path: Option<String>,

    #[arg(short, long, default_value(MKPSXISO_EXEC_NAME))]
    pub exec: String,

    #[arg(short, long)]
    pub xml: Option<String>,

    #[arg(short, long)]
    pub imagename: Option<String>,

    #[arg(short, long)]
    pub appid: Option<String>,
}
