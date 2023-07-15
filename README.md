# RSRSM - Rust Server Management Tool

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

RSRSM is a server management tool written in Rust language. It provides an efficient way to manage and control RUST servers. With RSRSM, you can automate command execution using cron syntax and utilize it as a console for your RUST server.

## Features

- Manage and control RUST servers with ease
- Automate command execution using cron syntax
- Use RSRSM as a console for your RUST server

## Installation

1. Make sure you have Rust and Cargo installed on your system. If not, please follow the [official Rust installation guide](https://www.rust-lang.org/tools/install) to install them.

2. Install RSRSM using Cargo:
   ```bash
   cargo install --git https://github.com/morioka22/rsrsm
   ```

3. Once the installation is complete, you can use the `rsrsm` command in your terminal to launch the tool.

## Usage

To start using RSRSM, follow the steps below:

1. Create a YAML configuration file (e.g., `config.yaml`) with the following content:

   ```yaml
   ip: SERVER_IP
   port: RCON_PORT
   password: RCON_PASSWORD
   server_log: true

   jobs:
     - name: "Auto Message (every hour)"
       cron: "0 0 * * * *"
       commands:
         - say This is an automatic message.
     - name: "restart (at 4:00 AM UST)"
       cron: "0 0 4 * * *"
       commands:
         - save
         - restart
   ```

   Replace `SERVER_IP`, `RCON_PORT`, and `RCON_PASSWORD` with the actual values for your RUST server.

2. Save the YAML file.

3. In your terminal, execute the following command:

   ```bash
   rsrsm --config ./config.yaml
   ```

   RSRSM will launch and read the configuration from the specified YAML file. It will start managing and controlling your RUST server based on the defined jobs and cron schedules.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more information.

## Contributing

Contributions are welcome! If you have any suggestions, improvements, or bug fixes, please open an issue or submit a pull request.

## Acknowledgements

- The Rust programming language: <https://www.rust-lang.org/>
- Cron syntax: <https://en.wikipedia.org/wiki/Cron>
