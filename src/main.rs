extern crate ggez;
extern crate rand;
extern crate nalgebra as na;

use ggez::conf;
use ggez::event::*;
use ggez::{GameResult, Context};
use ggez::graphics;
use ggez::graphics::{Color, DrawMode, Point, FilterMode};
use ggez::timer;
use std::time::Duration;
use std::f64;
use na::core::*;
use na::geometry::Point2;

use rand::{ThreadRng, thread_rng, Rng};

const PLANET_LIFE: f32 = 1.0;
const PLANET_BBOX: f32 = 100.0;
const PLANET_SHRINK: f32 = 0.001;
const SUCCESS_LIFE: f32 = 30.0;

const PLAYER_LIFE: f32 = 10.0;
const PLAYER_BBOX: f32 = 100.0;
const PLAYER_THRUST: f32 = 200.0; // pixels per second

const PLAYER_TURN_RATE: f32  = 3.05;

const MAX_PHYSICS_VEL: f32 = 250.0;

const PLANET_DENSITY: f32 = 8.0 * 1000.0;

const SHRINK_RATE: f32 = 0.40;

const G: f32 = 0.05;

fn vec_from_angle(angle: f32) -> Vector2<f32> {
    let vx = angle.sin();
    let vy = angle.cos();
    Vector2::new(vx, vy)
}

/// *********************************************************************
/// Main State
/// *********************************************************************
struct MainState {
    //planet: Actor,
    player: Actor,
    fire: Actor,
    attention: Actor,
    minions: Vec<Actor>,
    dead_minions: Vec<Actor>,
    body_reminder: Actor,
    rings: Vec<Actor>,
    success_five: Actor,
    state: u32,
    screen_width: u32,
    screen_height: u32,
    input: InputState,
    assets: Assets,
    score: u32,
    score_display: graphics::Text,
    timer_display: graphics::Text,
    rng: ThreadRng,
    timer: f32,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        //let planet = create_planet();
        let player = create_player();
        let success_five = create_success_five();
        let score_display = graphics::Text::new(ctx, "Score", &graphics::Font::default_font().unwrap())?;
        let timer_display = graphics::Text::new(ctx, "Timer", &graphics::Font::default_font().unwrap())?;
        let s = MainState {
            //planet: planet,
            state: 3,
            score: 0,
            attention: create_attention(),
            body_reminder: create_body_reminder(),
            rng: thread_rng(),
            fire: create_fire(((ctx.conf.window_width / 2) as f32)-100.0, (-1.0*(ctx.conf.window_height / 2) as f32)+100.0),
            minions: vec![],
            dead_minions: vec![],
            rings: vec![create_ring(),create_goal_ring()],
            player: player,
            success_five: success_five,
            screen_width: ctx.conf.window_width,
            screen_height: ctx.conf.window_height,
            input: InputState::default(),
            assets: assets,
            score_display: score_display,
            timer_display: timer_display,
            timer: 45.0,
        };
        Ok(s)
    }

    fn update_ui(&mut self, ctx: &mut Context){
        let font = &mut graphics::Font::new(ctx, "/OpenSans-Regular.ttf", 18).unwrap();
        let score_str = format!("Score: {}", self.score);
        let score_text = graphics::Text::new(ctx, &score_str, font).unwrap();

        let timer_str = format!("Timer: {}", self.timer as u32);
        let timer_text = graphics::Text::new(ctx, &timer_str, font).unwrap();
        self.score_display = score_text;
        self.timer_display = timer_text;
    }
}

/// ********************************************************************
/// InputState turns keyboard events into something state-based and
/// device-independent
/// ********************************************************************
#[derive(Debug)]
struct InputState {
    xaxis: f32,
    yaxis: f32,
    fire: bool,
    any_key: bool,
}

impl Default for InputState {
    fn default() -> Self {
        InputState {
            xaxis: 0.0,
            yaxis: 0.0,
            fire: false,
            any_key: false,
        }
    }
}

/// **********************************************************
/// Assets Code
/// **********************************************************

