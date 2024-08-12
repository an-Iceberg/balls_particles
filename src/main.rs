#![allow(mixed_script_confusables)]

mod particle;

use egui_macroquad::{draw, egui::{epaint::Shadow, Align2, Slider, Visuals, Window}, ui};
use macroquad::{color::BLACK, window::{clear_background, Conf}};
use macroquad::prelude::*;
use particle::Particle;

pub(crate) const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");
pub(crate) const AUTHORS: Option<&str> = option_env!("CARGO_PKG_AUTHORS");

#[allow(clippy::needless_return)]
pub fn window_config() -> Conf
{
  return Conf
  {
    window_title: "Balls".to_string(),
    window_width: 1290,
    window_height: 720,
    fullscreen: false,
    window_resizable: false,
    ..Conf::default()
  };
}

#[derive(Debug, PartialEq, Eq)]
enum Style
{ Random, Trail1, Trail2, Stars }

#[macroquad::main(window_config)]
async fn main()
{
  let mut particles: Vec<Particle> = vec![];

  let mut disappear_speed = 0.2;
  let mut particle_amount = 1;

  let mut stars_slowdown_rate = 5;
  let mut throttle = stars_slowdown_rate;
  let mut less_stars = false;

  let mut style = Style::Random;

  // Game loop
  loop
  {
    clear_background(BLACK);

    if style == Style::Trail1 || style == Style::Trail2
    { particle_amount = 1; }

    // If mouse moved spawn particles
    if mouse_delta_position().length().abs() > 0.
    {
      if style != Style::Stars || !less_stars
      {
        for _ in 0..=particle_amount
        {  particles.push(Particle::new(&style)); }
      }
      else
      {
        throttle -= 1;
        if throttle == 0
        {
          particles.push(Particle::new(&style));
          throttle = stars_slowdown_rate;
        }
      }
    }

    // Remove particles at the end of their lifespan
    match style
    {
      Style::Random | Style::Trail1 => particles.retain(|particle| particle.radius > 1.),
      Style::Trail2 => particles.retain(|particle| particle.color.a > 0.01),
      Style::Stars => particles.retain(|particle| particle.time > 0.),
    }

    particles.iter_mut()
      .for_each(|particle|
      {
        match style
        {
          Style::Random =>
          {
            // Apply velocity
            particle.position += particle.velocity * get_frame_time();
            // Make particle smaller
            particle.radius -= disappear_speed;
            // Reduce opacity
            particle.color.a *= 0.99;
          }
          Style::Trail1 =>
          {
            // Make particle smaller
            particle.radius -= disappear_speed;
            // Reduce opacity
            particle.color.a *= 0.99;
          }
          Style::Trail2 =>
          {
            // Make particle larger
            particle.radius += disappear_speed;
            // Reduce opacity
            particle.color.a *= 0.97;
          }
          Style::Stars =>
          {
            let δ_time = get_frame_time();
            // Apply velocity
            particle.position += particle.velocity * δ_time;
            // Reduce time
            particle.time -= δ_time;

            if particle.time > 5.5
            { particle.color.a += 0.01; }

            particle.color.a = clamp(particle.color.a, 0.0, 0.5);

            if particle.time < 5.
            { particle.color.a -= 0.005; }
          }
        }

        // Draw particle
        draw_circle(particle.position.x, particle.position.y, particle.radius, particle.color);
      });

    ui(|egui_context|
    {
      egui_context.set_visuals(Visuals
      {
        window_shadow: Shadow::NONE,
        ..Default::default()
      });

      Window::new("")
        .movable(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, egui_macroquad::egui::Vec2::new(-10., 10.))
        .fixed_size(egui_macroquad::egui::Vec2::new(152., 150.))
        .show(egui_context, |ui|
        {
          ui.radio_value(&mut style, Style::Random, "Random");
          ui.radio_value(&mut style, Style::Trail1, "Vanishing trail");
          ui.radio_value(&mut style, Style::Trail2, "Exploding trail");
          ui.radio_value(&mut style, Style::Stars, "Stars");
          ui.separator();

          ui.add_enabled_ui(style != Style::Stars, |ui|
          {
            ui.label("Disappearing speed:");
            ui.add(Slider::new(&mut disappear_speed, 0.1..=0.7));
          });

          ui.add_enabled_ui(style != Style::Trail1 && style != Style::Trail2, |ui|
          {
            ui.label("Spawn count:");
            ui.add(Slider::new(&mut particle_amount, 1..=10));
          });

          ui.add_enabled_ui(style == Style::Stars, |ui|
          {
            ui.checkbox(&mut less_stars, "Throttle star spawn rate");
            ui.add_enabled_ui(less_stars, |ui|
            {
              ui.label("Throttle amount:");
              ui.add(Slider::new(&mut stars_slowdown_rate, 1..=10));
            });
          });

          ui.separator();
          ui.label(format!("# of particles: {}", particles.len()));
          ui.separator();
          ui.label(format!("fps: {}", get_fps()));
          ui.separator();

          // --- CREDITS (!important) ---
          ui.horizontal(|ui|
          {
            ui.label(format!("v{}", VERSION.unwrap_or("unknown")));
            ui.separator();
            ui.label("Made by");
            ui.hyperlink_to(AUTHORS.unwrap_or("Sandra").to_string(), "https://github.com/an-Iceberg");
          });
        });
    });

    draw();

    next_frame().await;
  }
}
