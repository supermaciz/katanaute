use anyhow::Result;
use keyring::Entry;

const SERVICE_NAME: &str = "gtkata";
const TOKEN_KEY: &str = "auth_token";
const USER_KEY: &str = "user_email";

pub struct Config;

impl Config {
    pub fn save_token(token: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, TOKEN_KEY)?;
        entry.set_password(token)?;
        Ok(())
    }

    pub fn load_token() -> Result<Option<String>> {
        let entry = Entry::new(SERVICE_NAME, TOKEN_KEY)?;
        match entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn clear_token() -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, TOKEN_KEY)?;
        match entry.delete_credential() {
            Ok(_) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // Already cleared
            Err(e) => Err(e.into()),
        }
    }

    pub fn save_user_email(email: &str) -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, USER_KEY)?;
        entry.set_password(email)?;
        Ok(())
    }

    pub fn load_user_email() -> Result<Option<String>> {
        let entry = Entry::new(SERVICE_NAME, USER_KEY)?;
        match entry.get_password() {
            Ok(email) => Ok(Some(email)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn clear_user_email() -> Result<()> {
        let entry = Entry::new(SERVICE_NAME, USER_KEY)?;
        match entry.delete_credential() {
            Ok(_) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn clear_all() -> Result<()> {
        Self::clear_token()?;
        Self::clear_user_email()?;
        Ok(())
    }
}
