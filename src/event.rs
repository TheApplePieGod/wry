#[cfg(target_os = "linux")]
use gtk::gdk::{EventButton, EventKey, EventMotion, EventScroll, ModifierType};
#[cfg(target_os = "macos")]
use objc2_app_kit::NSEvent;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug, Copy, Clone)]
pub enum InputEvent {
  KeyDown {
    key: Key,
    modifiers: KeyModifiers,
  },
  KeyUp {
    key: Key,
    modifiers: KeyModifiers,
  },
  MouseDown {
    button: MouseButton,
    modifiers: KeyModifiers,
  },
  MouseUp {
    button: MouseButton,
    modifiers: KeyModifiers,
  },
  MouseMoved {
    location_in_webview: dpi::Position,
    location_in_parent: dpi::Position,
    modifiers: KeyModifiers,
  },
  ScrollWheel {
    delta_x: f64,
    delta_y: f64,
    modifiers: KeyModifiers,
  },
}

#[derive(Debug, Copy, Clone)]
pub struct KeyModifiers {
  pub shift: bool,
  pub control: bool,
  pub alt: bool,
  pub command: bool,
}

#[repr(i32)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum Key {
  Space,
  Quote,
  Comma,
  Minus,
  Period,
  Slash,
  Semicolon,
  Equal,
  Digit0,
  Digit1,
  Digit2,
  Digit3,
  Digit4,
  Digit5,
  Digit6,
  Digit7,
  Digit8,
  Digit9,
  A,
  B,
  C,
  D,
  E,
  F,
  G,
  H,
  I,
  J,
  K,
  L,
  M,
  N,
  O,
  P,
  Q,
  R,
  S,
  T,
  U,
  V,
  W,
  X,
  Y,
  Z,
  LeftBracket,
  Backslash,
  RightBracket,
  Backquote,
  Escape,
  Enter,
  Tab,
  Backspace,
  Insert,
  Delete,
  ArrowRight,
  ArrowLeft,
  ArrowDown,
  ArrowUp,
  PageUp,
  PageDown,
  Home,
  End,
  CapsLock,
  ScrollLock,
  NumLock,
  PrintScreen,
  Pause,
  F1,
  F2,
  F3,
  F4,
  F5,
  F6,
  F7,
  F8,
  F9,
  F10,
  F11,
  F12,
  Numpad0,
  Numpad1,
  Numpad2,
  Numpad3,
  Numpad4,
  Numpad5,
  Numpad6,
  Numpad7,
  Numpad8,
  Numpad9,
  NumpadDecimal,
  NumpadDivide,
  NumpadMultiply,
  NumpadSubtract,
  NumpadAdd,
  NumpadEnter,
  NumpadEqual,
  ShiftLeft,
  ControlLeft,
  AltLeft,
  SuperLeft,
  ShiftRight,
  ControlRight,
  AltRight,
  SuperRight,
}

#[derive(Debug, Copy, Clone)]
pub enum MouseButton {
  Left,
  Right,
  Other(u16),
}

