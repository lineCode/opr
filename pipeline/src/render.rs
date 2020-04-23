use std::path::{Path, PathBuf};
use std::fs;

use indicatif::{ProgressBar, ProgressStyle};

use sph_scene::Scene;

use nalgebra::Vector3;

extern crate raytracer;
use raytracer::{write_image, Camera, Light};
use raytracer::scene_config::*;

fn get_obj(folder: &Path) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files: Vec<PathBuf> = fs::read_dir(folder)?
        .filter_map(Result::ok)
        .filter_map(|d| d.path().to_str().and_then(|f| if f.ends_with(".obj") { Some(d) } else { None }))
        .map(|d| d.path())
        .collect();

    files.sort();

    Ok(files)
}

pub fn pipeline_render(_scene: &Scene, input_directory: &Path, dump_directory: &Path) -> Result<(), Box<dyn std::error::Error>> {
    fs::create_dir_all(dump_directory)?;

    let simulations: Vec<PathBuf> = get_obj(input_directory)?;

    let pb = ProgressBar::new(simulations.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
      .template("[{elapsed_precise}] [{per_sec}] [{eta_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7}"));
    pb.tick();

    let lights = vec![
        Light::ambient(Vector3::new(0.3, 0.3, 0.3)),
        Light::directional(Vector3::new(1., -1., 1.), Vector3::new(1., 1., 1.)),
    ];

    let camera = Camera::new(Vector3::new(0.0, 0.0, -4.0), Vector3::new(0.0, 1.0, 0.0), Vector3::new(0.0, 0.0, 1.0), 512., 512.);

    for idx in 0..simulations.len() {
        let ray_scene = SceneConfig {
            params: ParamsConfig::default(),
            objects: vec![
                ObjectConfig {
                    path: simulations[idx].to_str().unwrap().to_string(),
                    rotation: Vector3::zeros(),
                    position: Vector3::zeros(),
                },
            ],
            lights: lights.clone(),
            camera: camera.clone(),
        };

        let mut scene = raytracer::Scene::from_config(ray_scene, &Path::new("data/materials/white.mtl"))?;
        scene.build(12);

        let pixels  = scene.render(512, 512);

        write_image(&dump_directory.join(format!("{:08}.png", idx)), &pixels, 512, 512);

        pb.inc(1);
    }

    pb.finish_with_message("rendering done");

    Ok(())
}
