use anyhow::Result;
use clap::Command;
use clap_complete::{generate, shells::*};
use std::io;

pub fn run_completions(cmd: &mut Command) -> Result<()> {
    let name = cmd.get_name().to_string();

    println!("=== Bash ===");
    generate(Bash, cmd, &name, &mut io::stdout());

    println!("\n=== Zsh ===");
    generate(Zsh, cmd, &name, &mut io::stdout());

    println!("\n=== Fish ===");
    generate(Fish, cmd, &name, &mut io::stdout());

    println!("\n=== PowerShell ===");
    generate(PowerShell, cmd, &name, &mut io::stdout());

    Ok(())
}

pub fn run_completions_shell(cmd: &mut Command, shell: &str) -> Result<()> {
    let name = cmd.get_name().to_string();
    match shell {
        "bash" => generate(Bash, cmd, &name, &mut io::stdout()),
        "zsh" => generate(Zsh, cmd, &name, &mut io::stdout()),
        "fish" => generate(Fish, cmd, &name, &mut io::stdout()),
        "powershell" => generate(PowerShell, cmd, &name, &mut io::stdout()),
        _ => {
            eprintln!("Unknown shell: {shell}. Use: bash, zsh, fish, powershell");
            std::process::exit(1);
        }
    }
    Ok(())
}

pub fn run_manpage(cmd: &mut Command) -> Result<()> {
    let man = clap_mangen::Man::new(cmd.clone());
    let mut buf = Vec::new();
    man.render(&mut buf)?;
    println!("{}", String::from_utf8_lossy(&buf));
    Ok(())
}
