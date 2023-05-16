use askama::Template;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Template)]
#[template(path = "lib.askama", escape = "none")]
struct LibTemplate {
    mod_names: Vec<String>,
}

#[derive(Template)]
#[template(path = "vec.askama", escape = "none")]
struct VecTemplate<'a> {
    struct_name: String,
    comp_count: usize,
    comp_names: &'a [&'a str],
    comp_ty: &'a str,
    test_data_new: Vec<f32>,
}

fn main() {
    let mut mod_names = Vec::<String>::new();

    for comp_count in 2..=4_usize {
        let mod_name = format!("vec{comp_count}");
        let struct_name = format!("Vec{comp_count}");
        let comp_names = &["x", "y", "z", "w"][..comp_count];
        let comp_ty = "f32";
        let path = PathBuf::from(format!("src/{mod_name}.rs"));

        mod_names.push(mod_name.clone());

        let seed = format!("{mod_name:0>32}");
        let mut rng = StdRng::from_seed(seed.as_bytes().try_into().unwrap());

        let test_data_new: Vec<f32> = (0..comp_count)
            .map(|_| rng.gen_range(-100.0..100.0))
            .collect();

        let vec = VecTemplate {
            struct_name,
            comp_count,
            comp_names,
            comp_ty,
            test_data_new,
        };

        render_to_file(&vec, &path);
    }

    let path = PathBuf::from("src/lib.rs");

    let lib = LibTemplate { mod_names };

    render_to_file(&lib, &path);
}

fn render_to_file<T: Template>(template: &T, path: &PathBuf) {
    match template.render() {
        Ok(code) => {
            if let Err(error) = fs::write(path, code) {
                panic!("failed to write file `{path:?}`: {error}");
            }

            if let Err(error) = Command::new("rustfmt").arg(path).output() {
                panic!("failed to run rustfmt on file `{path:?}`: {error}");
            }
        }
        Err(error) => {
            panic!("failed to render template to file `{path:?}`: {error}");
        }
    }
}
