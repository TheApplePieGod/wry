#[cfg(target_os = "linux")]
use gtk::gdk::{EventButton, EventKey, EventMotion, EventScroll, ModifierType};
#[cfg(target_os = "macos")]
use objc2_app_kit::NSEvent;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::*;

#[derive(Debug, Clone)]
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
    location: (f64, f64),
    modifiers: KeyModifiers,
  },
  MouseUp {
    button: MouseButton,
    location: (f64, f64),
    modifiers: KeyModifiers,
  },
  MouseMoved {
    location: (f64, f64),
    modifiers: KeyModifiers,
  },
  ScrollWheel {
    delta_x: f64,
    delta_y: f64,
    location: (f64, f64),
    modifiers: KeyModifiers,
  },
}

#[derive(Debug, Clone)]
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
  Apostrophe,
  Comma,
  Minus,
  Period,
  Slash,
  Semicolon,
  Equal,
  Num0,
  Num1,
  Num2,
  Num3,
  Num4,
  Num5,
  Num6,
  Num7,
  Num8,
  Num9,
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
  GraveAccent,
  World1,
  World2,
  Escape,
  Enter,
  Tab,
  Backspace,
  Insert,
  Delete,
  Right,
  Left,
  Down,
  Up,
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
  Kp0,
  Kp1,
  Kp2,
  Kp3,
  Kp4,
  Kp5,
  Kp6,
  Kp7,
  Kp8,
  Kp9,
  KpDecimal,
  KpDivide,
  KpMultiply,
  KpSubtract,
  KpAdd,
  KpEnter,
  KpEqual,
  LeftShift,
  LeftControl,
  LeftAlt,
  LeftSuper,
  RightShift,
  RightControl,
  RightAlt,
  RightSuper,
  Menu,
  Unknown,
}

#[derive(Debug, Clone)]
pub enum MouseButton {
  Left,
  Right,
  Other(i16),
}

impl Key {
  #[cfg(target_os = "macos")]
  pub fn from_keycode(code: u16) -> Self {
    match code {
      // Letters
      0x00 => Key::A,
      0x0B => Key::B,
      0x08 => Key::C,
      0x02 => Key::D,
      0x0E => Key::E,
      0x03 => Key::F,
      0x05 => Key::G,
      0x04 => Key::H,
      0x22 => Key::I,
      0x26 => Key::J,
      0x28 => Key::K,
      0x25 => Key::L,
      0x2E => Key::M,
      0x2D => Key::N,
      0x1F => Key::O,
      0x23 => Key::P,
      0x0C => Key::Q,
      0x0F => Key::R,
      0x01 => Key::S,
      0x11 => Key::T,
      0x20 => Key::U,
      0x09 => Key::V,
      0x0D => Key::W,
      0x07 => Key::X,
      0x10 => Key::Y,
      0x06 => Key::Z,

      // Numbers
      0x12 => Key::Num1,
      0x13 => Key::Num2,
      0x14 => Key::Num3,
      0x15 => Key::Num4,
      0x17 => Key::Num5,
      0x16 => Key::Num6,
      0x1A => Key::Num7,
      0x1C => Key::Num8,
      0x19 => Key::Num9,
      0x1D => Key::Num0,

      // Special chars
      0x27 => Key::Semicolon,
      0x29 => Key::Apostrophe,
      0x2A => Key::GraveAccent,
      0x2B => Key::Comma,
      0x2C => Key::Slash,
      0x2F => Key::Period,
      0x1B => Key::Equal,
      0x18 => Key::Minus,
      0x21 => Key::LeftBracket,
      0x1E => Key::RightBracket,
      0x2A => Key::Backslash,

      // Whitespace / control
      0x31 => Key::Space,
      0x24 => Key::Enter,
      0x30 => Key::Tab,
      0x33 => Key::Backspace,
      0x35 => Key::Escape,

      // Navigation
      0x7B => Key::Left,
      0x7C => Key::Right,
      0x7E => Key::Up,
      0x7D => Key::Down,
      0x73 => Key::Home,
      0x77 => Key::End,
      0x74 => Key::PageUp,
      0x79 => Key::PageDown,
      0x72 => Key::Insert,
      0x75 => Key::Delete,

      // Modifiers
      0x38 => Key::LeftShift,
      0x3C => Key::RightShift,
      0x3B => Key::LeftControl,
      0x3E => Key::RightControl,
      0x3A => Key::LeftAlt,
      0x3D => Key::RightAlt,
      0x37 => Key::LeftSuper,
      0x36 => Key::RightSuper,
      0x39 => Key::CapsLock,

      // Function
      0x7A => Key::F1,
      0x78 => Key::F2,
      0x63 => Key::F3,
      0x76 => Key::F4,
      0x60 => Key::F5,
      0x61 => Key::F6,
      0x62 => Key::F7,
      0x64 => Key::F8,
      0x65 => Key::F9,
      0x6D => Key::F10,
      0x67 => Key::F11,
      0x6F => Key::F12,

      _ => Key::Unknown,
    }
  }

