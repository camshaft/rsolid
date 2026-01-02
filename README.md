# rsolid

OpenSCAD Utilities for Rust with optional in-process Manifold rendering

## Overview

rsolid provides a Rust API for creating 3D models with two rendering backends:

1. **OpenSCAD** (default) - Generates SCAD files and uses OpenSCAD for rendering
2. **Manifold** (optional) - In-process CSG rendering with measurement capabilities

## Manifold Backend

The manifold backend enables powerful new workflows:

- **In-process rendering** - No external OpenSCAD dependency required
- **Real-time measurements** - Query dimensions, bounding boxes, and mesh data as you build
- **Measurement-driven design** - Use measurements to create dependent geometry

### Enable Manifold Support

Add to your `Cargo.toml`:

```toml
[dependencies]
rsolid = { version = "0.1", features = ["manifold"] }
```

### Example with Manifold

```rust
use rsolid::manifold::render::*;

// Create primitives
let cube = cube(10.0, 10.0, 10.0);
let sphere = sphere(5.0, 32);

// Perform CSG operations
let result = cube.difference(&sphere);

// Take measurements
let dimensions = result.dimensions();
let bbox = result.bounding_box();

println!("Result dimensions: {:?}", dimensions);
println!("Bounding box: {:?}", bbox);

// Use measurements to create dependent geometry
let next_part = cube(dimensions[0] * 0.5, dimensions[1] * 0.5, 5.0)
    .translate(0.0, 0.0, dimensions[2]);
```

See `rsolid/examples/rsolid_manifold.rs` for a complete example.

## License

MIT

