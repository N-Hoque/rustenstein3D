use rsfml::system::Clock;

use super::{Animation, Data, PlayMode, PlayState};

impl Animation {
    pub fn new(
        texture_ids: Vec<i32>,
        state: PlayState,
        mode: PlayMode,
        lag: f32,
        offset: u32,
    ) -> Self {
        Self {
            state,
            mode,
            data: Self::make_data(offset, texture_ids, lag),
            current_texture: 0,
            clock: Clock::start(),
        }
    }

    pub fn set_state(&mut self, new_state: PlayState) {
        self.state = new_state;
        match self.state {
            PlayState::Stop => {
                self.current_texture = 0;
                self.clock.restart();
            }
            PlayState::Play if self.data.offset <= self.current_texture => {
                self.current_texture = self.data.a;
                self.clock.restart();
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
        self.data.texture_ids[self.current_texture as usize]
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
            if self.clock.elapsed_time().as_seconds() >= self.data.lag {
                if self.current_texture != self.data.texture_ids.len() as u32 - 1 {
                    self.current_texture += 1;
                } else {
                    self.current_texture = 0;
                    if let PlayMode::Once = self.mode {
                        self.state = PlayState::Stop
                    }
                }
                self.clock.restart();
            }
        }
    }

    fn make_data(offset: u32, texture_ids: Vec<i32>, lag: f32) -> Data {
        Data {
            a: 1,
            b: texture_ids.len() as u32 - 1u32,
            offset,
            texture_ids,
            lag,
        }
    }
}
