use core::{Material, RayCaster, RayIntersection, Color, ColorCalculator, IlluminationCaster, LightIntersection};
use defs::Vector3;

fn get_mirror_direction(view: &Vector3, normal: &Vector3) -> Vector3 {
    (-view + (normal * 2.0)).normalize()
}

pub struct SimpleColorCalculator {

}

impl SimpleColorCalculator {
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
        // match Ray::continue_ray_from_intersection(intersection, get_mirror_direction(&intersection.get_view_direction(), intersection.get_normal_vector())) {
        //     Ok(ray) => {
        //         match ray_caster.cast_ray(ray) {
        //             Some(color) => {

        //             },
        //             None => Color::zero()
        //         }
        //     },
        //     Err(_) => Color::zero()
        // }
        Color::zero()
    }

    fn get_refracted_color(&self, intersection: &RayIntersection, ray_caster: &RayCaster, illuminations: &Vec<LightIntersection>) -> Color {
        Color::zero()
    }
}

impl ColorCalculator for SimpleColorCalculator {
    fn get_color(&self, intersection: &RayIntersection, ray_caster: &RayCaster, illumination_caster: &IlluminationCaster) -> Option<Color> {
        let illuminations = illumination_caster.get_illumination_at(intersection);

        let result =    self.get_ambient_color(intersection) +
                        self.get_local_color(intersection, &illuminations) +
                        self.get_reflected_color(intersection, ray_caster) +
                        self.get_refracted_color(intersection, ray_caster, &illuminations);

        Some (result)
    }
}