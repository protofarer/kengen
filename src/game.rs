use anyhow::{anyhow, Context, Result};
use sdl2::{
    event::Event, keyboard::Keycode, pixels::Color, render::Canvas, version::revision,
    video::Window, EventPump,
};
use std::env;
use std::time::{Duration, Instant};

use crate::dsa::FixedSizeQueue;
use crate::ecs::Registry;
use crate::logger::{LogLevel, Logger};

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

pub struct GameConfiguration;
pub struct Game {
    pub run_state: RunState,
    canvas: Canvas<Window>,
    event_pump: EventPump,
    ms_prev_frame: Instant,
    fps: f64,
    fps_queue: FixedSizeQueue,
    is_debug_on: bool,
}

impl Game {
    pub fn new() -> Result<Self, anyhow::Error> {
        // todo 1. pass config struct
        // todo 2. let game init/new parse readline
        // todo 3. pass both, then readline args override config struct
        // todo 4. read a toml config file that can be overriden by readline

        let readline_args = parse_readline().unwrap_or_else(|e| {
            eprintln!("{e}");
            std::process::exit(1);
        });

        match readline_args.log_level {
            Some(level) => Logger::new(level, None),
            None => Logger::new(LogLevel::Debug, None),
        };

        Logger::dbg("Initializing game");
        Logger::info(&format!("{}", revision())); // SDL version

        let sdl_context = sdl2::init()
            .map_err(|e| anyhow::anyhow!("{}", e)) // convert init's result error type of String into an anyhow error
            // .map_err(|e| anyhow::anyhow) // shorthand here
            .with_context(|| "Failed to initialize SDL context".to_owned())?;

        let video_subsystem = sdl_context
            .video()
            .map_err(|e| anyhow::anyhow!("{}", e))
            .with_context(|| "Failed to initialize video subsystem".to_owned())?;

        let window_width = WINDOW_WIDTH;
        let window_height = WINDOW_HEIGHT;
        let window = video_subsystem
            .window("kengen", window_width, window_height)
            .position_centered()
            .build()
            .map_err(|e| anyhow::anyhow!("{}", e))
            .with_context(|| "Failed to create window".to_owned())?;

        let canvas = window
            .into_canvas()
            .build()
            .map_err(|e| anyhow::anyhow!("{}", e))
            .with_context(|| "Failed to create canvas".to_owned())?;

        let event_pump = sdl_context
            .event_pump()
            .map_err(|e| anyhow::anyhow!("{}", e))
            .with_context(|| "Failed to create event pump".to_owned())?;

        Logger::dbg("Finished initializing game");

        Ok(Self {
            run_state: RunState::Stopped,
            ms_prev_frame: Instant::now(),
            canvas,
            event_pump,
            fps: 0.0,
            fps_queue: FixedSizeQueue::new(60),
            is_debug_on: false,
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
        Logger::dbg("Game loop running");
        loop {
            self.handle_input();

            // how badly does this slow down the frame? Should branching/tests be averted and existential state only be run? How? Stick it in handle_input, to break rather than check for state? Should it be an event to pause/resume/stop?
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

            self.handle_tick();
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
                    self.is_debug_on = !self.is_debug_on;
                    let mode = if self.is_debug_on { "ON" } else { "OFF" };
                    Logger::dbg(&format!("Debug mode {}", mode));
                }
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {

        // TODO Update Systems
        // TODO Update Registry
    }

    fn handle_tick(&mut self) {
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

struct ReadlineArgs {
    log_level: Option<LogLevel>,
    log_output: Option<String>,
}

fn parse_readline() -> Result<ReadlineArgs, anyhow::Error> {
    // Usage: kengen [--loglevel | -l <LogLevel word>] [--logoutput | -o <log_file>]
    let mut args = env::args().peekable();
    let mut readline_args = ReadlineArgs {
        log_level: None,
        log_output: None,
    };

    // if malformed arg value, throw
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-l" | "--loglevel" => {
                if let Some(log_level_arg) = args.next() {
                    readline_args.log_level = Some(parse_log_level_value(&log_level_arg)?);
                } else {
                    return Err(anyhow!("Missing argument for log level"));
                }
            }
            "-o" | "--logoutput" => {
                if let Some(log_output_arg) = args.next() {
                    readline_args.log_output = Some(log_output_arg.to_owned());
                } else {
                    return Err(anyhow!("Missing argument for log output"));
                }
            }
            _ => {
                println!("Invalid option: {}", arg);
            }
        }
    }
    Ok(readline_args)
}

fn parse_log_level_value(s: &str) -> Result<LogLevel, anyhow::Error> {
    match s {
        "debug" => Ok(LogLevel::Debug),
        "info" => Ok(LogLevel::Info),
        "warning" => Ok(LogLevel::Warning),
        "error" => Ok(LogLevel::Error),
        "critical" => Ok(LogLevel::Critical),
        x => Err(anyhow!("Malformed or empty log level value: {x}")),
    }
}
