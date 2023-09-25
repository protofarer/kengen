use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, version::revision,
    video::Window,
};
use std::time::Duration;
mod utils;
use utils::logger::Logger;

const FPS: u8 = 60;
const MILLISECS_PER_FRAME: f64 = 1000.0 / FPS as f64;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Debug)]
pub enum InitError {
    SDLInitFailed,
    VideoSubsystemFailed,
    WindowCreationFailed,
    CanvasCreationFailed,
    EventPumpFailure,
}

impl std::fmt::Display for InitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Initialization Error: {:?}", self)
    }
}

impl std::error::Error for InitError {}
pub struct Game {
    pub is_running: bool,
    ms_prev_frame: u32,
    canvas: Canvas<Window>,
}

impl Game {
    pub fn new() -> Result<Self, InitError> {
        Logger::dbg("Initializing game");
        println!("{}", format!("{}", revision())); // SDL version

        let sdl_context = sdl2::init().map_err(|_| {
            Logger::crit("Failed SDL initialization");
            InitError::SDLInitFailed
        })?;

        // ? diff between SDL video_subsys Window and sys Window
        let video_subsystem = sdl_context.video().map_err(|_| {
            Logger::crit("Failed video_subsystem init");
            InitError::VideoSubsystemFailed
        })?;

        let window_width = WINDOW_WIDTH;
        let window_height = WINDOW_HEIGHT;
        let window = video_subsystem
            .window("kengen", window_width, window_height)
            .position_centered()
            .build()
            .map_err(|_| {
                Logger::crit("Failed window creation");
                InitError::WindowCreationFailed
            })?;

        let mut canvas = window.into_canvas().build().map_err(|_| {
            Logger::crit("Failed canvas creation");
            InitError::CanvasCreationFailed
        })?;

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.clear();
        canvas.present();
        let mut event_pump = sdl_context.event_pump().map_err(|e| {
            Logger::err(&format!("Error calling SDL event pump: {}", e));
            InitError::EventPumpFailure
        })?;
        let mut i = 0;
        'running: loop {
            i = (i + 1) % 255;
            canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }
        }

        // rest of game loop goes here
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));

        Logger::dbg("Finished initializing game");
        Ok(Self {
            is_running: true,
            ms_prev_frame: 33,
            canvas,
        })
    }

    fn setup(&self) {
        Logger::dbg("START initialize game");
        // Add systems that need to be processed
        // registry->AddSystem<MovementSystem>();
        // registry->AddSystem<RenderSystem>();

        // Entity tank = registry->CreateEntity();
        // tank.AddComponent<TransformComponent>(glm::vec2(10.0, 30.0), glm::vec2(1.0, 1.0), 0.0);
        // tank.AddComponent<RigidBodyComponent>(glm::vec2(40.0, 0.0));
        // tank.AddComponent<SpriteComponent>(10, 10);

        // Entity truck = registry->CreateEntity();
        // truck.AddComponent<TransformComponent>(glm::vec2(50.0, 100.0), glm::vec2(1.0, 1.0), 0.0);
        // truck.AddComponent<RigidBodyComponent>(glm::vec2(0.0, 50.0));
        // truck.AddComponent<SpriteComponent>(10, 50);
        Logger::dbg("END initialize game");
    }

    pub fn run(&self) -> () {
        while self.is_running {
            self.update();
        }
    }

    pub fn update(&self) {
        let get_ticks: u32 = 30; // placeholder for SDL_GetTicks()
        let time_to_wait: f64 = MILLISECS_PER_FRAME - (get_ticks - self.ms_prev_frame) as f64;

        if time_to_wait > 0.0 && time_to_wait <= MILLISECS_PER_FRAME {
            // todo!();
            // SDL_Delay(time_to_wait)
        }

        // let dt: f64 = (get_ticks - self.ms_prev_frame) as f64 / 1000.0;

        // TODO Update Systems
        // TODO Update Registry

        // self.ms_prev_frame = SDL_GetTicks();
        println!("loop");
    }

    pub fn render() -> () {}
    pub fn process_input() -> () {}
    pub fn destroy(&self) {
        Logger::dbg("Destroy game");
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        Logger::dbg("Drop game");
    }
}
