use rsolid_bindgen::{backend::rust as backend, definition::Definitions};

fn main() {
    let mut definitions = vec![];
    for arg in std::env::args().skip(1) {
        let file = std::fs::read_to_string(&arg).unwrap();
        let d: Definitions = toml::from_str(&file).unwrap();
        definitions.push(d);
    }

    let mut options = backend::Options::default();
    options.main_crate = true;
    let mut out = std::io::stdout();
    backend::generate(&options, &definitions, &mut out).unwrap();
}
