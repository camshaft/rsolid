//! Example demonstrating integration between rsolid and manifold
//!
//! This example shows how to use the manifold backend directly for
//! in-process rendering with measurement capabilities.

#[cfg(feature = "manifold")]
use rsolid::manifold::render::*;

#[cfg(feature = "manifold")]
fn main() {
    println!("=== Rsolid + Manifold Integration Example ===\n");

    // Create primitives using manifold
    println!("Creating primitives with manifold backend...");
    
    let cube1 = cube(10.0, 10.0, 10.0);
    println!("✓ Created 10x10x10 cube");
    
    let sphere1 = sphere(5.0, 32);
    println!("✓ Created sphere with radius 5.0");
    
    // Perform CSG operations
    println!("\nPerforming CSG operations...");
    
    let difference = cube1.difference(&sphere1);
    let dims = difference.dimensions();
    println!("✓ Created difference (cube - sphere)");
    println!("  Result dimensions: {:.2} x {:.2} x {:.2}", dims[0], dims[1], dims[2]);
    
    // Use measurements to create dependent geometry
    println!("\nUsing measurements to create dependent geometry...");
    
    let bbox = difference.bounding_box();
    let width = bbox[3] - bbox[0];
    let height = bbox[4] - bbox[1];
    
    println!("  Measured bounding box: width={:.2}, height={:.2}", width, height);
    
    // Create a new object based on measurements
    let scaled_cube = cube(width * 0.5, height * 0.5, width * 0.5);
    println!("✓ Created scaled cube based on measurements");
    println!("  Size: {:.2} x {:.2} x {:.2}", width * 0.5, height * 0.5, width * 0.5);
    
    // Chain transformations
    println!("\nChaining transformations...");
    
    let transformed = cylinder(3.0, 3.0, 15.0, 24)
        .translate(0.0, 0.0, 5.0)
        .rotate(0.0, 45.0, 0.0)
        .scale(1.5, 1.5, 1.5);
    
    let transformed_dims = transformed.dimensions();
    println!("✓ Created and transformed cylinder");
    println!("  Final dimensions: {:.2} x {:.2} x {:.2}", 
             transformed_dims[0], transformed_dims[1], transformed_dims[2]);
    
    // Complex composition
    println!("\nCreating complex composition...");
    
    let base = cube(20.0, 20.0, 2.0);
    let pillar1 = cylinder(2.0, 2.0, 10.0, 16).translate(5.0, 5.0, 2.0);
    let pillar2 = cylinder(2.0, 2.0, 10.0, 16).translate(-5.0, 5.0, 2.0);
    let pillar3 = cylinder(2.0, 2.0, 10.0, 16).translate(5.0, -5.0, 2.0);
    let pillar4 = cylinder(2.0, 2.0, 10.0, 16).translate(-5.0, -5.0, 2.0);
    
    let structure = base
        .union(&pillar1)
        .union(&pillar2)
        .union(&pillar3)
        .union(&pillar4);
    
    let structure_vertices = structure.vertices();
    println!("✓ Created structure with base and 4 pillars");
    println!("  Total vertices: {}", structure_vertices.len() / 3);
    
    // Demonstrate measurement during construction
    println!("\nDemonstrating measurement-driven construction...");
    
    let part1 = cube(10.0, 10.0, 10.0);
    let part1_dims = part1.dimensions();
    
    // Use part1's height to position part2
    let part2 = cube(8.0, 8.0, 5.0)
        .translate(1.0, 1.0, part1_dims[2]);
    
    let assembly = part1.union(&part2);
    let assembly_dims = assembly.dimensions();
    
    println!("✓ Created two-part assembly with measurement-driven positioning");
    println!("  Part 1 height: {:.2}", part1_dims[2]);
    println!("  Assembly total height: {:.2}", assembly_dims[2]);
    
    println!("\n=== Example Complete ===");
    println!("\nKey features demonstrated:");
    println!("  ✓ Direct manifold primitive creation");
    println!("  ✓ CSG operations (union, difference, intersection)");
    println!("  ✓ Real-time measurements (dimensions, bounding box)");
    println!("  ✓ Measurement-driven geometry creation");
    println!("  ✓ Transformation chaining");
    println!("  ✓ Complex compositions");
}

#[cfg(not(feature = "manifold"))]
fn main() {
    eprintln!("This example requires the 'manifold' feature.");
    eprintln!("Run with: cargo run --example rsolid_manifold --features manifold");
    std::process::exit(1);
}
