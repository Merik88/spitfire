
use quicksilver::{
    Result,
    geom::{Rectangle, Vector},
    graphics::{Background, Color},
    input::{GamepadButton, Key},
    lifecycle::{State, Window, run}
};
 
struct Game {
    player_position: Vector
}
 
impl State for Game {
    fn new() -> Result<Game> {
        Ok(Game {
            player_position: Vector::new(375, 530)
        })
    }
    
    fn update(&mut self, window: &mut Window) -> Result<()> {
        if window.keyboard()[Key::Right].is_down() ||
            window.gamepads().iter().any(|pad| pad[GamepadButton::DpadRight].is_down())
            {
            self.player_position.x += 5.0;
        }
        if window.keyboard()[Key::Left].is_down() ||
            window.gamepads().iter().any(|pad| pad[GamepadButton::DpadLeft].is_down())
            {
            self.player_position.x -= 5.0;
        }
        if window.keyboard()[Key::Up].is_down() ||
            window.gamepads().iter().any(|pad| pad[GamepadButton::DpadUp].is_down())
            {
            self.player_position.y -= 5.0;
        }
        if window.keyboard()[Key::Down].is_down() ||
            window.gamepads().iter().any(|pad| pad[GamepadButton::DpadDown].is_down())
            {
            self.player_position.y += 5.0;
        }
        Ok(())
    }
    
    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::BLUE)?;
        window.draw(
            &Rectangle::new(self.player_position,(50,60)),
            Background::Col(Color::GREEN)
        );
        Ok(())
    }
}
 
fn main() {
    run::<Game>("Spitfire", Vector::new(800, 600), Default::default());
}