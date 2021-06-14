use rsfml::system::Clock;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AnimationState {
    Play,
    Pause,
    Stop,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum AnimationPlayMode {
    Once,
    Infinite,
}

pub struct Animation {
    a: u32,
    b: u32,
    offset: u32,
    texture_ids: Vec<i32>,
    state: AnimationState,
    mode: AnimationPlayMode,
    lag: f32,
    current_texture: u32,
    clock: Clock,
}

impl Animation {
    pub fn new(
        texture_ids: Vec<i32>,
        state: AnimationState,
        mode: AnimationPlayMode,
        lag: f32,
        offset: u32,
    ) -> Animation {
        Animation {
            a: 1,
            b: texture_ids.len() as u32 - 1u32,
            offset,
            texture_ids,
            state,
            mode,
            lag,
            current_texture: 0,
            clock: Clock::start(),
        }
    }

    pub fn set_state(&mut self, new_state: AnimationState) -> () {
        self.state = new_state;
        match new_state {
            AnimationState::Stop => {
                self.current_texture = 0;
                self.clock.restart();
            }
            AnimationState::Play if self.offset <= self.current_texture => {
                self.current_texture = self.a;
                self.clock.restart();
            }
            _ => {}
        }
    }

    pub fn set_mode(&mut self, new_mode: AnimationPlayMode) -> () {
        self.mode = new_mode;
    }

    pub fn get_state(&self) -> AnimationState {
        self.state
    }

    pub fn get_mode(&self) -> AnimationPlayMode {
        self.mode
    }

    pub fn set_lag(&mut self, new_lag: f32) -> () {
        self.lag = new_lag
    }

    pub fn get_current_texture_id(&self) -> i32 {
        self.texture_ids[self.current_texture as usize]
    }

    pub fn set_loop_anim(&mut self, a: u32, b: u32) -> () {
        self.a = a;
        self.b = b;
    }

    pub fn set_need_anim_offset(&mut self, offset: u32) -> () {
        self.offset = offset
    }

    pub fn update(&mut self) -> () {
        if self.state != AnimationState::Play {
            return;
        }

        if self.clock.elapsed_time().as_seconds() < self.lag {
            return;
        }

        if self.current_texture != self.texture_ids.len() as u32 - 1 {
            self.current_texture += 1;
        } else {
            self.current_texture = 0;
            if let AnimationPlayMode::Once = self.mode {
                self.state = AnimationState::Stop
            }
        }

        self.clock.restart();
    }
}
