use crate::args::Args;
use crate::constants::MKPSXISO_EXEC_NAME;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use which::which;

pub fn is_executable(path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        let mode = metadata.permissions().mode();
        mode & 0o111 != 0
    } else {
        false
    }
}

pub fn get_mkpsxiso_run_command(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    // Checks for mkpsxiso
    match args.exec.as_str() {
        MKPSXISO_EXEC_NAME => match which(MKPSXISO_EXEC_NAME) {
            Ok(path) => Ok(path.into_os_string().into_string().unwrap()),
            Err(error) => {
                eprintln!("Couldn't find {}.", MKPSXISO_EXEC_NAME);
                return Err(Box::new(error));
            }
        },
        s => {
            if Path::new(s).exists() && is_executable(s) {
                Ok(s.into())
            } else {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Executable for mkpsxiso not existent or not executable.",
                )))
            }
        }
    }
}