  #[cfg(target_os = "windows")]
  pub fn from_keycode(code: u16) -> Self {
    match code {
      // Letters
      0x41 => Key::A,
      0x42 => Key::B,
      0x43 => Key::C,
      0x44 => Key::D,
      0x45 => Key::E,
      0x46 => Key::F,
      0x47 => Key::G,
      0x48 => Key::H,
      0x49 => Key::I,
      0x4A => Key::J,
      0x4B => Key::K,
      0x4C => Key::L,
      0x4D => Key::M,
      0x4E => Key::N,
      0x4F => Key::O,
      0x50 => Key::P,
      0x51 => Key::Q,
      0x52 => Key::R,
      0x53 => Key::S,
      0x54 => Key::T,
      0x55 => Key::U,
      0x56 => Key::V,
      0x57 => Key::W,
      0x58 => Key::X,
      0x59 => Key::Y,
      0x5A => Key::Z,

      // Numbers (top row)
      0x30 => Key::Num0,
      0x31 => Key::Num1,
      0x32 => Key::Num2,
      0x33 => Key::Num3,
      0x34 => Key::Num4,
      0x35 => Key::Num5,
      0x36 => Key::Num6,
      0x37 => Key::Num7,
      0x38 => Key::Num8,
      0x39 => Key::Num9,

      // Special chars
      0xBA => Key::Semicolon,    // ;
      0xDE => Key::Apostrophe,   // '
      0xC0 => Key::GraveAccent,  // `
      0xBC => Key::Comma,        // ,
      0xBF => Key::Slash,        // /
      0xBE => Key::Period,       // .
      0xBD => Key::Minus,        // -
      0xBB => Key::Equal,        // =
      0xDB => Key::LeftBracket,  // [
      0xDD => Key::RightBracket, // ]
      0xDC => Key::Backslash,    // '\'

      // Whitespace / control
      0x20 => Key::Space,
      0x0D => Key::Enter,
      0x09 => Key::Tab,
      0x08 => Key::Backspace,
      0x1B => Key::Escape,

      // Navigation
      0x25 => Key::Left,
      0x27 => Key::Right,
      0x26 => Key::Up,
      0x28 => Key::Down,
      0x24 => Key::Home,
      0x23 => Key::End,
      0x21 => Key::PageUp,
      0x22 => Key::PageDown,
      0x2D => Key::Insert,
      0x2E => Key::Delete,

      // Modifiers
      0xA0 => Key::LeftShift,
      0xA1 => Key::RightShift,
      0xA2 => Key::LeftControl,
      0xA3 => Key::RightControl,
      0xA4 => Key::LeftAlt,
      0xA5 => Key::RightAlt,
      0x5B => Key::LeftSuper,
      0x5C => Key::RightSuper,
      0x14 => Key::CapsLock,
      0x90 => Key::NumLock,
      0x91 => Key::ScrollLock,

      // Function keys
      0x70 => Key::F1,
      0x71 => Key::F2,
      0x72 => Key::F3,
      0x73 => Key::F4,
      0x74 => Key::F5,
      0x75 => Key::F6,
      0x76 => Key::F7,
      0x77 => Key::F8,
      0x78 => Key::F9,
      0x79 => Key::F10,
      0x7A => Key::F11,
      0x7B => Key::F12,

      // Keypad
      0x60 => Key::Kp0,
      0x61 => Key::Kp1,
      0x62 => Key::Kp2,
      0x63 => Key::Kp3,
      0x64 => Key::Kp4,
      0x65 => Key::Kp5,
      0x66 => Key::Kp6,
      0x67 => Key::Kp7,
      0x68 => Key::Kp8,
      0x69 => Key::Kp9,
      0x6E => Key::KpDecimal,
      0x6F => Key::KpDivide,
      0x6A => Key::KpMultiply,
      0x6D => Key::KpSubtract,
      0x6B => Key::KpAdd,
      0x0D => Key::KpEnter,   // Enter works for keypad too
      0x92 => Key::PrintScreen,
      0x13 => Key::Pause,

      _ => Key::Unknown,
    }
  }

