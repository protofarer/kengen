use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, version::revision,
    video::Window, EventPump,
};
use std::time::{Duration, Instant};

mod dsa;
use dsa::FixedSizeQueue;

pub mod logger;
pub use logger::Logger;

const FRAMERATE: u8 = 60;
const FRAME_LIMIT_MS: f64 = 1000.0 / FRAMERATE as f64;
const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

// Game Loop States
// eg:
// [init] => Stopped => [run] => Running => [input: pause] => Paused => [input: stop]
// => Stopped => [input: pause] => (no effect)Stopped => [input: start/resume] => Resuming => Running => [input: pause]
// => Paused => [input: unpause] => Running
pub enum RunState {
    Stopped, //  when render not running
    Running,
    Paused,
    Resuming, // transient state that marks going from Stopped -> Running
    Exiting,
}

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
    pub run_state: RunState,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ms_prev_frame: Instant,
    fps: f64,
    fps_queue: FixedSizeQueue,
    is_debug_mode: bool,
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

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|_| InitError::CanvasCreationFailed)?;

        let event_pump = sdl_context
            .event_pump()
            .map_err(|_| InitError::EventPumpFailure)?;

        Logger::dbg("Finished initializing game");

        Ok(Self {
            run_state: RunState::Stopped,
            ms_prev_frame: Instant::now(),
            canvas,
            event_pump,
            fps: 0.0,
            fps_queue: FixedSizeQueue::new(60),
            is_debug_mode: false,
        })
    }

    fn setup(&self) {
        Logger::dbg("START initialize game");
        // Add systems that need to be processed
        // registry->AddSystem<MovementSystem>();
        // registry->AddSystem<RenderSystem>();

        // Load tilemap/other assets and create entities and add components

        // Create entities and add components
        // Entity tank = registry->CreateEntity();
        // tank.AddComponent<TransformComponent>(glm::vec2(10.0, 30.0), glm::vec2(1.0, 1.0), 0.0);
        // tank.AddComponent<RigidBodyComponent>(glm::vec2(40.0, 0.0));
        // tank.AddComponent<SpriteComponent>(10, 10);

        Logger::dbg("END initialize game");
    }

    pub fn run(&mut self) -> () {
        self.setup();
        self.run_state = RunState::Running;
        Logger::info("Game loop running");
        loop {
            self.handle_input();

            match self.run_state {
                RunState::Running => {
                    self.update();
                }
                RunState::Paused => {
                    let sleep_duration = Duration::new(0, (FRAME_LIMIT_MS * 1000000.0) as u32);
                    ::std::thread::sleep(sleep_duration);
                }
                RunState::Stopped => {
                    continue;
                }
                RunState::Resuming => self.run_state = RunState::Running,
                RunState::Exiting => {
                    break;
                }
            }

            self.render();
        }
    }

    fn handle_input(&mut self) {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    // if game already stopped, then quit, eg takes 2 ESCs to exit game
                    match self.run_state {
                        RunState::Stopped => {
                            Logger::info("Game exiting");
                            self.run_state = RunState::Exiting;
                        }
                        _ => {
                            Logger::info("Game stopped");
                            self.run_state = RunState::Stopped;
                        }
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => match self.run_state {
                    RunState::Paused => {
                        Logger::info("Game unpaused");
                        self.run_state = RunState::Running;
                    }
                    RunState::Running => {
                        Logger::info("Game paused");
                        self.run_state = RunState::Paused;
                    }
                    _ => {}
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Semicolon),
                    ..
                } => match self.run_state {
                    RunState::Stopped => {
                        Logger::info("Game resuming");
                        self.run_state = RunState::Resuming;
                    }
                    RunState::Paused | RunState::Running => {
                        Logger::info("Game stopped");
                        self.run_state = RunState::Stopped;
                    }
                    _ => {
                        Logger::dbg("Cannot stop game while it is in process of resuming");
                    }
                },
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    self.is_debug_mode = !self.is_debug_mode;
                    let mode = if self.is_debug_mode { "ON" } else { "OFF" };
                    Logger::dbg(&format!("Debug mode {}", mode));
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {
        self.tick_loop();

        // TODO Update Systems
        // TODO Update Registry
    }

    fn tick_loop(&mut self) {
        let time_to_wait: f64 = FRAME_LIMIT_MS
            - Instant::now()
                .duration_since(self.ms_prev_frame)
                .as_millis() as f64;

        // fixed frame rate: if below threshold MILLISECS_PER_FRAME then sleep
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
        self.fps_queue
            .push(dt.to_owned().as_millis().try_into().unwrap()); // dt to millis is u128
        self.fps = self.fps_queue.avg().unwrap_or(0f64);
    }

    pub fn render(&mut self) {
        self.canvas.set_draw_color(Color::RGB(0, 255, 0));
        self.canvas.clear();
        self.canvas.present();
    }
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
