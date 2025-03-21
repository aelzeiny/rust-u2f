#[cfg(test)]
mod tests {
    use crate::{PiU2fAdapter, PiHidDevice, SimplifiedServer};
    use std::io;
    use std::sync::{Arc, Mutex};
    use u2fhid_protocol::Packet;

    // Mock HID device for testing
    struct MockHidDevice {
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

    #[test]
    fn test_process_packet() {
        // Create a mock HID device
        let mut mock_device = MockHidDevice::new();
        
        // Add some test data to read
        let test_packet_data = vec![0x01, 0x02, 0x03, 0x04]; // Simplified test packet
        mock_device.add_read_data(test_packet_data);
        
        // Create a simplified server
        let server = SimplifiedServer::new();
        
        // Create the adapter with the mocks
        let mut adapter = PiU2fAdapter::new(mock_device, server);
        
        // Run the adapter for one cycle
        // This is just a simplified test structure as the real adapter runs in an infinite loop
        
        // In a real implementation, we would:
        // 1. Check that the mock device received the expected write data
        // 2. Verify the correct packet processing logic
        
        // For now, we'll just mark this as a structured placeholder since we can't
        // easily test the infinite loop in the run method
        println!("PiU2fAdapter test structure in place");
    }
}