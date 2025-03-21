# Pi U2F Tests

This package contains tests for the Raspberry Pi U2F implementation. The tests are designed to verify the functionality of the components that make up the Pi U2F security key.

## Test Structure

The tests are organized into several categories:

1. **Unit tests** for individual components:
   - `pi-hid`: Tests for the HID device interface
   - `pi-u2f-adapter`: Tests for the U2FHID protocol adapter
   - `pi-u2f-daemon`: Tests for the U2F daemon functionality

2. **Integration tests** that verify the complete flow:
   - Registration flow
   - Authentication flow
   - Error handling

## Running Tests

To run all tests, use the provided script:

```bash
./run_tests.sh
```

To run tests for a specific package:

```bash
cargo test -p pi-hid
cargo test -p pi-u2f-adapter
cargo test -p pi-u2f-daemon
cargo test -p pi-u2f-tests
```

## Mock Components

The tests use mock implementations of various components:

- **MockHidDevice**: A mock implementation of the HID device interface that captures write operations and allows test code to inject read data
- **MockU2fServer**: A mock implementation of the U2F server that returns predefined responses

## Test Environment

Some tests are designed to be skipped when run on a non-Pi environment (like a development machine) where `/dev/hidg0` doesn't exist. The tests will detect this situation and skip the relevant tests.

## Manual Testing

After deploying the U2F daemon to a Raspberry Pi Zero:

1. Connect the Pi to a computer via USB
2. Verify that the computer recognizes a new U2F security key
3. Test registration at https://webauthn.io or https://demo.yubico.com/webauthn-technical/registration
4. Test authentication after registration