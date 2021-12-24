use rsfml::system::{Vector2f, Vector2i};

pub struct Map {
    map: Vec<i32>,
    map_size: Vector2i,
}

impl Clone for Map {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            map_size: self.map_size.clone(),
        }
    }
}

enum Orientation {
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
    pub fn new<'r>(map: Vec<i32>, map_size: &'r Vector2f) -> Map {
        Map {
            map,
            map_size: Vector2i {
                x: map_size.x as i32,
                y: map_size.y as i32,
            },
        }
    }

    pub fn get_block_with_orientation<'r>(
        &self,
        block_orientation: Orientation,
        position: &'r Vector2i,
    ) -> Option<i32> {
        match block_orientation {
            Top => self.handle_top(position),
            Bottom => self.handle_bottom(position),
            Left => self.handle_left(position),
            Right => self.handle_right(position),
            Top_Left => self.handle_top_left(position),
            Top_Right => self.handle_top_right(position),
            Bottom_Left => self.handle_bottom_left(position),
            Bottom_Right => self.handle_bottom_right(position),
        }
    }

    pub fn get_block<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        if position.x < 0
            || position.y < 0
            || position.x > self.map_size.x
            || position.y > self.map_size.y
        {
            return None;
        } else {
            return Some(self.map[(position.y * self.map_size.x + position.x) as usize]);
        }
    }

    pub fn get_map_size<'r>(&'r self) -> &'r Vector2i {
        &self.map_size
    }

    fn handle_top<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x - 1,
            y: position.y,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_bottom<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x + 1,
            y: position.y,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_left<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x,
            y: position.y - 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_right<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x,
            y: position.y + 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_top_left<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x - 1,
            y: position.y - 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_top_right<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x - 1,
            y: position.y + 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_bottom_left<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x + 1,
            y: position.y - 1,
        };
        self.get_block(&tmp_pos)
    }

    fn handle_bottom_right<'r>(&self, position: &'r Vector2i) -> Option<i32> {
        let tmp_pos = Vector2i {
            x: position.x + 1,
            y: position.y + 1,
        };
        self.get_block(&tmp_pos)
    }
}
