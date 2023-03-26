# mini-linux-control-api

One of the simplest Rust Linux control APIs for intranet low-power IoT devices.

This is a simple Rust-based HTTP server that listens on port 65535 and provides three APIs:

- `/info`: Returns the network interface information of the current Linux device.
- `/halt`: Executes the halt command to halt the system.
- `/reboot`: Executes the reboot command to reboot the system.

## Dependencies

- Rust programming language
- Cargo package manager
- `hyper` library (version 1.0.0-rc.3, with "full" features)
- `tokio` library (version 1, with "full" features)
- `http_body_util` library (for handling HTTP body data)

## Setup

1. Ensure Rust and Cargo are installed on your system. If not, follow the [official installation guide](https://www.rust-lang.org/tools/install).

2. Clone this repository to your local machine and navigate to the project directory:

    ```sh
    git clone https://github.com/your_username/rust_api_server.git
    cd rust_api_server
    ```

3. Build the project:

    ```sh
    cargo build --release
    ```

4. Run the server:

    ```sh
    cargo run
    ```

The server should now be running and listening on port 65535.

## Usage

You can interact with the API server using any HTTP client, such as `curl`, a web browser, or a REST client like Postman.

### Get network interface information

```sh
curl <http://localhost:65535/info>
```

### Halt the system

```sh
curl -X POST <http://localhost:65535/halt>
```

### Reboot the system

```sh
curl -X POST <http://localhost:65535/reboot>
```

**Note:** Executing the `/halt` and `/reboot` commands requires root privileges. You may need to run the server with root privileges to use these endpoints.

**Note:** The `#![deny(warnings)]` attribute in the code will cause all warnings to be treated as errors. You can comment out or remove this line if you wish to ignore the warnings.

## License

This project is licensed under the [MIT License](LICENSE).
