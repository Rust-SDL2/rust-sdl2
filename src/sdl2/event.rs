/*!
Event Handling
 */

use std::ffi::CStr;
use std::mem;
use libc::{c_int, c_void, uint32_t};
use num::FromPrimitive;
use std::ptr;
use std::borrow::ToOwned;
use std::iter::FromIterator;
use std::marker::PhantomData;

use controller;
use controller::{Axis, Button};
use joystick;
use joystick::HatState;
use keyboard;
use keyboard::Mod;
use sys::keycode::SDL_Keymod;
use keycode::KeyCode;
use mouse;
use mouse::{Mouse, MouseState};
use scancode::ScanCode;
use get_error;
use SdlResult;
use Sdl;

use sys::event as ll;

/// Types of events that can be delivered.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum EventType {
    First = ll::SDL_FIRSTEVENT,

    Quit = ll::SDL_QUIT,
    AppTerminating = ll::SDL_APP_TERMINATING,
    AppLowMemory = ll::SDL_APP_LOWMEMORY,
    AppWillEnterBackground = ll::SDL_APP_WILLENTERBACKGROUND,
    AppDidEnterBackground = ll::SDL_APP_DIDENTERBACKGROUND,
    AppWillEnterForeground = ll::SDL_APP_WILLENTERFOREGROUND,
    AppDidEnterForeground = ll::SDL_APP_DIDENTERFOREGROUND,

    Window = ll::SDL_WINDOWEVENT,
    // TODO: SysWM = ll::SDL_SYSWMEVENT,

    KeyDown = ll::SDL_KEYDOWN,
    KeyUp = ll::SDL_KEYUP,
    TextEditing = ll::SDL_TEXTEDITING,
    TextInput = ll::SDL_TEXTINPUT,

    MouseMotion = ll::SDL_MOUSEMOTION,
    MouseButtonDown = ll::SDL_MOUSEBUTTONDOWN,
    MouseButtonUp = ll::SDL_MOUSEBUTTONUP,
    MouseWheel = ll::SDL_MOUSEWHEEL,

    JoyAxisMotion = ll::SDL_JOYAXISMOTION,
    JoyBallMotion = ll::SDL_JOYBALLMOTION,
    JoyHatMotion = ll::SDL_JOYHATMOTION,
    JoyButtonDown = ll::SDL_JOYBUTTONDOWN,
    JoyButtonUp = ll::SDL_JOYBUTTONUP,
    JoyDeviceAdded = ll::SDL_JOYDEVICEADDED,
    JoyDeviceRemoved = ll::SDL_JOYDEVICEREMOVED,

    ControllerAxisMotion = ll::SDL_CONTROLLERAXISMOTION,
    ControllerButtonDown = ll::SDL_CONTROLLERBUTTONDOWN,
    ControllerButtonUp = ll::SDL_CONTROLLERBUTTONUP,
    ControllerDeviceAdded = ll::SDL_CONTROLLERDEVICEADDED,
    ControllerDeviceRemoved = ll::SDL_CONTROLLERDEVICEREMOVED,
    ControllerDeviceRemapped = ll::SDL_CONTROLLERDEVICEREMAPPED,

    FingerDown = ll::SDL_FINGERDOWN,
    FingerUp = ll::SDL_FINGERUP,
    FingerMotion = ll::SDL_FINGERMOTION,
    DollarGesture = ll::SDL_DOLLARGESTURE,
    DollarRecord = ll::SDL_DOLLARRECORD,
    MultiGesture = ll::SDL_MULTIGESTURE,

    ClipboardUpdate = ll::SDL_CLIPBOARDUPDATE,
    DropFile = ll::SDL_DROPFILE,

    User = ll::SDL_USEREVENT,
    Last = ll::SDL_LASTEVENT,
}

