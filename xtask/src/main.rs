use std::path::Path;
use xshell::{cmd, Shell};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Result<T = (), E = Error> = core::result::Result<T, E>;

fn main() -> Result {
    let sh = Shell::new()?;
    let ws = Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../")).canonicalize()?;
    let ws = &ws;
    sh.change_dir(ws);

    sh.create_dir("rsolid/src/primitive/")?;
    sh.create_dir("rsolid/src/extension/")?;
    sh.create_dir("rsolid/src/bosl/")?;

    let mut primitives = vec![];
    let mut extensions = vec![];
    let mut bosl = vec![];

    for file in sh.read_dir("definitions")? {
        let bindings = cmd!(sh, "cargo run --bin rsolid-bindgen -- {file}").read()?;

        let file = &file;
        let stem = file.file_stem().unwrap().to_str().unwrap();
        let (dir, name) = if let Some(name) = stem.strip_prefix("primitive-") {
            let name = name.replace('-', "_");
            primitives.push(name.clone());
            ("primitive", name)
        } else if let Some(name) = stem.strip_prefix("ext-") {
            let name = name.replace('-', "_");
            extensions.push(name.clone());
            ("extension", name)
        } else if let Some(name) = stem.strip_prefix("bosl-") {
            let name = name.replace('-', "_");
            bosl.push(name.clone());
            ("bosl", name)
        } else {
            unimplemented!("definition not yet handled: {stem:?}")
        };

        let out = Path::new("rsolid/src/")
            .join(dir)
            .join(name)
            .with_extension("rs");

        sh.write_file(out, bindings)?;
    }

    for (mods, name, flatten) in [
        (primitives, "primitive", true),
        (extensions, "extension", true),
        (bosl, "bosl", false),
    ] {
        let mut out = String::new();
        for m in mods {
            if flatten {
                out += &format!("mod {m};\npub use {m}::*;\n");
            } else {
                out += &format!("pub mod {m};\n\n");
            }
        }
        sh.write_file(Path::new("rsolid/src").join(name).with_extension("rs"), out)?;
    }

    cmd!(sh, "cargo +nightly fmt").run()?;
    cmd!(sh, "cargo test").run()?;

    for example in sh.read_dir("rsolid/examples")? {
        let name = example.file_stem().unwrap();
        cmd!(sh, "cargo run --example {name}").run()?;
    }

    Ok(())
}