struct Assets {
    dead_minion_image: graphics::Image,
    minion_image: graphics::Image,
    ring_image: graphics::Image,
    success_five_image: graphics::Image,
    attention_image: graphics::Image,
    fire_image: graphics::Image,
    player_image1_right: graphics::Image,
    player_image2_right: graphics::Image,
    player_image3_right: graphics::Image,
    player_image1_left: graphics::Image,
    player_image2_left: graphics::Image,
    player_image3_left: graphics::Image,
    body_reminder_image: graphics::Image,
    start1_image: graphics::Image,
    start2_image: graphics::Image,
    start3_image: graphics::Image,
    start4_image: graphics::Image,
    controls_image: graphics::Image,
    loading_image: graphics::Image,
    end_screen_image: graphics::Image,
}

impl Assets {
    fn new(ctx: &mut Context) -> GameResult<Assets> {
        let ring_image = graphics::Image::new(ctx, "/ring.png")?;
        let dead_minion_image = sprite(ctx,"/dead_minion.png")?;
        let minion_image = sprite(ctx,"/minion.png")?;
        let player_image1_right = sprite(ctx, "/boss1_right.png")?;
        let player_image2_right = sprite(ctx, "/boss2_right.png")?;
        let player_image3_right = sprite(ctx, "/boss3_right.png")?;
        let player_image1_left = sprite(ctx, "/boss1_left.png")?;
        let player_image2_left = sprite(ctx, "/boss2_left.png")?;
        let player_image3_left = sprite(ctx, "/boss3_left.png")?;
        let start1_image = sprite(ctx, "/start_screen1.png")?;
        let start2_image = sprite(ctx, "/start_screen2.png")?;
        let start3_image = sprite(ctx, "/start_screen3.png")?;
        let start4_image = sprite(ctx, "/start_screen4.png")?;
        let controls_image = sprite(ctx, "/controls.png")?;
        let loading_image = sprite(ctx, "/loading.png")?;

        let body_reminder_image = sprite(ctx, "/body_reminder.png")?;

        let success_five_image = graphics::Image::new(ctx, "/success_five.png")?;
        let attention_image = graphics::Image::new(ctx, "/attention.png")?;
        let fire_image = graphics::Image::new(ctx, "/fire.png")?;
        let end_screen_image = graphics::Image::new(ctx, "/end_screen.png")?;

        Ok(Assets{
            ring_image: ring_image,
            success_five_image: success_five_image,
            minion_image: minion_image,
            dead_minion_image: dead_minion_image,
            body_reminder_image: body_reminder_image,
            player_image1_right: player_image1_right,
            player_image2_right: player_image2_right,
            player_image3_right: player_image3_right,
            player_image1_left: player_image1_left,
            player_image2_left: player_image2_left,
            player_image3_left: player_image3_left,
            start1_image: start1_image,
            start2_image: start2_image,
            start3_image: start3_image,
            start4_image: start4_image,
            controls_image: controls_image,
            loading_image: loading_image,
            end_screen_image: end_screen_image,
            attention_image: attention_image,
            fire_image: fire_image,
        })
    }

    fn actor_image(&mut self, actor: &Actor) -> &mut graphics::Image {
        match actor.tag {
            ActorType::Minion => &mut self.minion_image,
            ActorType::Attention => &mut self.attention_image,
            ActorType::SuccessFive => &mut self.success_five_image,
            ActorType::Ring => &mut self.ring_image,
            ActorType::DeadMinion => &mut self.dead_minion_image,
            ActorType::Fire => &mut self.fire_image,
            ActorType::Player => &mut self.player_image1_right,
            ActorType::BodyReminder => &mut self.body_reminder_image,
        }
    }
}

fn sprite(ctx: &mut Context, s: &str) -> GameResult<graphics::Image> {
    let mut sprite = graphics::Image::new(ctx, s)?;
    sprite.set_filter(graphics::FilterMode::Nearest);
    Ok(sprite)
}
/// ********************************************************************
/// Actor Code
/// ********************************************************************

#[derive(Debug)]
#[derive(PartialEq)]
enum ActorType {
    Fire,
    SuccessFive,
    Ring,
    DeadMinion,
    Minion,
    Player,
    Attention,
    BodyReminder,
}

#[derive(Debug)]
struct Actor {
    tag: ActorType,
    pos: Point2<f32>,
    facing: f32,
    velocity: Vector2<f32>,
    accel: Vector2<f32>,
    rvel: f32,
    bbox_size: f32,
    scale: Point,
    life: f32,
}

