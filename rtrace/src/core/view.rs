use defs::{Point3, Vector3, Point2Int, FloatType, IntType};
use core::{Ray};
use tools;

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
    horizontal_resolution: FloatType,
    vertical_resolution: FloatType,
}

impl Screen {
    pub fn new(center: Point3, normal: Vector3, up: Vector3, width: FloatType, height, FloatType, h_res: IntType, v_res: IntType) -> Self {
        if h_res <= 0 || v_res <= 0 || width.less_eq_eps(0.0) || height.less_eq_eps(0.0) {
            panic!("Invalid screen input values");
        }

        Self {  center: center,
                normal: normal.normalize(),
                up: up.normalize(),
                left: normal.normalize(), up.normalize()
                width: width,
                height: height,
                horizontal_resolution: h_res as FloatType,
                vertical_resolution, v_res as FloatType}
    }

    pub fn new_unit(center: Point3, normal: Vector3, up: Vector3, width_to_height_ratio: FloatType, height: FloatType, v_res: IntType) -> Self {
        Self::new(center, normal, up, width_to_height_ratio * height, height, (v_res * width_to_height_ratio).floor() as IntType, v_res)
    }

    fn get_pixel_coord_core(&self, coord: Point2Int) -> Point3 {
        let left = -((coord.x() as FloatType - self.horizontal_resolution / 2.0) * self.width);
        let up = -((coord.y() as FloatType - self.vertical_resolution / 2.0) * self.height);

        self.center + (self.up * up + self.left * left)
    }

    pub fn get_pixel_coord(&self, coord: Point2Int) -> Result<Point3, ScreenError> {
        if coord.x().between(0, self.vertical_resolution) && coord.y().between(0, self.vertical_resolution) {
            Ok(self.get_pixel_coord_core())
        } else {
            Err(ScreenError::PixelOutOfBoundsError)
        }
    }
}

pub struct Eye {
    position: Point3
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

    pub fn get_direction(&self) -> Vector3 {
        &self.direction)
    }
}

pub enum ViewError {
    ScreenRelated(ScreenError)
}

pub struct View {
    screen: Screen,
    eye: Eye
}

impl View {
    fn new(screen: Screen, eye: Eye) -> Self {
        Self {  screen: screen,
                eye: eye}
    }

    fn new_unit(eye_position: Point3, eye_direction: Vector3, screen_up: Vector3, screen_width_to_height_ratio: FloatType, screen_height: FloatType, screen_v_res: IntType) {
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

    fn get_ray_to_screen_coordinate(coordinate: Point2Int) -> Result<Ray, ViewError> {
        match self.screen.get_pixel_coord(coordinate) {
            Ok(point) => {
                let eye_coord = self.eye.get_direction();
                let ray_direction = point - eye_coord;
                Ok(Ray::new(eye.get_position(), ray_direction))
            },
            Err(err) => Err(ViewError::ScreenRelated(err))
        }
    }
}