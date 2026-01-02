# Manifold Integration Architecture

This document explains the architecture and design decisions for the manifold integration in rsolid.

## Overview

The manifold integration enables in-process CSG rendering as an alternative to the default OpenSCAD backend. This addresses the key requirement from the issue: enabling measurements of objects during operations.

## Architecture

### Workspace Structure

```
rsolid/
├── rsolid/                    # Core library
│   ├── src/
│   │   ├── manifold.rs       # Optional manifold integration module
│   │   └── ...
│   └── Cargo.toml            # With optional 'manifold' feature
│
└── rsolid-manifold/          # Manifold backend crate
    ├── src/
    │   ├── lib.rs            # C++ bindings via cxx
    │   ├── backend.rs        # High-level Rust API
    │   ├── manifold_rs.cpp   # C++ wrapper code
    │   └── manifold_rs.h     # C++ header
    ├── glm/                  # GLM library (gitignored)
    ├── manifold/             # Manifold library (gitignored)
    └── Cargo.toml
```

### Key Design Decisions

#### 1. Separate Crate for Manifold

**Decision**: Create `rsolid-manifold` as a separate crate instead of embedding it in `rsolid`.

**Rationale**:
- Manifold has significant C++ dependencies (requires CMake build)
- Users who only need SCAD generation shouldn't pay the compilation cost
- Allows independent versioning and development
- Prevents circular dependencies (rsolid doesn't depend on rsolid-manifold)

#### 2. Optional Feature Flag

**Decision**: Make manifold support opt-in via feature flag in `rsolid`.

**Rationale**:
- Keeps default rsolid lightweight
- Users explicitly choose the backend they need
- Allows conditional compilation of integration code

Usage:
```toml
[dependencies]
rsolid = { version = "0.1", features = ["manifold"] }
```

#### 3. Measurement-Focused API

**Decision**: Design the backend API around measurement capabilities.

**Rationale**:
- This was the key requirement from the issue
- Enables new workflows not possible with OpenSCAD backend
- Makes the value proposition clear to users

Key capabilities:
- `dimensions()` - Get width, height, depth
- `bounding_box()` - Get min/max coordinates
- `vertices()` / `indices()` - Access mesh data
- All measurements available immediately after operations

#### 4. Standalone Backend Module

**Decision**: Create `backend.rs` module separate from the FFI bindings.

**Rationale**:
- Clean separation between low-level C++ bindings and high-level Rust API
- `backend.rs` provides ergonomic Rust types and methods
- FFI layer (`lib.rs`) handles C++ interop via cxx
- Users typically use the backend module, not FFI directly

#### 5. Build System Integration

**Decision**: Use cxx and CMake for C++ integration.

**Rationale**:
- cxx provides type-safe Rust/C++ interop
- CMake is standard build system for Manifold library
- Follows existing pattern from manifold-rs upstream

**Trade-offs**:
- Requires CMake at build time
- Longer compilation times
- Large git submodules (glm, manifold) - addressed via .gitignore

#### 6. No Automatic Conversion from rsolid Types

**Decision**: Don't implement automatic conversion from rsolid primitives to manifold.

**Rationale**:
- rsolid types are designed for SCAD code generation
- Conversion would require maintaining parallel implementations
- Better to provide direct manifold constructors for immediate use
- Users who need rsolid → manifold can implement custom converters

Current approach:
```rust
// Direct manifold usage (recommended)
use rsolid::manifold::render::*;
let cube = cube(10.0, 10.0, 10.0);

// Rather than:
// use rsolid::*;
// let cube = cube([10, 10, 10]).to_manifold(); // Not implemented
```

## Key Components

### RenderedObject

The main type users interact with. Wraps a Manifold instance and provides:
- Measurement methods (dimensions, bounding box)
- CSG operations (union, difference, intersection)
- Transformations (translate, rotate, scale)
- Mesh access (vertices, indices)

### ManifoldBuilder

Factory for creating primitives:
- `cube(x, y, z)`
- `sphere(radius, segments)`
- `cylinder(r_low, r_high, height, segments)`
- `empty()`
- `tetrahedron()`

### Integration Points

1. **Direct usage**: Import `rsolid::manifold::render` when manifold feature is enabled
2. **Export function**: Placeholder for future STL/mesh export support
3. **Examples**: Comprehensive examples showing measurement-driven workflows

## Future Enhancements

### Short Term
- Implement STL/mesh export functions
- Add volume calculation (manifold library supports this)
- Provide more primitive types (torus, polyhedron from points)

### Medium Term
- Implement automatic conversion from rsolid types to manifold
- Support for 2D operations (extrude, revolve from profiles)
- Parallel operation support

### Long Term
- Consider making manifold the default backend
- Performance optimizations
- Integration with other mesh libraries (OpenCASCADE, etc.)

## Testing Strategy

### Unit Tests
- `backend.rs`: Test measurement accuracy, CSG operations
- FFI layer: Basic smoke tests for C++ integration

### Integration Tests
- Examples serve as integration tests
- `measurements.rs`: Comprehensive measurement capabilities
- `rsolid_manifold.rs`: Integration with rsolid feature flags

### Manual Testing
```bash
# Test basic build
cargo build

# Test with manifold
cargo build --features rsolid/manifold

# Run tests
cargo test --package rsolid-manifold
cargo test --package rsolid --features manifold

# Run examples
cargo run --package rsolid-manifold --example measurements
cargo run --package rsolid --example rsolid_manifold --features manifold
```

## Performance Considerations

### Build Time
- First build compiles Manifold C++ library (~30-60 seconds)
- Subsequent builds are incremental
- Consider caching the build directory in CI

### Runtime
- Manifold operations are in-process (fast)
- No subprocess spawning overhead
- Memory efficient (no intermediate files)
- Triangle count affects performance (use appropriate segment counts)

## Maintenance Notes

### Dependencies
- Manifold library via git submodules (glm, manifold)
- Submodules are gitignored (large files)
- Users need to `git submodule update --init --recursive` if they clone directly
- Or use the provided setup in the repository

### Updating Manifold
To update to a newer Manifold version:
1. Update submodule: `cd rsolid-manifold/manifold && git pull origin main`
2. Test compatibility
3. Update version in Cargo.toml if needed

### Cross-Platform Support
- Linux: Fully supported (primary development platform)
- macOS: Should work (needs testing)
- Windows: May require additional setup (MSVC, CMake)

## Credits

This integration is based on [manifold-rs](https://github.com/WilstonOreo/manifold-rs) by WilstonOreo.
