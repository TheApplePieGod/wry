#[cfg(target_os = "macos")]
use objc2_app_kit::NSEvent;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::*;
#[cfg(target_os = "linux")]
use gtk::gdk::{EventButton, EventKey, EventMotion, EventScroll, ModifierType};

#[derive(Debug, Clone)]
pub enum InputEvent {
  KeyDown {
    key_code: u16,
    characters: Option<String>,
    modifiers: KeyModifiers,
  },
  KeyUp {
    key_code: u16,
    characters: Option<String>,
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

#[derive(Debug, Clone)]
pub enum MouseButton {
  Left,
  Right,
  Other(i16),
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
        let characters = unsafe { event.characters() }
          .map(|chars| chars.to_string());

        Some(InputEvent::KeyDown {
          key_code,
          characters,
          modifiers,
        })
      }
      NSEventType::KeyUp => {
        let key_code = unsafe { event.keyCode() };
        let characters = unsafe { event.characters() }
          .map(|chars| chars.to_string());

        Some(InputEvent::KeyUp {
          key_code,
          characters,
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
      NSEventType::MouseMoved | NSEventType::LeftMouseDragged | NSEventType::RightMouseDragged | NSEventType::OtherMouseDragged => {
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
          key_code,
          characters: None, // Windows WM_KEYDOWN doesn't provide character info
          modifiers,
        })
      }
      WM_KEYUP | WM_SYSKEYUP => {
        let key_code = wparam as u16;
        Some(InputEvent::KeyUp {
          key_code,
          characters: None,
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
        key_code: keyval as u16,
        characters: event.string().map(|s| s.to_string()),
        modifiers: key_modifiers,
      }),
      gtk::gdk::EventType::KeyRelease => Some(InputEvent::KeyUp {
        key_code: keyval as u16,
        characters: event.string().map(|s| s.to_string()),
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
