extern crate gltf;

 use std::{fs, io};

 fn import_json(filename: &str) -> gltf::json::Root {
     let file = fs::File::open(filename).unwrap();
     let reader = io::BufReader::new(file);
     gltf::json::from_reader(reader).unwrap()
 }

 #[test]
 fn test_something() {
     // file derived from minimal.gltf with changed min/max values
     let gltf = gltf::Gltf::from_json(import_json("gltf-importer/tests/minimal.gltf"));
     let mesh = &gltf.meshes().nth(0).unwrap();
     let _prim = mesh.primitives().nth(0).unwrap();
 }