/// *****************************************************
/// Actor Initializer functions
/// *****************************************************
fn create_body_reminder() -> Actor {
    Actor{
        tag: ActorType::BodyReminder,
        pos: Point2::origin(),
        facing: 0.0,
        velocity: Vector2::zeros(),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: 1.0,
        scale: Point::new(1.0,1.0),
    }
}

fn create_fire(posx: f32, posy: f32) -> Actor {
    Actor{
        tag: ActorType::Fire,
        pos: Point2::new(posx,posy),
        facing: 0.0,
        velocity: Vector2::zeros(),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: PLANET_LIFE,
        scale: Point::new(1.0,1.0),
    }
}

fn create_attention() -> Actor {
    Actor{
        tag: ActorType::Attention,
        pos: Point2::origin(),
        facing: 0.0,
        velocity: Vector2::zeros(),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: 1.0,
        scale: Point::new(1.0,1.0),
    }
}
fn create_success_five() -> Actor {
    Actor{
        tag: ActorType::SuccessFive,
        pos: Point2::origin(),
        facing: 0.0,
        velocity: Vector2::zeros(),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: SUCCESS_LIFE,
        scale: Point::new(1.0,1.0),
    }
}
fn create_ring() -> Actor {
    Actor{
        tag: ActorType::Ring,
        pos: Point2::origin(),
        facing: 0.0,
        velocity: Vector2::new(SHRINK_RATE,SHRINK_RATE),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: PLANET_LIFE,
        scale: Point::new(1.0,1.0),
    }
}

fn create_goal_ring() -> Actor {
    Actor{
        tag: ActorType::Ring,
        pos: Point2::origin(),
        facing: 0.0,
        velocity: Vector2::zeros(),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: PLANET_LIFE,
        scale: Point::new(0.3,0.3),
    }
}

fn create_dead_minion(posx: f32, posy: f32) -> Actor {
    Actor{
        tag: ActorType::DeadMinion,
        pos: Point2::new(posx,posy),
        facing: 0.0,
        velocity: Vector2::zeros(),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: PLANET_LIFE,
        scale: Point::new(0.5,0.5),
    }
}

fn create_minion(posx: f32, posy: f32) -> Actor {
    Actor{
        tag: ActorType::Minion,
        pos: Point2::new(posx,posy),
        facing: 0.0,
        velocity: Vector2::zeros(),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLANET_BBOX,
        life: PLANET_LIFE,
        scale: Point::new(0.5,0.5),
    }
}

fn create_player() -> Actor {
    Actor{
        tag: ActorType::Player,
        pos: Point2::new(500.0,0.0),
        facing: -1.0,
        velocity: Vector2::new(0.0,0.0),
        accel: Vector2::zeros(),
        rvel: 0.0,
        bbox_size: PLAYER_BBOX,
        life: PLAYER_LIFE,
        scale: Point::new(1.0,1.0),
    }
}

/// **********************************************************************
/// Actor Helping Functions
/// **********************************************************************

fn player_handle_input(actor: &mut Actor, input: &InputState, dt: f32) {
    //actor.facing += dt * PLAYER_TURN_RATE * input.xaxis;
    match input.xaxis as i32{
        0 => {
            actor.velocity.x = 0.0;
        }
        1 => {
            actor.velocity.x += PLAYER_THRUST*(dt);
        }
        -1 => {
            actor.velocity.x -= PLAYER_THRUST*(dt);
        }
        _ => (), // Do nothing
    }
    match input.yaxis as i32{
        0 => {
            actor.velocity.y = 0.0;
        }
        1 => {
            actor.velocity.y += PLAYER_THRUST*(dt);
        }
        -1 => {
            actor.velocity.y -= PLAYER_THRUST*(dt);
        }
        _ => (), // Do nothing

    }
}

fn add_minion(game: &mut MainState) -> bool {
    let x_coord = game.rng.gen_range(-1.0*(game.screen_width/2)as f32 + 90.0, (game.screen_width/2) as f32 - 290.0);
    let y_coord = game.rng.gen_range(-1.0*(game.screen_height/2)as f32 + 160.0, (game.screen_height/2) as f32 - 160.0);
    let new_minion = create_minion(x_coord,y_coord);

    let mut not_too_close = true;

    for x in 0..game.minions.len() {
        if ((game.minions[x].pos.x - new_minion.pos.x).abs() < 170.0 && (game.minions[x].pos.y - new_minion.pos.y).abs() < 310.0) {
            //println!("{}",(game.minions[x].pos.x - new_minion.pos.x).abs());
            not_too_close = false;
        }
    }

    if not_too_close {
        game.minions.push(new_minion);
    }
    return not_too_close;
}