impl FromPrimitive for EventType {
    fn from_i64(n: i64) -> Option<EventType> {
        use self::EventType::*;

        Some( match n as ll::SDL_EventType {
            ll::SDL_FIRSTEVENT => First,

            ll::SDL_QUIT => Quit,
            ll::SDL_APP_TERMINATING => AppTerminating,
            ll::SDL_APP_LOWMEMORY => AppLowMemory,
            ll::SDL_APP_WILLENTERBACKGROUND => AppWillEnterBackground,
            ll::SDL_APP_DIDENTERBACKGROUND => AppDidEnterBackground,
            ll::SDL_APP_WILLENTERFOREGROUND => AppWillEnterForeground,
            ll::SDL_APP_DIDENTERFOREGROUND => AppDidEnterForeground,

            ll::SDL_WINDOWEVENT => Window,

            ll::SDL_KEYDOWN => KeyDown,
            ll::SDL_KEYUP => KeyUp,
            ll::SDL_TEXTEDITING => TextEditing,
            ll::SDL_TEXTINPUT => TextInput,

            ll::SDL_MOUSEMOTION => MouseMotion,
            ll::SDL_MOUSEBUTTONDOWN => MouseButtonDown,
            ll::SDL_MOUSEBUTTONUP => MouseButtonUp,
            ll::SDL_MOUSEWHEEL => MouseWheel,

            ll::SDL_JOYAXISMOTION => JoyAxisMotion,
            ll::SDL_JOYBALLMOTION => JoyBallMotion,
            ll::SDL_JOYHATMOTION => JoyHatMotion,
            ll::SDL_JOYBUTTONDOWN => JoyButtonDown,
            ll::SDL_JOYBUTTONUP => JoyButtonUp,
            ll::SDL_JOYDEVICEADDED => JoyDeviceAdded,
            ll::SDL_JOYDEVICEREMOVED => JoyDeviceRemoved,

            ll::SDL_CONTROLLERAXISMOTION => ControllerAxisMotion,
            ll::SDL_CONTROLLERBUTTONDOWN => ControllerButtonDown,
            ll::SDL_CONTROLLERBUTTONUP => ControllerButtonUp,
            ll::SDL_CONTROLLERDEVICEADDED => ControllerDeviceAdded,
            ll::SDL_CONTROLLERDEVICEREMOVED => ControllerDeviceRemoved,
            ll::SDL_CONTROLLERDEVICEREMAPPED => ControllerDeviceRemapped,

            ll::SDL_FINGERDOWN => FingerDown,
            ll::SDL_FINGERUP => FingerUp,
            ll::SDL_FINGERMOTION => FingerMotion,
            ll::SDL_DOLLARGESTURE => DollarGesture,
            ll::SDL_DOLLARRECORD => DollarRecord,
            ll::SDL_MULTIGESTURE => MultiGesture,

            ll::SDL_CLIPBOARDUPDATE => ClipboardUpdate,
            ll::SDL_DROPFILE => DropFile,

            ll::SDL_USEREVENT => User,
            ll::SDL_LASTEVENT => Last,

            _ => return None,
        })
    }

    fn from_u64(n: u64) -> Option<EventType> { FromPrimitive::from_i64(n as i64) }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
/// An enum of window events.
pub enum WindowEventId {
    None,
    Shown,
    Hidden,
    Exposed,
    Moved,
    Resized,
    SizeChanged,
    Minimized,
    Maximized,
    Restored,
    Enter,
    Leave,
    FocusGained,
    FocusLost,
    Close,
}

impl WindowEventId {
    fn from_ll(id: u8) -> WindowEventId {
        match id {
            1  => WindowEventId::Shown,
            2  => WindowEventId::Hidden,
            3  => WindowEventId::Exposed,
            4  => WindowEventId::Moved,
            5  => WindowEventId::Resized,
            6  => WindowEventId::SizeChanged,
            7  => WindowEventId::Minimized,
            8  => WindowEventId::Maximized,
            9  => WindowEventId::Restored,
            10 => WindowEventId::Enter,
            11 => WindowEventId::Leave,
            12 => WindowEventId::FocusGained,
            13 => WindowEventId::FocusLost,
            14 => WindowEventId::Close,
            _  => WindowEventId::None
        }
    }
}

#[derive(Clone, PartialEq)]
/// Different event types.
pub enum Event {
    Quit { timestamp: u32 },
    AppTerminating { timestamp: u32 },
    AppLowMemory { timestamp: u32 },
    AppWillEnterBackground { timestamp: u32 },
    AppDidEnterBackground { timestamp: u32 },
    AppWillEnterForeground { timestamp: u32 },
    AppDidEnterForeground { timestamp: u32 },

