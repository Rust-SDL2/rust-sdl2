use std::error;
use std::ffi::{CString, NulError};
use std::fmt;
use std::ptr;
use std::os::raw::{c_char,c_int};

use crate::video::Window;
use crate::get_error;

use crate::sys;

bitflags! {
    pub struct MessageBoxFlag: u32 {
        const ERROR =
            sys::SDL_MessageBoxFlags::SDL_MESSAGEBOX_ERROR as u32;
        const WARNING =
            sys::SDL_MessageBoxFlags::SDL_MESSAGEBOX_WARNING as u32;
        const INFORMATION =
            sys::SDL_MessageBoxFlags::SDL_MESSAGEBOX_INFORMATION as u32;
    }
}

bitflags! {
    pub struct MessageBoxButtonFlag: u32 {
        const ESCAPEKEY_DEFAULT =
            sys::SDL_MessageBoxButtonFlags::SDL_MESSAGEBOX_BUTTON_ESCAPEKEY_DEFAULT as u32;
        const RETURNKEY_DEFAULT =
            sys::SDL_MessageBoxButtonFlags::SDL_MESSAGEBOX_BUTTON_RETURNKEY_DEFAULT as u32;
        const NOTHING = 0;
    }
}

#[derive(Debug)]
pub struct MessageBoxColorScheme {
    pub background:(u8,u8,u8),
    pub text:(u8,u8,u8),
    pub button_border:(u8,u8,u8),
    pub button_background:(u8,u8,u8),
    pub button_selected:(u8,u8,u8)
}

impl Into<sys::SDL_MessageBoxColorScheme> for MessageBoxColorScheme {
    fn into(self) -> sys::SDL_MessageBoxColorScheme {
        sys::SDL_MessageBoxColorScheme { colors: self.into() }
    }
}

impl From<sys::SDL_MessageBoxColorScheme> for MessageBoxColorScheme {
    fn from(prim: sys::SDL_MessageBoxColorScheme) -> MessageBoxColorScheme {
        prim.colors.into()
    }
}

/// `button_id` is the integer that will be returned
/// by `show_message_box`. It is not sed by SDL2,
/// and should only be used to know which button has been triggered
#[derive(Debug)]
pub struct ButtonData<'a> {
    pub flags:MessageBoxButtonFlag,
    pub button_id:i32,
    pub text:&'a str
}

#[derive(Debug)]
pub enum ClickedButton<'a> {
    CloseButton,
    CustomButton(&'a ButtonData<'a>)
}

impl From<MessageBoxColorScheme> for [sys::SDL_MessageBoxColor ; 5] {
    fn from(scheme:MessageBoxColorScheme) -> [sys::SDL_MessageBoxColor ; 5] {
        fn to_message_box_color(t:(u8,u8,u8)) -> sys::SDL_MessageBoxColor {
            sys::SDL_MessageBoxColor {
                r:t.0,
                g:t.1,
                b:t.2
            }
        };
        [to_message_box_color(scheme.background),
        to_message_box_color(scheme.text),
        to_message_box_color(scheme.button_border),
        to_message_box_color(scheme.button_background),
        to_message_box_color(scheme.button_selected)]
    }
}

impl Into<MessageBoxColorScheme> for [sys::SDL_MessageBoxColor ; 5] {
    fn into(self) -> MessageBoxColorScheme {
        fn from_message_box_color(prim_color: sys::SDL_MessageBoxColor) -> (u8, u8, u8) {
            (prim_color.r, prim_color.g, prim_color.b)
        };
        MessageBoxColorScheme{
            background: from_message_box_color(self[0]),
            text: from_message_box_color(self[1]),
            button_border: from_message_box_color(self[2]),
            button_background: from_message_box_color(self[3]),
            button_selected: from_message_box_color(self[4]),
        }
    }
}

#[derive(Debug)]
pub enum ShowMessageError {
    InvalidTitle(NulError),
    InvalidMessage(NulError),
    /// Second argument of the tuple (i32) corresponds to the
    /// first button_id having an error
    InvalidButton(NulError,i32),
    SdlError(String),
}

impl fmt::Display for ShowMessageError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::ShowMessageError::*;

        match *self {
            InvalidTitle(ref e) => write!(f, "Invalid title: {}", e),
            InvalidMessage(ref e) => write!(f, "Invalid message: {}", e),
            InvalidButton(ref e, value) => write!(f,
                "Invalid button ({}): {}", value, e),
            SdlError(ref e) => write!(f, "SDL error: {}", e)
        }
    }
}

