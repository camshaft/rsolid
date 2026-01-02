//! Example demonstrating manifold rendering with measurements
//!
//! This example shows how to:
//! - Create 3D objects using manifold
//! - Perform CSG operations
//! - Take measurements during operations

use rsolid_manifold::backend::{ManifoldBuilder, RenderedObject};

fn main() {
    println!("=== Manifold Rendering with Measurements ===\n");

    // Create a cube
    let cube = ManifoldBuilder::cube(10.0, 10.0, 10.0);
    println!("Created a 10x10x10 cube");
    print_measurements(&cube, "Cube");

    // Create a sphere
    let sphere = ManifoldBuilder::sphere(3.0, 32);
    println!("\nCreated a sphere with radius 3.0");
    print_measurements(&sphere, "Sphere");

    // Perform difference operation (cube with sphere hole)
    let result = cube.difference(&sphere);
    println!("\nCreated cube with sphere hole (difference operation)");
    print_measurements(&result, "Cube - Sphere");

    // Perform union operation
    let cube2 = ManifoldBuilder::cube(10.0, 10.0, 10.0).translate(5.0, 0.0, 0.0);
    println!("\nCreated second cube translated by (5, 0, 0)");
    print_measurements(&cube2, "Translated Cube");

    let union = ManifoldBuilder::cube(10.0, 10.0, 10.0).union(&cube2);
    println!("\nCreated union of two cubes");
    print_measurements(&union, "Union");

    // Demonstrate scaling based on measurements
    let _original_dims = cube.dimensions(); // Could use this for dynamic scaling
    let scale_factor = 2.0;
    let scaled = ManifoldBuilder::cube(10.0, 10.0, 10.0).scale(scale_factor, scale_factor, scale_factor);
    println!("\nScaled cube by factor of {}", scale_factor);
    print_measurements(&scaled, "Scaled Cube");

    // Create a cylinder
    let cylinder = ManifoldBuilder::cylinder(5.0, 5.0, 20.0, 32);
    println!("\nCreated a cylinder (radius 5.0, height 20.0)");
    print_measurements(&cylinder, "Cylinder");

    println!("\n=== Demonstration Complete ===");
}

fn print_measurements(obj: &RenderedObject, name: &str) {
    let dims = obj.dimensions();
    let bbox = obj.bounding_box();
    let vertices = obj.vertices();
    let indices = obj.indices();

    println!("  {} Measurements:", name);
    println!("    Dimensions: {:.2} x {:.2} x {:.2}", dims[0], dims[1], dims[2]);
    println!("    Bounding Box:");
    println!("      Min: ({:.2}, {:.2}, {:.2})", bbox[0], bbox[1], bbox[2]);
    println!("      Max: ({:.2}, {:.2}, {:.2})", bbox[3], bbox[4], bbox[5]);
    println!("    Mesh: {} vertices, {} triangles", vertices.len() / 3, indices.len() / 3);
}
