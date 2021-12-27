# printimg-rust

[![crates.io](https://img.shields.io/crates/v/printimg.svg)](https://crates.io/crates/printimg/)
[![crates.io](https://img.shields.io/crates/d/printimg)](https://crates.io/crates/printimg/)

Print an image or a video in terminal.  

![ferris](https://raw.githubusercontent.com/oza6ut0ne/printimg-rust/v0.4.3/pic/ferris.png)

## Installation (build manually with OpenCV)

### Ubuntu 20.04

```sh
$ sudo apt install libopencv-dev clang libclang-dev
$ cargo install printimg
```

### Ubuntu 18.04

```sh
$ sudo apt install libopencv-dev clang libclang-dev
$ cargo install printimg --features opencv-32
```

### Other (Use Docker)

Multi-architecture [docker image](https://hub.docker.com/r/oza6ut0ne/opencv) is available.

```sh
$ docker pull oza6ut0ne/opencv:4.3.0  # already installed in the image!
```

### Windows (experimental)

1. Install OpenCV and LLVM with chocolatey and set environment variables.  
(See [README.md of `twistedfall/opencv-rust`](https://github.com/twistedfall/opencv-rust/tree/v0.53.1#windows-package))
1. Then, install with `cargo`.

    ```cmd
    cargo install printimg
    ```

## Installation (without OpenCV)

```sh
cargo install printimg --no-default-features --features image
```

or download prebuilt binary from [Releases](https://github.com/oza6ut0ne/printimg-rust/releases).  
In this case only image files are supported, but OpenCV is not required.  
This is useful if OpenCV cannot be installed in the environment.

## Usage

```sh
# Print image.
$ printi foo.png

# Print video. (Requires OpenCV)
$ printi bar.mp4

# Print video from USB camera 0. (Requires OpenCV)
$ printi 0

# Print from url. (Requires OpenCV)
$ printi https://rustacean.net/assets/rustacean-flat-happy.png
```