fn update_player_position(game: &mut MainState, dt: f32) {
    let norm_sq = game.player.velocity.norm_squared();
    if norm_sq > MAX_PHYSICS_VEL.powi(2) {
        game.player.velocity = game.player.velocity / norm_sq.sqrt() * MAX_PHYSICS_VEL;
    }

    game.player.pos += game.player.velocity*dt; // + 0.5*actor.accel*dt.powi(2);

    if game.player.pos.x > (game.screen_width/2) as f32
    || game.player.pos.x < -1.0*(game.screen_width/2) as f32
    || game.player.pos.y > (game.screen_height/2) as f32
    || game.player.pos.y < -1.0*(game.screen_height/2) as f32{
        game.player.pos -= game.player.velocity*dt;
        game.player.velocity = Vector2::zeros();
    }
}
/// **********************************************************************
/// Actor Drawing
/// **********************************************************************
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
    let rotation = 0.0;
    let pos_scale = (world_coords.1 *5)as f32 /(actor.pos.y+(world_coords.1 as f32 *5.5));
    let mut scale = actor.scale;
    if actor.tag == ActorType::Minion || actor.tag == ActorType::DeadMinion || actor.tag == ActorType::Attention || actor.tag == ActorType::BodyReminder{
        scale.x *= pos_scale;
        scale.y *= pos_scale;
    }

    graphics::draw_ex(ctx, 
        image, 
        graphics::DrawParam{
            dest: dest_point,
            rotation: rotation,
            scale: scale,
            ..Default::default()
        }
    )
}

fn draw_text(ctx: &mut Context, text: &mut graphics::Text, dest_point: Point) -> GameResult<()> {
    let rotation = 0.0;
        graphics::draw_ex(ctx, 
        text, 
        graphics::DrawParam{
            dest: dest_point,
            rotation: rotation,
            scale: Point::new(1.0,1.0),
            ..Default::default()
        })
}
fn draw_player(assets: &mut Assets, ctx: &mut Context, 
    player: &mut Actor,world_coords: (u32, u32)) -> GameResult<()> {
    let (screen_w, screen_h) = world_coords;
    let pos = world_to_screen_coords(screen_w, screen_h, player.pos);
    let px = pos.x as f32;
    let py = pos.y as f32;
    let dest_point = graphics::Point::new(px,py);
    let mut image;

    if player.velocity.x > 0.0 {
        player.facing = 1.0;
    }
    else if player.velocity.x < 0.0{
        player.facing = -1.0;
    }

    if player.velocity != Vector2::zeros(){
        if player.life >= 0.0 && player.life < 0.25 {
            if player.facing == 1.0{
                image = &mut assets.player_image2_right;
            }
            else {
                image = &mut assets.player_image2_left;
            }
        }
        else if player.life >= 0.25 && player.life < 0.5 {
            if player.facing == 1.0{
                image = &mut assets.player_image3_right;
            }
            else {
                image = &mut assets.player_image3_left;
            }
        }
        else if player.life > 0.5 && player.life < 0.75 {
            if player.facing == 1.0{
                image = &mut assets.player_image1_right;
            }
            else {
                image = &mut assets.player_image1_left;
            }
        }
        else{
            if player.facing == 1.0 {
                image = &mut assets.player_image1_right;
            }
            else {
                image = &mut assets.player_image1_left;
            }
            player.life = 0.0;
        }
    }
    else {
        if player.facing == 1.0 {
            image = &mut assets.player_image1_right;
        }
        else {
            image = &mut assets.player_image1_left;
        }
    }
    let rotation = 0.0;
    let pos_scale = (world_coords.1 *5)as f32 /(player.pos.y+(world_coords.1 as f32 *5.5));
    let mut scale = player.scale;
    scale.x *= pos_scale;
    scale.y *= pos_scale;
    graphics::draw_ex(ctx, 
        image, 
        graphics::DrawParam{
            dest: dest_point,
            rotation: rotation,
            scale: scale,
            ..Default::default()
        }
    )
}

