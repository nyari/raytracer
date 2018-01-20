use rtrace::core::{Color, View, PortionableViewIterator, RayCaster};
use rtrace::defs::{Point2Int};


pub trait RendererOutput {
    fn set_output(&mut self, coord: Point2Int, color: Color) -> bool;
}


pub struct SingleThreadedRenderer<WorldType, OutputType> {
    world: WorldType,
    view: View,
    output: OutputType,
}


impl<WorldType: RayCaster,
     OutputType: RendererOutput> 
    SingleThreadedRenderer<WorldType, OutputType> {

    pub fn new(world: WorldType, view: View, output: OutputType) -> Self {
        Self {  world: world,
                view: view,
                output: output}
    }

    pub fn execute(&mut self) {
        for (ray, coord) in PortionableViewIterator::new(&self.view) {
            match self.world.cast_ray(&ray) {
                Some(color) => {
                    self.output.set_output(coord, color);
                },
                None => ()
            }
        }
    }

    pub fn get_renderer_output(&self) -> &OutputType {
        &self.output
    }
}


// pub struct ParalellRenderer<WorldType, OutputType> {
//     thread_count: u32,
//     world: WorldType,
//     view: View,
//     output: Mutex<OutputType>,
// }


// impl<WorldType: RayCaster,
//      OutputType: RendererOutput> 
//     ParalellRenderer<WorldType, OutputType> {

//     pub fn new(thread_count: u32, world: WorldType, view: View, output: OutputType) -> Self {
//         Self {  thread_count: thread_count,
//                 world: world,
//                 view: View,
//                 output: Mutex::new(output)}
//     }

//     pub fn execute(&mut self) {

//     }
// }