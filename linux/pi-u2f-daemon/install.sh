#!/bin/bash
set -e

echo "Building Pi U2F daemon..."
cargo build --release -p pi-u2f-daemon

echo "Installing Pi U2F daemon..."
sudo cp ../target/release/pi-u2f-daemon /usr/local/bin/
sudo chmod +x /usr/local/bin/pi-u2f-daemon

echo "Installing systemd service..."
sudo cp pi-u2f.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable pi-u2f
sudo systemctl start pi-u2f

echo "Installation complete!"
echo "The Pi Zero U2F key service is now running."