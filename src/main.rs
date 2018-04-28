extern crate rtrace;
extern crate image;

#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;

mod renderer;

use rtrace::basic::{SimpleIlluminator, SimpleIntersector, SimpleColorCalculator, WorldViewTaskProducer, 
                    GlobalIlluminationShaderTaskProducer, GlobalIlluminationShader, MedianFilter};
use rtrace::basic::model::{SolidSphere, SolidPlane};
use rtrace::basic::lightsource::{DotLightSource};
use rtrace::core::{ModelViewModelWrapper, Material, Color, ThreadSafeIterator,
                   FresnelIndex, View, World, WorldView, WorldViewTrait, RenderingTaskProducer, ScreenIterator,
                   OrderedTaskProducers, SceneBufferLayering, ImmutableSceneBuffer, MutableSceneBuffer ,ImmutableSceneBufferWrapper,
                   BasicSceneBuffer};
use rtrace::defs::{Point3, Point2Int, Vector3, FloatType};
use image::{DynamicImage, Rgba, Pixel, GenericImage, ImageFormat};
use renderer::{SingleThreadedRenderer, ParalellRenderer, RendererOutput};

use na::{Unit};
use std::sync::{Arc};
use std::borrow::{Borrow};

use std::thread;
use std::f64::consts::{PI, FRAC_PI_2};

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
    let solid_diffuse_red = Material::new_diffuse(Color::new(1.0, 0.2, 0.2), None);
    let solid_diffuse_green = Material::new_diffuse(Color::new(0.2, 1.0, 0.2), None);
    let solid_diffuse_blue = Material::new_diffuse(Color::new(0.2, 0.2, 1.0), None);
    let solid_diffuse_white = Material::new_diffuse(Color::new(1.0, 1.0, 1.0), None);
    let solid_shiny_red = Material::new_shiny(Color::new(0.87, 0.17, 0.08), (Color::new(1.0, 1.0, 1.0), 7.0), None);
    let solid_shiny_green = Material::new_shiny(Color::new(0.07, 0.90, 0.11), (Color::new(1.0, 1.0, 1.0), 7.0), None);
    let solid_shiny_blue = Material::new_shiny(Color::new(0.03, 0.07, 0.93), (Color::new(1.0, 1.0, 1.0), 7.0), None);
    let solid_shiny_white = Material::new_shiny(Color::new(1.0, 1.0, 1.0), (Color::new(1.0, 1.0, 1.0), 4.0), None);
    let silver = Material::new_reflective(FresnelIndex::new(0.17, 0.17, 0.17), FresnelIndex::new(2.0, 2.0, 2.0), None, Some((Color::one(), 100.0)), None);
    let glass = Material::new_reflective_and_refractive(FresnelIndex::new(1.55, 1.5, 1.45), FresnelIndex::one(), None, None, None);
    let light_bulb_mat = Material::new_light_source(Color::one(), None);

    let tan_60 = (PI/3.0).tan();
    
    let sphere_1 = SolidSphere::new_positioned(solid_shiny_red, Point3::new(-1.0, -tan_60 * 0.5, 1.5), 1.0);
    let sphere_2 = SolidSphere::new_positioned(solid_shiny_green, Point3::new(1.0, -tan_60 * 0.5, 0.0), 1.0);
    let sphere_3 = SolidSphere::new_positioned(solid_shiny_blue, Point3::new(0.0, 0.5*tan_60, 3.0), 1.0);
    let light_bulb = SolidSphere::new_positioned(light_bulb_mat, Point3::new(7.0, -7.0, 8.0), 1.0);

    let plane = SolidPlane::new_positioned(solid_diffuse_white, Point3::new(0.0, 0.0, -2.0), Unit::new_unchecked(Vector3::new(0.0, 0.0, 1.0)));
    let mirror = SolidPlane::new_positioned(silver, Point3::new(0.0, 7.0, 0.0), Unit::new_unchecked(Vector3::new(0.0, 1.0, 0.0)));
    let mirror_2 = SolidPlane::new_positioned(solid_diffuse_red, Point3::new(-7.0, 0.0, 0.0), Unit::new_unchecked(Vector3::new(-1.0, 0.0, 0.0)));

    let lens = {
        let mut result = ModelViewModelWrapper::new_identity(SolidSphere::new(glass));
        result.scale_non_uniform(Vector3::new(1.0, 0.4, 1.0));
        result.rotate(Vector3::new(0.0, 0.0, 1.0), PI/5.0);
        result.translate(Vector3::new(6.5, -3.5, 3.0));
        result
    };

    let light = DotLightSource::new_natural(Color::new(1.0, 1.0, 1.0), 60.0, Point3::new(7.0, -7.0, 8.0));