  #[cfg(target_os = "linux")]
  pub fn from_keycode(code: u16) -> Self {
    match code {
      // Letters
      38 => Key::A,
      56 => Key::B,
      54 => Key::C,
      40 => Key::D,
      26 => Key::E,
      41 => Key::F,
      42 => Key::G,
      43 => Key::H,
      31 => Key::I,
      44 => Key::J,
      45 => Key::K,
      46 => Key::L,
      58 => Key::M,
      57 => Key::N,
      32 => Key::O,
      33 => Key::P,
      24 => Key::Q,
      27 => Key::R,
      39 => Key::S,
      28 => Key::T,
      30 => Key::U,
      55 => Key::V,
      25 => Key::W,
      53 => Key::X,
      29 => Key::Y,
      52 => Key::Z,

      // Numbers (top row)
      10 => Key::Num1,
      11 => Key::Num2,
      12 => Key::Num3,
      13 => Key::Num4,
      14 => Key::Num5,
      15 => Key::Num6,
      16 => Key::Num7,
      17 => Key::Num8,
      18 => Key::Num9,
      19 => Key::Num0,

      // Special characters (US QWERTY positions)
      34 => Key::LeftBracket,   // [
      35 => Key::RightBracket,  // ]
      51 => Key::Backslash,     // '\'
      49 => Key::GraveAccent,   // `
      47 => Key::Semicolon,     // ;
      48 => Key::Apostrophe,    // '
      20 => Key::Minus,         // -
      21 => Key::Equal,         // =
      59 => Key::Comma,         // ,
      60 => Key::Period,        // .
      61 => Key::Slash,         // /

      // Whitespace
      65 => Key::Space,
      36 => Key::Enter,
      22 => Key::Backspace,
      23 => Key::Tab,
      9  => Key::Escape,

      // Navigation
      113 => Key::Left,
      114 => Key::Right,
      111 => Key::Up,
      116 => Key::Down,
      110 => Key::Home,
      115 => Key::End,
      112 => Key::PageUp,
      117 => Key::PageDown,
      118 => Key::Insert,
      119 => Key::Delete,

      // Modifiers
      50 => Key::LeftShift,
      62 => Key::RightShift,
      37 => Key::LeftControl,
      105 => Key::RightControl,
      64 => Key::LeftAlt,
      108 => Key::RightAlt,
      133 => Key::LeftSuper,
      134 => Key::RightSuper,
      66 => Key::CapsLock,

      // Function keys
      67 => Key::F1,
      68 => Key::F2,
      69 => Key::F3,
      70 => Key::F4,
      71 => Key::F5,
      72 => Key::F6,
      73 => Key::F7,
      74 => Key::F8,
      75 => Key::F9,
      76 => Key::F10,
      95 => Key::F11,
      96 => Key::F12,

      // Lock keys & misc
      78 => Key::ScrollLock,
      77 => Key::NumLock,
      107 => Key::PrintScreen,
      127 => Key::Pause,

      // Keypad
      90 => Key::Kp0,
      87 => Key::Kp1,
      88 => Key::Kp2,
      89 => Key::Kp3,
      83 => Key::Kp4,
      84 => Key::Kp5,
      85 => Key::Kp6,
      79 => Key::Kp7,
      80 => Key::Kp8,
      81 => Key::Kp9,
      91 => Key::KpDecimal,
      63 => Key::KpMultiply,
      82 => Key::KpSubtract,
      86 => Key::KpAdd,
      104 => Key::KpEnter,
      106 => Key::KpEqual,

      _ => Key::Unknown,
    }
  }
}