impl error::Error for ShowMessageError {
    fn description(&self) -> &str {
        use self::ShowMessageError::*;

        match *self {
            InvalidTitle(_) => "invalid title",
            InvalidMessage(_) => "invalid message",
            InvalidButton(..) => "invalid button",
            SdlError(ref e) => e
        }
    }
}

/// Show a simple message box, meant to be informative only.
///
/// There is no way to know if the user clicked "Ok" or closed the message box,
/// If you want to retrieve which button was clicked and customize a bit more
/// your message box, use `show_message_box` instead.
pub fn show_simple_message_box<'a, W>(flags: MessageBoxFlag, title: &str,
        message: &str, window: W)
        -> Result<(), ShowMessageError>
where W: Into<Option<&'a Window>>
{
    use self::ShowMessageError::*;
    let result = unsafe {
        let title = match CString::new(title) {
            Ok(s) => s,
            Err(err) => return Err(InvalidTitle(err)),
        };
        let message = match CString::new(message) {
            Ok(s) => s,
            Err(err) => return Err(InvalidMessage(err)),
        };
        sys::SDL_ShowSimpleMessageBox(
            flags.bits(),
            title.as_ptr() as *const c_char,
            message.as_ptr() as *const c_char,
            window.into().map_or(ptr::null_mut(), |win| win.raw())
        )
    } == 0;

    if result {
        Ok(())
    } else {
        Err(SdlError(get_error()))
    }
}

/// Show a customizable message box.
///
/// An array of buttons is required for it to work. The array can be empty,
/// but it will have no button beside the close button.
///
/// On success, it will return either the button clicked or the close button.
/// Note that the variant of the `ClickedButton` enum will also be returned if the message box
/// has been forcefully closed (Alt-F4, ...)
///
pub fn show_message_box<'a, 'b, W, M>(flags:MessageBoxFlag, buttons:&'a [ButtonData], title:&str,
    message:&str, window: W, scheme: M)
    -> Result<ClickedButton<'a>,ShowMessageError>
where W: Into<Option<&'b Window>>,
      M: Into<Option<MessageBoxColorScheme>>,
{
    let window = window.into();
    let scheme = scheme.into();

    use self::ShowMessageError::*;
    let mut button_id : c_int = 0;
    let title = match CString::new(title) {
        Ok(s) => s,
        Err(err) => return Err(InvalidTitle(err)),
    };
    let message = match CString::new(message) {
        Ok(s) => s,
        Err(err) => return Err(InvalidMessage(err)),
    };
    let button_texts : Result<Vec<_>,(_,i32)> = buttons.iter().map(|b|{
        CString::new(b.text).map_err(|e|(e,b.button_id))
    }).collect(); // Create CString for every button; and catch any CString Error
    let button_texts = match button_texts {
        Ok(b) => b,
        Err(e) => return Err(InvalidButton(e.0,e.1))
    };
    let raw_buttons : Vec<sys::SDL_MessageBoxButtonData> =
        buttons.iter().zip(button_texts.iter()).map(|(b,b_text)|{
        sys::SDL_MessageBoxButtonData {
            flags:b.flags.bits(),
            buttonid:b.button_id as c_int,
            text:b_text.as_ptr()
        }
    }).collect();
    let result = unsafe {
        let msg_box_data = sys::SDL_MessageBoxData {
            flags:flags.bits(),
            window:window.map_or(ptr::null_mut(), |win| win.raw()),
            title: title.as_ptr() as *const c_char,
            message: message.as_ptr() as *const c_char,
            numbuttons: raw_buttons.len() as c_int,
            buttons: raw_buttons.as_ptr(),
            colorScheme: if let Some(scheme) = scheme {
                &sys::SDL_MessageBoxColorScheme {
                    colors:From::from(scheme)
                } as *const _
            } else {
                ptr::null()
            }
        };
        sys::SDL_ShowMessageBox(
            &msg_box_data as *const _,
            &mut button_id as &mut _
        )
    } == 0;
    if result {
        match button_id {
            -1 => Ok(ClickedButton::CloseButton),
            id => {
                let button = buttons.iter().find(|b| b.button_id == id);
                Ok(ClickedButton::CustomButton(button.unwrap()))
            }
        }
    } else {
        Err(SdlError(get_error()))
    }
}
