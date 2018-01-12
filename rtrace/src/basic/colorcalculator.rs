use core::{FresnelData, RayCaster, RayIntersection, Color, ColorCalculator, IlluminationCaster, LightIntersection, Ray, RayError};
use defs::Vector3;
use na::Unit;

fn get_mirror_direction(intersection: &RayIntersection) -> Unit<Vector3> {
    let view = intersection.get_view_direction();
    let normal = intersection.get_normal_vector();

    Unit::new_normalize(-view + (normal * 2.0))
}

fn get_refract_direction(intersection: &RayIntersection, fresnel_data: &FresnelData) -> Unit<Vector3> {
    let view = intersection.get_view_direction();
    let normal = intersection.get_normal_vector();

    let cosa = normal.dot(&view);
    let rooted = 1.0-((1.0-cosa.powi(2)) / fresnel_data.n_avg.powi(2));
    if rooted < 0.0 {
        panic!("Impossible value for view and normal direction");
    }

    let nf = if intersection.was_inside() {
        fresnel_data.n_avg.recip()
    } else {
        fresnel_data.n_avg
    };

    Unit::new_normalize(view * (-nf.recip()) + normal * (cosa/nf - rooted.sqrt()))
}

pub struct SimpleColorCalculator {

}

impl SimpleColorCalculator {
    pub fn new() -> Self {
        Self {}
    }

    fn get_ambient_color(&self, intersection: &RayIntersection) -> Color {
        let material = intersection.get_material();
        *material.get_ambient_color().unwrap_or(&Color::zero())
    }

    fn get_local_color(&self, intersection: &RayIntersection, illuminations: &Vec<LightIntersection>) -> Color { 
        let material = intersection.get_material();
        let view_direction = intersection.get_view_direction();
        let surface_normal = intersection.get_normal_vector();

        illuminations.iter().fold(Color::zero(), |acc, light_intersection|{
            let illumination = light_intersection.get_illumination();

            let diffuse_color = material.get_diffuse_color().and_then(|color| {
                let light_direction = light_intersection.get_light_direction();
                let cosln = light_direction.dot(surface_normal).max(0.0);
                Some ((*color * *illumination).mul_scalar(&cosln))
            });
            let specular_color = material.get_specular_color().and_then(|color_shiny| {
                let (color, shininess) = *color_shiny;
                let light_direction = light_intersection.get_light_direction();
                let half_direction = (view_direction + light_direction).normalize();
                let coshn = half_direction.dot(surface_normal).max(0.0).powf(shininess);
                Some((color * *illumination).mul_scalar(&coshn))
            });
            
            acc + specular_color.unwrap_or(Color::zero()) + diffuse_color.unwrap_or(Color::zero())
        }).normalized()
    }

    fn get_reflected_color(&self, intersection: &RayIntersection, ray_caster: &RayCaster) -> Color {
        let material = intersection.get_material();
        if material.is_reflective() {
            let mirror_direction = get_mirror_direction(intersection).unwrap();
            match Ray::continue_ray_from_intersection(intersection, mirror_direction) {
                Ok(mirror_ray) => {
                    let ray_cast_result = ray_caster.cast_ray(&mirror_ray);

                    match ray_cast_result {
                        Some(color) => {
                            let fresnel_reflect = material.get_fresnel_data().unwrap();
                            fresnel_reflect.get_fresnel_reflect(intersection) * color
                        },
                        None => Color::zero()
                    }
                },
                Err(RayError::DepthLimitReached) => Color::zero(),
                Err(_) => panic!("Unhandled ray continuation error!")
            }
        } else {
            Color::zero()
        }
    }

    fn get_refracted_color(&self, intersection: &RayIntersection, ray_caster: &RayCaster) -> Color {
        let material = intersection.get_material();
        if material.is_refractive() {
            let fresnel_data = material.get_fresnel_data().unwrap();
            let refract_direction = get_refract_direction(intersection, fresnel_data).unwrap();
            match Ray::continue_ray_from_intersection(intersection, refract_direction) {
                Ok(refract_ray) => {
                    let ray_cast_result = ray_caster.cast_ray(&refract_ray);

                    match ray_cast_result {
                        Some(color) => {
                            fresnel_data.get_fresnel_refract(intersection) * color
                        },
                        None => Color::zero()
                    }
                },
                Err(RayError::DepthLimitReached) => Color::zero(),
                Err(_) => panic!("Unhandled ray continuation error!")
            }
        } else {
            Color::zero()
        }
    }
}

impl ColorCalculator for SimpleColorCalculator {
    fn get_color(&self, intersection: &RayIntersection, ray_caster: &RayCaster, illumination_caster: &IlluminationCaster) -> Option<Color> {
        let illuminations = illumination_caster.get_illumination_at(intersection);

        let result =    self.get_ambient_color(intersection) +
                        self.get_local_color(intersection, &illuminations) +
                        self.get_reflected_color(intersection, ray_caster) +
                        self.get_refracted_color(intersection, ray_caster);

        Some (result)
    }
}