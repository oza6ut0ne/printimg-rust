# printimg-rust

[![crates.io](https://img.shields.io/crates/v/printimg.svg)](https://crates.io/crates/printimg/)
[![crates.io](https://img.shields.io/crates/d/printimg)](https://crates.io/crates/printimg/)

Print an imgae or an video in terminal using OpenCV 4.  
Currently Linux is only supported.

![ferris](https://raw.githubusercontent.com/oza6ut0ne/printimg-rust/v0.1.0/pic/ferris.png)

## Installation

### Ubuntu 20.04

```sh
$ sudo apt install libopencv-dev clang clang-dev
$ cargo install printimg
```

### Other (Use Docker)

Multi-architecture [docker image](https://hub.docker.com/repository/docker/oza6ut0ne/opencv) is available.

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

