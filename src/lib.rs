use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, version::revision,
    video::Window, EventPump,
};
use std::time::{Duration, Instant};
mod utils;
use utils::logger::Logger;
const FPS: u8 = 60;
const FRAME_LIMIT_MS: f64 = 1000.0 / FPS as f64;
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
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ms_prev_frame: Instant,
}

impl Game {
    pub fn new() -> Result<Self, InitError> {
        Logger::dbg("Initializing game");
        Logger::info(&format!("{}", revision())); // SDL version

        let sdl_context = sdl2::init().map_err(|_| InitError::SDLInitFailed)?;

        let video_subsystem = sdl_context
            .video()
            .map_err(|_| InitError::VideoSubsystemFailed)?;

        let window_width = WINDOW_WIDTH;
        let window_height = WINDOW_HEIGHT;
        let window = video_subsystem
            .window("kengen", window_width, window_height)
            .position_centered()
            .build()
            .map_err(|_| InitError::WindowCreationFailed)?;

        let mut canvas = window
            .into_canvas()
            .build()
            .map_err(|_| InitError::CanvasCreationFailed)?;

        let mut event_pump = sdl_context
            .event_pump()
            .map_err(|_| InitError::EventPumpFailure)?;

        canvas.set_draw_color(Color::RGB(0, 255, 0));
        canvas.clear();
        canvas.present();

        Logger::dbg("Finished initializing game");

        Ok(Self {
            is_running: true,
            ms_prev_frame: Instant::now(),
            canvas,
            event_pump,
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

    pub fn run(&mut self) -> () {
        while self.is_running {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        self.is_running = false;
                        // break;
                    }
                    _ => {}
                }
            }

            self.update();

            self.canvas.clear();
            self.canvas.present();
        }
    }

    // TODO not calculating ms since last update correctly
    pub fn update(&mut self) {
        let time_to_wait: f64 = FRAME_LIMIT_MS
            - Instant::now()
                .duration_since(self.ms_prev_frame)
                .as_millis() as f64;

        // fixed frame rate when below threshold MILLISECS_PER_FRAME
        if time_to_wait > 0.0 && time_to_wait <= FRAME_LIMIT_MS {
            let sleep_duration = Duration::new(0, (time_to_wait * 1000000.0) as u32);
            ::std::thread::sleep(sleep_duration);
            if sleep_duration.as_millis() <= 2 {
                Logger::info(&format!(
                    "Frames getting tight: sleeping {:?}ms",
                    sleep_duration.as_millis()
                ));
            }
        }

        let dt = Instant::now().duration_since(self.ms_prev_frame);
        self.ms_prev_frame = Instant::now();

        // TODO Update Systems
        // TODO Update Registry
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
