extern crate rtrace;
extern crate image;

#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;

mod renderer;

use rtrace::basic::{SimpleIlluminator, SimpleIntersector, SimpleColorCalculator, GlobalIlluminationColorCalculator};
use rtrace::basic::model::{SolidSphere, SolidPlane};
use rtrace::basic::lightsource::{DotLightSource};
use rtrace::core::{ModelViewModelWrapper, Material, Color, FresnelIndex, View, World};
use rtrace::defs::{Point3, Point2Int, Vector3, FloatType};
use image::{DynamicImage, Rgba, Pixel, GenericImage, ImageFormat};
use renderer::{SingleThreadedRenderer, ParalellRenderer, RendererOutput};

use na::{Unit};

use std::f64::consts::{PI};

struct ImageRendererOutput {
    image: DynamicImage,
}

impl ImageRendererOutput {
    pub fn new(width: u32, height: u32) -> Self {
        Self {  image: DynamicImage::new_rgb8(width, height) }
    }

    pub fn get_image(&self) -> &DynamicImage {
        &self.image
    }
}

impl RendererOutput for ImageRendererOutput {
    fn set_output(&mut self, coord: Point2Int, color: Color) -> bool {
        let (r, g, b) = color.normalized().mul_scalar(&(u8::max_value() as FloatType)).get();
        let pixel_color = Rgba::from_channels(r as u8, g as u8, b as u8, u8::max_value());
        self.image.put_pixel(coord.x as u32, coord.y as u32, pixel_color);
        true
    }
}


fn main() {
    let solid_diffuse_red = Material::new_diffuse(Color::new(1.0, 0.0, 0.0), None);
    let solid_diffuse_green = Material::new_diffuse(Color::new(0.0, 1.0, 0.0), None);
    let solid_diffuse_blue = Material::new_diffuse(Color::new(0.0, 0.0, 1.0), None);
    let solid_diffuse_white = Material::new_diffuse(Color::new(1.0, 1.0, 1.0), None);
    let solid_shiny_red = Material::new_shiny(Color::new(0.87, 0.17, 0.08), (Color::new(1.0, 1.0, 1.0), 7.0), None);
    let solid_shiny_green = Material::new_shiny(Color::new(0.07, 0.90, 0.11), (Color::new(1.0, 1.0, 1.0), 7.0), None);
    let solid_shiny_blue = Material::new_shiny(Color::new(0.03, 0.07, 0.93), (Color::new(1.0, 1.0, 1.0), 7.0), None);
    let solid_shiny_white = Material::new_shiny(Color::new(1.0, 1.0, 1.0), (Color::new(1.0, 1.0, 1.0), 4.0), None);
    let silver = Material::new_reflective(FresnelIndex::new(0.17, 0.17, 0.17), FresnelIndex::new(2.0, 2.0, 2.0), None, Some((Color::one(), 100.0)), None);
    let glass = Material::new_reflective_and_refractive(FresnelIndex::new(1.55, 1.5, 1.45), FresnelIndex::one(), None, None, None);

    let tan_60 = (PI/3.0).tan();
    

    // let sphere_1 = {
    //     let mut result = ModelViewModelWrapper::new_identity(SolidSphere::new(silver));
    //     result.translate(Vector3::new(-1.0, -tan_60 * 0.5, 0.0));
    //     result
    // };
    // let sphere_2 = {
    //     let mut result = ModelViewModelWrapper::new_identity(SolidSphere::new(solid_shiny_green));
    //     result.translate(Vector3::new(1.0, -tan_60 * 0.5, 0.0));
    //     result
    // };
    // let sphere_3 = {
    //     let mut result = ModelViewModelWrapper::new_identity(SolidSphere::new(solid_shiny_blue));
    //     result.translate(Vector3::new(0.0, 0.5*tan_60, 0.0));
    //     result
    // };
    let plane = SolidPlane::new_positioned(solid_diffuse_green, Point3::new(0.0, 0.0, -2.0), Unit::new_unchecked(Vector3::new(0.0, 0.0, 1.0)));
    let mirror = SolidPlane::new_positioned(solid_diffuse_red, Point3::new(0.0, 7.0, 0.0), Unit::new_unchecked(Vector3::new(0.0, 1.0, 0.0)));
    let mirror_2 = SolidPlane::new_positioned(solid_diffuse_blue, Point3::new(-7.0, 0.0, 0.0), Unit::new_unchecked(Vector3::new(-1.0, 0.0, 0.0)));

    // let lens = {
    //     let mut result = ModelViewModelWrapper::new_identity(SolidSphere::new(glass));
    //     result.scale_non_uniform(Vector3::new(1.0, 0.4, 1.0));
    //     result.rotate(Vector3::new(0.0, 0.0, 1.0), PI/5.0);
    //     result.translate(Vector3::new(6.5, -3.5, 3.0));
    //     result
    // };

    let light = DotLightSource::new_natural(Color::new(1.0, 1.0, 1.0), 60.0, Point3::new(7.0, -7.0, 8.0));
    let light_2 = DotLightSource::new_natural(Color::new(1.0, 1.0, 1.0), 40.0, Point3::new(0.0, 0.0, 8.0));

    let intersector = SimpleIntersector::new(vec![//Box::new(sphere_1),
//                                                Box::new(sphere_2),
//                                                Box::new(sphere_3),
                                                  Box::new(plane),
                                                  Box::new(mirror),
                                                  Box::new(mirror_2),
//                                                Box::new(lens)
                                                  ]);
    let illuminator = SimpleIlluminator::new(vec![Box::new(light),
                                                  Box::new(light_2)]);
    let color_calculator = SimpleColorCalculator::new();

    let world = World::new(intersector, color_calculator, illuminator, 8);
    let view = View::new_unit(Point3::new(7.0, -7.0, 4.0),
                              Vector3::new(-7.0, 7.0, -1.0), 
                              Vector3::new(0.0, 0.0, 1.0),
                              1.77777777, 0.8, 480);


    let (screen_hor_res, screen_ver_res) = view.get_screen().get_resolutoion();

    let renderer_output = ImageRendererOutput::new(screen_hor_res as u32, screen_ver_res as u32);
    let mut renderer = ParalellRenderer::new(8, world, view, renderer_output);
    renderer.execute();

    match std::fs::File::create("output/result.png") {
        Ok(ref mut file) => {
            if let Err(err_msg) = renderer.get_renderer_output().get_image().save(file, ImageFormat::PNG) {
                eprintln!("Couldnt save output file: {:?}", err_msg);
            }
        },
        Err(_) => ()
    }
}
