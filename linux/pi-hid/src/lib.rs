use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

pub struct PiHidDevice {
    file: File,
}

impl PiHidDevice {
    pub fn open() -> io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open("/dev/hidg0")?;
            
        Ok(Self { file })
    }
    
    pub fn write_report(&mut self, data: &[u8]) -> io::Result<usize> {
        self.file.write(data)
    }
    
    pub fn read_report(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.file.read(buf)
    }
}

#[cfg(test)]
mod tests;