    Window {
        timestamp: u32 ,
        window_id: u32,
        win_event_id: WindowEventId,
        data1: i32,
        data2: i32
    },
    // TODO: SysWMEvent

    KeyDown {
        timestamp: u32 ,
        window_id: u32,
        keycode: KeyCode,
        scancode: ScanCode,
        keymod: Mod,
        repeat: bool
    },
    KeyUp {
        timestamp: u32 ,
        window_id: u32,
        keycode: KeyCode,
        scancode: ScanCode,
        keymod: Mod,
        repeat: bool
    },

    TextEditing {
        timestamp: u32,
        window_id: u32,
        text: String,
        start: i32,
        length: i32
    },

    TextInput {
        timestamp: u32,
        window_id: u32,
        text: String
    },

    MouseMotion {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mousestate: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32
    },

    MouseButtonDown {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: Mouse,
        x: i32,
        y: i32
    },
    MouseButtonUp {
        timestamp: u32,
        window_id: u32,
        which: u32,
        mouse_btn: Mouse,
        x: i32,
        y: i32
    },

    MouseWheel {
        timestamp: u32,
        window_id: u32,
        which: u32,
        x: i32,
        y: i32
    },

    JoyAxisMotion {
        timestamp: u32,
        which: i32,
        axis_idx: u8,
        value: i16
    },

    JoyBallMotion {
        timestamp: u32,
        which: i32,
        ball_idx: u8,
        xrel: i16,
        yrel: i16
    },

    JoyHatMotion {
        timestamp: u32,
        which: i32,
        hat_idx: u8,
        state: HatState
    },

    JoyButtonDown {
        timestamp: u32,
        which: i32,
        button_idx: u8
    },
    JoyButtonUp {
        timestamp: u32,
        which: i32,
        button_idx: u8
    },

    JoyDeviceAdded {
        timestamp: u32,
        which: i32
    },
    JoyDeviceRemoved {
        timestamp: u32,
        which: i32
    },

    ControllerAxisMotion {
        timestamp: u32,
        which: i32,
        axis: Axis,
        value: i16
    },

    ControllerButtonDown {
        timestamp: u32,
        which: i32,
        button: Button
    },
    ControllerButtonUp {
        timestamp: u32,
        which: i32,
        button: Button
    },

    ControllerDeviceAdded {
        timestamp: u32,
        which: i32
    },
    ControllerDeviceRemoved {
        timestamp: u32,
        which: i32
    },
    ControllerDeviceRemapped {
        timestamp: u32,
        which: i32
    },

