use std::fs;
use std::path::PathBuf;
use buffer_graphics_lib::color::{BLACK, WHITE};
use buffer_graphics_lib::Graphics;
use clap::{arg, command, value_parser};
use color_eyre::eyre::eyre;
use color_eyre::Result;
use ec8_common::{MAX_X, MAX_Y};
use ec8_core::input::Key;
use ec8_core::EmmaChip8;
use env_logger::Builder;
use log::{LevelFilter, warn};
use pixels_graphics_lib::{run, System, WindowScaling};
use winit::event::VirtualKeyCode;
use winit::event::VirtualKeyCode::*;
use clap::ValueHint::FilePath;
use ec8_core::State::Running;


const RUN_RATE: f32 = 0.001;
const TIMER_UPDATE_RATE: f32 = 1.0 / 60.0;

struct EC8Hardware {
    ec8: EmmaChip8,
    next_run: f32,
    next_timer: f32,
}

impl EC8Hardware {
    pub fn new() -> Self {
        Self {
            ec8: EmmaChip8::new(),
            next_run: 0.0,
            next_timer: 0.0,
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;
    setup_logging();

    let matches = command!()
        .arg(
            arg!([INPUT_FILE] "EC8 file (*.c8)")
                .required(true)
                .value_hint(FilePath)
                .value_parser(value_parser!(PathBuf)),
        )
        .get_matches();

    let input_file = matches
        .get_one::<PathBuf>("INPUT_FILE")
        .cloned()
        .expect("Program file must be provided");
    if !input_file.is_file() {
        return Err(eyre!(format!(
            "Program file {} is not a file",
            input_file.display()
        )));
    }

    let bytes = fs::read(input_file)?;

    let mut system = Box::new(EC8Hardware::new());
    system.ec8.load_program(&bytes)?;
    run(
        MAX_X,
        MAX_Y,
        WindowScaling::AutoFixed(6),
        "EmmaChip8",
        system,
    )?;
    Ok(())
}

fn setup_logging() {
    Builder::from_default_env()
        .format_module_path(false)
        .format_level(false)
        .format_target(false)
        .format_timestamp(None)
        .filter_level(LevelFilter::Warn)
        .filter(Some("ec8_core"), LevelFilter::Info)
        .init();
}

impl System for EC8Hardware {
    fn update(&mut self, delta: f32) {
        if self.ec8.state == Running {
            if self.next_run <= 0.0 {
                self.ec8.run();
                if self.ec8.state != Running {
                    warn!("{:?}", self.ec8.state);
                }
                self.next_run = RUN_RATE;
            } else {
                self.next_run -= delta;
            }
            if self.next_timer <= 0.0 {
                self.ec8.delay = self.ec8.delay.saturating_sub(1);
                self.ec8.sound = self.ec8.sound.saturating_sub(1);
                self.next_timer = TIMER_UPDATE_RATE;
            } else {
                self.next_timer -= delta;
            }
        }
    }

    fn render(&self, graphics: &mut Graphics) {
        // graphics.clear(BLACK);
        for x in 0..MAX_X {
            for y in 0..MAX_Y {
                let i = y * MAX_X + x;
                let color = if self.ec8.output[i] { WHITE } else { BLACK };
                graphics.set_pixel(x as isize, y as isize, color)
            }
        }
    }

    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![Key1, Key2, Key3, Key4, Q, W, E, R, A, S, D, F, Z, X, C, V]
    }

    fn on_key_down(&mut self, keys: Vec<VirtualKeyCode>) {
        for key in keys {
            if let Some(key) = Key::from_lefthand_layout(key_to_chr(key)) {
                self.ec8.on_key_pressed(key);
            }
        }
    }

    fn on_key_up(&mut self, keys: Vec<VirtualKeyCode>) {
        for key in keys {
            if let Some(key) = Key::from_lefthand_layout(key_to_chr(key)) {
                self.ec8.on_key_pressed(key);
            }
        }
    }
}

fn key_to_chr(code: VirtualKeyCode) -> char {
    match code {
        Key1 => '1',
        Key2 => '2',
        Key3 => '3',
        Key4 => '4',
        Q => 'q',
        W => 'w',
        E => 'e',
        R => 'r',
        A => 'a',
        S => 's',
        D => 'd',
        F => 'f',
        Z => 'z',
        X => 'x',
        C => 'c',
        V => 'v',
        _ => ' ',
    }
}