//  let light_2 = DotLightSource::new_natural(Color::new(1.0, 1.0, 1.0), 40.0, Point3::new(0.0, 0.0, 8.0));

    let intersector = SimpleIntersector::new(vec![Box::new(sphere_1),
                                                  Box::new(sphere_2),
                                                  Box::new(sphere_3),
                                                  Box::new(light_bulb),
                                                  Box::new(plane),
                                                  Box::new(mirror),
                                                  Box::new(mirror_2),
                                                  Box::new(lens)
                                                  ]);
    let illuminator = SimpleIlluminator::new(vec![Box::new(light)]);

    let color_calculator = SimpleColorCalculator::new();

    let world = World::new(intersector, color_calculator, illuminator, 8);
    let view = View::new_unit(Point3::new(7.0, -7.0, 4.0),
                              Vector3::new(-7.0, 7.0, -1.0), 
                              Vector3::new(0.0, 0.0, 1.0),
                              1.77777777, 1.6, 720);

    let worldview:Arc<WorldViewTrait> = Arc::new(WorldView::new(world, view));
    let shader_global_illumination = Arc::new(GlobalIlluminationShader::new(Arc::clone(&worldview), 1000, FRAC_PI_2 * (4.0/5.0)));

    let task_producer_list = vec![WorldViewTaskProducer::new(Arc::clone(&worldview)),
                                  GlobalIlluminationShaderTaskProducer::new(Arc::clone(&shader_global_illumination))
                                  ];
    
    let task_producer = OrderedTaskProducers::new(task_producer_list);
    let task_iterator = Arc::new(task_producer.create_task_iterator());

    let mut thread_container: Vec<std::thread::JoinHandle<()>> = Vec::new();
    for _counter in 0..8 {
        let intermediate_task_iterator = Arc::clone(&task_iterator);
        thread_container.push(thread::spawn(move || {
            while let Some(executable_task) = intermediate_task_iterator.next() {
                executable_task.execute();
            }
        }));
    }

    for thread_joiner in thread_container {
        thread_joiner.join().unwrap();
    }

    let gi_overlay = Arc::new(BasicSceneBuffer::new(*worldview.get_screen()));
    let mut thread_container: Vec<std::thread::JoinHandle<()>> = Vec::new();
    let model_ids = shader_global_illumination.get_all_model_ids_on_buffer().unwrap();
    for model_id in model_ids {
        let worldview_clone = Arc::clone(&worldview);
        let shader = Arc::clone(&shader_global_illumination);
        let overlay = Arc::clone(&gi_overlay);
        thread_container.push(thread::spawn(move || {
            let model_buffer = shader.get_model_buffer(model_id).unwrap();
            let immutable_model_buffer = ImmutableSceneBufferWrapper::new(model_buffer.as_ref());
            let median_filter = MedianFilter::new(&immutable_model_buffer, 3);
            worldview_clone.combine_buffer(&median_filter);
        }));
    }

    for thread_joiner in thread_container {
        thread_joiner.join().unwrap();
    }

    // worldview.layer_buffer(SceneBufferLayering::Over, 
    //                        &ImmutableSceneBufferWrapper::new(shader_global_illumination.get_entire_buffer().unwrap().as_ref()));
    // worldview.layer_buffer(SceneBufferLayering::Over, 
    //                        &ImmutableSceneBufferWrapper::new(gi_overlay.as_ref()));

    let screen = worldview.get_view().get_screen();
    let (width, height) = screen.get_resolution();
    let mut result_image = DynamicImage::new_rgb8(width as u32, height as u32);
    for coord in ScreenIterator::new(worldview.get_view().get_screen()) {
        if let Ok(Some(color)) = worldview.get_pixel_value(coord) {
            let (r, g, b) = color.normalized().mul_scalar(&(u8::max_value() as FloatType)).get();
            let pixel_color = Rgba::from_channels(r as u8, g as u8, b as u8, u8::max_value());
            result_image.put_pixel(coord.x as u32, coord.y as u32, pixel_color);
        }
    }

    // let renderer_output = ImageRendererOutput::new(screen_hor_res as u32, screen_ver_res as u32);
    // let mut renderer = ParalellRenderer::new(8, world, view, renderer_output);
    // renderer.execute();

    match std::fs::File::create("output/result.png") {
        Ok(ref mut file) => {
            if let Err(err_msg) = result_image.write_to(file, ImageFormat::PNG) {
                eprintln!("Couldnt save output file: {:?}", err_msg);
            }
        },
        Err(_) => ()
    }
}
