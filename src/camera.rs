extern crate nalgebra_glm as glm;

pub struct Camera {
  pub eye: glm::Vec3,
  pub center: glm::Vec3,
  pub up: glm::Vec3,
  pub fov: f32,
  pub width: i32,
  pub height: i32,
  pub view: glm::Mat4,
  pub asp_ratio: f32,
  pub perspective: glm::Mat4,
  pub inverse_vp: glm::Mat4
}

pub struct Ray {
  pub origin: glm::Vec3,
  pub direction: glm::Vec3,
  pub material: Material,
  pub radiance: glm::Vec3
}

impl Camera {
  pub fn new(
    eye: glm::Vec3,
    center: glm::Vec3,
    up: glm::Vec3,
    fov: f32,
  ) -> Camera {
    let view = glm::look_at(&eye, &center, &up);
    let asp_ratio = crate::WIDTH_F / crate::HEIGHT_F;
    let perspective = glm::perspective(fov, asp_ratio, 0.1, 100.0);

    Camera {
      eye: eye,
      center: center,
      up: up,
      fov: fov,
      width: crate::WIDTH,
      height: crate::HEIGHT,
      view: view,
      asp_ratio: asp_ratio,
      perspective: perspective,
      inverse_vp: glm::inverse(&(view * perspective))
    }
  }

  pub fn cast_ray(&self, pix_x: f32, pix_y: f32, param_x: f32, param_y: f32) -> Ray {
    if ray_not_in_range(pix_x, pix_y, param_x, param_y) {
      println!("Invalid arguments cast to ray!");
      return Ray {
        origin: glm::vec3(0., 0., 0.),
        direction: glm::vec3(0., 0., 0.),
        material: None,
        radiance: glm::vec3(0., 0., 0.)
      };
    } else {
      let from4 = self.inverse_vp * glm::vec4(
        ((pix_x + param_x) / crate::WIDTH_F - 0.5) * 2.,
        ((pix_y + param_y) / crate::HEIGHT_F - 0.5) * 2.,
        1., 1.
      );

      let to4 = self.inverse_vp * glm::vec4(
        ((pix_x + param_x) / crate::WIDTH_F - 0.5) * 2.,
        ((pix_y + param_y) / crate::HEIGHT_F - 0.5) * 2.,
        -1., 1.
      );

      let to = glm::vec3(to4[0], to4[1], to4[2]) * to4.w;
      let from = glm::vec3(from4[0], from4[1], from4[2]) * from4.w;
      let direction = glm::normalize(&(to - from));

      return Ray {
        origin: self.eye,
        direction: direction,
        material: Material::Air,
        radiance: glm::vec3(1., 1., 1.)
      };
    }
  }

}

fn ray_not_in_range(pix_x: f32, pix_y: f32, param_x: f32, param_y: f32) -> bool {
  pix_x < 0. || pix_x > crate::WIDTH_F - 1. ||
  pix_y < 0. || pix_y > crate::HEIGHT_F - 1. ||
  param_x < -0.5 || param_x > 0.5 ||
  param_y < -0.5 || param_y > 0.5
}
