macro_rules! assert_2d_snapshot {
    ($expr:expr) => {{
        insta::assert_snapshot!($expr);

        struct __LOCATION__;

        $crate::testing::persist_snapshot(
            $expr,
            "svg",
            file!(),
            core::any::type_name::<__LOCATION__>(),
        );
    }};
}

macro_rules! assert_3d_snapshot {
    ($expr:expr) => {
        insta::assert_snapshot!($expr);
        struct __LOCATION__;

        $crate::testing::persist_snapshot(
            $expr,
            "stl",
            file!(),
            core::any::type_name::<__LOCATION__>(),
        );
    };
}

pub fn persist_snapshot(expr: impl core::fmt::Display, out: &str, source: &str, module: &str) {
    let mut module: Vec<_> = module.split("::").collect();
    module.pop();
    let module = module.join("__");

    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join(source.trim_start_matches("rsolid/"))
        .parent()
        .unwrap()
        .join("snapshots")
        .join(module)
        .with_extension("scad");

    std::fs::create_dir_all(src.parent().unwrap()).unwrap();
    std::fs::write(&src, expr.to_string()).unwrap();

    let dest = src.with_extension(out);

    let out = std::process::Command::new("openscad")
        .arg("-o")
        .arg(&dest)
        .arg("--export-format")
        .arg(out)
        .arg(&src)
        .output()
        .unwrap();

    if !out.status.success() {
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&out.stderr));
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&out.stdout));
        panic!("openscad exited with {}", out.status);
    }
}
