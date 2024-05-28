use evdev::{ Device, InputEventKind };
use std::collections::HashMap;
use std::time::Duration;

struct Gamepad {
    joy_device: Option<Device>,
    last: HashMap<&'static str, i32>,
}

impl Gamepad {
    fn new() -> Self {
        let mut gamepad = Gamepad {
            joy_device: None,
            last: [
                ("ABS_X", 0),
                ("ABS_Y", 0),
                ("ABS_RZ", 0),
                ("ABS_Z", 0),
            ]
                .iter()
                .cloned()
                .collect(),
        };
        gamepad.reconnect();
        gamepad
    }

    fn reconnect(&mut self) {
        for (path, _device) in evdev::enumerate() {
            if let Ok(device) = Device::open(&path) {
                if device.name().map_or(false, |name| name == "Your Device Name") {
                    self.joy_device = Some(device);
                    break;
                }
            }
        }
    }

    fn process_events(&mut self) {
        if let Some(device) = &mut self.joy_device {
            for event in device.fetch_events().unwrap() {
                match event.kind() {
                    InputEventKind::Key(key) => {
                        if key == evdev::Key::BTN_0 && event.value() == 1 {
                            // Adjusted to BTN_0
                            println!("Hi");
                        }
                    }
                    InputEventKind::AbsAxis(axis) => {
                        match axis {
                            evdev::AbsoluteAxisType::ABS_HAT0X => {
                                if event.value() == -1 {
                                    println!("left");
                                } else if event.value() == 1 {
                                    println!("right");
                                }
                            }
                            evdev::AbsoluteAxisType::ABS_HAT0Y => {
                                if event.value() == -1 {
                                    println!("forward");
                                } else if event.value() == 1 {
                                    println!("back");
                                }
                            }
                            evdev::AbsoluteAxisType::ABS_X => {
                                self.last.insert("ABS_X", event.value() - 128);
                            }
                            evdev::AbsoluteAxisType::ABS_Y => {
                                self.last.insert("ABS_Y", event.value() - 128);
                            }
                            evdev::AbsoluteAxisType::ABS_Z => {
                                self.last.insert("ABS_Z", event.value() - 128);
                            }
                            evdev::AbsoluteAxisType::ABS_RZ => {
                                self.last.insert("ABS_RZ", event.value() - 128);
                            }
                            _ => {}
                        }

                        for key in &["ABS_X", "ABS_Y", "ABS_Z", "ABS_RZ"] {
                            if let Some(value) = self.last.get_mut(*key) {
                                if value.abs() < 5 {
                                    *value = 0;
                                }
                            }
                        }

                        println!("{:?}", self.last);
                    }
                    _ => {}
                }
            }
        }
    }
}

fn main() {
    let mut gamepad = Gamepad::new();
    while gamepad.joy_device.is_none() {
        gamepad.reconnect();
        std::thread::sleep(Duration::from_secs(1));
    }
    println!("Gamepad connected");

    loop {
        gamepad.process_events();
        std::thread::sleep(Duration::from_millis(10));
    }
}
