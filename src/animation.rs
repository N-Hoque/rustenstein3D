use rsfml::system::Clock;

use crate::core::Update;

#[derive(Clone, Copy)]
pub enum PlayState {
    Play,
    Stop,
}

pub struct Animation {
    state: PlayState,
    data: Data,
    active_texture: u32,
    clock: Clock,
}

struct Data {
    start_tid: u32,
    offset: u32,
    texture_ids: Box<[i32]>,
    lag: f32,
}

impl Data {
    fn new(texture_ids: &[i32], offset: u32, lag: f32) -> Self {
        Self {
            start_tid: 1,
            offset,
            texture_ids: texture_ids.into(),
            lag,
        }
    }
}

impl Animation {
    pub(crate) fn new(texture_ids: &[i32], offset: u32, lag: f32, state: PlayState) -> Self {
        Self {
            state,
            data: Data::new(texture_ids, offset, lag),
            active_texture: 0,
            clock: Clock::start(),
        }
    }

    pub(crate) fn create_weapon_animation(texture_ids: &[i32]) -> Self {
        Self::new(texture_ids, 3, 0.07, PlayState::Stop)
    }

    pub(crate) const fn get_current_texture_id(&self) -> i32 {
        self.data.texture_ids[self.active_texture as usize]
    }

    pub(crate) fn set_state(&mut self, new_state: PlayState) {
        self.state = new_state;
        self.set_active_texture();
    }
}

impl Animation {
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
            PlayState::Play => {}
        }
    }
}

impl Update for Animation {
    fn update(&mut self) {
        if let PlayState::Play = self.state {
            if self.clock.elapsed_time().as_seconds() >= self.data.lag {
                if self.active_texture == self.data.texture_ids.len() as u32 - 1 {
                    self.active_texture = 0;
                    self.state = PlayState::Stop;
                } else {
                    self.active_texture += 1;
                }
                self.clock.restart();
            }
        }
    }
}