    FingerDown {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerUp {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },
    FingerMotion {
        timestamp: u32,
        touch_id: i64,
        finger_id: i64,
        x: f32,
        y: f32,
        dx: f32,
        dy: f32,
        pressure: f32
    },

    DollarGesture {
        timestamp: u32,
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32
    },
    DollarRecord {
        timestamp: u32,
        touch_id: i64,
        gesture_id: i64,
        num_fingers: u32,
        error: f32,
        x: f32,
        y: f32
    },

    MultiGesture {
        timestamp: u32,
        touch_id: i64,
        d_theta: f32,
        d_dist: f32,
        x: f32,
        y: f32,
        num_fingers: u16
    },

    ClipboardUpdate {
        timestamp: u32
    },

    DropFile {
        timestamp: u32,
        filename: String
    },

    User {
        timestamp: u32,
        window_id: u32,
        type_: u32,
        code: i32
    },

    Unknown {
        timestamp: u32,
        type_: u32
    }
}

impl ::std::fmt::Debug for Event {
    fn fmt(&self, out: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        out.write_str(match *self {
            Event::Quit{..} => "Event::Quit",
            Event::AppTerminating{..} => "Event::AppTerminating",
            Event::AppLowMemory{..} => "Event::AppLowMemory",
            Event::AppWillEnterBackground{..} => "Event::AppWillEnterBackground",
            Event::AppDidEnterBackground{..} => "Event::AppDidEnterBackground",
            Event::AppWillEnterForeground{..} => "Event::AppWillEnterForeground",
            Event::AppDidEnterForeground{..} => "Event::AppDidEnterForeground",
            Event::Window{..} => "Event::Window",
            Event::KeyDown{..} => "Event::KeyDown",
            Event::KeyUp{..} => "Event::KeyUp",
            Event::TextEditing{..} => "Event::TextEditing",
            Event::TextInput{..} => "Event::TextInput",
            Event::MouseMotion{..} => "Event::MouseMotion",
            Event::MouseButtonDown{..} => "Event::MouseButtonDown",
            Event::MouseButtonUp{..} => "Event::MouseButtonUp",
            Event::MouseWheel{..} => "Event::MouseWheel",
            Event::JoyAxisMotion{..} => "Event::JoyAxisMotion",
            Event::JoyBallMotion{..} => "Event::JoyBallMotion",
            Event::JoyHatMotion{..} => "Event::JoyHatMotion",
            Event::JoyButtonDown{..} => "Event::JoyButtonDown",
            Event::JoyButtonUp{..} => "Event::JoyButtonUp",
            Event::JoyDeviceAdded{..} => "Event::JoyDeviceAdded",
            Event::JoyDeviceRemoved{..} => "Event::JoyDeviceRemoved",
            Event::ControllerAxisMotion{..} => "Event::ControllerAxisMotion",
            Event::ControllerButtonDown{..} => "Event::ControllerButtonDown",
            Event::ControllerButtonUp{..} => "Event::ControllerButtonUp",
            Event::ControllerDeviceAdded{..} => "Event::ControllerDeviceAdded",
            Event::ControllerDeviceRemoved{..} => "Event::ControllerDeviceRemoved",
            Event::ControllerDeviceRemapped{..} => "Event::ControllerDeviceRemapped",
            Event::FingerDown{..} => "Event::FingerDown",
            Event::FingerUp{..} => "Event::FingerUp",
            Event::FingerMotion{..} => "Event::FingerMotion",
            Event::DollarGesture{..} => "Event::DollarGesture",
            Event::DollarRecord{..} => "Event::DollarRecord",
            Event::MultiGesture{..} => "Event::MultiGesture",
            Event::ClipboardUpdate{..} => "Event::ClipboardUpdate",
            Event::DropFile{..} => "Event::DropFile",
            Event::User{..} => "Event::User",
            Event::Unknown{..} => "Event::Unknown",
        })
    }
}

// TODO: Remove this when from_utf8 is updated in Rust
impl Event {
    fn to_ll(self) -> Option<ll::SDL_Event> {
        let mut ret = unsafe { mem::uninitialized() };
        match self {
            // just ignore timestamp
            Event::User { window_id, type_, code, .. } => {
                let event = ll::SDL_UserEvent {
                    type_: type_ as uint32_t,
                    timestamp: 0,
                    windowID: window_id,
                    code: code as i32,
                    data1: ptr::null_mut(),
                    data2: ptr::null_mut(),
                };
                unsafe {
                    ptr::copy(&event, &mut ret as *mut ll::SDL_Event as *mut ll::SDL_UserEvent, 1);
                }
                Some(ret)
            },
            _ => {
                // don't know how to convert!
                None
            }
        }
    }

