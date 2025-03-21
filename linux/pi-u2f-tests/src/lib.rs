#[cfg(test)]
mod tests {
    use std::io;
    use std::sync::{Arc, Mutex};
    use u2f_core::{AppId, Challenge};
    use pi_u2f_adapter::PiHidDevice;
    
    // Mock HID device for integration testing
    pub struct MockHidDevice {
        read_data: Arc<Mutex<Vec<Vec<u8>>>>,
        write_data: Arc<Mutex<Vec<Vec<u8>>>>,
    }

    impl MockHidDevice {
        fn new() -> Self {
            Self {
                read_data: Arc::new(Mutex::new(Vec::new())), 
                write_data: Arc::new(Mutex::new(Vec::new())),
            }
        }

        fn add_read_data(&self, data: Vec<u8>) {
            self.read_data.lock().unwrap().push(data);
        }

        fn get_write_data(&self) -> Vec<Vec<u8>> {
            self.write_data.lock().unwrap().clone()
        }
    }

    impl PiHidDevice for MockHidDevice {
        fn read_report(&mut self, buf: &mut [u8]) -> io::Result<usize> {
            if let Some(data) = self.read_data.lock().unwrap().pop() {
                let len = std::cmp::min(data.len(), buf.len());
                buf[..len].copy_from_slice(&data[..len]);
                Ok(len)
            } else {
                // No data to read, simulate blocking by returning 0
                Ok(0)
            }
        }

        fn write_report(&mut self, data: &[u8]) -> io::Result<usize> {
            self.write_data.lock().unwrap().push(data.to_vec());
            Ok(data.len())
        }
    }
    
    // Integration test that simulates a complete U2F registration flow
    #[tokio::test]
    async fn test_u2f_registration_flow() {
        // Set up test components
        let device = MockHidDevice::new();
        
        // Simulate a U2F register request from a browser
        let app_id = AppId::from_bytes(&[0u8; 32]); // Simplified test AppId
        // For simplicity, we won't use Challenge directly since it doesn't have a public constructor
        // We'll just work with the raw packet data
        
        // Create a Register request packet
        // In a real implementation, this would be properly formatted
        // according to the U2FHID protocol
        let register_packet_data = vec![
            0x83, 0x01, // REGISTER command
            0x00, 0x20, // Challenge length (32 bytes)
            // Challenge data would go here (32 bytes)
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            // Application ID would go here (32 bytes)
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
            0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88,
        ];
        
        // Feed the packet to our mock device
        device.add_read_data(register_packet_data);
        
        // In a full integration test, we would:
        // 1. Start a complete PiU2fAdapter with a real U2fService
        // 2. Process the registration request
        // 3. Verify the response format
        // 4. Test authentication flows
        
        // For now, this is a structured placeholder for the integration test
        println!("U2F Registration flow integration test structure in place");
    }
}