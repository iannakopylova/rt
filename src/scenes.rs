//! Audit scene builders (RT-011+).

use crate::camera::Camera;
use crate::light::Light;
use crate::material::Material;
use crate::objects::{Cube, Cylinder, Plane, Sphere};
use crate::scene::{Object, Scene};
use crate::vec3::{Color, Vec3};

/// Scene 1 — sphere only (plus a ground plane so shadows are visible).
///
/// # Configurable knobs
/// Edit the constants below to move/resize the sphere or retarget the camera.
pub fn scene1_sphere(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let sphere_center = Vec3::new(0.0, 0.0, -4.0);
    let sphere_radius = 1.0;
    let ground_y = -1.0;
    let light_pos = Vec3::new(3.0, 5.0, 1.0);
    let eye = Vec3::new(0.0, 1.2, 3.0);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 50.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.55, 0.55, 0.58)),
        )))
        .add(Object::Sphere(Sphere::with_albedo(
            sphere_center,
            sphere_radius,
            Color::new(0.9, 0.25, 0.2),
        )))
        .add_light(Light::scene1_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

/// Scene 2 — plane + cube with **lower** light brightness than Scene 1.
///
/// Brightness: Scene 1 uses [`crate::light::SCENE1_LIGHT_INTENSITY`] (1.0);
/// this scene uses [`crate::light::SCENE2_LIGHT_INTENSITY`] (0.45).
pub fn scene2_plane_cube(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let cube_center = Vec3::new(0.0, 0.0, -4.0);
    let cube_edge = 1.6;
    let ground_y = -1.0;
    // Same placement as Scene 1 so the dimmer intensity is the clear difference.
    let light_pos = Vec3::new(3.0, 5.0, 1.0);
    let eye = Vec3::new(0.0, 1.5, 3.5);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 50.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.55, 0.55, 0.58)),
        )))
        .add(Object::Cube(Cube::with_albedo(
            cube_center,
            cube_edge,
            Color::new(0.25, 0.45, 0.9),
        )))
        .add_light(Light::scene2_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

/// Shared world for Scene 3 / Scene 4 — cube, sphere, cylinder, plane + key light.
///
/// RT-014 reuses this layout and only swaps the camera.
pub fn scene3_world() -> Scene {
    // --- configurable object layout ---
    let ground_y = -1.0;
    let sphere_center = Vec3::new(-1.6, 0.0, -3.8);
    let sphere_radius = 0.85;
    let cube_center = Vec3::new(1.6, 0.0, -4.0);
    let cube_edge = 1.4;
    let cylinder_mid = Vec3::new(0.0, 0.0, -5.2);
    let cylinder_radius = 0.55;
    let cylinder_height = 2.0;
    let light_pos = Vec3::new(4.0, 6.0, 2.0);
    // ----------------------------------

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.55, 0.55, 0.58)),
        )))
        .add(Object::Sphere(Sphere::with_albedo(
            sphere_center,
            sphere_radius,
            Color::new(0.9, 0.25, 0.2),
        )))
        .add(Object::Cube(Cube::from_center_extent(
            cube_center,
            cube_edge,
            // RT-018: textured when `-t` is set; identical solid albedo otherwise,
            // so Scene 3 / Scene 4 stay byte-for-byte unchanged without the flag.
            Material::solid(Color::new(0.25, 0.45, 0.9)).with_texture("textures/stripes_cube.ppm"),
        )))
        .add(Object::Cylinder(Cylinder::from_midpoint(
            cylinder_mid,
            cylinder_radius,
            cylinder_height,
            Material::solid(Color::new(0.2, 0.75, 0.35)).with_texture("textures/bands_cylinder.ppm"),
        )))
        .add_light(Light::scene1_key(light_pos));

    scene
}