impl Key {
  #[cfg(target_os = "macos")]
  pub fn from_keycode(code: u16) -> Option<Self> {
    match code {
      // Letters
      0x00 => Some(Key::A),
      0x0B => Some(Key::B),
      0x08 => Some(Key::C),
      0x02 => Some(Key::D),
      0x0E => Some(Key::E),
      0x03 => Some(Key::F),
      0x05 => Some(Key::G),
      0x04 => Some(Key::H),
      0x22 => Some(Key::I),
      0x26 => Some(Key::J),
      0x28 => Some(Key::K),
      0x25 => Some(Key::L),
      0x2E => Some(Key::M),
      0x2D => Some(Key::N),
      0x1F => Some(Key::O),
      0x23 => Some(Key::P),
      0x0C => Some(Key::Q),
      0x0F => Some(Key::R),
      0x01 => Some(Key::S),
      0x11 => Some(Key::T),
      0x20 => Some(Key::U),
      0x09 => Some(Key::V),
      0x0D => Some(Key::W),
      0x07 => Some(Key::X),
      0x10 => Some(Key::Y),
      0x06 => Some(Key::Z),

      // Numbers
      0x12 => Some(Key::Digit1),
      0x13 => Some(Key::Digit2),
      0x14 => Some(Key::Digit3),
      0x15 => Some(Key::Digit4),
      0x17 => Some(Key::Digit5),
      0x16 => Some(Key::Digit6),
      0x1A => Some(Key::Digit7),
      0x1C => Some(Key::Digit8),
      0x19 => Some(Key::Digit9),
      0x1D => Some(Key::Digit0),

      // Special chars
      0x29 => Some(Key::Semicolon),
      0x27 => Some(Key::Quote),
      0x32 => Some(Key::Backquote),
      0x2B => Some(Key::Comma),
      0x2C => Some(Key::Slash),
      0x2F => Some(Key::Period),
      0x18 => Some(Key::Equal),
      0x1B => Some(Key::Minus),
      0x21 => Some(Key::LeftBracket),
      0x1E => Some(Key::RightBracket),
      0x2A => Some(Key::Backslash),

      // Whitespace / control
      0x31 => Some(Key::Space),
      0x24 => Some(Key::Enter),
      0x30 => Some(Key::Tab),
      0x33 => Some(Key::Backspace),
      0x35 => Some(Key::Escape),

      // Navigation
      0x7B => Some(Key::ArrowLeft),
      0x7C => Some(Key::ArrowRight),
      0x7E => Some(Key::ArrowUp),
      0x7D => Some(Key::ArrowDown),
      0x73 => Some(Key::Home),
      0x77 => Some(Key::End),
      0x74 => Some(Key::PageUp),
      0x79 => Some(Key::PageDown),
      0x72 => Some(Key::Insert),
      0x75 => Some(Key::Delete),

      // Modifiers
      0x38 => Some(Key::ShiftLeft),
      0x3C => Some(Key::ShiftRight),
      0x3B => Some(Key::ControlLeft),
      0x3E => Some(Key::ControlRight),
      0x3A => Some(Key::AltLeft),
      0x3D => Some(Key::AltRight),
      0x37 => Some(Key::SuperLeft),
      0x36 => Some(Key::SuperRight),
      0x39 => Some(Key::CapsLock),

      // Function
      0x7A => Some(Key::F1),
      0x78 => Some(Key::F2),
      0x63 => Some(Key::F3),
      0x76 => Some(Key::F4),
      0x60 => Some(Key::F5),
      0x61 => Some(Key::F6),
      0x62 => Some(Key::F7),
      0x64 => Some(Key::F8),
      0x65 => Some(Key::F9),
      0x6D => Some(Key::F10),
      0x67 => Some(Key::F11),
      0x6F => Some(Key::F12),

      _ => None,
    }
  }