impl InputEvent {
  #[cfg(target_os = "macos")]
  pub fn from_ns_event(event: &NSEvent) -> Option<Self> {
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
          key: Key::from_keycode(key_code),
          modifiers,
        })
      }
      NSEventType::KeyUp => {
        let key_code = unsafe { event.keyCode() };

        Some(InputEvent::KeyUp {
          key: Key::from_keycode(key_code),
          modifiers,
        })
      }
      NSEventType::LeftMouseDown => {
        let location_in_window = unsafe { event.locationInWindow() };
        Some(InputEvent::MouseDown {
          button: MouseButton::Left,
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      NSEventType::LeftMouseUp => {
        let location_in_window = unsafe { event.locationInWindow() };
        Some(InputEvent::MouseUp {
          button: MouseButton::Left,
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      NSEventType::RightMouseDown => {
        let location_in_window = unsafe { event.locationInWindow() };
        Some(InputEvent::MouseDown {
          button: MouseButton::Right,
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      NSEventType::RightMouseUp => {
        let location_in_window = unsafe { event.locationInWindow() };
        Some(InputEvent::MouseUp {
          button: MouseButton::Right,
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      NSEventType::OtherMouseDown => {
        let location_in_window = unsafe { event.locationInWindow() };
        let button_number = unsafe { event.buttonNumber() } as i16;
        Some(InputEvent::MouseDown {
          button: MouseButton::Other(button_number),
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      NSEventType::OtherMouseUp => {
        let location_in_window = unsafe { event.locationInWindow() };
        let button_number = unsafe { event.buttonNumber() } as i16;
        Some(InputEvent::MouseUp {
          button: MouseButton::Other(button_number),
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      NSEventType::MouseMoved
      | NSEventType::LeftMouseDragged
      | NSEventType::RightMouseDragged
      | NSEventType::OtherMouseDragged => {
        let location_in_window = unsafe { event.locationInWindow() };
        Some(InputEvent::MouseMoved {
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      NSEventType::ScrollWheel => {
        let location_in_window = unsafe { event.locationInWindow() };
        let delta_x = unsafe { event.scrollingDeltaX() };
        let delta_y = unsafe { event.scrollingDeltaY() };
        Some(InputEvent::ScrollWheel {
          delta_x,
          delta_y,
          location: (location_in_window.x, location_in_window.y),
          modifiers,
        })
      }
      _ => None,
    }
  }

  #[cfg(target_os = "windows")]
  pub fn from_windows_message(msg: u32, wparam: usize, lparam: isize) -> Option<Self> {
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
          key: Key::from_keycode(key_code),
          modifiers,
        })
      }
      WM_KEYUP | WM_SYSKEYUP => {
        let key_code = wparam as u16;
        Some(InputEvent::KeyUp {
          key: Key::from_keycode(key_code),
          modifiers,
        })
      }
      WM_LBUTTONDOWN => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        Some(InputEvent::MouseDown {
          button: MouseButton::Left,
          location: (x, y),
          modifiers,
        })
      }
      WM_LBUTTONUP => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        Some(InputEvent::MouseUp {
          button: MouseButton::Left,
          location: (x, y),
          modifiers,
        })
      }
      WM_RBUTTONDOWN => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        Some(InputEvent::MouseDown {
          button: MouseButton::Right,
          location: (x, y),
          modifiers,
        })
      }
      WM_RBUTTONUP => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        Some(InputEvent::MouseUp {
          button: MouseButton::Right,
          location: (x, y),
          modifiers,
        })
      }
      WM_MBUTTONDOWN => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        Some(InputEvent::MouseDown {
          button: MouseButton::Other(2), // Middle button
          location: (x, y),
          modifiers,
        })
      }
      WM_MBUTTONUP => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        Some(InputEvent::MouseUp {
          button: MouseButton::Other(2),
          location: (x, y),
          modifiers,
        })
      }
      WM_MOUSEMOVE => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        Some(InputEvent::MouseMoved {
          location: (x, y),
          modifiers,
        })
      }
      WM_MOUSEWHEEL => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        let delta = ((wparam >> 16) & 0xFFFF) as i16 as f64 / 120.0; // WHEEL_DELTA is 120
        Some(InputEvent::ScrollWheel {
          delta_x: 0.0,
          delta_y: delta,
          location: (x, y),
          modifiers,
        })
      }
      WM_MOUSEHWHEEL => {
        let x = (lparam & 0xFFFF) as i16 as f64;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as f64;
        let delta = ((wparam >> 16) & 0xFFFF) as i16 as f64 / 120.0;
        Some(InputEvent::ScrollWheel {
          delta_x: delta,
          delta_y: 0.0,
          location: (x, y),
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
        key: Key::from_keycode(keyval),
        modifiers: key_modifiers,
      }),
      gtk::gdk::EventType::KeyRelease => Some(InputEvent::KeyUp {
        key: Key::from_keycode(keyval),
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
      other => MouseButton::Other(other as i16),
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
