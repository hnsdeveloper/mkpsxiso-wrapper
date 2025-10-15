mod args;
mod command;
mod constants;
mod xml;

use args::Args;
use clap::Parser;
use command::get_mkpsxiso_run_command;
use std::{
    fs::File,
    io::{BufWriter, Write},
    process::{Command, Stdio},
};
use xml::generate_raw_assets_xml;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let mkpsxiso_exec_path = get_mkpsxiso_run_command(&args)?;
    let image_name = if let Some(name) = args.imagename {
        name
    } else {
        // Uses directory name for image name
        directory_name()
    };
    let appid = if let Some(appid) = args.appid {
        appid
    } else {
        directory_name().to_uppercase()
    };
    let xml_path = if let Some(path) = args.xml {
        path
    } else {
        let v = generate_raw_assets_xml(&args.path.unwrap(), &image_name, "PLAYSTATION", &appid)?;
        let file_name = format!("{}.xml", image_name);
        BufWriter::new(File::create(&file_name)?).write(&v)?;
        file_name
    };

    let mut cmd = Command::new(mkpsxiso_exec_path.as_str());
    cmd.arg(format!("{}", xml_path))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit());

    let mut child = cmd.spawn()?;
    let _ = child.wait()?;

    Ok(())
}

fn directory_name() -> String {
    let c_dir = std::env::current_dir().unwrap();
    let dir = c_dir.as_os_str().to_str().unwrap();
    let mut rev_idx = 0;
    if let Some((i, _)) = dir.chars().rev().enumerate().find(|(_, v)| *v == '/') {
        rev_idx = i;
    }
    String::from(&dir[dir.len() - rev_idx..dir.len()])
}
