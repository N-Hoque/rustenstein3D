use rsfml::system::{Vector2f, Vector2i};

#[derive(Clone)]
pub struct Map {
    map: Box<[i32]>,
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
            Orientation::Top => Self::new(-1, 0),
            Orientation::Bottom => Self::new(1, 0),
            Orientation::Left => Self::new(0, -1),
            Orientation::Right => Self::new(0, 1),
            Orientation::TopLeft => Self::new(-1, -1),
            Orientation::TopRight => Self::new(-1, 1),
            Orientation::BottomLeft => Self::new(1, -1),
            Orientation::BottomRight => Self::new(1, 1),
        }
    }
}

impl Map {
    pub(crate) fn new(map: &[i32], map_size: Vector2f) -> Self {
        Self {
            map: map.into(),
            map_size: Vector2i {
                x: map_size.x as i32,
                y: map_size.y as i32,
            },
        }
    }

    pub(crate) const fn get_block(&self, position: Vector2i) -> Option<i32> {
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

    pub(crate) const fn get_map_size(&self) -> Vector2i {
        self.map_size
    }
}
