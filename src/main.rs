use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: zcrab <file>");
        return;
    }

    let verbose = args.contains(&"-v".to_string()) || args.contains(&"--verbose".to_string());

    let mut file_path = "";
    for arg in args.iter().skip(1) {
        if arg != "-v" && arg != "--verbose" {
            file_path = arg;
            break;
        }
    }

    if file_path.is_empty() {
        println!("Usage: zcrab <file>");
        return;
    }

    if !Path::new(file_path).exists() {
        if verbose {
            println!("File '{}' not found.", file_path);
        }
        return;
    }

    match infer::get_from_path(file_path) {
        Ok(Some(kind)) => {
            if verbose {
                println!("Detected {} (.{})", kind.mime_type(), kind.extension());
            }

            match kind.extension() {
                "zip" => zip(file_path, verbose),
                "gz"  => tar(file_path, verbose),
                "xz"  => tar(file_path, verbose),
                "zst" => tar(file_path, verbose),
                "tar" => tar(file_path, verbose),
                "lz4" => lz4(file_path, verbose),
                _ => {
                    if verbose {
                        println!("No support (yet) for .{}", kind.extension());
                    }
                }
            }
        }
        Ok(None) => {
            if verbose {
                println!("Unknown Signature.");
            }
        }
        Err(e) => {
            if verbose {
                println!("Error: {}", e);
            }
        }
    }
}

fn is_installed(program: &str) -> bool {
    Command::new(program)
        .arg("--version")
        .output()
        .is_ok()
}

fn zip(file: &str, _verbose: bool) {
    if is_installed("unzip") {
        let _ = Command::new("unzip")
            .arg(file)
            .status();
    } else {
        println!("Unzip is not installed. Please install unzip for your system")
    }
}

fn tar(file: &str, _verbose: bool) {
    if is_installed("tar") {
        let _ = Command::new("tar")
            .arg("-xf")
            .arg(file)
            .status();
    } else {
        println!("Tar is not installed. Please install tar for your system")
    }
}

fn lz4(file: &str, _verbose: bool) {
    if is_installed("tar") {
        if is_installed("lz4") {
            let _ = Command::new("tar")
                .arg("-I")
                .arg("lz4")
                .arg("-xf")
                .arg(file)
                .status();
        } else {
            println!("lz4 is not installed. Please install lz4 for your system")
        }
    } else {
        println!("Tar is not installed. Please install tar for your system")
    }
}