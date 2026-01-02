# rsolid-manifold

Manifold integration for rsolid - enabling in-process CSG rendering with real-time measurements.

## Overview

This crate provides a rendering backend for rsolid that uses the [Manifold](https://github.com/elalish/manifold) library for in-process geometric operations. Unlike the default OpenSCAD backend, manifold allows you to:

- **Render geometry in-process** without external dependencies
- **Take measurements** of objects as you build them
- **Use measurements** to drive subsequent geometry creation
- **Export meshes** to various formats

## Key Features

### In-Process Rendering
No need to shell out to OpenSCAD - all CSG operations happen within your Rust process.

### Measurement Capabilities
```rust
use rsolid_manifold::backend::ManifoldBuilder;

let cube = ManifoldBuilder::cube(10.0, 10.0, 10.0);
let dimensions = cube.dimensions(); // [10.0, 10.0, 10.0]
let bbox = cube.bounding_box();     // [0.0, 0.0, 0.0, 10.0, 10.0, 10.0]
```

### CSG Operations
```rust
let sphere = ManifoldBuilder::sphere(5.0, 32);
let difference = cube.difference(&sphere);
let union = cube.union(&sphere);
let intersection = cube.intersection(&sphere);
```

### Transformations
```rust
let transformed = cube
    .translate(5.0, 0.0, 0.0)
    .rotate(0.0, 45.0, 0.0)
    .scale(2.0, 2.0, 2.0);
```

### Measurement-Driven Design
```rust
let base = ManifoldBuilder::cube(20.0, 20.0, 5.0);
let base_height = base.dimensions()[2];

// Use the base height to position the next part
let top = ManifoldBuilder::cube(15.0, 15.0, 3.0)
    .translate(2.5, 2.5, base_height);

let assembly = base.union(&top);
```

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
rsolid = { version = "0.1", features = ["manifold"] }
```

Or use rsolid-manifold directly:

```toml
[dependencies]
rsolid-manifold = "0.1"
```

## Examples

See the `examples/` directory for complete examples:

- `simple.rs` - Basic manifold usage
- `measurements.rs` - Demonstrating measurement capabilities
- `write_stl.rs` - Exporting to STL format (requires `output` feature)

Run an example:

```bash
cargo run --example measurements
```

## Integration with rsolid

When using rsolid with the `manifold` feature enabled, you can access manifold rendering directly:

```rust
use rsolid::manifold::render::*;

let cube = cube(10.0, 10.0, 10.0);
let dims = cube.dimensions();
println!("Cube dimensions: {:?}", dims);
```

## Credits

This crate builds upon [manifold-rs](https://github.com/WilstonOreo/manifold-rs) by WilstonOreo, 
which provides Rust bindings for the Manifold C++ library.

## License

MIT