/// Translates the game coordinate system, with Y point up
/// and the origin at the center to screen coordinate system,
/// which has Y pointing down and origin at top-left corner
fn world_to_screen_coords(screen_width: u32, screen_height: u32, point: Point2<f32>) -> Point2<f32> {
    let width = screen_width as f32;
    let height = screen_height as f32;
    let x = point.x + width / 2.0;
    let y = height - (point.y + height / 2.0);
    Point2::new(x, y)
}

/// ********************************************************************
/// Event Handler
/// ********************************************************************
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context, dt: Duration) -> GameResult<()> {
        let time_passed = timer::duration_to_f64(dt) as f32;
        self.update_ui(ctx);

        match self.state{
            0 => {
                update0(self, ctx, time_passed);
                if self.attention.life == 1.0 && self.input.fire {
                    self.state = 1;
                    self.input.fire = false;
                    self.rings[0].scale = Point::new(1.0,1.0);
                    let shrink_speed = self.rng.gen_range(1.2,2.3);
                    self.rings[0].velocity = Vector2::new(shrink_speed, shrink_speed);
                    let goal_scale = self.rng.gen_range(0.2, 0.8);
                    self.rings[1].scale = Point::new(goal_scale,goal_scale);
                }
                if self.timer <= 0.0 {
                    self.state = 9;
                }
            }
            1 => {
                let result = update1(self, ctx, time_passed);
                match result {
                    1 => {
                        for x in 0..self.minions.len(){
                            if na::distance(&self.player.pos,&(self.minions[x].pos+Vector2::new(75.0,0.0))) < 50.0{
                                self.minions.remove(x);
                                break;
                            }
                        }
                        let mut succeeded = false;
                        while !succeeded{
                            succeeded = add_minion(self);
                        }
                        self.attention.life = 0.0;
                        self.state = 2;       
                        self.score += 1;
                        if self.score < 10{
                            self.timer += 5.0;
                        }
                        else {
                            self.timer += 3.0;
                        }
                    }
                    2 => {
                        for x in 0..self.minions.len(){
                            if na::distance(&self.player.pos,&(self.minions[x].pos+Vector2::new(75.0,0.0))) < 50.0{
                                self.dead_minions.push(create_dead_minion(self.minions[x].pos.x, self.minions[x].pos.y));
                                self.minions.remove(x);
                                break;
                            }
                        }
                        let mut succeeded = false;
                        while !succeeded{
                            succeeded = add_minion(self);
                        }
                        self.state = 0;
                    }
                    _ => (),
                }

            }
            2 => {
                update0(self, ctx, time_passed);
                self.success_five.life -= 1.0;
                if self.success_five.life == 0.0 {
                    self.success_five.life = SUCCESS_LIFE;
                    self.state = 0;
                }
            }
            3 => {
                let mut succeeded = false;
                for _x in 0..3{
                    while !succeeded{
                        succeeded = add_minion(self);
                    }
                    succeeded = false;
                 }
                self.state = 4;
            }
            4 => {
                if self.input.any_key {
                    self.state = 5;
                    self.input.any_key = false;
                }
            }
            5 => {
                if self.input.any_key {
                    self.state = 6;
                    self.input.any_key = false;
                }
            }
            6 => {
                if self.input.any_key {
                    self.state = 7;
                    self.input.any_key = false;
                }
            }
            7 => {
                if self.input.any_key {
                    self.state = 8;
                    self.input.any_key = false;
                }
            }
            8 => {
                if self.input.any_key {
                    self.state = 0;
                    self.input.any_key = false;
                }
            }
            9 => {

            }
            _ => (),
        }

        if self.state < 3 {
            self.timer -= time_passed;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        
        {
            let assets = &mut self.assets;
            let coords = (self.screen_width, self.screen_height);

            let score_dest = Point::new((self.score_display.width() / 2) as f32 + 200.0,
                                                    (self.score_display.height() / 2) as f32 + 10.0);
            let timer_dest = Point::new(self.screen_width as f32 - 200.0 + (self.timer_display.width()/2) as f32 ,
                                                    20.0);

            match self.state{
                0 => {
                    let player = &mut self.player;

                    for x in 0..self.minions.len(){
                        draw_actor(assets,ctx,&self.minions[x],coords)?;                    
                    }

                    draw_actor(assets,ctx,&self.fire,coords)?;

                    if !self.input.fire {
                        for x in 0..self.dead_minions.len(){
                            draw_actor(assets,ctx,&self.dead_minions[x],coords)?;                    
                        }
                    }

                    draw_player(assets,ctx,player,coords)?;

                    if self.input.fire {
                        for x in 0..self.dead_minions.len(){
                            draw_actor(assets,ctx,&self.dead_minions[x],coords)?;                    
                        }
                    }

                    if self.attention.life == 1.0 {
                        draw_actor(assets,ctx, &self.attention,coords)?;
                    }
                    if self.body_reminder.life == 1.0 {
                        draw_actor(assets,ctx, &self.body_reminder,coords)?;
                    }


                }
                1 => {
                    draw_actor(assets,ctx,&self.rings[0],coords)?;
                    draw_actor(assets,ctx,&self.rings[1],coords)?;
                }
                2 => {
                    let player = &self.player;

                    draw_actor(assets,ctx,player,coords)?;
                    draw_actor(assets,ctx,&self.success_five,coords)?;                
                }
                3 => {
                    graphics::draw(ctx, &assets.loading_image, Point::new((coords.0/2) as f32,(coords.1/2) as f32),0.0)?;
                }
                4 => {
                    graphics::draw(ctx, &assets.start1_image, Point::new((coords.0/2) as f32,(coords.1/2) as f32),0.0)?;
                }
                5 => {
                    graphics::draw(ctx, &assets.start2_image, Point::new((coords.0/2) as f32,(coords.1/2) as f32),0.0)?;
                }
                6 => {
                    graphics::draw(ctx, &assets.start3_image, Point::new((coords.0/2) as f32,(coords.1/2) as f32),0.0)?;
                }
                7 => {
                    graphics::draw(ctx, &assets.start4_image, Point::new((coords.0/2) as f32,(coords.1/2) as f32),0.0)?;
                }
                8 => {
                    graphics::draw(ctx, &assets.controls_image, Point::new((coords.0/2) as f32,(coords.1/2) as f32),0.0)?;
                }
                9 => {
                    graphics::draw(ctx, &assets.end_screen_image, Point::new((coords.0/2) as f32,(coords.1/2) as f32),0.0)?;
                    let font = &mut graphics::Font::new(ctx, "/OpenSans-ExtraBold.ttf", 32).unwrap();
                    let end_str1 = format!("Congratulations, You Have Made {} Friends", self.score);
                    let mut end_text1 = graphics::Text::new(ctx, &end_str1, font).unwrap();
                    let end_dest1 = Point::new((self.screen_width/2) as f32 - 100.0,
                                                    (self.screen_height/2) as f32);
                    let end_str2 = format!("Press Esc To Exit Game");
                    let mut end_text2 = graphics::Text::new(ctx, &end_str2, font).unwrap();
                    let end_dest2 = Point::new((self.screen_width/2) as f32 - 100.0,
                                                     (self.screen_height/2) as f32 + 50.0);
                    graphics::set_color(ctx, graphics::BLACK)?;
                    draw_text(ctx, &mut end_text1, end_dest1)?;
                    draw_text(ctx, &mut end_text2, end_dest2)?;
                    graphics::set_color(ctx, graphics::WHITE)?;
                }
                _ => (),
            }

            if self.state < 3{
                graphics::set_color(ctx, graphics::BLACK)?;
                draw_text(ctx,&mut self.score_display, score_dest)?;
                draw_text(ctx,&mut self.timer_display, timer_dest)?;
                graphics::set_color(ctx, graphics::WHITE)?;
            }

        }

        graphics::present(ctx);
        Ok(())
    }

    fn key_down_event(&mut self,
                      keycode: Keycode,
                      _keymod: Mod,
                      _repeat: bool) {
        self.input.any_key = true;
        match keycode {
            Keycode::W => {
                self.input.yaxis = 1.0;
            }
            Keycode::S => {
                self.input.yaxis = -1.0;
            }
            Keycode::A => {
                self.input.xaxis = -1.0;
            }
            Keycode::D => {
                self.input.xaxis = 1.0;
            }
            Keycode::Up => {
                self.input.yaxis = 1.0;
            }
            Keycode::Down => {
                self.input.yaxis = -1.0;
            }
            Keycode::Left => {
                self.input.xaxis = -1.0;
            }
            Keycode::Right => {
                self.input.xaxis = 1.0;
            }
            Keycode::Space => {
                self.input.fire = true;
            }
            _ => (), // Do nothing
        }
    }


    fn key_up_event(&mut self, keycode: Keycode, _keymod: Mod, _repeat: bool) {
        self.input.any_key = false;
        match keycode {
            Keycode::W | Keycode::S => {
                self.input.yaxis = 0.0;
            }
            Keycode::A | Keycode::D => {
                self.input.xaxis = 0.0;
            }
            Keycode::Up | Keycode::Down => {
                self.input.yaxis = 0.0;
            }
            Keycode::Left | Keycode::Right => {
                self.input.xaxis = 0.0;
            }
            Keycode::Space => {
                self.input.fire = false;
            }
            _ => (), // Do nothing
        }
    }
}
/// ********************************************************************
/// State Updates
/// ********************************************************************
fn update0(game: &mut MainState, _ctx: &mut Context, dt: f32) {
    player_handle_input(&mut game.player, &game.input, dt);
    update_player_position(game, dt);
    game.attention.pos = game.player.pos + Vector2::new(50.0,100.0);
    game.body_reminder.pos = game.player.pos + Vector2::new(120.0,100.0);

    game.attention.life = 0.0;
    game.body_reminder.life = 0.0;
    game.player.life += dt;

    //Detecting if player is close to minion
    for x in 0..game.minions.len() {
        if na::distance(&game.player.pos,&(game.minions[x].pos+Vector2::new(75.0,0.0))) < 50.0 && game.dead_minions.len() == 0{
            game.attention.life = 1.0;
            break;
        }
        if na::distance(&game.player.pos,&(game.minions[x].pos+Vector2::new(75.0,0.0))) < 50.0 && game.dead_minions.len() != 0{
            game.body_reminder.life = 1.0;
            break;
        }
    }

    //Detecting if player is close to dead_minion
    for x in 0..game.dead_minions.len() {
        if na::distance(&game.player.pos,&game.dead_minions[x].pos) < 70.0 && game.input.fire{
            game.dead_minions[x].pos = game.player.pos + Vector2::new(10.0,10.0);
            break;
        }
    }
    // Burning the bodies
    for x in 0..game.dead_minions.len() {
        if na::distance(&game.fire.pos,&game.dead_minions[x].pos) < 50.0 && game.input.fire{
            game.dead_minions.remove(x);
            break;
        }
    }
}

