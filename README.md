# pugl-rs
`pugl-rs` - safe bindings for [pugl](https://github.com/lv2/pugl), a minimal portability layer for GUIs which is suitable for use in plugins and applications. 

> This project is WIP. Use at your own risk!

Pugl is vaguely similar to libraries like GLUT and GLFW, but has different
goals and priorities:

 * Minimal in scope, providing only a thin interface to isolate
   platform-specific details from applications.

 * Zero dependencies, aside from standard system libraries.

 * Support for embedding in native windows, for example as a plugin or
   component within a larger application that is not based on Pugl.

 * Explicit context and no static data, so that several instances can be used
   within a single program at once.

 * Consistent event-based API that makes dispatching in application or toolkit
   code easy with minimal boilerplate.

 * Suitable for both continuously rendering applications like games, and
   event-driven applications that only draw when necessary.

 * Well-integrated with windowing systems, with support for tracking and
   manipulating special window types, states, and styles.

 * Small, liberally licensed implementation that is suitable for vendoring
   and/or static linking.  Pugl can be installed as a library, or used by
   simply copying the implementation into a project.

## Status

Current `pugl` version is `0.5.5` (commit [66afe80](https://github.com/lv2/pugl/commit/66afe808e8c17f41cf6122158df96361cb42cccb))

At the moment most of the `pugl` functions are documented and available, except for:
- non-text clipboard handing
- Cairo and Vulkan backends (feel free to ask me if you need them!)

The bindings are tested on Linux, Windows and OSX (VM):
  - `pugl` links and builds successfully, stub backend works
  - OpenGL works on Linux and Windows (builds successfully on OSX, unable to verify if it actually works)

This project is WIP. Use at your own risk!

## Installation

Add the following to your `Cargo.toml`
```toml
pugl-rs = { git = "https://github.com/blepfx/pugl-rs", features = ["opengl", "cairo", "vulkan"] }
```

## Documentation

Go to the [examples](pugl-rs/examples) folder to see the usage examples of the Rust bindings
