#[cfg(test)]
mod tests {
    use crate::{FileSecretStore, AutoApproveUserPresence, Error};
    use tempfile::TempDir;
    use u2f_core::{AppId, KeyHandle, SecretStore, UserPresence};
    use std::io;

    #[test]
    fn test_file_secret_store() -> Result<(), Error> {
        // Create a temporary directory for our test
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let test_path = temp_dir.path().to_path_buf();
        
        // Create a test FileSecretStore with the temporary directory
        let store = FileSecretStore {
            storage_path: test_path.clone(),
        };
        
        // Create test AppId and KeyHandle
        let app_id = AppId::from_bytes(&[0u8; 32]);
        let key_handle = KeyHandle::from(&vec![0u8; 64]);
        
        // Test retrieving a non-existent key
        let result = SecretStore::retrieve_application_key(&store, &app_id, &key_handle)?;
        assert!(result.is_none(), "Expected None for non-existent key");
        
        // Test counter for a new application
        let counter = SecretStore::get_and_increment_counter(&store, &app_id, &key_handle)?;
        assert_eq!(counter, 0, "Initial counter should be 0");
        
        // Test counter increment
        let counter2 = SecretStore::get_and_increment_counter(&store, &app_id, &key_handle)?;
        assert_eq!(counter2, 1, "Counter should have incremented to 1");
        
        Ok(())
    }

    #[test]
    fn test_auto_approve_user_presence() {
        let presence = AutoApproveUserPresence;
        
        // Run the test in tokio runtime
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            // Test registration approval
            let app_id = AppId::from_bytes(&[0u8; 32]);
            let result = UserPresence::approve_registration(&presence, &app_id).await.unwrap();
            assert!(result, "Expected approval for registration");
            
            // Test authentication approval
            let result = UserPresence::approve_authentication(&presence, &app_id).await.unwrap();
            assert!(result, "Expected approval for authentication");
            
            // Test wink
            let result = UserPresence::wink(&presence).await;
            assert!(result.is_ok(), "Expected OK for wink");
        });
    }
}