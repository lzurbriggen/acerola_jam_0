use macroquad::{prelude::*, ui::hash};

use crate::{
    entity::animated_sprite::AnimatedSprite, game_data::GameData,
    sprite::indexed_sprite::IndexedSprite,
};

use super::button::button;

pub struct IntroScreen {
    pub sprite: AnimatedSprite,
}

impl IntroScreen {
    pub fn update_and_draw(&mut self, data: &mut GameData) -> bool {
        let mut should_start = false;

        let start_id = hash!();

        let ids = vec![start_id];

        if data.ui.focus.is_none() {
            data.ui.focus = Some(ids[0]);
        }

        let center = vec2(360. / 2., 240. / 2.);

        self.sprite.update();
        self.sprite.draw(Vec2::ZERO);

        let button_width = 90.;
        if button(
            data,
            &Rect::new(center.x - button_width / 2., 160., button_width, 20.),
            true,
            "Start Game",
            None,
            Vec2::ZERO,
        ) {
            should_start = true;
        }

        should_start
    }
}
