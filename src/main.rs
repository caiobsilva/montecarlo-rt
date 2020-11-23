mod camera;
mod scene;
mod spectral_distribution;

extern crate nalgebra_glm as glm;
use rand::distributions::Distribution;

const WIDTH: i32 = 1024;
const HEIGHT: i32 = 768;
const WIDTH_F: f32 = 1024.0;
const HEIGHT_F: f32 = 768.0;
const SUB_SAMPLING_CAUSTICS: i32 = 10;
const SUB_SAMPLING_MONTE_CARLO: i32 = 500;
const SUB_SAMPLING_DIRECT_SPECULAR: i32 = 100;
const NUMBER_OF_PHOTONS_EMISSION: i32 = 2000000;
const PI: f32 = std::f32::consts::PI;

fn main() {
  // criar cena aqui, erro se n existir.
  // scene = new Scene(...)

  // render camera for ray casting
  let camera = camera::Camera::new(
    glm::vec3(0.0, 0., 3.2),
    glm::vec3(0., 0., 1.),
    glm::vec3(0., 1., 0.),
    PI / 3. // might be wrong. try radians if it doesnt work
  );

  let camera_plane_normal = glm::normalize(&(camera.center - camera.eye));
  // gerando distribuições aleatórias para o renderer
  let urd = rand::distributions::uniform::Uniform::from(-0.5..0.5);
  let mut rng = rand::thread_rng();
}