    fn from_ll(mut raw: ll::SDL_Event) -> Event {
        let raw_type = raw.type_();
        let raw_type = if raw_type.is_null() {
            panic!("Event payload is null")
        } else {
            unsafe { *raw_type }
        };

        // if event type has not been defined, treat it as a UserEvent
        let event_type: EventType = FromPrimitive::from_usize(raw_type as usize).unwrap_or(EventType::User);
        unsafe { match event_type {
            EventType::Quit => {
                let ref event = *raw.quit();
                Event::Quit { timestamp: event.timestamp }
            }
            EventType::AppTerminating => {
                let ref event = *raw.common();
                Event::AppTerminating { timestamp: event.timestamp }
            }
            EventType::AppLowMemory => {
                let ref event = *raw.common();
                Event::AppLowMemory { timestamp: event.timestamp }
            }
            EventType::AppWillEnterBackground => {
                let ref event = *raw.common();
                Event::AppWillEnterBackground { timestamp: event.timestamp }
            }
            EventType::AppDidEnterBackground => {
                let ref event = *raw.common();
                Event::AppDidEnterBackground { timestamp: event.timestamp }
            }
            EventType::AppWillEnterForeground => {
                let ref event = *raw.common();
                Event::AppWillEnterForeground { timestamp: event.timestamp }
            }
            EventType::AppDidEnterForeground => {
                let ref event = *raw.common();
                Event::AppDidEnterForeground { timestamp: event.timestamp }
            }

            EventType::Window => {
                let ref event = *raw.window();

                Event::Window {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    win_event_id: WindowEventId::from_ll(event.event),
                    data1: event.data1,
                    data2: event.data2
                }
            }
            // TODO: SysWMEventType

            EventType::KeyDown => {
                let ref event = *raw.key();

                Event::KeyDown {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    keycode: FromPrimitive::from_i32(event.keysym.sym)
                                 .unwrap_or(KeyCode::Unknown),
                    scancode: FromPrimitive::from_u32(event.keysym.scancode)
                                 .unwrap_or(ScanCode::Unknown),
                    keymod: keyboard::Mod::from_bits(event.keysym._mod as SDL_Keymod).unwrap(),
                    repeat: event.repeat != 0
                }
            }
            EventType::KeyUp => {
                let ref event = *raw.key();

                Event::KeyUp {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    keycode: FromPrimitive::from_i32(event.keysym.sym)
                               .unwrap_or(KeyCode::Unknown),
                    scancode: FromPrimitive::from_u32(event.keysym.scancode)
                               .unwrap_or(ScanCode::Unknown),
                    keymod: keyboard::Mod::from_bits(event.keysym._mod as SDL_Keymod).unwrap(),
                    repeat: event.repeat != 0
                }
            }
            EventType::TextEditing => {
                let ref event = *raw.edit();

                let text = String::from_utf8_lossy(
                        &event.text.iter()
                            .take_while(|&b| (*b) != 0i8)
                            .map(|&b| b as u8)
                            .collect::<Vec<u8>>()
                    ).to_owned().into_owned();
                Event::TextEditing {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    text: text,
                    start: event.start,
                    length: event.length
                }
            }
            EventType::TextInput => {
                let ref event = *raw.text();

                let text = String::from_utf8_lossy(
                        &event.text.iter()
                            .take_while(|&b| (*b) != 0i8)
                            .map(|&b| b as u8)
                            .collect::<Vec<u8>>()
                    ).to_owned().into_owned();
                Event::TextInput {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    text: text
                }
            }

            EventType::MouseMotion => {
                let ref event = *raw.motion();

                Event::MouseMotion {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    mousestate: mouse::MouseState::from_flags(event.state),
                    x: event.x,
                    y: event.y,
                    xrel: event.xrel,
                    yrel: event.yrel
                }
            }
            EventType::MouseButtonDown => {
                let ref event = *raw.button();

                Event::MouseButtonDown {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    mouse_btn: mouse::wrap_mouse(event.button),
                    x: event.x,
                    y: event.y
                }
            }
            EventType::MouseButtonUp => {
                let ref event = *raw.button();

                Event::MouseButtonUp {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    mouse_btn: mouse::wrap_mouse(event.button),
                    x: event.x,
                    y: event.y
                }
            }
            EventType::MouseWheel => {
                let ref event = *raw.wheel();

                Event::MouseWheel {
                    timestamp: event.timestamp,
                    window_id: event.windowID,
                    which: event.which,
                    x: event.x,
                    y: event.y
                }
            }

            EventType::JoyAxisMotion => {
                let ref event = *raw.jaxis();
                Event::JoyAxisMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    axis_idx: event.axis,
                    value: event.value
                }
            }
            EventType::JoyBallMotion => {
                let ref event = *raw.jball();
                Event::JoyBallMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    ball_idx: event.ball,
                    xrel: event.xrel,
                    yrel: event.yrel
                }
            }
            EventType::JoyHatMotion => {
                let ref event = *raw.jhat();
                Event::JoyHatMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    hat_idx: event.hat,
                    state: joystick::HatState::from_raw(event.value),
                }
            }
            EventType::JoyButtonDown => {
                let ref event = *raw.jbutton();
                Event::JoyButtonDown {
                    timestamp: event.timestamp,
                    which: event.which,
                    button_idx: event.button
                }
            }
            EventType::JoyButtonUp => {
                let ref event = *raw.jbutton();
                Event::JoyButtonUp {
                    timestamp: event.timestamp,
                    which: event.which,
                    button_idx: event.button
                }
            }
            EventType::JoyDeviceAdded => {
                let ref event = *raw.jdevice();
                Event::JoyDeviceAdded {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }
            EventType::JoyDeviceRemoved => {
                let ref event = *raw.jdevice();
                Event::JoyDeviceRemoved {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }

            EventType::ControllerAxisMotion => {
                let ref event = *raw.caxis();
                let axis = controller::wrap_controller_axis(event.axis);

                Event::ControllerAxisMotion {
                    timestamp: event.timestamp,
                    which: event.which,
                    axis: axis,
                    value: event.value
                }
            }
            EventType::ControllerButtonDown => {
                let ref event = *raw.cbutton();
                let button = controller::wrap_controller_button(event.button);

                Event::ControllerButtonDown {
                    timestamp: event.timestamp,
                    which: event.which,
                    button: button
                }
            }
            EventType::ControllerButtonUp => {
                let ref event = *raw.cbutton();
                let button = controller::wrap_controller_button(event.button);

                Event::ControllerButtonUp {
                    timestamp: event.timestamp,
                    which: event.which,
                    button: button
                }
            }
            EventType::ControllerDeviceAdded => {
                let ref event = *raw.cdevice();
                Event::ControllerDeviceAdded {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }
            EventType::ControllerDeviceRemoved => {
                let ref event = *raw.cdevice();
                Event::ControllerDeviceRemoved {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }
            EventType::ControllerDeviceRemapped => {
                let ref event = *raw.cdevice();
                Event::ControllerDeviceRemapped {
                    timestamp: event.timestamp,
                    which: event.which
                }
            }

            EventType::FingerDown => {
                let ref event = *raw.tfinger();
                Event::FingerDown {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    finger_id: event.fingerId,
                    x: event.x,
                    y: event.y,
                    dx: event.dx,
                    dy: event.dy,
                    pressure: event.pressure
                }
            }
            EventType::FingerUp => {
                let ref event = *raw.tfinger();
                Event::FingerUp {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    finger_id: event.fingerId,
                    x: event.x,
                    y: event.y,
                    dx: event.dx,
                    dy: event.dy,
                    pressure: event.pressure
                }
            }
            EventType::FingerMotion => {
                let ref event = *raw.tfinger();
                Event::FingerMotion {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    finger_id: event.fingerId,
                    x: event.x,
                    y: event.y,
                    dx: event.dx,
                    dy: event.dy,
                    pressure: event.pressure
                }
            }
            EventType::DollarGesture => {
                let ref event = *raw.dgesture();
                Event::DollarGesture {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    gesture_id: event.gestureId,
                    num_fingers: event.numFingers,
                    error: event.error,
                    x: event.x,
                    y: event.y
                }
            }
            EventType::DollarRecord => {
                let ref event = *raw.dgesture();
                Event::DollarRecord {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    gesture_id: event.gestureId,
                    num_fingers: event.numFingers,
                    error: event.error,
                    x: event.x,
                    y: event.y
                }
            }
            EventType::MultiGesture => {
                let ref event = *raw.mgesture();
                Event::MultiGesture {
                    timestamp: event.timestamp,
                    touch_id: event.touchId,
                    d_theta: event.dTheta,
                    d_dist: event.dDist,
                    x: event.x,
                    y: event.y,
                    num_fingers: event.numFingers
                }
            }

            EventType::ClipboardUpdate => {
                let ref event = *raw.common();
                Event::ClipboardUpdate {
                    timestamp: event.timestamp
                }
            }
            EventType::DropFile => {
                let ref event = *raw.drop();

                let buf = CStr::from_ptr(event.file).to_bytes();
                let text = String::from_utf8_lossy(buf).to_string();
                ll::SDL_free(event.file as *mut c_void);

                Event::DropFile {
                    timestamp: event.timestamp,
                    filename: text
                }
            }

            EventType::First => panic!("Unused event, EventType::First, was encountered"),
            EventType::Last => panic!("Unusable event, EventType::Last, was encountered"),

            // If we have no other match and the event type is >= 32768
            // this is a user event
            EventType::User => {
                if raw_type < 32768 {
                    // The type is unknown to us.
                    // It's a newer SDL2 type.
                    let ref event = *raw.common();

                    Event::Unknown {
                        timestamp: event.timestamp,
                        type_: event.type_
                    }
                } else {
                    let ref event = *raw.user();

                    Event::User {
                        timestamp: event.timestamp,
                        window_id: event.windowID,
                        type_: raw_type,
                        code: event.code
                    }
                }
            }
        }}                      // close unsafe & match
    }
}

unsafe fn poll_event() -> Option<Event> {
    let mut raw = mem::uninitialized();
    let has_pending = ll::SDL_PollEvent(&mut raw) == 1;

    if has_pending { Some(Event::from_ll(raw)) }
    else { None }
}

unsafe fn wait_event() -> Event {
    let mut raw = mem::uninitialized();
    let success = ll::SDL_WaitEvent(&mut raw) == 1;

    if success { Event::from_ll(raw) }
    else { panic!(get_error()) }
}

unsafe fn wait_event_timeout(timeout: u32) -> Option<Event> {
    let mut raw = mem::uninitialized();
    let success = ll::SDL_WaitEventTimeout(&mut raw, timeout as c_int) == 1;

    if success { Some(Event::from_ll(raw)) }
    else { None }
}

/// A thread-safe type that encapsulates SDL event-pumping functions.
pub struct EventPump<'sdl> {
    _sdl:    PhantomData<&'sdl ()>,

