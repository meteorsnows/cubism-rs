# cubism-rs: Rust bindings for Live2D Cubism

A rust wrapper around the [Live2D Cubism SDK](https://live2d.github.io/) with extra functionality.

This library currently requires nightly to work due to the use of the allocator api. I would love
to remove this constraint but I do not know of a way to allocate a dst with a specific alignment
without the use of the raw allocator api.


The exposed api is completely unstable atm and is very likely to change!

![Demo](cubism_demo.gif)

## General usage notes

The `cubism-sys` crate requires the Live2DCubismCore library to build and link properly.
The build script finds the library by reading the environment variable 'CUBISM_CORE' for the path.

If you set the variable to 'third-party' for example, then your project layout should look like this:
```
your-project:
    src/
        *.rs
    third-party/
        lib/
            windows/
                x86/
                    Live2DCubismCore.lib
                x86_64/
                    Live2DCubismCore.lib
             ...
    Cargo.toml
```
