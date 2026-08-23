use minifb::{Key::{self}, Scale, ScaleMode, Window, WindowOptions};
use rodio::{DeviceSinkBuilder, MixerDeviceSink, Player, source::SineWave};

use crate::chip8::Chip8;

pub struct Emulator {
    cpu : Chip8,
    window : Window,
    player: Player,
    _audio_stream: MixerDeviceSink
}
const WIDTH: usize = 64;
const HEIGHT: usize = 32;

impl Emulator {
    pub fn new(path : &str) -> Self {
        let cpu:Chip8 = Chip8::load_from_rom(path);
        let mut window =  Window::new(
            "Chip8",
            WIDTH,
            HEIGHT,
            WindowOptions {
                borderless: false,
                transparency: false,
                title: true,
                resize: true,
                scale: Scale::X16,
                scale_mode: ScaleMode::Stretch,
                topmost: false,
                none: false,
            }
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

        // Limit to max ~60 fps update rate
        window.set_target_fps(60);

        let stream = DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = Player::connect_new(&stream.mixer());
        player.append(SineWave::new(440.0));

        return Self {
            cpu : cpu,
            window : window,
            player : player,
            _audio_stream : stream
        }
    }

    /*
    The emulator runs at 60 frames per second (matching the original display refresh rate).
    Each frame: execute 10 CPU instructions (~600 instructions/second), then decrement timers.
    Timers (delay and sound) tick down at 60Hz — once per frame.
     */
    pub fn run(&mut self) {
        let window = &mut self.window;
        let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

        while window.is_open() {
            Self::fill_pressed_key(window, &mut self.cpu.keypad);

            // Run cpu, 
            for _i in 0..10 {
                self.cpu.tick();
            }

            // render dispay
            Self::update_buffer(&mut buffer, self.cpu.display);
            window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();


            // Play sound
            if self.cpu.st > 0 {
                self.player.play();
            }
            else {
                self.player.pause();
            }

            // Decrement timers
            if self.cpu.dt > 0 {
                self.cpu.dt = self.cpu.dt - 1;
            }
            if self.cpu.st > 0 {
                self.cpu.st = self.cpu.st -1;
            }

            
        }
    }

    fn fill_pressed_key(window: &Window, keyboard:&mut[bool; 16]) {
        for i in 0..keyboard.len() {
            keyboard[i] = false;
        }

        let keys:Vec<Key> = vec![Key::Key0, Key::Key1, Key::Key2, Key::Key3, Key::Key4, Key::Key5, Key::Key6, Key::Key7, Key::Key8, Key::Key9, Key::A, Key::B, Key::C, Key::D, Key::E, Key::F, Key::Escape];
        for key in keys {
            if window.is_key_down(key) {
                match key {
                    Key::Key0 => keyboard[0]   = true,
                    Key::Key1 => keyboard[1]   = true,
                    Key::Key2 => keyboard[2]   = true,
                    Key::Key3 => keyboard[3]   = true,
                    Key::Key4 => keyboard[4]   = true,
                    Key::Key5 => keyboard[5]   = true,
                    Key::Key6 => keyboard[6]   = true,
                    Key::Key7 => keyboard[7]   = true,
                    Key::Key8 => keyboard[8]   = true,
                    Key::Key9 => keyboard[9]   = true,
                    Key::A    => keyboard[0xA] = true,
                    Key::B    => keyboard[0xB] = true,
                    Key::C    => keyboard[0xC] = true,
                    Key::D    => keyboard[0xD] = true,
                    Key::E    => keyboard[0xE] = true,
                    Key::F    => keyboard[0xF] = true,
                    Key::Escape =>  std::process::exit(0),
                    _ => (),
                }
            }
        }
    }

    fn update_buffer(buffer: &mut Vec<u32>, arr:[u8; HEIGHT * WIDTH]) {
    for i in 0..arr.len() {
        if arr[i] == 0 {
            buffer[i] = 0;
        }
        else {
            buffer[i] = 0xFFFFFF;
        }
    }
}
}
