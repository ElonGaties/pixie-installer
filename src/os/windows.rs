#[cfg(target_os = "windows")]
use std::{env, error::Error};
#[cfg(target_os = "windows")]
use winreg::{enums::*, RegKey};

#[cfg(target_os = "windows")]
pub fn delete_env_var(name: &str) -> Result<(), Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";

    let (reg_env, _) = hklm.create_subkey(path)?;
    reg_env.delete_value(name)?;

    env::remove_var(name);
    Ok(())
}

#[cfg(target_os = "windows")]
pub fn set_env_var(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let path = "SYSTEM\\CurrentControlSet\\Control\\Session Manager\\Environment";

    let (reg_env, _) = hklm.create_subkey(path)?;
    reg_env.set_value(name, &value)?;

    env::set_var(name, value);
    Ok(())
}
