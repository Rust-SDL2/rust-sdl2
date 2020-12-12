use std::convert::TryFrom;
use std::intrinsics::transmute;

use crate::sys::SDL_EventType::*;
/// Types of events that can be delivered.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
#[repr(u32)]
pub enum EventType {
    First = SDL_FIRSTEVENT as u32,

    Quit = SDL_QUIT as u32,
    AppTerminating = SDL_APP_TERMINATING as u32,
    AppLowMemory = SDL_APP_LOWMEMORY as u32,
    AppWillEnterBackground = SDL_APP_WILLENTERBACKGROUND as u32,
    AppDidEnterBackground = SDL_APP_DIDENTERBACKGROUND as u32,
    AppWillEnterForeground = SDL_APP_WILLENTERFOREGROUND as u32,
    AppDidEnterForeground = SDL_APP_DIDENTERFOREGROUND as u32,

    Window = SDL_WINDOWEVENT as u32,
    // TODO: SysWM = sys::SDL_SYSWMEVENT as u32,
    KeyDown = SDL_KEYDOWN as u32,
    KeyUp = SDL_KEYUP as u32,
    TextEditing = SDL_TEXTEDITING as u32,
    TextInput = SDL_TEXTINPUT as u32,

    MouseMotion = SDL_MOUSEMOTION as u32,
    MouseButtonDown = SDL_MOUSEBUTTONDOWN as u32,
    MouseButtonUp = SDL_MOUSEBUTTONUP as u32,
    MouseWheel = SDL_MOUSEWHEEL as u32,

    JoyAxisMotion = SDL_JOYAXISMOTION as u32,
    JoyBallMotion = SDL_JOYBALLMOTION as u32,
    JoyHatMotion = SDL_JOYHATMOTION as u32,
    JoyButtonDown = SDL_JOYBUTTONDOWN as u32,
    JoyButtonUp = SDL_JOYBUTTONUP as u32,
    JoyDeviceAdded = SDL_JOYDEVICEADDED as u32,
    JoyDeviceRemoved = SDL_JOYDEVICEREMOVED as u32,

    ControllerAxisMotion = SDL_CONTROLLERAXISMOTION as u32,
    ControllerButtonDown = SDL_CONTROLLERBUTTONDOWN as u32,
    ControllerButtonUp = SDL_CONTROLLERBUTTONUP as u32,
    ControllerDeviceAdded = SDL_CONTROLLERDEVICEADDED as u32,
    ControllerDeviceRemoved = SDL_CONTROLLERDEVICEREMOVED as u32,
    ControllerDeviceRemapped = SDL_CONTROLLERDEVICEREMAPPED as u32,

    FingerDown = SDL_FINGERDOWN as u32,
    FingerUp = SDL_FINGERUP as u32,
    FingerMotion = SDL_FINGERMOTION as u32,
    DollarGesture = SDL_DOLLARGESTURE as u32,
    DollarRecord = SDL_DOLLARRECORD as u32,
    MultiGesture = SDL_MULTIGESTURE as u32,

    ClipboardUpdate = SDL_CLIPBOARDUPDATE as u32,
    DropFile = SDL_DROPFILE as u32,
    DropText = SDL_DROPTEXT as u32,
    DropBegin = SDL_DROPBEGIN as u32,
    DropComplete = SDL_DROPCOMPLETE as u32,

    AudioDeviceAdded = SDL_AUDIODEVICEADDED as u32,
    AudioDeviceRemoved = SDL_AUDIODEVICEREMOVED as u32,

    RenderTargetsReset = SDL_RENDER_TARGETS_RESET as u32,
    RenderDeviceReset = SDL_RENDER_DEVICE_RESET as u32,

    User = SDL_USEREVENT as u32,
    Last = SDL_LASTEVENT as u32,
}

impl TryFrom<u32> for EventType {
    type Error = ();

    fn try_from(n: u32) -> Result<Self, Self::Error> {
        use self::EventType::*;

        Ok(match unsafe { transmute(n) } {
            SDL_FIRSTEVENT => First,

            SDL_QUIT => Quit,
            SDL_APP_TERMINATING => AppTerminating,
            SDL_APP_LOWMEMORY => AppLowMemory,
            SDL_APP_WILLENTERBACKGROUND => AppWillEnterBackground,
            SDL_APP_DIDENTERBACKGROUND => AppDidEnterBackground,
            SDL_APP_WILLENTERFOREGROUND => AppWillEnterForeground,
            SDL_APP_DIDENTERFOREGROUND => AppDidEnterForeground,

            SDL_WINDOWEVENT => Window,

            SDL_KEYDOWN => KeyDown,
            SDL_KEYUP => KeyUp,
            SDL_TEXTEDITING => TextEditing,
            SDL_TEXTINPUT => TextInput,

            SDL_MOUSEMOTION => MouseMotion,
            SDL_MOUSEBUTTONDOWN => MouseButtonDown,
            SDL_MOUSEBUTTONUP => MouseButtonUp,
            SDL_MOUSEWHEEL => MouseWheel,

            SDL_JOYAXISMOTION => JoyAxisMotion,
            SDL_JOYBALLMOTION => JoyBallMotion,
            SDL_JOYHATMOTION => JoyHatMotion,
            SDL_JOYBUTTONDOWN => JoyButtonDown,
            SDL_JOYBUTTONUP => JoyButtonUp,
            SDL_JOYDEVICEADDED => JoyDeviceAdded,
            SDL_JOYDEVICEREMOVED => JoyDeviceRemoved,

            SDL_CONTROLLERAXISMOTION => ControllerAxisMotion,
            SDL_CONTROLLERBUTTONDOWN => ControllerButtonDown,
            SDL_CONTROLLERBUTTONUP => ControllerButtonUp,
            SDL_CONTROLLERDEVICEADDED => ControllerDeviceAdded,
            SDL_CONTROLLERDEVICEREMOVED => ControllerDeviceRemoved,
            SDL_CONTROLLERDEVICEREMAPPED => ControllerDeviceRemapped,

            SDL_FINGERDOWN => FingerDown,
            SDL_FINGERUP => FingerUp,
            SDL_FINGERMOTION => FingerMotion,
            SDL_DOLLARGESTURE => DollarGesture,
            SDL_DOLLARRECORD => DollarRecord,
            SDL_MULTIGESTURE => MultiGesture,

            SDL_CLIPBOARDUPDATE => ClipboardUpdate,
            SDL_DROPFILE => DropFile,
            SDL_DROPTEXT => DropText,
            SDL_DROPBEGIN => DropBegin,
            SDL_DROPCOMPLETE => DropComplete,

            SDL_AUDIODEVICEADDED => AudioDeviceAdded,
            SDL_AUDIODEVICEREMOVED => AudioDeviceRemoved,

            SDL_RENDER_TARGETS_RESET => RenderTargetsReset,
            SDL_RENDER_DEVICE_RESET => RenderDeviceReset,

            SDL_USEREVENT => User,
            SDL_LASTEVENT => Last,

            _ => return Err(()),
        })
    }
}
