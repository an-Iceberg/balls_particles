use macroquad::{color::{Color, MAGENTA, ORANGE}, input::mouse_position, math::{vec2, Vec2}, rand::gen_range};

use crate::Style;

pub(crate) struct Particle
{
  pub(crate) position: Vec2,
  pub(crate) velocity: Vec2,
  pub(crate) color: Color,
  pub(crate) radius: f32,
  pub(crate) time: f32,
}

impl Particle
{
  #[allow(clippy::needless_return)]
  pub fn new(style: &Style) -> Self
  {
    return match style
    {
      Style::Random => Particle
      {
        position: Vec2::from(mouse_position()),
        velocity: vec2(
          gen_range(-1., 1.),
          gen_range(-1., 1.)
        ).normalize() * gen_range(1., 200.),
        color: Color::from_rgba(
          gen_range(0, 255),
          gen_range(0, 255),
          gen_range(0, 255),
          gen_range(128, 255)
        ),
        radius: gen_range(5., 25.),
        time: 1.,
      },
      Style::Trail1 => Particle
      {
        position: Vec2::from(mouse_position()),
        velocity: Vec2::ZERO,
        color: MAGENTA,
        radius: 30.,
        time: 1.,
      },
      Style::Trail2 => Particle
      {
        position: Vec2::from(mouse_position()),
        velocity: Vec2::ZERO,
        color: ORANGE,
        radius: 5.,
        time: 1.,
      },
      Style::Stars => Particle
      {
        // Place particle close to mouse
        position: Vec2::from(mouse_position()) + vec2(gen_range(-100., 100.), gen_range(-100., 100.)),
        velocity: vec2(
          gen_range(-1., 1.),
          gen_range(-1., 1.)
        ).normalize() * gen_range(1., 50.),
        color: Color::from_rgba(255, 255, 255, 1),
        radius: 5.,
        time: 7.,
      },
    };
  }
}
