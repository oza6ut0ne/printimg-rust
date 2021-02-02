# printimg-rust

[![crates.io](https://img.shields.io/crates/v/printimg.svg)](https://crates.io/crates/printimg/)
[![crates.io](https://img.shields.io/crates/d/printimg)](https://crates.io/crates/printimg/)

Print an imgae or an video in terminal using OpenCV.  
Currently only Linux is supported.

![ferris](https://raw.githubusercontent.com/oza6ut0ne/printimg-rust/v0.2.1/pic/ferris.png)

## Installation

### Ubuntu 20.04

```sh
$ sudo apt install libopencv-dev clang libclang-dev
$ cargo install printimg
```

### Ubuntu 18.04

```sh
$ sudo apt install libopencv-dev clang libclang-dev
$ cargo install printimg --no-default-features --features opencv-32
```

### Other (Use Docker)

Multi-architecture [docker image](https://hub.docker.com/r/oza6ut0ne/opencv) is available.

```sh
$ docker pull oza6ut0ne/opencv:4.3.0  # already installed in the image!
```

### Usage

```sh
# print image.
$ printi foo.png

# print video.
$ printi bar.mp4

# print video from USB camera 0.
$ printi 0

# print from url.
$ printi https://rustacean.net/assets/rustacean-flat-happy.png
```

