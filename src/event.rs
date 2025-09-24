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
      0x31 => Key::Space,
      0x33 => Key::Backspace,
      0x30 => Key::Tab,
      0x24 => Key::Enter,
      0x35 => Key::Escape,
      0x1B => Key::CapsLock,
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
      0x7D => Key::Down,
      0x7B => Key::Left,
      0x7C => Key::Right,
      0x7E => Key::Up,
      0x73 => Key::Home,
      0x77 => Key::End,
      0x74 => Key::PageUp,
      0x79 => Key::PageDown,
      0x72 => Key::Insert,
      0x75 => Key::Delete,
      0x38 => Key::LeftShift,
      0x3B => Key::LeftControl,
      0x3A => Key::LeftAlt,
      0x37 => Key::LeftSuper,
      0x3C => Key::RightShift,
      0x3E => Key::RightControl,
      0x3D => Key::RightAlt,
      0x36 => Key::RightSuper,
      _ => Key::Unknown,
    }
  }

  #[cfg(target_os = "windows")]
  pub fn from_keycode(code: u16) -> Self {
    match code {
      0x20 => Key::Space,       // VK_SPACE
      0x0D => Key::Enter,       // VK_RETURN
      0x08 => Key::Backspace,   // VK_BACK
      0x09 => Key::Tab,         // VK_TAB
      0x1B => Key::Escape,      // VK_ESCAPE
      0x14 => Key::CapsLock,    // VK_CAPITAL
      0x25 => Key::Left,        // VK_LEFT
      0x27 => Key::Right,       // VK_RIGHT
      0x26 => Key::Up,          // VK_UP
      0x28 => Key::Down,        // VK_DOWN
      0x24 => Key::Home,        // VK_HOME
      0x23 => Key::End,         // VK_END
      0x21 => Key::PageUp,      // VK_PRIOR
      0x22 => Key::PageDown,    // VK_NEXT
      0x2D => Key::Insert,      // VK_INSERT
      0x2E => Key::Delete,      // VK_DELETE
      0xA0 => Key::LeftShift,   // VK_LSHIFT
      0xA1 => Key::RightShift,  // VK_RSHIFT
      0xA2 => Key::LeftControl, // VK_LCONTROL
      0xA3 => Key::RightControl,// VK_RCONTROL
      0xA4 => Key::LeftAlt,     // VK_LMENU
      0xA5 => Key::RightAlt,    // VK_RMENU
      0x5B => Key::LeftSuper,   // VK_LWIN
      0x5C => Key::RightSuper,  // VK_RWIN
      0x70 => Key::F1,          // VK_F1
      0x71 => Key::F2,          // VK_F2
      0x72 => Key::F3,          // VK_F3
      0x73 => Key::F4,          // VK_F4
      0x74 => Key::F5,          // VK_F5
      0x75 => Key::F6,          // VK_F6
      0x76 => Key::F7,          // VK_F7
      0x77 => Key::F8,          // VK_F8
      0x78 => Key::F9,          // VK_F9
      0x79 => Key::F10,         // VK_F10
      0x7A => Key::F11,         // VK_F11
      0x7B => Key::F12,         // VK_F12
      0x41..=0x5A => {
        // Letters A-Z
        let index = (code - 0x41) as i32; // 0..25
        unsafe { std::mem::transmute(Key::A as i32 + index) }
      }
      _ => Key::Unknown,
    }
  }

  #[cfg(target_os = "linux")]
  pub fn from_keycode(code: u16) -> Self {
    // GTK key codes from GDK (simplified)
    match code {
      0x20 => Key::Space,
      0xFF08 => Key::Backspace,
      0xFF09 => Key::Tab,
      0xFF0D => Key::Enter,
      0xFF1B => Key::Escape,
      0xFFE1 => Key::LeftShift,
      0xFFE2 => Key::RightShift,
      0xFFE3 => Key::LeftControl,
      0xFFE4 => Key::RightControl,
      0xFFE9 => Key::LeftAlt,
      0xFFEA => Key::RightAlt,
      0xFFEB => Key::LeftSuper,
      0xFFEC => Key::RightSuper,
      0xFF50 => Key::Home,
      0xFF57 => Key::End,
      0xFF55 => Key::PageUp,
      0xFF56 => Key::PageDown,
      0xFF51 => Key::Left,
      0xFF53 => Key::Right,
      0xFF52 => Key::Up,
      0xFF54 => Key::Down,
      0xFFBE => Key::F1,
      0xFFBF => Key::F2,
      0xFFC0 => Key::F3,
      0xFFC1 => Key::F4,
      0xFFC2 => Key::F5,
      0xFFC3 => Key::F6,
      0xFFC4 => Key::F7,
      0xFFC5 => Key::F8,
      0xFFC6 => Key::F9,
      0xFFC7 => Key::F10,
      0xFFC8 => Key::F11,
      0xFFC9 => Key::F12,
      0x0061..=0x007A => {
        // a-z
        unsafe { std::mem::transmute((code - 0x0061) as i32 + Key::A as i32) }
      }
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
    let keyval = event.keyval();
    let modifiers = event.state();

    let key_modifiers = KeyModifiers {
      shift: modifiers.contains(ModifierType::SHIFT_MASK),
      control: modifiers.contains(ModifierType::CONTROL_MASK),
      alt: modifiers.contains(ModifierType::MOD1_MASK),
      command: modifiers.contains(ModifierType::SUPER_MASK),
    };

    match event.event_type() {
      gtk::gdk::EventType::KeyPress => Some(InputEvent::KeyDown {
        key: Key::from_keycode(*keyval as u16),
        modifiers: key_modifiers,
      }),
      gtk::gdk::EventType::KeyRelease => Some(InputEvent::KeyUp {
        key: Key::from_keycode(*keyval as u16),
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
