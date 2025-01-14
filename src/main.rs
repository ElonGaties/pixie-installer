use std::error::Error;
use std::io::{self, Read, Write};
use inquire::{Confirm, Select, Text};
use windows_elevate::check_elevated;
use crate::installer::Installer;

mod os;
mod installer;

fn main() -> Result<(), Box<dyn Error>> {
    if !check_elevated()? {
        windows_elevate::elevate()?;
    }

    println!("Pixie.rip - The best Essential client modification.");

    let option_advanced = Confirm::new("Enable advanced mode")
        .with_default(false)
        .prompt()?;

    let installer = match option_advanced {
        true => {
            let name = Text::new("Var name: ")
                .with_placeholder("ESSENTIAL_CM_HOST")
                .prompt()?;
            let value = Text::new("Var value: ")
                .with_placeholder("wss://connect.pixie.rip/v1")
                .prompt()?;

            Installer::new(name, value)
        },
        false => Installer::default()
    };

    match Select::new("Install or Uninstall", vec!["Install", "Uninstall"]).prompt()? {
        "Install" => installer.install()?,
        "Uninstall" => installer.uninstall()?,
        _ => Err("unknown option (this should not happen)")?,
    }

    {
        print!("  \n  Press any key to continue!");
        io::stdout().flush().unwrap();
        let _ = io::stdin().read(&mut [0u8]).unwrap();
    }

    Ok(())
}