# Soundcore Device Faker

## Running Locally

TODO

## Running Remotely

The USB bluetooth adapter that I use doesn't seem to work with [Bumble](https://github.com/google/bumble), so I instead run this project on a spare Raspberry Pi I had. If you end up in the same situation, consider this option.

### Setup

Copy `.env.example` to `.env`. Set `SOUNDCORE_DEVICE_FAKER_REMOTE_USER` to your username on the remote machine and `SOUNDCORE_DEVICE_FAKER_REMOTE_HOST` to the address of the remote machine.

`SOUNDCORE_DEVICE_FAKER_REMOTE_TRANSPORT_SPEC` tells [Bumble](https://github.com/google/bumble) what bluetooth adapter to use. The value in `.env.example` is appropriate for a Raspberry Pi 3 Model B, and presumably other Rasperry Pis too. See [https://google.github.io/bumble/transports/index.html](https://google.github.io/bumble/transports/index.html) for information on selecting a transport.

Once `.env` is set up, run `just remote-init` to copy over the source files, create a venv, and install dependencies with pip.

### Running

To run on the configured remote machine, use `just remote-run a3004`, replacing a3004 with the name of your device configuration file excluding .toml. Whenever the file is modified, responses will be reloaded automatically. This does not affect the name or rfcomm uuid.

In order to keep files on the remote machine up to date while editing locally, use `just remote-sync-watch`. Files will be synchronized whenever anything changes.