  #[cfg(target_os = "windows")]
  pub fn from_keycode(code: u16) -> Option<Self> {
    match code {
      // Letters
      0x41 => Some(Key::A),
      0x42 => Some(Key::B),
      0x43 => Some(Key::C),
      0x44 => Some(Key::D),
      0x45 => Some(Key::E),
      0x46 => Some(Key::F),
      0x47 => Some(Key::G),
      0x48 => Some(Key::H),
      0x49 => Some(Key::I),
      0x4A => Some(Key::J),
      0x4B => Some(Key::K),
      0x4C => Some(Key::L),
      0x4D => Some(Key::M),
      0x4E => Some(Key::N),
      0x4F => Some(Key::O),
      0x50 => Some(Key::P),
      0x51 => Some(Key::Q),
      0x52 => Some(Key::R),
      0x53 => Some(Key::S),
      0x54 => Some(Key::T),
      0x55 => Some(Key::U),
      0x56 => Some(Key::V),
      0x57 => Some(Key::W),
      0x58 => Some(Key::X),
      0x59 => Some(Key::Y),
      0x5A => Some(Key::Z),

      // Numbers (top row)
      0x30 => Some(Key::Digit0),
      0x31 => Some(Key::Digit1),
      0x32 => Some(Key::Digit2),
      0x33 => Some(Key::Digit3),
      0x34 => Some(Key::Digit4),
      0x35 => Some(Key::Digit5),
      0x36 => Some(Key::Digit6),
      0x37 => Some(Key::Digit7),
      0x38 => Some(Key::Digit8),
      0x39 => Some(Key::Digit9),

      // Special chars
      0xBA => Some(Key::Semicolon),    // ;
      0xDE => Some(Key::Quote),        // '
      0xC0 => Some(Key::Backquote),    // `
      0xBC => Some(Key::Comma),        // ,
      0xBF => Some(Key::Slash),        // /
      0xBE => Some(Key::Period),       // .
      0xBD => Some(Key::Minus),        // -
      0xBB => Some(Key::Equal),        // =
      0xDB => Some(Key::LeftBracket),  // [
      0xDD => Some(Key::RightBracket), // ]
      0xDC => Some(Key::Backslash),    // \

      // Whitespace / control
      0x20 => Some(Key::Space),
      0x0D => Some(Key::Enter),
      0x09 => Some(Key::Tab),
      0x08 => Some(Key::Backspace),
      0x1B => Some(Key::Escape),

      // Navigation
      0x25 => Some(Key::ArrowLeft),
      0x27 => Some(Key::ArrowRight),
      0x26 => Some(Key::ArrowUp),
      0x28 => Some(Key::ArrowDown),
      0x24 => Some(Key::Home),
      0x23 => Some(Key::End),
      0x21 => Some(Key::PageUp),
      0x22 => Some(Key::PageDown),
      0x2D => Some(Key::Insert),
      0x2E => Some(Key::Delete),

      // Modifiers
      0xA0 => Some(Key::ShiftLeft),
      0xA1 => Some(Key::ShiftRight),
      0xA2 => Some(Key::ControlLeft),
      0xA3 => Some(Key::ControlRight),
      0xA4 => Some(Key::AltLeft),
      0xA5 => Some(Key::AltRight),
      0x5B => Some(Key::SuperLeft),
      0x5C => Some(Key::SuperRight),
      0x14 => Some(Key::CapsLock),
      0x90 => Some(Key::NumLock),
      0x91 => Some(Key::ScrollLock),

      // Function keys
      0x70 => Some(Key::F1),
      0x71 => Some(Key::F2),
      0x72 => Some(Key::F3),
      0x73 => Some(Key::F4),
      0x74 => Some(Key::F5),
      0x75 => Some(Key::F6),
      0x76 => Some(Key::F7),
      0x77 => Some(Key::F8),
      0x78 => Some(Key::F9),
      0x79 => Some(Key::F10),
      0x7A => Some(Key::F11),
      0x7B => Some(Key::F12),

      // Keypad
      0x60 => Some(Key::Numpad0),
      0x61 => Some(Key::Numpad1),
      0x62 => Some(Key::Numpad2),
      0x63 => Some(Key::Numpad3),
      0x64 => Some(Key::Numpad4),
      0x65 => Some(Key::Numpad5),
      0x66 => Some(Key::Numpad6),
      0x67 => Some(Key::Numpad7),
      0x68 => Some(Key::Numpad8),
      0x69 => Some(Key::Numpad9),
      0x6E => Some(Key::NumpadDecimal),
      0x6F => Some(Key::NumpadDivide),
      0x6A => Some(Key::NumpadMultiply),
      0x6D => Some(Key::NumpadSubtract),
      0x6B => Some(Key::NumpadAdd),
      //0x0D => Some(Key::NumpadEnter),
      0x92 => Some(Key::PrintScreen),
      0x13 => Some(Key::Pause),

      _ => None,
    }
  }