    // Prevents the event pump from moving to other threads.
    // SDL events can only be pumped on the main thread.
    _nosend: PhantomData<*mut ()>
}
impl<'sdl> EventPump<'sdl> {
    /// Query if an event type is enabled.
    pub fn is_event_enabled(&self, event_type: EventType) -> bool {
        let result = unsafe { ll::SDL_EventState(event_type as u32, ll::SDL_QUERY) };

        result != ll::SDL_DISABLE
    }

    /// Enable an event type. Returns if the event type was enabled before the call.
    pub fn enable_event(&mut self, event_type: EventType) -> bool {
        let result = unsafe { ll::SDL_EventState(event_type as u32, ll::SDL_ENABLE) };

        result != ll::SDL_DISABLE
    }

    /// Disable an event type. Returns if the event type was enabled before the call.
    pub fn disable_event(&mut self, event_type: EventType) -> bool {
        let result = unsafe { ll::SDL_EventState(event_type as u32, ll::SDL_DISABLE) };

        result != ll::SDL_DISABLE
    }

    /// Polls for currently pending events.
    ///
    /// If no events are pending, `None` is returned.
    pub fn poll_event(&mut self) -> Option<Event> {
        unsafe { poll_event() }
    }

    /// Returns a polling iterator that calls `poll_event()`.
    /// The iterator will terminate once there are no more pending events.
    ///
    /// # Example
    /// ```no_run
    /// let mut sdl_context = sdl2::init().everything().unwrap();
    ///
    /// for event in sdl_context.event_pump().poll_iter() {
    ///     use sdl2::event::Event;
    ///     match event {
    ///         Event::KeyDown {..} => { /*...*/ },
    ///         _ => ()
    ///     }
    /// }
    /// ```
    pub fn poll_iter(&mut self) -> EventPollIterator {
        EventPollIterator {
            _marker: PhantomData
        }
    }