fn update1(game: &mut MainState, _ctx: &mut Context, dt: f32) -> u32 {
    shrink_ring(&mut game.rings[0], dt);
    if game.input.fire {
        let ring_difference = game.rings[0].scale.x - game.rings[1].scale.x;
        if ring_difference.abs() <= 0.1 {
            return 1;
        }
        else if ring_difference.abs() > 0.1 {
            return 2;
        }
    }
    return 0;
}

fn shrink_ring(ring: &mut Actor, dt: f32) {
    if ring.velocity.x > 0.0 {
        ring.scale.x -= ring.velocity.x * (dt);
        ring.scale.y -= ring.velocity.x * (dt); 
        if ring.scale.x < 0.01 {
            ring.velocity.x *= -1.0;
            ring.velocity.y *= -1.0;
        }       
    }
    else if ring.velocity.x < 0.0 {
        ring.scale.x -= ring.velocity.x * (dt);
        ring.scale.y -= ring.velocity.x * (dt); 
        if ring.scale.x > 1.0 {
            ring.velocity.x *= -1.0;
            ring.velocity.y *= -1.0;
        }    
    }

}
/// ********************************************************************
/// Draw Updates
/// ********************************************************************

/// ********************************************************************
/// Main Function
/// ********************************************************************
pub fn main() {
    let mut c = conf::Conf::new();
    c.window_title = "Lord of High Fives".to_string();
    c.window_width = 1280;
    c.window_height = 720;
    let ctx = &mut Context::load_from_conf("Lord of High Fives", "Nathaniel", c).unwrap();
    
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