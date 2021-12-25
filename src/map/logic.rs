use rsfml::system::{Vector2f, Vector2i};

use super::{Map, Orientation};

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
            Orientation::Top => self.handle_top(position),
            Orientation::Bottom => self.handle_bottom(position),
            Orientation::Left => self.handle_left(position),
            Orientation::Right => self.handle_right(position),
            Orientation::TopLeft => self.handle_top_left(position),
            Orientation::TopRight => self.handle_top_right(position),
            Orientation::BottomLeft => self.handle_bottom_left(position),
            Orientation::BottomRight => self.handle_bottom_right(position),
        }
    }

    pub fn get_block(&self, position: &Vector2i) -> Option<i32> {
        if position.x < 0
            || position.y < 0
            || position.x > self.map_size.x
            || position.y > self.map_size.y
        {
            None
        } else {
            Some(self.map[(position.y * self.map_size.x + position.x) as usize])
        }
    }

    pub fn get_map_size(&self) -> &Vector2i {
        &self.map_size
    }

    fn handle_top(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x - 1,
            y: position.y,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_bottom(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x + 1,
            y: position.y,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_left(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x,
            y: position.y - 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_right(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x,
            y: position.y + 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_top_left(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x - 1,
            y: position.y - 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_top_right(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x - 1,
            y: position.y + 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_bottom_left(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x + 1,
            y: position.y - 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_bottom_right(&self, position: &Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x + 1,
            y: position.y + 1,
        };
        self.get_block(&tmp_pos)
    }
}
