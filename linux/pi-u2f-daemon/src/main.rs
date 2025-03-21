use std::io;
use std::path::PathBuf;

use async_trait::async_trait;
use clap::Parser;
use pi_hid::PiHidDevice;
use pi_u2f_adapter::{PiU2fAdapter, SimplifiedServer};
use thiserror::Error;
use tokio::time::{sleep, Duration};
use tracing::{error, info};
use tracing_subscriber::FmtSubscriber;
use u2f_core::{try_reverse_app_id, AppId, OpenSSLCryptoOperations, U2fService, UserPresence};

#[cfg(test)]
mod tests;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to open HID device: {0}")]
    HidDevice(io::Error),

    #[error("Home directory path could not be retrieved from the operating system")]
    HomeDirectoryNotFound,
}

/// Simple UserPresence implementation that automatically approves all requests
/// For a real implementation, you would want to connect this to a physical button
pub struct AutoApproveUserPresence;

#[async_trait]
impl UserPresence for AutoApproveUserPresence {
    async fn approve_registration(&self, application: &AppId) -> Result<bool, io::Error> {
        let site_name = try_reverse_app_id(application).unwrap_or(String::from("site"));
        info!("Auto-approving registration for {}", site_name);
        // Add a small delay to simulate user interaction
        sleep(Duration::from_millis(200)).await;
        Ok(true)
    }

    async fn approve_authentication(&self, application: &AppId) -> Result<bool, io::Error> {
        let site_name = try_reverse_app_id(application).unwrap_or(String::from("site"));
        info!("Auto-approving authentication for {}", site_name);
        // Add a small delay to simulate user interaction
        sleep(Duration::from_millis(200)).await;
        Ok(true)
    }

    async fn wink(&self) -> Result<(), io::Error> {
        info!("Wink request received");
        Ok(())
    }
}

/// Simple file-based secret store that stores keys in the user's home directory
pub struct FileSecretStore {
    storage_path: PathBuf,
}

impl FileSecretStore {
    pub fn new() -> Result<Self, Error> {
        let home_dir = dirs::home_dir().ok_or(Error::HomeDirectoryNotFound)?;
        let storage_path = home_dir.join(".config").join("pi-u2f");
        
        // Ensure the directory exists
        std::fs::create_dir_all(&storage_path).map_err(Error::Io)?;
        
        Ok(Self { storage_path })
    }
}

impl u2f_core::SecretStore for FileSecretStore {
    fn add_application_key(&self, key: &u2f_core::ApplicationKey) -> io::Result<()> {
        // A simple implementation that serializes keys to JSON files
        let key_file = self.storage_path.join(format!("{}.json", hex::encode(key.application.as_ref())));
        let key_data = serde_json::to_string(key)?;
        std::fs::write(key_file, key_data)?;
        Ok(())
    }

    fn get_and_increment_counter(
        &self,
        application: &AppId,
        _handle: &u2f_core::KeyHandle,
    ) -> io::Result<u2f_core::Counter> {
        // A simple implementation that keeps counters in separate files
        let counter_file = self.storage_path.join(format!("{}.counter", hex::encode(application.as_ref())));
        
        let counter = if counter_file.exists() {
            let data = std::fs::read_to_string(&counter_file)?;
            data.trim().parse::<u32>().unwrap_or(0)
        } else {
            0
        };
        
        // Increment and write back
        std::fs::write(&counter_file, (counter + 1).to_string())?;
        
        Ok(counter)
    }

    fn retrieve_application_key(
        &self,
        application: &AppId,
        handle: &u2f_core::KeyHandle,
    ) -> io::Result<Option<u2f_core::ApplicationKey>> {
        let key_file = self.storage_path.join(format!("{}.json", hex::encode(application.as_ref())));
        
        if key_file.exists() {
            let data = std::fs::read_to_string(key_file)?;
            let key: u2f_core::ApplicationKey = serde_json::from_str(&data)?;
            
            // Verify the key handle matches
            if key.handle.eq_consttime(handle) {
                Ok(Some(key))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    
    // Initialize logging
    let subscriber = FmtSubscriber::builder()
        .with_max_level(if args.verbose {
            tracing::Level::DEBUG
        } else {
            tracing::Level::INFO
        })
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set default subscriber");

    info!("Starting Pi Zero U2F daemon");
    
    if let Err(err) = run().await {
        error!("Error encountered, exiting: {}", err);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Error> {
    // Create the U2F service components
    let attestation = u2f_core::self_signed_attestation();
    let crypto = OpenSSLCryptoOperations::new(attestation);
    let user_presence = AutoApproveUserPresence;
    let secrets = FileSecretStore::new()?;
    
    // Create the U2F service
    let u2f_service = U2fService::new(secrets, crypto, user_presence);
    
    // Create a simplified server for now
    let server = SimplifiedServer::new();
    
    // Open the Pi HID device
    info!("Opening HID device /dev/hidg0");
    let hid_device = PiHidDevice::open().map_err(Error::HidDevice)?;
    
    // Create and run the adapter
    info!("U2F security key is ready!");
    let mut adapter = PiU2fAdapter::new(hid_device, server);
    adapter.run().map_err(Error::Io)
}