/// Front camera for Scene 3 (also the default for [`scene3_all`]).
pub fn scene3_camera_front(aspect: f64) -> Camera {
    // --- configurable ---
    let eye = Vec3::new(0.0, 2.0, 4.5);
    let look_at = Vec3::new(0.0, 0.0, -4.2);
    let vfov_degrees = 55.0;
    // --------------------
    Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    )
}

/// Alternate camera for Scene 4 — same look-at, different eye (side / elevated angle).
pub fn scene3_camera_alt(aspect: f64) -> Camera {
    // --- configurable ---
    let eye = Vec3::new(4.2, 2.8, 1.5);
    let look_at = Vec3::new(0.0, 0.0, -4.2);
    let vfov_degrees = 55.0;
    // --------------------
    Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    )
}

/// Scene 3 — all four primitives, front camera.
pub fn scene3_all(aspect: f64) -> (Scene, Camera) {
    (scene3_world(), scene3_camera_front(aspect))
}

/// Scene 4 — identical world as Scene 3, alternate camera.
pub fn scene4_alt_camera(aspect: f64) -> (Scene, Camera) {
    (scene3_world(), scene3_camera_alt(aspect))
}

/// Bonus RT-016 demo: metal sphere + colored cube over a ground plane.
///
/// Enable recursive reflections with `--reflection` when rendering.
pub fn scene_reflection_demo(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let ground_y = -1.0;
    let metal_center = Vec3::new(-0.8, 0.0, -4.0);
    let metal_radius = 1.0;
    let cube_center = Vec3::new(1.4, 0.0, -3.5);
    let cube_edge = 1.2;
    let light_pos = Vec3::new(3.0, 6.0, 2.0);
    let eye = Vec3::new(0.0, 1.8, 3.5);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 50.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.06);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.45, 0.45, 0.5)),
        )))
        .add(Object::Sphere(Sphere::new(
            metal_center,
            metal_radius,
            Material::metal(Color::new(0.95, 0.95, 0.98), 0.92),
        )))
        .add(Object::Cube(Cube::with_albedo(
            cube_center,
            cube_edge,
            Color::new(0.85, 0.25, 0.2),
        )))
        .add_light(Light::scene1_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

/// Bonus RT-017 demo: glass sphere in front of a colored cube.
///
/// Enable Snell's-law refraction with `--refraction` when rendering.
pub fn scene_refraction_demo(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let ground_y = -1.0;
    let glass_center = Vec3::new(-0.4, 0.0, -3.5);
    let glass_radius = 1.0;
    let cube_center = Vec3::new(1.5, 0.0, -5.0);
    let cube_edge = 1.6;
    let light_pos = Vec3::new(2.5, 6.0, 2.0);
    let eye = Vec3::new(0.0, 1.4, 3.2);
    let look_at = Vec3::new(0.2, 0.0, -4.0);
    let vfov_degrees = 48.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.06);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.4, 0.42, 0.48)),
        )))
        .add(Object::Sphere(Sphere::new(
            glass_center,
            glass_radius,
            Material::glass(Color::new(0.95, 0.97, 1.0), 1.5),
        )))
        .add(Object::Cube(Cube::with_albedo(
            cube_center,
            cube_edge,
            Color::new(0.9, 0.25, 0.15),
        )))
        .add_light(Light::scene1_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

/// Bonus RT-018 demo: single textured sphere over a solid ground plane.
///
/// Enable texture sampling with `--textures` when rendering; without it the
/// sphere falls back to its solid albedo (same framing as [`scene1_sphere`]).
pub fn scene_texture_sphere_demo(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let ground_y = -1.0;
    let sphere_center = Vec3::new(0.0, 0.0, -4.0);
    let sphere_radius = 1.0;
    let light_pos = Vec3::new(3.0, 5.0, 1.0);
    let eye = Vec3::new(0.0, 1.2, 3.0);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 50.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.55, 0.55, 0.58)),
        )))
        .add(Object::Sphere(Sphere::new(
            sphere_center,
            sphere_radius,
            Material::textured(Color::new(0.9, 0.25, 0.2), "textures/checker_red.ppm"),
        )))
        .add_light(Light::scene1_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

/// Bonus RT-018 demo: textured ground plane with a solid sphere for scale and shadow.
pub fn scene_texture_plane_demo(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let ground_y = -1.0;
    let tile_size = 2.0;
    let sphere_center = Vec3::new(0.0, 0.0, -4.0);
    let sphere_radius = 0.9;
    let light_pos = Vec3::new(3.0, 5.0, 1.0);
    let eye = Vec3::new(0.0, 1.4, 3.4);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 55.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.08);
    scene
        .add(Object::Plane(
            Plane::from_point_normal(
                Vec3::new(0.0, ground_y, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Material::textured(Color::new(0.55, 0.55, 0.58), "textures/tile_floor.ppm"),
            )
            .with_tile_size(tile_size),
        ))
        .add(Object::Sphere(Sphere::with_albedo(
            sphere_center,
            sphere_radius,
            Color::new(0.85, 0.85, 0.9),
        )))
        .add_light(Light::scene1_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

/// Bonus RT-018 + RT-016 interplay demo: a sphere that is **both** textured and a
/// mirror, so a reflected bounce must show the sampled texture color at the
/// reflection's own hit point (not the base albedo, and not a flat/default color).
/// Needs both `--reflection` and `--textures` to see the full effect.
pub fn scene_texture_reflection_demo(aspect: f64) -> (Scene, Camera) {
    // --- configurable ---
    let ground_y = -1.0;
    let metal_center = Vec3::new(-0.8, 0.0, -4.0);
    let metal_radius = 1.0;
    let cube_center = Vec3::new(1.4, 0.0, -3.5);
    let cube_edge = 1.2;
    let light_pos = Vec3::new(3.0, 6.0, 2.0);
    let eye = Vec3::new(0.0, 1.8, 3.5);
    let look_at = Vec3::new(0.0, 0.0, -4.0);
    let vfov_degrees = 50.0;
    // --------------------

    let mut scene = Scene::new().with_ambient(0.06);
    scene
        .add(Object::Plane(Plane::ground(
            ground_y,
            Material::solid(Color::new(0.45, 0.45, 0.5)),
        )))
        .add(Object::Sphere(Sphere::new(
            metal_center,
            metal_radius,
            Material::metal(Color::new(0.95, 0.95, 0.98), 0.92)
                .with_texture("textures/checker_blue.ppm"),
        )))
        .add(Object::Cube(Cube::with_albedo(
            cube_center,
            cube_edge,
            Color::new(0.85, 0.25, 0.2),
        )))
        .add_light(Light::scene1_key(light_pos));

    let camera = Camera::look_at(
        eye,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        vfov_degrees,
        aspect,
    );

    (scene, camera)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::light::{SCENE1_LIGHT_INTENSITY, SCENE2_LIGHT_INTENSITY};
    use crate::tracer::trace;

    #[test]
    fn scene1_has_sphere_and_ground() {
        let (scene, _) = scene1_sphere(4.0 / 3.0);
        assert_eq!(scene.objects.len(), 2);
        assert_eq!(scene.lights.len(), 1);
        assert!(matches!(scene.objects[0], Object::Plane(_)));
        assert!(matches!(scene.objects[1], Object::Sphere(_)));
    }

    #[test]
    fn scene1_uses_bright_key_light() {
        let (scene, _) = scene1_sphere(4.0 / 3.0);
        assert!((scene.lights[0].intensity() - SCENE1_LIGHT_INTENSITY).abs() < 1e-12);
    }

    #[test]
    fn scene1_center_ray_hits_sphere() {
        let (scene, cam) = scene1_sphere(4.0 / 3.0);
        let ray = cam.get_ray(0.5, 0.5);
        let hit = scene.hit(&ray, 0.001, f64::INFINITY).unwrap();
        let color = trace(&scene, &ray);
        assert!(hit.material.albedo.r > 0.5);
        assert!(color.r > color.g);
        assert!(color.r > color.b);
    }

    #[test]
    fn scene1_ground_shadow_darker_than_open_ground() {
        let (scene, _) = scene1_sphere(4.0 / 3.0);
        let under = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -4.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        let open = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -1.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        assert!(open.r + open.g + open.b > under.r + under.g + under.b);
    }

    #[test]
    fn scene2_has_plane_and_cube() {
        let (scene, _) = scene2_plane_cube(4.0 / 3.0);
        assert_eq!(scene.objects.len(), 2);
        assert!(matches!(scene.objects[0], Object::Plane(_)));
        assert!(matches!(scene.objects[1], Object::Cube(_)));
    }

    #[test]
    fn scene2_light_dimmer_than_scene1() {
        let (s1, _) = scene1_sphere(4.0 / 3.0);
        let (s2, _) = scene2_plane_cube(4.0 / 3.0);
        assert!(s2.lights[0].intensity() < s1.lights[0].intensity());
        assert!((s2.lights[0].intensity() - SCENE2_LIGHT_INTENSITY).abs() < 1e-12);
    }

    #[test]
    fn scene2_center_ray_hits_cube() {
        let (scene, cam) = scene2_plane_cube(4.0 / 3.0);
        let ray = cam.get_ray(0.5, 0.5);
        let hit = scene.hit(&ray, 0.001, f64::INFINITY).unwrap();
        let color = trace(&scene, &ray);
        assert!(hit.material.albedo.b > hit.material.albedo.r);
        assert!(color.b > 0.05);
    }

    #[test]
    fn scene2_ground_shadow_darker_than_open_ground() {
        let (scene, _) = scene2_plane_cube(4.0 / 3.0);
        let under = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -4.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        let open = trace(
            &scene,
            &crate::ray::Ray::new(Vec3::new(4.0, 0.5, -1.0), Vec3::new(-1.0, -0.4, 0.0)),
        );
        assert!(open.r + open.g + open.b > under.r + under.g + under.b);
    }

    #[test]
    fn scene3_has_all_four_primitives() {
        let scene = scene3_world();
        assert_eq!(scene.objects.len(), 4);
        assert!(scene.objects.iter().any(|o| matches!(o, Object::Plane(_))));
        assert!(scene.objects.iter().any(|o| matches!(o, Object::Sphere(_))));
        assert!(scene.objects.iter().any(|o| matches!(o, Object::Cube(_))));
        assert!(scene
            .objects
            .iter()
            .any(|o| matches!(o, Object::Cylinder(_))));
        assert_eq!(scene.lights.len(), 1);
    }

    #[test]
    fn scene3_rays_hit_each_object_type() {
        let (scene, cam) = scene3_all(4.0 / 3.0);

        // Sphere on the left of frame.
        let sphere_hit = scene
            .hit(&cam.get_ray(0.32, 0.48), 0.001, f64::INFINITY)
            .unwrap();
        assert!(sphere_hit.material.albedo.r > 0.5);

        // Cube on the right.
        let cube_hit = scene
            .hit(&cam.get_ray(0.68, 0.48), 0.001, f64::INFINITY)
            .unwrap();
        assert!(cube_hit.material.albedo.b > cube_hit.material.albedo.r);

        // Cylinder near center / slightly low.
        let cyl_hit = scene
            .hit(&cam.get_ray(0.50, 0.42), 0.001, f64::INFINITY)
            .unwrap();
        assert!(cyl_hit.material.albedo.g > cyl_hit.material.albedo.r);

        // Ground near bottom of frame.
        let ground_hit = scene
            .hit(&cam.get_ray(0.50, 0.12), 0.001, f64::INFINITY)
            .unwrap();
        assert!((ground_hit.material.albedo.r - 0.55).abs() < 0.05);
    }

    #[test]
    fn scene4_reuses_scene3_world() {
        let (s3, _) = scene3_all(4.0 / 3.0);
        let (s4, _) = scene4_alt_camera(4.0 / 3.0);
        assert_eq!(s3.objects.len(), s4.objects.len());
        assert_eq!(s3.lights.len(), s4.lights.len());
        assert_eq!(s3.lights[0].intensity(), s4.lights[0].intensity());
    }

    #[test]
    fn scene4_camera_differs_from_scene3() {
        let aspect = 4.0 / 3.0;
        let (_, c3) = scene3_all(aspect);
        let (_, c4) = scene4_alt_camera(aspect);
        assert_ne!(c3.eye(), c4.eye());
        assert_ne!(c3.get_ray(0.5, 0.5).direction, c4.get_ray(0.5, 0.5).direction);
    }

    // --- RT-018 textures -----------------------------------------------

    #[test]
    fn scene3_cube_and_cylinder_are_textured_with_unchanged_albedo() {
        // Same albedo as before RT-018 so a `-t`-off render stays byte-identical;
        // only `texture_path` is new.
        let scene = scene3_world();
        for object in &scene.objects {
            match object {
                Object::Cube(c) => {
                    assert!(c.material.texture_path.is_some());
                    assert_eq!(c.material.albedo, Color::new(0.25, 0.45, 0.9));
                }
                Object::Cylinder(cyl) => {
                    assert!(cyl.material.texture_path.is_some());
                    assert_eq!(cyl.material.albedo, Color::new(0.2, 0.75, 0.35));
                }
                Object::Sphere(s) => assert!(s.material.texture_path.is_none()),
                Object::Plane(p) => assert!(p.material.texture_path.is_none()),
            }
        }
    }

    #[test]
    fn texture_sphere_demo_sphere_is_textured() {
        let (scene, _) = scene_texture_sphere_demo(4.0 / 3.0);
        assert_eq!(scene.objects.len(), 2);
        let sphere = scene
            .objects
            .iter()
            .find_map(|o| match o {
                Object::Sphere(s) => Some(s),
                _ => None,
            })
            .expect("scene should contain a sphere");
        assert!(sphere.material.texture_path.is_some());
    }

    #[test]
    fn texture_plane_demo_plane_textured_sphere_solid() {
        let (scene, _) = scene_texture_plane_demo(4.0 / 3.0);
        for object in &scene.objects {
            match object {
                Object::Plane(p) => assert!(p.material.texture_path.is_some()),
                Object::Sphere(s) => assert!(s.material.texture_path.is_none()),
                _ => {}
            }
        }
    }

    #[test]
    fn texture_reflection_demo_sphere_is_textured_and_metal() {
        let (scene, _) = scene_texture_reflection_demo(4.0 / 3.0);
        let sphere = scene
            .objects
            .iter()
            .find_map(|o| match o {
                Object::Sphere(s) => Some(s),
                _ => None,
            })
            .expect("scene should contain a sphere");
        assert!(sphere.material.texture_path.is_some());
        assert!(sphere.material.reflectivity > 0.0);
    }

    #[test]
    fn texture_demo_center_rays_hit_something() {
        // Smoke test: each new demo scene actually has a subject in frame.
        for aspect in [4.0 / 3.0] {
            let (scene, cam) = scene_texture_sphere_demo(aspect);
            assert!(scene.hit(&cam.get_ray(0.5, 0.5), 0.001, f64::INFINITY).is_some());

            let (scene, cam) = scene_texture_plane_demo(aspect);
            assert!(scene.hit(&cam.get_ray(0.5, 0.15), 0.001, f64::INFINITY).is_some());

            let (scene, cam) = scene_texture_reflection_demo(aspect);
            assert!(scene.hit(&cam.get_ray(0.5, 0.5), 0.001, f64::INFINITY).is_some());
        }
    }
}
