use clap::Parser;
use std::env;
use std::process::Command;


/// For CTF: invoke specified docker environment and mount current directory 
#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    /// want Docker image（ex: ubuntu:16.04）
    #[arg(default_value = "ubuntu:16.04")]
    image: String,

   
    /// flag to build-essential automatically install
    #[arg(short, long)]
    install_tools: bool,
}

fn main() {
    let args = Args::parse();

    // current working directory
    let cwd = env::current_dir().unwrap();
    let cwd_str = cwd.to_string_lossy().to_string();

    // Docker command 
    let mut docker_cmd = vec![
        "run".to_string(),
        "-it".to_string(),
        "--rm".to_string(),
        "-v".to_string(),
        format!("{}:/workspace", cwd_str),
        args.image.clone(),
    ];

    // build-essential automatically install mode
    if args.install_tools {
        docker_cmd.push("bash".to_string());
        docker_cmd.push("-c".to_string());
        docker_cmd.push(
            "apt update && apt install -y build-essential gdb && cd /workspace && bash".to_string(),
        );
    }

    println!("==> Launching Docker: {:?}", docker_cmd);

    // execute
    let status = Command::new("docker")
        .args(&docker_cmd)
        .status()
        .expect("failed to run docker");

    if !status.success() {
        eprintln!("Docker exited with error");
    }
}
