# rraylib

Provides an all-in-one solution for using raylib in Rust in a way that is both safe and idiomatic - or not, if you prefer.

| Library                                                          | Version | Description                                              |
|------------------------------------------------------------------|---------|----------------------------------------------------------|
| [raylib](https://github.com/raysan5/raylib/releases/tag/5.5)     | v5.5    | Provides a framework for writing 2D and 3D applications. |
| [raymath](https://github.com/raysan5/raylib/releases/tag/5.5)    | v5.5    | Provides a set of advanced math functions.               |
| [rlgl](https://github.com/raysan5/raylib/releases/tag/5.5)       | v5.5    | Provides an abstraction layer for OpenGL.                |
| [raygui](https://github.com/raysan5/raygui/releases/tag/4.0)     | v4.0    | Provides a set of simple GUI elements.                   |
| [Physac](https://github.com/victorfisac/Physac/releases/tag/1.1) | v1.1    | Provides a basic 2D physics engine.                      |

## Unsafe Bindings

Unsafe bindings are provided by the `sys` module in this crate, and you may use raylib as you would in C if you prefer.

## Motivation

I was unsatisfied with the existing offerings for using raylib in Rust, so I decided to roll my own solution.

## Platform Support

This crate is developed on Linux and as such that's where development will focus. Windows support will also be officially maintained, however it's not a priority.

MacOS should work, but it is not officially supported and will not be fixed if or when it breaks.

This crate does not provide compilation support for Android, Web, or Raspberry Pi and there are no plans to change that.

Pull requests are welcome for any of the above, or any other issue.

## Features

| Feature         | Description                                                     |
|-----------------|-----------------------------------------------------------------|
| `wayland`       | Use Wayland on Linux.                                           |
| `x11`           | Use X11 on Linux (default).                                     |
| `opengl_43`     | Enables OpenGL 4.3 support.                                     |
| `opengl_33`     | Enables OpenGL 3.3 support (default).                           |
| `opengl_21`     | Enables OpenGL 2.1 support.                                     |
| `opengl_11`     | Enables OpenGL 1.1 support.                                     |
| `opengl_es_30`  | Enables OpenGL ES 3.0 support.                                  |
| `opengl_es_20`  | Enables OpenGL ES 2.0 support.                                  |
| `raygui`        | Enables raygui support.                                         |
| `physac`        | Enables Physac support.                                         |
| `sdl`           | Enables the SDL backend.                                        |
| `external_glfw` | Use the system GLFW lib instead of the one bundled with raylib. |