    /// Pumps the event loop, gathering events from the input devices.
    pub fn pump_events(&mut self) {
        unsafe { ll::SDL_PumpEvents(); };
    }

    /// Waits indefinitely for the next available event.
    pub fn wait_event(&mut self) -> Event {
        unsafe { wait_event() }
    }

    /// Waits until the specified timeout (in milliseconds) for the next available event.
    pub fn wait_event_timeout(&mut self, timeout: u32) -> Option<Event> {
        unsafe { wait_event_timeout(timeout) }
    }

    /// Returns a waiting iterator that calls `wait_event()`.
    ///
    /// Note: The iterator will never terminate.
    pub fn wait_iter(&mut self) -> EventWaitIterator {
        EventWaitIterator {
            _marker: PhantomData
        }
    }

    /// Returns a waiting iterator that calls `wait_event_timeout()`.
    ///
    /// Note: The iterator will never terminate, unless waiting for an event
    /// exceeds the specified timeout.
    pub fn wait_timeout_iter(&mut self, timeout: u32) -> EventWaitTimeoutIterator {
        EventWaitTimeoutIterator {
            _marker: PhantomData,
            timeout: timeout
        }
    }

    /// Obtains the SDL event pump.
    pub fn new(_sdl: &'sdl mut Sdl) -> EventPump<'sdl> {
        // Called on the main SDL thread.
        EventPump {
            _sdl: PhantomData,
            _nosend: PhantomData,
        }
    }
}

/// An iterator that calls `EventPump::poll_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventPollIterator<'a> {
    _marker: PhantomData<&'a ()>
}

impl<'a> Iterator for EventPollIterator<'a> {
    type Item = Event;

