// 0 should not be used in bitflags, but here it is. Removing it will break existing code.
#![allow(clippy::bad_bit_mask)]

use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::common::validate_string;
use crate::video::Window;
use crate::Error;

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
    pub background: (u8, u8, u8),
    pub text: (u8, u8, u8),
    pub button_border: (u8, u8, u8),
    pub button_background: (u8, u8, u8),
    pub button_selected: (u8, u8, u8),
}

impl From<MessageBoxColorScheme> for sys::SDL_MessageBoxColorScheme {
    fn from(val: MessageBoxColorScheme) -> Self {
        sys::SDL_MessageBoxColorScheme { colors: val.into() }
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
    pub flags: MessageBoxButtonFlag,
    pub button_id: i32,
    pub text: &'a str,
}

#[derive(Debug)]
pub enum ClickedButton<'a> {
    CloseButton,
    CustomButton(&'a ButtonData<'a>),
}

impl From<MessageBoxColorScheme> for [sys::SDL_MessageBoxColor; 5] {
    fn from(scheme: MessageBoxColorScheme) -> [sys::SDL_MessageBoxColor; 5] {
        fn to_message_box_color(t: (u8, u8, u8)) -> sys::SDL_MessageBoxColor {
            sys::SDL_MessageBoxColor {
                r: t.0,
                g: t.1,
                b: t.2,
            }
        }
        [
            to_message_box_color(scheme.background),
            to_message_box_color(scheme.text),
            to_message_box_color(scheme.button_border),
            to_message_box_color(scheme.button_background),
            to_message_box_color(scheme.button_selected),
        ]
    }
}

impl From<[sys::SDL_MessageBoxColor; 5]> for MessageBoxColorScheme {
    fn from(val: [sys::SDL_MessageBoxColor; 5]) -> Self {
        fn from_message_box_color(prim_color: sys::SDL_MessageBoxColor) -> (u8, u8, u8) {
            (prim_color.r, prim_color.g, prim_color.b)
        }
        MessageBoxColorScheme {
            background: from_message_box_color(val[0]),
            text: from_message_box_color(val[1]),
            button_border: from_message_box_color(val[2]),
            button_background: from_message_box_color(val[3]),
            button_selected: from_message_box_color(val[4]),
        }
    }
}

/// Show a simple message box, meant to be informative only.
///
/// There is no way to know if the user clicked "Ok" or closed the message box,
/// If you want to retrieve which button was clicked and customize a bit more
/// your message box, use `show_message_box` instead.
#[doc(alias = "SDL_ShowSimpleMessageBox")]
pub fn show_simple_message_box<'a, W>(
    flags: MessageBoxFlag,
    title: &str,
    message: &str,
    window: W,
) -> Result<(), Error>
where
    W: Into<Option<&'a Window>>,
{
    let title = as_cstring!(title)?;
    let message = as_cstring!(message)?;
    let result = unsafe {
        sys::SDL_ShowSimpleMessageBox(
            flags.bits(),
            title.as_ptr() as *const c_char,
            message.as_ptr() as *const c_char,
            window.into().map_or(ptr::null_mut(), |win| win.raw()),
        )
    };

    if result == 0 {
        Ok(())
    } else {
        Err(Error::from_sdl_error())
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
#[doc(alias = "SDL_ShowMessageBox")]
pub fn show_message_box<'a, 'b, W, M>(
    flags: MessageBoxFlag,
    buttons: &'a [ButtonData],
    title: &str,
    message: &str,
    window: W,
    scheme: M,
) -> Result<ClickedButton<'a>, Error>
where
    W: Into<Option<&'b Window>>,
    M: Into<Option<MessageBoxColorScheme>>,
{
    let window = window.into();
    let scheme = scheme.into();

    let mut button_id: c_int = 0;
    let title = as_cstring!(title)?;
    let message = as_cstring!(message)?;
    let button_texts = buttons
        .iter()
        .map(|b| validate_string(b.text, "buttons"))
        .collect::<Result<Vec<_>, _>>()?;
    let raw_buttons: Vec<sys::SDL_MessageBoxButtonData> = buttons
        .iter()
        .zip(button_texts.iter())
        .map(|(b, b_text)| sys::SDL_MessageBoxButtonData {
            flags: b.flags.bits(),
            buttonid: b.button_id as c_int,
            text: b_text.as_ptr(),
        })
        .collect();
    let scheme = scheme.map(|scheme| sys::SDL_MessageBoxColorScheme {
        colors: scheme.into(),
    });
    let msg_box_data = sys::SDL_MessageBoxData {
        flags: flags.bits(),
        window: window.map_or(ptr::null_mut(), |win| win.raw()),
        title: title.as_ptr() as *const c_char,
        message: message.as_ptr() as *const c_char,
        numbuttons: raw_buttons.len() as c_int,
        buttons: raw_buttons.as_ptr(),
        colorScheme: scheme
            .as_ref()
            .map(|p| p as *const _)
            .unwrap_or(ptr::null()),
    };
    let result =
        unsafe { sys::SDL_ShowMessageBox(&msg_box_data as *const _, &mut button_id as &mut _) };
    if result == 0 {
        match button_id {
            -1 => Ok(ClickedButton::CloseButton),
            id => {
                let button = buttons.iter().find(|b| b.button_id == id);
                Ok(ClickedButton::CustomButton(button.unwrap()))
            }
        }
    } else {
        Err(Error::from_sdl_error())
    }
}
