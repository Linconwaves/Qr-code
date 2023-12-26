# Rust QR Code Generator with Image Overlay

This Rust program generates a QR code containing a specified message and overlays it with an image, placing the image at the center of the QR code. The resulting image is then saved to an output file.

## Prerequisites

Before running the program, ensure you have Rust installed on your system. You can install Rust by following the instructions on the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)

## Usage

1. Clone the repository to your local machine:

    ```bash
    git clone https://github.com/linconwaves/Qr-code.git
    ```

2. Navigate to the project directory:

    ```bash
    cd Qr-code
    ```

3. Update the `data` variable in the `main` function of `qr-code.rs` with your desired message.

4. Place the image you want to overlay at the center of the QR code in the specified path (update `center_image_path`).

5. Run the program:

    ```bash
    cargo run
    ```

6. The resulting QR code with the overlaid image will be saved to the specified output path (update `output_path`).

## Configuration

- Update the `data` variable to change the message encoded in the QR code.
- Update the `center_image_path` variable to specify the path to the image you want to overlay.
- Update the `output_path` variable to set the path where the final image will be saved.

## Dependencies

- [qrcode](https://crates.io/crates/qrcode): A Rust library for QR code generation.
- [image](https://crates.io/crates/image): A Rust image processing library.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
