extern crate rtrace;
extern crate image;

mod renderer;

use rtrace::basic::{SimpleWorld, SimpleIlluminator, SimpleIntersector, SimpleColorCalculator};
use rtrace::basic::model::{SolidUnitSphere, SolidXYPlane};
use rtrace::basic::lightsource::{DotLightSource};
use rtrace::core::{ModelViewModelWrapper, Material, Color, View, PortionableViewIterator, RayCaster};
use rtrace::defs::{Point3, Point2Int, Vector3, FloatType};
use image::{DynamicImage, Rgba, Pixel, GenericImage, ImageFormat};
use renderer::{SingleThreadedRenderer, RendererOutput};

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
    let solid_shiny_red = Material::new_shiny(Color::new(0.87, 0.17, 0.08), (Color::new(1.0, 1.0, 1.0), 7.0), None);
    let sphere = {
        let mut result = ModelViewModelWrapper::new_identity(SolidUnitSphere::new(solid_shiny_red));
        result.scale_uniform(2.0);
        result
    };

    let light = DotLightSource::new_natural(Color::one(), 40.0, Point3::new(8.0, -8.0, 0.0));

    let intersector = SimpleIntersector::new(vec![Box::new(sphere)]);
    let illuminator = SimpleIlluminator::new(vec![Box::new(light)]);
    let color_calculator = SimpleColorCalculator::new();

    let world = SimpleWorld::new(intersector, color_calculator, illuminator, 5);
    let view = View::new_unit(Point3::new(0.0, -5.0, 0.0),
                              Vector3::new(0.0, 5.0, 0.0), 
                              Vector3::new(0.0, 0.0, 1.0),
                              1.77777777, 0.001, 720);

    let (screen_hor_res, screen_ver_res) = view.get_screen().get_resolutoion();

    let renderer_output = ImageRendererOutput::new(screen_hor_res as u32, screen_ver_res as u32);
    let mut renderer = SingleThreadedRenderer::new(world, view, renderer_output);
    renderer.execute();

    match std::fs::File::create("result.png") {
        Ok(ref mut file) => {
            renderer.get_renderer_output().get_image().save(file, ImageFormat::PNG);
        },
        Err(_) => ()
    }
}
