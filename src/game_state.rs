#[derive(Default, PartialEq)]
pub enum GameState {
    #[default]
    Intro,
    Playing,
    Paused,
}
