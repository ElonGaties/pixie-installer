use std::error::Error;
use crate::os;

pub struct Installer {
    name: String,
    value: String,
}

impl Installer {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }

    pub fn install(&self) -> Result<(), Box<dyn Error>> {
        match os::set_env_var(&self.name, &self.value) {
            Ok(()) => {
                println!("  Successfully installed!\n");

                println!("  Important:");
                println!("   - Restart your Launcher/Game to apply changes!\n");

                println!("  Need help?");
                println!("  Website: pixie.rip");
                println!("  Discord: discord.gg/pixiemc\n");
                println!("  If you run into any problems, join our Discord for support!\n");
                Ok(())
            }
            Err(e) => {
                println!("  \nInstallation failed!");
                println!("  Please run the installer again as administrator.\n");
                println!("  Need help? Join our Discord: discord.gg/pixiemc\n");
                Err(e)
            }
        }
    }

    pub fn uninstall(&self) -> Result<(), Box<dyn Error>> {
        match os::delete_env_var(&self.name) {
            Ok(()) => {
                println!("  Successfully uninstalled!\n");

                println!("  Important:");
                println!("   - Restart your Launcher/Game to apply changes!\n");

                println!("  Need help?");
                println!("  Website: pixie.rip");
                println!("  Discord: discord.gg/pixiemc\n");
                println!("  If you run into any problems, join our Discord for support!\n");
                Ok(())
            }
            Err(e) => {
                println!("  \nUninstallation failed!");
                println!("  Please run the installer again as administrator.\n");
                println!("  Need help? Join our Discord: discord.gg/pixiemc\n");
                Err(e)
            }
        }
    }
}

impl Default for Installer {
    fn default() -> Self {
        Self {
            name: "ESSENTIAL_CM_HOST".to_string(),
            value: "wss://connect.pixie.rip/v1".to_string(),
        }
    }
}