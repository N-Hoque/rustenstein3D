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

impl From<Orientation> for Vector2i {
    fn from(o: Orientation) -> Self {
        match o {
            Orientation::Top => Vector2i::new(-1, 0),
            Orientation::Bottom => Vector2i::new(1, 0),
            Orientation::Left => Vector2i::new(0, -1),
            Orientation::Right => Vector2i::new(0, 1),
            Orientation::TopLeft => Vector2i::new(-1, -1),
            Orientation::TopRight => Vector2i::new(-1, 1),
            Orientation::BottomLeft => Vector2i::new(1, -1),
            Orientation::BottomRight => Vector2i::new(1, 1),
        }
    }
}

impl Map {
    pub fn new(map: &[i32], map_size: Vector2f) -> Map {
        Map {
            map: map.to_vec(),
            map_size: Vector2i {
                x: map_size.x as i32,
                y: map_size.y as i32,
            },
        }
    }

    pub fn get_block(&self, position: Vector2i) -> Option<i32> {
        if position.x < 0
            || position.x > self.map_size.x
            || position.y < 0
            || position.y > self.map_size.y
        {
            None
        } else {
            Some(self.map[(position.y * self.map_size.x + position.x) as usize])
        }
    }

    pub fn get_map_size(&self) -> Vector2i {
        self.map_size
    }
}