    fn next(&mut self) -> Option<Event> { unsafe { poll_event() } }
}

/// An iterator that calls `EventPump::wait_event()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitIterator<'a> {
    _marker: PhantomData<&'a ()>
}

impl<'a> Iterator for EventWaitIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> { unsafe { Some(wait_event()) } }
}

/// An iterator that calls `EventPump::wait_event_timeout()`.
#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct EventWaitTimeoutIterator<'a> {
    _marker: PhantomData<&'a ()>,
    timeout: u32
}

impl<'a> Iterator for EventWaitTimeoutIterator<'a> {
    type Item = Event;
    fn next(&mut self) -> Option<Event> { unsafe { wait_event_timeout(self.timeout) } }
}

/// Removes all events in the event queue that match the specified event type.
pub fn flush_event(event_type: EventType) {
    unsafe { ll::SDL_FlushEvent(event_type as uint32_t) };
}

/// Removes all events in the event queue that match the specified type range.
pub fn flush_events(min_type: u32, max_type: u32) {
    unsafe { ll::SDL_FlushEvents(min_type, max_type) };
}

/// Reads the events at the front of the event queue, until the maximum amount
/// of events is read.
///
/// The events will _not_ be removed from the queue.
///
/// # Example
/// ```no_run
/// use sdl2::event::{Event, peek_events};
///
/// // Read up to 1024 events
/// let events: Vec<Event> = peek_events(1024);
///
/// // Print each one
/// for event in events {
///     println!("{:?}", event);
/// }
/// ```
pub fn peek_events<B>(max_amount: u32) -> B
where B: FromIterator<Event>
{
    unsafe {
        let mut events = Vec::with_capacity(max_amount as usize);

        let result = {
            let events_ptr = events.as_mut_ptr();

            ll::SDL_PeepEvents(
                events_ptr,
                max_amount as c_int,
                ll::SDL_PEEKEVENT,
                ll::SDL_FIRSTEVENT,
                ll::SDL_LASTEVENT
            )
        };

        if result < 0 {
            // The only error possible is "Couldn't lock event queue"
            panic!(get_error());
        } else {
            events.set_len(max_amount as usize);

            events.into_iter().map(|event_raw| {
                Event::from_ll(event_raw)
            }).collect()
        }
    }
}

/// Pushes an event to the event queue.
pub fn push_event(event: Event) -> SdlResult<()> {
    match event.to_ll() {
        Some(mut raw_event) => {
            let ok = unsafe { ll::SDL_PushEvent(&mut raw_event) == 1 };
            if ok { Ok(()) }
            else { Err(get_error()) }
        },
        None => {
            Err(format!("Cannot push unsupported event type to the queue"))
        }
    }
}
