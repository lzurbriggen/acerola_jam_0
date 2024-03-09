use crate::timer::Timer;

pub struct ScreenDimmer {
    timer: Timer,
    pub dimming: bool,
    pub just_dimmed: bool,
}

impl ScreenDimmer {
    pub fn new() -> Self {
        Self {
            dimming: false,
            timer: Timer::new(0.5, false),
            just_dimmed: false,
        }
    }

    pub fn dim(&mut self) {
        self.dimming = true;
        self.timer.reset();
    }

    pub fn update(&mut self) {
        self.just_dimmed = false;
        self.timer.update();
        if self.dimming {
            if self.timer.just_completed() {
                self.just_dimmed = true;
                self.dimming = false;
                self.timer.reset();
            }
            return;
        }
    }

    pub fn progress(&self) -> f32 {
        self.timer.progress()
    }
}
