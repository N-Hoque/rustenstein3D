use rsfml::system::{Vector2f, Vector2i};

#[derive(Clone)]
pub struct Map {
    map: Vec<i32>,
    map_size: Vector2i,
}

pub enum Orientation {
    Top,
    Bottom,
    Left,
    Right,
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl Map {
    pub fn new(map: Vec<i32>, map_size: &Vector2f) -> Map {
        Map {
            map,
            map_size: Vector2i {
                x: map_size.x as i32,
                y: map_size.y as i32,
            },
        }
    }

    pub fn get_block_with_orientation(
        &self,
        block_orientation: Orientation,
        position: &Vector2i,
    ) -> Option<i32> {
        match block_orientation {
            Orientation::Top => self.handle_orientation(position, -1, 0),
            Orientation::Bottom => self.handle_orientation(position, 1, 0),
            Orientation::Left => self.handle_orientation(position, 0, -1),
            Orientation::Right => self.handle_orientation(position, 0, 1),
            Orientation::TopLeft => self.handle_orientation(position, -1, -1),
            Orientation::TopRight => self.handle_orientation(position, -1, 1),
            Orientation::BottomLeft => self.handle_orientation(position, 1, -1),
            Orientation::BottomRight => self.handle_orientation(position, 1, 1),
        }
    }

    pub fn get_block(&self, position: &Vector2i) -> Option<i32> {
        return if position.x >= 0 && position.x <= self.map_size.x &&
            position.y >= 0 && position.y <= self.map_size.y {
            Some(self.map[(position.y * self.map_size.x + position.x) as usize])
        } else {
            None
        };
    }

    pub fn get_map_size(&self) -> &Vector2i {
        &self.map_size
    }

    fn handle_orientation(&self, position: &Vector2i, x_offset: i32, y_offset: i32) -> Option<i32> {
        self.get_block(&(*position + Vector2i { x: x_offset, y: y_offset }))
    }
}
