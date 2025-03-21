#[cfg(test)]
mod tests {
    use crate::PiHidDevice;
    use std::fs::File;
    use std::io::{self, Write};
    use std::path::{Path, PathBuf};
    use tempfile::tempdir;

    // Mock device for testing
    struct MockHidDevice {
        file: File,
        path: PathBuf,
    }

    impl MockHidDevice {
        fn new() -> io::Result<Self> {
            let dir = tempdir()?;
            let path = dir.path().join("hidg0");
            let file = File::create(&path)?;
            Ok(Self { file, path: path.clone() })
        }

        fn path(&self) -> &Path {
            &self.path
        }

        fn write_data(&mut self, data: &[u8]) -> io::Result<usize> {
            self.file.write(data)
        }
    }

    #[test]
    fn test_write_report() -> io::Result<()> {
        let mock_device = MockHidDevice::new()?;
        let device_path = mock_device.path().to_string_lossy().to_string();
        
        // Create test data
        let test_data = [0x01, 0x02, 0x03, 0x04];
        
        // This test is structured but will be skipped since we can't actually
        // open the device file on a non-Pi system
        if Path::new("/dev/hidg0").exists() {
            let mut device = PiHidDevice::open()?;
            let written = device.write_report(&test_data)?;
            assert_eq!(written, test_data.len());
        } else {
            println!("Skipping PiHidDevice test as /dev/hidg0 doesn't exist");
        }
        
        Ok(())
    }
}