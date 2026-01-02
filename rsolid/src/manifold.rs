//! Manifold rendering backend for rsolid
//!
//! This module provides integration with the manifold library for in-process
//! CSG rendering, enabling real-time measurements and operations.

#[cfg(feature = "manifold")]
pub use rsolid_manifold;

#[cfg(feature = "manifold")]
pub mod render {
    //! Rendering utilities for converting rsolid objects to manifold
    
    use rsolid_manifold::backend::{ManifoldBuilder, RenderedObject};
    
    /// Render backend trait for converting rsolid primitives to manifold
    pub trait ToManifold {
        fn to_manifold(&self) -> RenderedObject;
    }
    
    // Note: Full integration would require implementing ToManifold for all rsolid types
    // This is a starting point that users can extend for their specific needs
    
    /// Example helper to create a cube using manifold
    pub fn cube(x: f64, y: f64, z: f64) -> RenderedObject {
        ManifoldBuilder::cube(x, y, z)
    }
    
    /// Example helper to create a sphere using manifold
    pub fn sphere(radius: f64, segments: u32) -> RenderedObject {
        ManifoldBuilder::sphere(radius, segments)
    }
    
    /// Example helper to create a cylinder using manifold
    pub fn cylinder(radius_low: f64, radius_high: f64, height: f64, segments: u32) -> RenderedObject {
        ManifoldBuilder::cylinder(radius_low, radius_high, height, segments)
    }
}

#[cfg(feature = "manifold")]
pub fn export_manifold<P: AsRef<std::path::Path>>(
    path: P,
    _obj: &rsolid_manifold::backend::RenderedObject,
    formats: &[&str],
) -> std::io::Result<()> {
    let path = path.as_ref();
    let stem = std::path::Path::new("target/rsolid").join(path);
    std::fs::create_dir_all(stem.parent().unwrap())?;
    
    for format in formats {
        match *format {
            "stl" => {
                let _out_path = stem.with_extension("stl");
                eprintln!("STL export via manifold not yet implemented");
            }
            _ => {
                eprintln!("Unsupported format: {}", format);
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "manifold")]
    #[test]
    fn test_manifold_helpers() {
        use super::render::*;
        
        let cube = cube(10.0, 10.0, 10.0);
        let dims = cube.dimensions();
        assert!((dims[0] - 10.0).abs() < 0.1);
        
        let sphere = sphere(5.0, 16);
        let vertices = sphere.vertices();
        assert!(!vertices.is_empty());
    }
}
