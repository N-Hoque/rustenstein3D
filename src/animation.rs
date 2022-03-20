use rsfml::system::Clock;

#[derive(Clone, Copy)]
pub enum PlayState {
    Play,
    Stop,
}

struct Data {
    start_tid: u32,
    offset: u32,
    texture_ids: Vec<i32>,
    lag: f32,
}

pub struct Animation {
    state: PlayState,
    data: Data,
    active_texture: u32,
    clock: Clock,
}

impl Animation {
    pub fn new(texture_ids: &[i32], state: PlayState, lag: f32, offset: u32) -> Self {
        Self {
            state,
            data: Self::make_data(offset, texture_ids, lag),
            active_texture: 0,
            clock: Clock::start(),
        }
    }

    pub fn get_current_texture_id(&self) -> i32 {
        self.data.texture_ids[self.active_texture as usize]
    }

    pub fn set_state(&mut self, new_state: PlayState) {
        self.state = new_state;
        self.set_active_texture();
    }

    fn set_active_texture(&mut self) {
        match self.state {
            PlayState::Stop => {
                self.active_texture = 0;
                self.clock.restart();
            }
            PlayState::Play if self.data.offset <= self.active_texture => {
                self.active_texture = self.data.start_tid;
                self.clock.restart();
            }
            _ => {}
        }
    }

    pub fn update(&mut self) {
        if let PlayState::Play = self.state {
            if self.clock.elapsed_time().as_seconds() >= self.data.lag {
                if self.active_texture != self.data.texture_ids.len() as u32 - 1 {
                    self.active_texture += 1;
                } else {
                    self.active_texture = 0;
                    self.state = PlayState::Stop
                }
                self.clock.restart();
            }
        }
    }

    fn make_data(offset: u32, texture_ids: &[i32], lag: f32) -> Data {
        Data {
            start_tid: 1,
            offset,
            texture_ids: texture_ids.to_vec(),
            lag,
        }
    }
}
