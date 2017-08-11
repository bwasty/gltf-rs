
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf_importer;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use std::{fs, path};
use std::collections::HashMap;

use gltf_importer::{Error as ImportError, Importer};
use std::boxed::Box;
use std::error::Error as StdError;

fn import_from_path(path: &path::Path) -> Result<(), ImportError> {
    let mut importer = Importer::new(path);
    match importer.import() {
        Ok(_) => {
            println!("{:?}: Ok", importer.path());
            Ok(())
        },
        Err(err) => {
            println!("{:?}: Err({:?})", importer.path(), err);
            Err(err)
        },
    }
}

fn run() -> Result<(), Box<StdError>> {
    let sample_dir_path = path::Path::new("../glTF-Sample-Models/2.0");
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                // Import standard glTF
                let mut gltf_path = entry_path.join("glTF").join(file_name);
                gltf_path.set_extension("gltf");
                let _ = import_from_path(&gltf_path)?;

                // Import binary glTF
                let mut glb_path = entry_path.join("glTF-Binary").join(file_name);
                glb_path.set_extension("glb");
                if glb_path.exists() {
                    let _ = import_from_path(&glb_path)?;
                }
            }
        }
    }
    Ok(())
}

// #[test]
// fn import() {
//     let _ = run().expect("no errors");
// }

#[derive(Debug, Deserialize)]
struct SampleModel {
    name: String,
    screenshot: String,
    // variants: SampleModelVariants,
    variants: HashMap<String, String>,
}

// #[derive(Debug, Deserialize)]
// struct SampleModelVariants {
//     #[serde(rename = "glTF")]
//     glTF: String,
//     #[serde(rename = "glTF-Binary")]
//     binary: Option<String>,
//     #[serde(rename = "glTF-Embedded")]
//     embedded: Option<String>,
//     #[serde(rename = "glTF-MaterialsCommon")]
//     materials_common: Option<String>,
//     #[serde(rename = "glTF-pbrSpecularGlossiness")]
//     pbr_specular_glossiness: Option<String>,
//     #[serde(rename = "glTF-techniqueWebGL")]
//     technique_webgl: Option<String>,
// }

use std::fs::File;
#[test]
fn get_sample_models() {
    let file = File::open("../../../glTF-Sample-Models/2.0/model-index.json").unwrap();
    let models: Vec<SampleModel> = serde_json::from_reader(file).unwrap();
    println!("{:#?}", models);
    for model in &models {
        println!("https://raw.githubusercontent.com/KhronosGroup/lTF-Sample-Models/master/2.0/{}/glTF/{}",
            model.name, model.variants["glTF"])
    }
}
