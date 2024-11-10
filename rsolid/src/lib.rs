#[cfg(test)]
#[macro_use]
mod testing;

mod block;
pub mod bosl;
mod ext;
mod extension;
mod helpers;
pub mod import;
pub mod mask;
mod object;
mod operator;
mod parameter;
mod primitive;
pub mod scad;
mod shape;
mod types;
// TODO
//mod var;

pub use ext::*;
pub use extension::*;
pub use helpers::*;
pub use import::*;
pub use object::{IntoObject, Object};
pub use operator::Operator;
pub use primitive::*;
pub use shape::*;
pub use types::*;

pub fn export<V: scad::Scad>(v: &V, path: &std::path::Path, renders: &[&str]) {
    let out = v.to_scad();

    let stem = std::path::Path::new("target/rsolid").join(path);
    std::fs::create_dir_all(stem.parent().unwrap()).unwrap();

    let scad = stem.with_extension("scad");
    eprintln!("rendering {}...", scad.display());
    std::fs::write(&scad, out).unwrap();
    eprintln!("  done");

    for ext in renders {
        let out = stem.with_extension(ext);
        eprintln!("rendering {}...", out.display());

        let cmd = std::process::Command::new("openscad")
            .arg("-o")
            .arg("-")
            .arg("--export-format")
            .arg(ext)
            .arg("--render")
            .arg("true")
            .arg(&scad)
            .stderr(std::process::Stdio::inherit())
            .stdout(std::process::Stdio::piped())
            .spawn()
            .unwrap()
            .wait_with_output()
            .unwrap();

        assert!(cmd.status.success());

        std::fs::write(out, cmd.stdout).unwrap();

        println!("  done");
    }
}

#[macro_export]
macro_rules! export {
    ($value:expr) => {
        $crate::export!($value, &[]);
    };
    ($value:expr, $extra:expr) => {{
        let path = std::path::Path::new(concat!(env!("CARGO_PKG_NAME"), ".scad"));
        $crate::export(&$value, path, $extra);
    }};
    ($value:expr, $name:expr, $extra:expr) => {{
        let path = std::path::Path::new(env!("CARGO_PKG_NAME"))
            .join($name)
            .with_extension("scad");
        $crate::export(&$value, &path, $extra);
    }};
}