  #[cfg(target_os = "linux")]
  pub fn from_keycode(code: u16) -> Option<Self> {
    match code {
      // Letters
      38 => Some(Key::A),
      56 => Some(Key::B),
      54 => Some(Key::C),
      40 => Some(Key::D),
      26 => Some(Key::E),
      41 => Some(Key::F),
      42 => Some(Key::G),
      43 => Some(Key::H),
      31 => Some(Key::I),
      44 => Some(Key::J),
      45 => Some(Key::K),
      46 => Some(Key::L),
      58 => Some(Key::M),
      57 => Some(Key::N),
      32 => Some(Key::O),
      33 => Some(Key::P),
      24 => Some(Key::Q),
      27 => Some(Key::R),
      39 => Some(Key::S),
      28 => Some(Key::T),
      30 => Some(Key::U),
      55 => Some(Key::V),
      25 => Some(Key::W),
      53 => Some(Key::X),
      29 => Some(Key::Y),
      52 => Some(Key::Z),

      // Numbers (top row)
      10 => Some(Key::Digit1),
      11 => Some(Key::Digit2),
      12 => Some(Key::Digit3),
      13 => Some(Key::Digit4),
      14 => Some(Key::Digit5),
      15 => Some(Key::Digit6),
      16 => Some(Key::Digit7),
      17 => Some(Key::Digit8),
      18 => Some(Key::Digit9),
      19 => Some(Key::Digit0),

      // Special characters (US QWERTY positions)
      34 => Some(Key::LeftBracket),  // [
      35 => Some(Key::RightBracket), // ]
      51 => Some(Key::Backslash),    // \
      49 => Some(Key::Backquote),    // `
      47 => Some(Key::Semicolon),    // ;
      48 => Some(Key::Quote),        // '
      20 => Some(Key::Minus),        // -
      21 => Some(Key::Equal),        // =
      59 => Some(Key::Comma),        // ,
      60 => Some(Key::Period),       // .
      61 => Some(Key::Slash),        // /

      // Whitespace
      65 => Some(Key::Space),
      36 => Some(Key::Enter),
      22 => Some(Key::Backspace),
      23 => Some(Key::Tab),
      9 => Some(Key::Escape),

      // Navigation
      113 => Some(Key::ArrowLeft),
      114 => Some(Key::ArrowRight),
      111 => Some(Key::ArrowUp),
      116 => Some(Key::ArrowDown),
      110 => Some(Key::Home),
      115 => Some(Key::End),
      112 => Some(Key::PageUp),
      117 => Some(Key::PageDown),
      118 => Some(Key::Insert),
      119 => Some(Key::Delete),

      // Modifiers
      50 => Some(Key::ShiftLeft),
      62 => Some(Key::ShiftRight),
      37 => Some(Key::ControlLeft),
      105 => Some(Key::ControlRight),
      64 => Some(Key::AltLeft),
      108 => Some(Key::AltRight),
      133 => Some(Key::SuperLeft),
      134 => Some(Key::SuperRight),
      66 => Some(Key::CapsLock),

      // Function keys
      67 => Some(Key::F1),
      68 => Some(Key::F2),
      69 => Some(Key::F3),
      70 => Some(Key::F4),
      71 => Some(Key::F5),
      72 => Some(Key::F6),
      73 => Some(Key::F7),
      74 => Some(Key::F8),
      75 => Some(Key::F9),
      76 => Some(Key::F10),
      95 => Some(Key::F11),
      96 => Some(Key::F12),

      // Lock keys & misc
      78 => Some(Key::ScrollLock),
      77 => Some(Key::NumLock),
      107 => Some(Key::PrintScreen),
      127 => Some(Key::Pause),

      // Keypad
      90 => Some(Key::Numpad0),
      87 => Some(Key::Numpad1),
      88 => Some(Key::Numpad2),
      89 => Some(Key::Numpad3),
      83 => Some(Key::Numpad4),
      84 => Some(Key::Numpad5),
      85 => Some(Key::Numpad6),
      79 => Some(Key::Numpad7),
      80 => Some(Key::Numpad8),
      81 => Some(Key::Numpad9),
      91 => Some(Key::NumpadDecimal),
      63 => Some(Key::NumpadMultiply),
      82 => Some(Key::NumpadSubtract),
      86 => Some(Key::NumpadAdd),
      104 => Some(Key::NumpadEnter),
      106 => Some(Key::NumpadEqual),

      _ => None,
    }
  }
}

