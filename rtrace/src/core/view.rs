use defs::{Point3, Vector3, Point2Int, FloatType, IntType};
use core::{Ray};
use tools::{CompareWithTolerance, Between};

#[derive(Debug)]
pub enum ScreenError {
    PixelOutOfBoundsError
}

pub struct Screen {
    center: Point3,
    normal: Vector3,
    up: Vector3,
    left: Vector3,
    width: FloatType,
    height: FloatType,
    horizontal_resolution: IntType,
    vertical_resolution: IntType,
}

impl Screen {
    pub fn new(center: Point3, normal: Vector3, up: Vector3, width: FloatType, height: FloatType, h_res: IntType, v_res: IntType) -> Self {
        if h_res <= 0 || v_res <= 0 || width.less_eq_eps(&0.0) || height.less_eq_eps(&0.0) {
            panic!("Invalid screen input values");
        }
        if !normal.dot(&up).near_zero_eps() {
            panic!("Screen normal and up vectors not in right angle");
        }

        let normal_normalized = normal.normalize();
        let up_normalized = up.normalize();

        Self {  center: center,
                normal: normal_normalized,
                up: up_normalized,
                left: normal_normalized.cross(&up_normalized),
                width: width,
                height: height,
                horizontal_resolution: h_res,
                vertical_resolution: v_res}
    }

    pub fn new_unit(center: Point3, normal: Vector3, up: Vector3, width_to_height_ratio: FloatType, height: FloatType, v_res: IntType) -> Self {
        let vertical_resolution = v_res as f64;
        Self::new(center, normal, up, width_to_height_ratio * height, height, (vertical_resolution * width_to_height_ratio).floor() as IntType, v_res)
    }

    pub fn get_resolutoion(&self) -> (IntType, IntType) {
        (self.horizontal_resolution, self.vertical_resolution)
    }

    pub fn get_pixel_count(&self) -> IntType {
        self.horizontal_resolution * self.vertical_resolution
    }

    fn get_pixel_coord_core(&self, coord: Point2Int) -> Point3 {
        let left = -((coord.x as FloatType - (self.horizontal_resolution as FloatType) / 2.0) * self.width);
        let up = -((coord.y as FloatType - (self.vertical_resolution as FloatType) / 2.0) * self.height);

        self.center + (self.up * up + self.left * left)
    }

    pub fn get_pixel_coord(&self, coord: Point2Int) -> Result<Point3, ScreenError> {
        if coord.x.between(&0, &self.vertical_resolution) && coord.y.between(&0, &self.vertical_resolution) {
            Ok(self.get_pixel_coord_core(coord))
        } else {
            Err(ScreenError::PixelOutOfBoundsError)
        }
    }

    pub fn get_nth_pixel_coord(&self, pixel_index: IntType) -> Result<Point3, ScreenError> {
        let x = pixel_index % self.horizontal_resolution;
        let y = pixel_index / self.horizontal_resolution;
        self.get_pixel_coord(Point2Int::new(x, y))
    }
}

pub struct Eye {
    position: Point3,
    direction: Vector3,
}

impl Eye {
    pub fn new(position: Point3, direction: Vector3) -> Self {
        Self {  position: position,
                direction: direction.normalize()}
    }

    pub fn get_position(&self) -> &Point3 {
        &self.position
    }

    pub fn get_direction(&self) -> &Vector3 {
        &self.direction
    }
}

#[derive(Debug)]
pub enum ViewError {
    ScreenRelated(ScreenError)
}

pub struct View {
    screen: Screen,
    eye: Eye
}

impl View {
    pub fn new(screen: Screen, eye: Eye) -> Self {
        Self {  screen: screen,
                eye: eye}
    }

    pub fn new_unit(eye_position: Point3, eye_direction: Vector3, screen_up: Vector3, screen_width_to_height_ratio: FloatType, screen_height: FloatType, screen_v_res: IntType) -> Self{
        Self {  screen: Screen::new_unit(eye_position + eye_direction,
                                         eye_direction, 
                                         screen_up,
                                         screen_width_to_height_ratio,
                                         screen_height,
                                         screen_v_res),
                eye: Eye::new(eye_position, 
                              eye_direction)
        }
    }

    pub fn get_ray_to_screen_coordinate(&self, coordinate: Point2Int) -> Result<Ray, ViewError> {
        match self.screen.get_pixel_coord(coordinate) {
            Ok(point) => {
                let eye_coord = self.eye.get_position();
                let ray_direction = point - eye_coord;
                Ok(Ray::new(*self.eye.get_position(), ray_direction))
            },
            Err(err) => Err(ViewError::ScreenRelated(err))
        }
    }

    pub fn get_ray_to_screen_pixel_index(&self, index: IntType) -> Result<Ray, ViewError> {
        match self.screen.get_nth_pixel_coord(index) {
            Ok(point) => {
                let eye_coord = self.eye.get_position();
                let ray_direction = point - eye_coord;
                Ok(Ray::new(*self.eye.get_position(), ray_direction))
            },
            Err(err) => Err(ViewError::ScreenRelated(err))
        }
    }

    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_eye(&self) -> &Eye {
        &self.eye
    }

    pub fn get_screen_pixel_count(&self) -> IntType {
        self.screen.get_pixel_count()
    }
}

pub struct PortionableViewIterator<'view> {
    portion_count: IntType,
    portion_index: IntType,
    portion_state: IntType,
    view: &'view View,
}

impl<'view> PortionableViewIterator<'view> {
    pub fn new(view: &'view View) -> Self {
        Self {  view: view,
                portion_count: 1,
                portion_index: 0,
                portion_state: 0}
    }

    pub fn new_portioned(view: &'view View, count: IntType, index: IntType) -> Self {
        if count < 0 || count > view.get_screen_pixel_count() || index >= count {
            panic!("Incorrect portioned PortionableViewIterator initializaion");
        }

        Self {  view: view,
                portion_count: count,
                portion_index: index,
                portion_state: 0}
    }
}

impl<'view> Iterator for PortionableViewIterator<'view> {
    type Item = Ray;
    fn next(&mut self) -> Option<Ray> {
        let all_count = self.view.get_screen_pixel_count();
        let portion_length = all_count / self.portion_count;
        if self.portion_state < portion_length {
            let index = portion_length * self.portion_index + self.portion_state;
            self.portion_state += 1;
            Some(self.view.get_ray_to_screen_pixel_index(index).expect("PortionableViewIterator internal error"))
        } else if self.portion_index == self.portion_count - 1 {
            let index = portion_length * self.portion_index + self.portion_state;
            if index < all_count {
                self.portion_state += 1;
                Some(self.view.get_ray_to_screen_pixel_index(index).expect("PortionableViewIterator internal error"))
            } else {
                None
            }
        } else {
            None
        }
    }
}