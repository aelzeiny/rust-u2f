#!/bin/bash

# Make sure configfs is mounted
mount -t configfs none /sys/kernel/config 2>/dev/null || true

# Change to the USB gadget configuration directory
cd /sys/kernel/config/usb_gadget/

# Create a new FIDO gadget
mkdir -p fido
cd fido

# Set USB identification parameters to match Google Inc. tk-x001
echo 0x18d1 > idVendor  # Google Inc.
echo 0x5022 > idProduct # Custom product ID
echo 0x0200 > bcdUSB    # USB 2.0
echo 0x0100 > bcdDevice # Device version 1.0

# Setup device strings
mkdir -p strings/0x409
echo "Google Inc." > strings/0x409/manufacturer
echo "tk-x001" > strings/0x409/product

# Create configuration
mkdir -p configs/c.1/strings/0x409
echo "FIDO Configuration" > configs/c.1/strings/0x409/configuration
echo 120 > configs/c.1/MaxPower

# Create HID function for FIDO U2F
mkdir -p functions/hid.usb0

# Configure HID function
echo 0 > functions/hid.usb0/protocol
echo 0 > functions/hid.usb0/subclass
# 64 byte report length as per U2FHID protocol specification
echo 64 > functions/hid.usb0/report_length

# Standard U2F HID descriptor with 64-byte packets
# This descriptor defines two reports (IN and OUT) with 64 bytes each
echo -ne \\x06\\xd0\\xf1\\x09\\x01\\xa1\\x01\\x09\\x20\\x15\\x00\\x26\\xff\\x00\\x75\\x08\\x95\\x40\\x81\\x02\\x09\\x21\\x15\\x00\\x26\\xff\\x00\\x75\\x08\\x95\\x40\\x91\\x02\\xc0 > functions/hid.usb0/report_desc

# Link function to configuration
ln -s functions/hid.usb0 configs/c.1/

# Enable the gadget by binding to the UDC driver
ls /sys/class/udc > UDC

# Give proper permissions to the HID device
sleep 1
chmod 666 /dev/hidg0

# Print success message
echo "FIDO U2F gadget device configured successfully"
echo "The device is accessible at /dev/hidg0"