impl InputEvent {
  #[cfg(target_os = "macos")]
  pub fn from_ns_event(event: &NSEvent, webview: &crate::WryWebView) -> Option<Self> {
    use objc2_app_kit::{NSEventModifierFlags, NSEventType};

    let event_type = unsafe { event.r#type() };
    let modifier_flags = unsafe { event.modifierFlags() };

    let modifiers = KeyModifiers {
      shift: modifier_flags.contains(NSEventModifierFlags::Shift),
      control: modifier_flags.contains(NSEventModifierFlags::Control),
      alt: modifier_flags.contains(NSEventModifierFlags::Option),
      command: modifier_flags.contains(NSEventModifierFlags::Command),
    };

    match event_type {
      NSEventType::KeyDown => {
        let key_code = unsafe { event.keyCode() };

        Some(InputEvent::KeyDown {
          key: Key::from_keycode(key_code)?,
          modifiers,
        })
      }
      NSEventType::KeyUp => {
        let key_code = unsafe { event.keyCode() };

        Some(InputEvent::KeyUp {
          key: Key::from_keycode(key_code)?,
          modifiers,
        })
      }
      NSEventType::LeftMouseDown => Some(InputEvent::MouseDown {
        button: MouseButton::Left,
        modifiers,
      }),
      NSEventType::LeftMouseUp => Some(InputEvent::MouseUp {
        button: MouseButton::Left,
        modifiers,
      }),
      NSEventType::RightMouseDown => Some(InputEvent::MouseDown {
        button: MouseButton::Right,
        modifiers,
      }),
      NSEventType::RightMouseUp => Some(InputEvent::MouseUp {
        button: MouseButton::Right,
        modifiers,
      }),
      NSEventType::OtherMouseDown => {
        let button_number = unsafe { event.buttonNumber() } as u16;
        Some(InputEvent::MouseDown {
          button: MouseButton::Other(button_number),
          modifiers,
        })
      }
      NSEventType::OtherMouseUp => {
        let button_number = unsafe { event.buttonNumber() } as u16;
        Some(InputEvent::MouseUp {
          button: MouseButton::Other(button_number),
          modifiers,
        })
      }
      NSEventType::MouseMoved
      | NSEventType::LeftMouseDragged
      | NSEventType::RightMouseDragged
      | NSEventType::OtherMouseDragged => {
        let location_in_window = unsafe { event.locationInWindow() };
        let location_in_parent = unsafe {
          // Window has inverted y coordinate so need to invert
          let mtm = objc2::MainThreadMarker::new().unwrap();
          let window = event.window(mtm).unwrap();
          let frame = window.frame();
          objc2_core_foundation::CGPoint {
            x: location_in_window.x,
            y: frame.size.height - location_in_window.y,
          }
        };
        let location_in_view = webview.convertPoint_fromView(location_in_window, None);
        Some(InputEvent::MouseMoved {
          location_in_webview: dpi::LogicalPosition::new(location_in_view.x, location_in_view.y)
            .into(),
          location_in_parent: dpi::LogicalPosition::new(location_in_parent.x, location_in_parent.y)
            .into(),
          modifiers,
        })
      }
      NSEventType::ScrollWheel => {
        let delta_x = unsafe { event.scrollingDeltaX() };
        let delta_y = unsafe { event.scrollingDeltaY() };
        Some(InputEvent::ScrollWheel {
          delta_x,
          delta_y,
          modifiers,
        })
      }
      NSEventType::FlagsChanged => {
        let key_code = unsafe { event.keyCode() };
        let key = Key::from_keycode(key_code)?;

        let is_pressed = match key_code {
          0x38 | 0x3C => modifiers.shift,   // Left/Right Shift
          0x3B | 0x3E => modifiers.control, // Left/Right Control
          0x3A | 0x3D => modifiers.alt,     // Left/Right Alt/Option
          0x37 | 0x36 => modifiers.command, // Left/Right Command/Super
          0x39 => modifiers.shift,          // CapsLock (treated as shift modifier)
          _ => return None,                 // Unknown modifier key
        };

        Some(if is_pressed {
          InputEvent::KeyDown { key, modifiers }
        } else {
          InputEvent::KeyUp { key, modifiers }
        })
      }
      _ => None,
    }
  }

  #[cfg(target_os = "windows")]
  pub fn from_windows_message(parent_pos: Option<(f64, f64)>, msg: u32, wparam: usize, lparam: isize) -> Option<Self> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;

    let modifiers = KeyModifiers {
      shift: unsafe { GetKeyState(VK_SHIFT.0 as i32) } < 0,
      control: unsafe { GetKeyState(VK_CONTROL.0 as i32) } < 0,
      alt: unsafe { GetKeyState(VK_MENU.0 as i32) } < 0,
      command: false, // Windows doesn't have a command key
    };

    match msg {
      WM_KEYDOWN | WM_SYSKEYDOWN => {
        let key_code = wparam as u16;
        Some(InputEvent::KeyDown {
          key: Key::from_keycode(key_code)?,
          modifiers,
        })
      }
      WM_KEYUP | WM_SYSKEYUP => {
        let key_code = wparam as u16;
        Some(InputEvent::KeyUp {
          key: Key::from_keycode(key_code)?,
          modifiers,
        })
      }
      WM_LBUTTONDOWN => {
        Some(InputEvent::MouseDown {
          button: MouseButton::Left,
          modifiers,
        })
      }
      WM_LBUTTONUP => {
        Some(InputEvent::MouseUp {
          button: MouseButton::Left,
          modifiers,
        })
      }
      WM_RBUTTONDOWN => {
        Some(InputEvent::MouseDown {
          button: MouseButton::Right,
          modifiers,
        })
      }
      WM_RBUTTONUP => {
        Some(InputEvent::MouseUp {
          button: MouseButton::Right,
          modifiers,
        })
      }
      WM_MBUTTONDOWN => {
        Some(InputEvent::MouseDown {
          button: MouseButton::Other(2), // Middle button
          modifiers,
        })
      }
      WM_MBUTTONUP => {
        Some(InputEvent::MouseUp {
          button: MouseButton::Other(2),
          modifiers,
        })
      }
      WM_MOUSEMOVE => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        let parent_pos = match parent_pos {
          Some(pos) => pos,
          None => (x, y),
        };
        Some(InputEvent::MouseMoved {
          location_in_parent: dpi::LogicalPosition::new(parent_pos.0, parent_pos.1).into(),
          location_in_webview: dpi::LogicalPosition::new(x, y).into(),
          modifiers,
        })
      }
      WM_MOUSEWHEEL => {
        let delta = ((wparam >> 16) & 0xFFFF) as i16 as f64 / 120.0; // WHEEL_DELTA is 120
        Some(InputEvent::ScrollWheel {
          delta_x: 0.0,
          delta_y: delta,
          modifiers,
        })
      }
      WM_MOUSEHWHEEL => {
        let delta = ((wparam >> 16) & 0xFFFF) as i16 as f64 / 120.0;
        Some(InputEvent::ScrollWheel {
          delta_x: delta,
          delta_y: 0.0,
          modifiers,
        })
      }
      _ => None,
    }
  }

  #[cfg(target_os = "linux")]
  pub fn from_gdk_event_key(event: &EventKey) -> Option<Self> {
    let keyval = event.hardware_keycode();
    let modifiers = event.state();

    let key_modifiers = KeyModifiers {
      shift: modifiers.contains(ModifierType::SHIFT_MASK),
      control: modifiers.contains(ModifierType::CONTROL_MASK),
      alt: modifiers.contains(ModifierType::MOD1_MASK),
      command: modifiers.contains(ModifierType::SUPER_MASK),
    };

    match event.event_type() {
      gtk::gdk::EventType::KeyPress => Some(InputEvent::KeyDown {
        key: Key::from_keycode(keyval)?,
        modifiers: key_modifiers,
      }),
      gtk::gdk::EventType::KeyRelease => Some(InputEvent::KeyUp {
        key: Key::from_keycode(keyval)?,
        modifiers: key_modifiers,
      }),
      _ => None,
    }
  }

  #[cfg(target_os = "linux")]
  pub fn from_gdk_event_button(event: &EventButton) -> Option<Self> {
    let modifiers = event.state();
    let (x, y) = event.position();

    let key_modifiers = KeyModifiers {
      shift: modifiers.contains(ModifierType::SHIFT_MASK),
      control: modifiers.contains(ModifierType::CONTROL_MASK),
      alt: modifiers.contains(ModifierType::MOD1_MASK),
      command: modifiers.contains(ModifierType::SUPER_MASK),
    };

    let button = match event.button() {
      1 => MouseButton::Left,
      2 => MouseButton::Other(2), // Middle button
      3 => MouseButton::Right,
      other => MouseButton::Other(other as u16),
    };

    match event.event_type() {
      gtk::gdk::EventType::ButtonPress => Some(InputEvent::MouseDown {
        button,
        location: (x, y),
        modifiers: key_modifiers,
      }),
      gtk::gdk::EventType::ButtonRelease => Some(InputEvent::MouseUp {
        button,
        location: (x, y),
        modifiers: key_modifiers,
      }),
      _ => None,
    }
  }

  #[cfg(target_os = "linux")]
  pub fn from_gdk_event_motion(event: &EventMotion) -> Option<Self> {
    let modifiers = event.state();
    let (x, y) = event.position();

    let key_modifiers = KeyModifiers {
      shift: modifiers.contains(ModifierType::SHIFT_MASK),
      control: modifiers.contains(ModifierType::CONTROL_MASK),
      alt: modifiers.contains(ModifierType::MOD1_MASK),
      command: modifiers.contains(ModifierType::SUPER_MASK),
    };

    Some(InputEvent::MouseMoved {
      location: (x, y),
      modifiers: key_modifiers,
    })
  }

  #[cfg(target_os = "linux")]
  pub fn from_gdk_event_scroll(event: &EventScroll) -> Option<Self> {
    let modifiers = event.state();
    let (x, y) = event.position();
    let (delta_x, delta_y) = event.delta();

    let key_modifiers = KeyModifiers {
      shift: modifiers.contains(ModifierType::SHIFT_MASK),
      control: modifiers.contains(ModifierType::CONTROL_MASK),
      alt: modifiers.contains(ModifierType::MOD1_MASK),
      command: modifiers.contains(ModifierType::SUPER_MASK),
    };

    Some(InputEvent::ScrollWheel {
      delta_x,
      delta_y,
      location: (x, y),
      modifiers: key_modifiers,
    })
  }
}
