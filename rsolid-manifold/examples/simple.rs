fn main() {
    let sphere = rsolid_manifold::Manifold::sphere(4.0, 32);
    let vertices = sphere.to_mesh().vertices();
    let indices = sphere.to_mesh().indices();

    assert!(!vertices.is_empty());
    assert!(!indices.is_empty());

    println!("Vertices:\n{vertices:?}");
    println!("Indices:\n{indices:?}");
}
