extern crate ggez;
extern crate nalgebra as na;

use ggez::conf;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point};
use ggez::timer;
use std::time::Duration;
use na::core::*;


/// *********************************************************************
/// Main State
/// *********************************************************************
#[allow(dead_code)]
struct MainState {
    planet: Actor,
    screen_width: u32,
    screen_height: u32,
    assets: Assets,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;

        let planet = create_planet();

        let s = MainState {
            planet: planet,
            screen_width: ctx.conf.window_width,
            screen_height: ctx.conf.window_height,
            assets: assets,
        };
        Ok(s)
    }
}

/// ********************************************************************
/// Actor Code
/// ********************************************************************

#[derive(Debug)]
enum ActorType {
    Planet
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point,
    facing: f32,
    velocity: Vector2<f32>,
    rvel: f32,
    bbox_size: f32,

    // I am going to lazily overload "life" with a
    // double meaning:
    // for shots, it is the time left to live,
    // for players and rocks, it is the actual hit points.
    life: f32,
}
const PLANET_LIFE: f32 = 1.0;
const PLANET_BBOX: f32 = 100.0;

#[allow(dead_code)]
fn create_planet() -> Actor {
    Actor{
        tag: ActorType::Planet,
        pos: Point::zero(),
        facing: 0.0,
        velocity: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: PLANET_LIFE,
    }
}
#[allow(dead_code)]
struct Assets {
    planet_image: graphics::Image,
}

impl Assets {
    #[allow(dead_code)]
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let planet_image = graphics::Image::new(ctx, "/planet.png")?;
        Ok(Assets{
            planet_image: planet_image,
        })
    }

    #[allow(dead_code)]
    fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.tag {
            ActorType::Planet => &mut self.planet_image,
        }
    }
}

#[allow(dead_code)]
fn draw_actor(assets: &mut Assets,
              ctx: &mut Context,
              actor: &Actor,
              world_coords: (u32, u32)) -> GameResult<()> {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, actor.pos);
    let px = pos.x as f32;
    let py = pos.y as f32;
    let dest_point = graphics::Point::new(px,py);
    let image = assets.actor_image(actor);
    graphics::draw(ctx, image, dest_point, actor.facing as f32)
}

/// Translates the game coordinate system, with Y point up
/// and the origin at the center to screen coordinate system,
/// which has Y pointing down and origin at top-left corner
fn world_to_screen_coords(screen_width: u32, screen_height: u32, point: Point) -> Point {
    let width = screen_width as f32;
    let height = screen_height as f32;
    let x = point.x + width / 2.0;
    let y = height - (point.y + height / 2.0);
    Point::new(x, y)
}

/// ********************************************************************
/// Event Handler
/// ********************************************************************
impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        
        {
            let assets = &mut self.assets;
            let coords = (self.screen_width, self.screen_height);

            let planet = &self.planet;
            draw_actor(assets,ctx,planet,coords)?;
        }

        graphics::present(ctx);
        Ok(())
    }
}


/// ********************************************************************
/// Main Function
/// ********************************************************************
pub fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "small_world".to_string();
    c.window_width = 1280;
    c.window_height = 720;
    let ctx = &mut Context::load_from_conf("small_world", "Nathaniel", c).unwrap();
    
    match MainState::new(ctx) {
        Err(e) => {
            println!("Could not load game!");
            println!("Error: {}", e);
        }
        Ok(ref mut game) => {
            let result = run(ctx,game);
            if let Err(e) = result {
                println!("Error encountered running game: {}", e);
            } else {
                println!("Game exited cleanly.");
            }
        }
    }
}