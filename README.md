# `reflect`

A minimal yet powerful library for ray reflection simulation in Rust.

Requires the latest stable version of the [Rust Compiler](https://www.rust-lang.org/tools/install)

Uses [`nalgebra`](https://nalgebra.org/) for general computation.

## Crates

The core of this project is the `reflect` crate, containing the main `Mirror` trait and simulation logic

The `reflect_mirrors` crate contains several example implementations of reflective surfaces that can be used in simulations. This is where you should look if you need an example of how to implement your own custom mirror shapes.

There are integrations extending this library with more functionality such as:

- `reflect_glium` Which enables running and visualising 2D and 3D simulations using OpenGL.
- `reflect_json` Which enables serialisation/deserialisation of simulation data with the JSON format. Some example simulations in their JSON representation can be found in the `assets` directory.

Other third-party integrations can easily be created over the simple API of the `reflect` crate. It is advised to check it's documentation:

```shell
cargo doc -p reflect --no-deps --open
```

The binary crate `run_sim_json` can deserialise, run, then view simulations using `reflect_glium` and `reflect_json`:

```shell
cargo run -r -p run_sim_json "<path/to/simulation.json>" [max_reflection_count]
```

### Controls for `reflect_glium`

The `reflect_glium` binary crate allows viewing simulations where you can move around and rotate the camera. Here are the controls:

- Use the WASD keys (or ZQSD) to move forward, left, backward, and right, respectively.
- Use the space bar to move up and the shift key to move down.
- Click and drag your mouse on the screen to look around, and rotate the camera.
- Use the right/left arrow key to increase/decrease camera rotation sensitivity.
- Use the up/down key to increase/decrease movement speed.
