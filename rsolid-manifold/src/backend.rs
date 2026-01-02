//! Manifold backend for rsolid - enables in-process CSG rendering
//! 
//! This module provides a rendering backend that converts rsolid objects
//! into manifold geometry, enabling real-time measurements and operations.

use crate::{Manifold, Mesh};

/// A rendered manifold object with measurement capabilities
pub struct RenderedObject {
    manifold: Manifold,
}

impl RenderedObject {
    /// Create a new rendered object from a manifold
    pub fn new(manifold: Manifold) -> Self {
        Self { manifold }
    }

    /// Get the underlying manifold
    pub fn manifold(&self) -> &Manifold {
        &self.manifold
    }

    /// Get the mesh representation
    pub fn to_mesh(&self) -> Mesh {
        self.manifold.to_mesh()
    }

    /// Get the vertices of the mesh
    pub fn vertices(&self) -> Vec<f32> {
        self.to_mesh().vertices()
    }

    /// Get the indices of the mesh
    pub fn indices(&self) -> Vec<u32> {
        self.to_mesh().indices()
    }

    /// Get a bounding box of the object as [min_x, min_y, min_z, max_x, max_y, max_z]
    pub fn bounding_box(&self) -> [f64; 6] {
        let vertices = self.vertices();
        if vertices.is_empty() {
            return [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        }

        let mut min_x = f64::INFINITY;
        let mut min_y = f64::INFINITY;
        let mut min_z = f64::INFINITY;
        let mut max_x = f64::NEG_INFINITY;
        let mut max_y = f64::NEG_INFINITY;
        let mut max_z = f64::NEG_INFINITY;

        // Vertices are stored as [x, y, z, x, y, z, ...]
        for i in (0..vertices.len()).step_by(3) {
            let x = vertices[i] as f64;
            let y = vertices[i + 1] as f64;
            let z = vertices[i + 2] as f64;

            min_x = min_x.min(x);
            min_y = min_y.min(y);
            min_z = min_z.min(z);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
            max_z = max_z.max(z);
        }

        [min_x, min_y, min_z, max_x, max_y, max_z]
    }

    /// Get the dimensions (width, height, depth) of the object
    pub fn dimensions(&self) -> [f64; 3] {
        let bbox = self.bounding_box();
        [
            bbox[3] - bbox[0], // width (x)
            bbox[4] - bbox[1], // height (y)
            bbox[5] - bbox[2], // depth (z)
        ]
    }

    /// Get the volume of the object (approximate, based on mesh)
    pub fn volume(&self) -> f64 {
        // This is a simplified volume calculation
        // For accurate volume, manifold library should provide this
        // For now, we return 0.0 as a placeholder
        0.0
    }

    /// Perform union with another object
    pub fn union(&self, other: &Self) -> Self {
        Self::new(self.manifold.union(&other.manifold))
    }

    /// Perform difference with another object
    pub fn difference(&self, other: &Self) -> Self {
        Self::new(self.manifold.difference(&other.manifold))
    }

    /// Perform intersection with another object
    pub fn intersection(&self, other: &Self) -> Self {
        Self::new(self.manifold.intersection(&other.manifold))
    }

    /// Translate the object
    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        Self::new(self.manifold.translate(x, y, z))
    }

    /// Scale the object
    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        Self::new(self.manifold.scale(x, y, z))
    }

    /// Rotate the object (angles in degrees)
    pub fn rotate(&self, x: f64, y: f64, z: f64) -> Self {
        Self::new(self.manifold.rotate(x, y, z))
    }
}

/// Builder for creating manifold primitives
pub struct ManifoldBuilder;

impl ManifoldBuilder {
    /// Create a cube with the given dimensions
    pub fn cube(x: f64, y: f64, z: f64) -> RenderedObject {
        RenderedObject::new(Manifold::cube(x, y, z))
    }

    /// Create a sphere with the given radius and segment count
    pub fn sphere(radius: f64, segments: u32) -> RenderedObject {
        RenderedObject::new(Manifold::sphere(radius, segments))
    }

    /// Create a cylinder with given parameters
    pub fn cylinder(radius_low: f64, radius_high: f64, height: f64, segments: u32) -> RenderedObject {
        RenderedObject::new(Manifold::cylinder(radius_low, radius_high, height, segments))
    }

    /// Create an empty manifold
    pub fn empty() -> RenderedObject {
        RenderedObject::new(Manifold::empty())
    }

    /// Create a tetrahedron
    pub fn tetrahedron() -> RenderedObject {
        RenderedObject::new(Manifold::tetrahedron())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cube_dimensions() {
        let cube = ManifoldBuilder::cube(10.0, 20.0, 30.0);
        let dims = cube.dimensions();
        
        // Check dimensions are approximately correct
        assert!((dims[0] - 10.0).abs() < 0.1);
        assert!((dims[1] - 20.0).abs() < 0.1);
        assert!((dims[2] - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_sphere_creation() {
        let sphere = ManifoldBuilder::sphere(5.0, 32);
        let vertices = sphere.vertices();
        let indices = sphere.indices();
        
        assert!(!vertices.is_empty());
        assert!(!indices.is_empty());
    }

    #[test]
    fn test_union() {
        let cube1 = ManifoldBuilder::cube(10.0, 10.0, 10.0);
        let cube2 = ManifoldBuilder::cube(10.0, 10.0, 10.0).translate(5.0, 0.0, 0.0);
        let union = cube1.union(&cube2);
        
        let vertices = union.vertices();
        assert!(!vertices.is_empty());
    }

    #[test]
    fn test_difference() {
        let cube = ManifoldBuilder::cube(10.0, 10.0, 10.0);
        let sphere = ManifoldBuilder::sphere(3.0, 16);
        let result = cube.difference(&sphere);
        
        let vertices = result.vertices();
        assert!(!vertices.is_empty());
    }

    #[test]
    fn test_bounding_box() {
        let cube = ManifoldBuilder::cube(10.0, 20.0, 30.0);
        let bbox = cube.bounding_box();
        
        // Cube is created with one corner at origin
        assert!(bbox[0] <= 0.1); // min_x close to 0
        assert!(bbox[1] <= 0.1); // min_y close to 0
        assert!(bbox[2] <= 0.1); // min_z close to 0
        assert!((bbox[3] - 10.0).abs() < 0.1); // max_x
        assert!((bbox[4] - 20.0).abs() < 0.1); // max_y
        assert!((bbox[5] - 30.0).abs() < 0.1); // max_z
    }
}
