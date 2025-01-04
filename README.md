# rraylib

Provides an all-in-one solution for using raylib in Rust in a way that is both safe and idiomatic - or not, if you prefer.

| Library                                                          | Version | Description                                              |
|------------------------------------------------------------------|---------|----------------------------------------------------------|
| [raylib](https://github.com/raysan5/raylib/releases/tag/5.5)     | v5.5    | Provides a framework for writing 2D and 3D applications. |
| [raymath](https://github.com/raysan5/raylib/releases/tag/5.5)    | v5.5    | Provides a set of advanced math functions.               |
| [raygui](https://github.com/raysan5/raygui/releases/tag/4.0)     | v4.0    | Provides a set of simple GUI elements.                   |
| [Physac](https://github.com/victorfisac/Physac/releases/tag/1.1) | v1.1    | Provides a 2D physics engine for raylib.                 |

Unsafe bindings are provided by the `sys` module in this crate, and you may use raylib as you would in C if you prefer.

## Motivation

I was unsatisfied with the existing offerings for using raylib in Rust, so I decided to roll my own solution.

## Platform Support

Only Linux and Windows are officially supported. This crate does not provide compilation support for Android, Web, or Raspberry Pi.

MacOS should work, but it is not officially supported and will not be fixed if or when it breaks.

If you want to use this crate on a platform that is not officially supported, pull requests are welcome.
