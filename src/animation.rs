use rsfml::system::Clock;

#[derive(Clone, Copy)]
pub enum PlayState {
    Play,
    Pause,
    Stop,
}

#[derive(Clone, Copy)]
pub enum PlayMode {
    Once,
    Infinite,
}

struct Data {
    a: u32,
    b: u32,
    offset: u32,
    texture_ids: Vec<i32>,
    lag: f32,
    current_texture: u32,
    clock: Clock,
}

pub struct Animation {
    state: PlayState,
    mode: PlayMode,
    data: Data,
}

impl Animation {
    pub fn new(
        texture_ids: Vec<i32>,
        state: PlayState,
        mode: PlayMode,
        lag: f32,
        offset: u32,
    ) -> Animation {
        let data = Data {
            a: 1,
            b: texture_ids.len() as u32 - 1u32,
            offset,
            texture_ids,
            lag,
            current_texture: 0,
            clock: Clock::start(),
        };
        Animation { state, mode, data }
    }

    pub fn set_state(&mut self, new_state: PlayState) {
        self.state = new_state;
        match self.state {
            PlayState::Stop => {
                self.data.current_texture = 0;
                self.data.clock.restart();
            }
            PlayState::Play if self.data.offset <= self.data.current_texture => {
                self.data.current_texture = self.data.a;
                self.data.clock.restart();
            }
            _ => {}
        }
    }

    pub fn set_mode(&mut self, new_mode: PlayMode) {
        self.mode = new_mode;
    }

    pub fn get_state(&self) -> PlayState {
        self.state
    }

    pub fn get_mode(&self) -> PlayMode {
        self.mode
    }

    pub fn set_lag(&mut self, new_lag: f32) {
        self.data.lag = new_lag
    }

    pub fn get_current_texture_id(&self) -> i32 {
        self.data.texture_ids[self.data.current_texture as usize]
    }

    pub fn set_loop_anim(&mut self, a: u32, b: u32) {
        self.data.a = a;
        self.data.b = b;
    }

    pub fn set_need_anim_offset(&mut self, offset: u32) {
        self.data.offset = offset
    }

    pub fn update(&mut self) {
        if let PlayState::Play = self.state {
            if self.data.clock.elapsed_time().as_seconds() >= self.data.lag {
                if self.data.current_texture != self.data.texture_ids.len() as u32 - 1 {
                    self.data.current_texture += 1;
                } else {
                    self.data.current_texture = 0;
                    if let PlayMode::Once = self.mode {
                        self.state = PlayState::Stop
                    }
                }
                self.data.clock.restart();
            }
        }
    }
}
