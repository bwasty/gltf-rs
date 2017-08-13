
// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf_importer;
extern crate gltf;

use std::{fs, path};

use gltf_importer::{Error as ImportError, Importer};
use gltf::{Gltf, Loaded, Node, Primitive};
use gltf::mesh::{Colors, Joints, Weights};

use std::boxed::Box;
use std::error::Error as StdError;

fn import_from_path(path: &path::Path) -> Result<(), ImportError> {
    let mut importer = Importer::new(path);
    match importer.import() {
        Ok(gltf) => {
            println!("{:?}: Ok", path);
            iterate_gltf(&gltf);
            Ok(())
        },
        Err(err) => {
            println!("{:?}: Err({:?})", path, err);
            Err(err)
        },
    }
}

pub fn iterate_gltf(gltf: &Loaded<&Gltf>) {
    let _accessors: Vec<_> = gltf.accessors().collect();
    let _animations: Vec<_> = gltf.animations().collect();
    let _buffers: Vec<_> = gltf.buffers().collect();
    let _views: Vec<_> = gltf.views().collect();
    let _cameras: Vec<_> = gltf.cameras().collect();
    let _images: Vec<_> = gltf.images().collect();
    let _meshes: Vec<_> = gltf.meshes().collect();
    let _nodes: Vec<_> = gltf.nodes().collect();
    let _samplers: Vec<_> = gltf.samplers().collect();
    let _skins: Vec<_> = gltf.skins().collect();
    let _textures: Vec<_> = gltf.textures().collect();

    for scene in gltf.scenes() {
        for ref node in scene.nodes() {
            iterate_node(node)
        }
    }

    for mat in gltf.materials() {
        let _json = mat.as_json();

        let pbr_metallic_roughness = mat.pbr_metallic_roughness();
        let _base_color_texture = pbr_metallic_roughness.base_color_texture();
        let _metallic_roughness_texture = pbr_metallic_roughness.metallic_roughness_texture();

        let pbr_metallic_roughness = &*pbr_metallic_roughness;
        let _json = pbr_metallic_roughness.as_json();
        let _base_color_texture_factor = pbr_metallic_roughness.base_color_factor();
        let _base_color_texture_factor = pbr_metallic_roughness.metallic_factor();
        let _extras = pbr_metallic_roughness.extras();

        let _normal_texture = mat.normal_texture();
        let _occlusion_texture = mat.occlusion_texture();
        let _emissive_texture = mat.emissive_texture();
    }
}

fn iterate_node(node: &Loaded<Node>) {
    let _json = node.as_json();
    let _camera = node.camera();
    let _extras = node.extras();
    // NOTE: this requires to run with --all-features!
    let _name = node.name();
    let _matrix = node.matrix();
    let _rotation = node.rotation();
    let _scale = node.scale();
    let _translation = node.translation();
    let _weights = node.weights();
    let _skin = node.skin();

    if let Some(mesh) = node.mesh() {
        for prim in mesh.primitives() {
            iterate_primitive(&prim);
        }
    }

    for ref child in node.children() {
        iterate_node(child);
    }
}

#[derive(Default)]
struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tangent: [f32; 4],
    pub tex_coord_0: [f32; 2],
    pub color_0: [f32; 3],
    pub joints_0: [u16; 4],
    pub weights_0: [f32; 4],
}

fn iterate_primitive(prim: &Loaded<Primitive>) {
    let _json = prim.as_json();
    let _vertices: Vec<Vertex> = vec![];
    let _material = prim.material();

    // positions
    let positions = prim.positions().unwrap();
    let mut vertices: Vec<Vertex> = positions
        .map(|position| {
            Vertex {
                position: position,
                ..Vertex::default()
            }
        }).collect();

    // normals
    if let Some(normals) = prim.normals() {
        for (i, normal) in normals.enumerate() {
            vertices[i].normal = normal;
        }
    }

    // tangents
    if let Some(tangents) = prim.tangents() {
        for (i, tangent) in tangents.enumerate() {
            vertices[i].tangent = tangent;
        }
    }

    // texture coordinates
    if let Some(tex_coords) = prim.tex_coords_f32(0) {
        for (i, tex_coord) in tex_coords.enumerate() {
            vertices[i].tex_coord_0 = tex_coord;
        }
    }

    // colors
    if let Some(colors) = prim.colors(0) {
        let colors = match colors {
            Colors::RgbF32(c) => c,
            _ => unimplemented!()
        };
        for (i, color) in colors.enumerate() {
            vertices[i].color_0 = color;
        }
    }

    if let Some(joints) = prim.joints(0) {
        let joints = match joints {
            Joints::U16(joints) => joints,
            Joints::U8(_) => unimplemented!()
        };
        for (i, joint) in joints.enumerate() {
            vertices[i].joints_0 = joint
        }
    }
    if let Some(weights) = prim.weights(0) {
        let weights = match weights {
            Weights::F32(weights) => weights,
            _ => unimplemented!()
        };
        for (i, joint) in weights.enumerate() {
            vertices[i].weights_0 = joint
        }
    }

    let _indices: Option<Vec<u32>> = prim.indices_u32().map(|indices| indices.collect());
}

fn run() -> Result<(), Box<StdError>> {
    let sample_dir_path = path::Path::new("../glTF-Sample-Models/2.0");
    for entry in fs::read_dir(&sample_dir_path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        if metadata.is_dir() {
            let entry_path = entry.path();
            if let Some(file_name) = entry_path.file_name() {
                if file_name == "WaterBottle" {
                    // TODO!: TMP, until loader fixed
                    continue
                }

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

#[test]
fn import() {
    assert!(run().is_ok());
}
