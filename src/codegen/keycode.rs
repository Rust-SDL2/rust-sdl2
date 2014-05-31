use std::io::{IoResult,Writer};
use std::path::BytesContainer;
use super::get_writer;

struct Key {
    code: uint,
    ident: &'static str,
}

impl Ord for Key {
    fn lt (&self, other: &Key) -> bool {
        if self.code < other.code {
            true
        } else {
            false
        }
    }
}

impl Eq for Key {
    fn eq (&self, other: &Key) -> bool {
        if self.code == other.code {
            true
        } else {
            false
        }
    }
}

impl TotalOrd for Key {
    fn cmp(&self, other: &Key) -> Ordering {
        if self.code < other.code {
            Less
        } else if self.code > other.code {
            Greater
        } else { Equal }
    }
}
impl TotalEq for Key {
}

#[allow(non_snake_case_functions)]
fn Key(code: uint, ident: &'static str) -> Key {
    Key { code: code, ident: ident }
}

impl Key {
    fn ident(&self) -> String {
        self.ident.to_string()
    }

    fn padded_ident(&self) -> String {
        self.ident().append(" ".repeat(unsafe { longest_ident } - self.ident().len()).as_slice())
    }

}

static mut longest_ident: uint = 0;

pub fn generate(output_dir: &Path) -> IoResult<()> {
    let mut out = get_writer(output_dir, "keycode.rs");
    let mut entries = [
        Key(0, "UnknownKey"),
        Key(13, "ReturnKey"),
        Key(27, "EscapeKey"),
        Key(8, "BackspaceKey"),
        Key(9, "TabKey"),
        Key(32, "SpaceKey"),
        Key(33, "ExclaimKey"),
        Key(34, "QuotedblKey"),
        Key(35, "HashKey"),
        Key(37, "PercentKey"),
        Key(36, "DollarKey"),
        Key(38, "AmpersandKey"),
        Key(39, "QuoteKey"),
        Key(40, "LeftParenKey"),
        Key(41, "RightParenKey"),
        Key(42, "AsteriskKey"),
        Key(43, "PlusKey"),
        Key(44, "CommaKey"),
        Key(45, "MinusKey"),
        Key(46, "PeriodKey"),
        Key(47, "SlashKey"),
        Key(48, "Num0Key"),
        Key(49, "Num1Key"),
        Key(50, "Num2Key"),
        Key(51, "Num3Key"),
        Key(52, "Num4Key"),
        Key(53, "Num5Key"),
        Key(54, "Num6Key"),
        Key(55, "Num7Key"),
        Key(56, "Num8Key"),
        Key(57, "Num9Key"),
        Key(58, "ColonKey"),
        Key(59, "SemicolonKey"),
        Key(60, "LessKey"),
        Key(61, "EqualsKey"),
        Key(62, "GreaterKey"),
        Key(63, "QuestionKey"),
        Key(64, "AtKey"),
        Key(91, "LeftBracketKey"),
        Key(92, "BackslashKey"),
        Key(93, "RightBracketKey"),
        Key(94, "CaretKey"),
        Key(95, "UnderscoreKey"),
        Key(96, "BackquoteKey"),
        Key(97, "AKey"),
        Key(98, "BKey"),
        Key(99, "CKey"),
        Key(100, "DKey"),
        Key(101, "EKey"),
        Key(102, "FKey"),
        Key(103, "GKey"),
        Key(104, "HKey"),
        Key(105, "IKey"),
        Key(106, "JKey"),
        Key(107, "KKey"),
        Key(108, "LKey"),
        Key(109, "MKey"),
        Key(110, "NKey"),
        Key(111, "OKey"),
        Key(112, "PKey"),
        Key(113, "QKey"),
        Key(114, "RKey"),
        Key(115, "SKey"),
        Key(116, "TKey"),
        Key(117, "UKey"),
        Key(118, "VKey"),
        Key(119, "WKey"),
        Key(120, "XKey"),
        Key(121, "YKey"),
        Key(122, "ZKey"),
        Key(1073741881, "CapsLockKey"),
        Key(1073741882, "F1Key"),
        Key(1073741883, "F2Key"),
        Key(1073741884, "F3Key"),
        Key(1073741885, "F4Key"),
        Key(1073741886, "F5Key"),
        Key(1073741887, "F6Key"),
        Key(1073741888, "F7Key"),
        Key(1073741889, "F8Key"),
        Key(1073741890, "F9Key"),
        Key(1073741891, "F10Key"),
        Key(1073741892, "F11Key"),
        Key(1073741893, "F12Key"),
        Key(1073741894, "PrintScreenKey"),
        Key(1073741895, "ScrollLockKey"),
        Key(1073741896, "PauseKey"),
        Key(1073741897, "InsertKey"),
        Key(1073741898, "HomeKey"),
        Key(1073741899, "PageUpKey"),
        Key(127, "DeleteKey"),
        Key(1073741901, "EndKey"),
        Key(1073741902, "PageDownKey"),
        Key(1073741903, "RightKey"),
        Key(1073741904, "LeftKey"),
        Key(1073741905, "DownKey"),
        Key(1073741906, "UpKey"),
        Key(1073741907, "NumLockClearKey"),
        Key(1073741908, "KpDivideKey"),
        Key(1073741909, "KpMultiplyKey"),
        Key(1073741910, "KpMinusKey"),
        Key(1073741911, "KpPlusKey"),
        Key(1073741912, "KpEnterKey"),
        Key(1073741913, "Kp1Key"),
        Key(1073741914, "Kp2Key"),
        Key(1073741915, "Kp3Key"),
        Key(1073741916, "Kp4Key"),
        Key(1073741917, "Kp5Key"),
        Key(1073741918, "Kp6Key"),
        Key(1073741919, "Kp7Key"),
        Key(1073741920, "Kp8Key"),
        Key(1073741921, "Kp9Key"),
        Key(1073741922, "Kp0Key"),
        Key(1073741923, "KpPeriodKey"),
        Key(1073741925, "ApplicationKey"),
        Key(1073741926, "PowerKey"),
        Key(1073741927, "KpEqualsKey"),
        Key(1073741928, "F13Key"),
        Key(1073741929, "F14Key"),
        Key(1073741930, "F15Key"),
        Key(1073741931, "F16Key"),
        Key(1073741932, "F17Key"),
        Key(1073741933, "F18Key"),
        Key(1073741934, "F19Key"),
        Key(1073741935, "F20Key"),
        Key(1073741936, "F21Key"),
        Key(1073741937, "F22Key"),
        Key(1073741938, "F23Key"),
        Key(1073741939, "F24Key"),
        Key(1073741940, "ExecuteKey"),
        Key(1073741941, "HelpKey"),
        Key(1073741942, "MenuKey"),
        Key(1073741943, "SelectKey"),
        Key(1073741944, "StopKey"),
        Key(1073741945, "AgainKey"),
        Key(1073741946, "UndoKey"),
        Key(1073741947, "CutKey"),
        Key(1073741948, "CopyKey"),
        Key(1073741949, "PasteKey"),
        Key(1073741950, "FindKey"),
        Key(1073741951, "MuteKey"),
        Key(1073741952, "VolumeUpKey"),
        Key(1073741953, "VolumeDownKey"),
        Key(1073741957, "KpCommaKey"),
        Key(1073741958, "KpEqualsAS400Key"),
        Key(1073741977, "AltEraseKey"),
        Key(1073741978, "SysreqKey"),
        Key(1073741979, "CancelKey"),
        Key(1073741980, "ClearKey"),
        Key(1073741981, "PriorKey"),
        Key(1073741982, "Return2Key"),
        Key(1073741983, "SeparatorKey"),
        Key(1073741984, "OutKey"),
        Key(1073741985, "OperKey"),
        Key(1073741986, "ClearAgainKey"),
        Key(1073741987, "CrSelKey"),
        Key(1073741988, "ExSelKey"),
        Key(1073742000, "Kp00Key"),
        Key(1073742001, "Kp000Key"),
        Key(1073742002, "ThousandsSeparatorKey"),
        Key(1073742003, "DecimalSeparatorKey"),
        Key(1073742004, "CurrencyUnitKey"),
        Key(1073742005, "CurrencySubUnitKey"),
        Key(1073742006, "KpLeftParenKey"),
        Key(1073742007, "KpRightParenKey"),
        Key(1073742008, "KpLeftBraceKey"),
        Key(1073742009, "KpRightBraceKey"),
        Key(1073742010, "KpTabKey"),
        Key(1073742011, "KpBackspaceKey"),
        Key(1073742012, "KpAKey"),
        Key(1073742013, "KpBKey"),
        Key(1073742014, "KpCKey"),
        Key(1073742015, "KpDKey"),
        Key(1073742016, "KpEKey"),
        Key(1073742017, "KpFKey"),
        Key(1073742018, "KpXorKey"),
        Key(1073742019, "KpPowerKey"),
        Key(1073742020, "KpPercentKey"),
        Key(1073742021, "KpLessKey"),
        Key(1073742022, "KpGreaterKey"),
        Key(1073742023, "KpAmpersandKey"),
        Key(1073742024, "KpDblAmpersandKey"),
        Key(1073742025, "KpVerticalBarKey"),
        Key(1073742026, "KpDblVerticalBarKey"),
        Key(1073742027, "KpColonKey"),
        Key(1073742028, "KpHashKey"),
        Key(1073742029, "KpSpaceKey"),
        Key(1073742030, "KpAtKey"),
        Key(1073742031, "KpExclamKey"),
        Key(1073742032, "KpMemStoreKey"),
        Key(1073742033, "KpMemRecallKey"),
        Key(1073742034, "KpMemClearKey"),
        Key(1073742035, "KpMemAddKey"),
        Key(1073742036, "KpMemSubtractKey"),
        Key(1073742037, "KpMemMultiplyKey"),
        Key(1073742038, "KpMemDivideKey"),
        Key(1073742039, "KpPlusMinusKey"),
        Key(1073742040, "KpCearKey"),
        Key(1073742041, "KpClearEntryKey"),
        Key(1073742042, "KpBinaryKey"),
        Key(1073742043, "KpOctalKey"),
        Key(1073742044, "KpDecimalKey"),
        Key(1073742045, "KpHexadecimalKey"),
        Key(1073742048, "LCtrlKey"),
        Key(1073742049, "LShiftKey"),
        Key(1073742050, "LAltKey"),
        Key(1073742051, "LGuiKey"),
        Key(1073742052, "RCtrlKey"),
        Key(1073742053, "RShiftKey"),
        Key(1073742054, "RAltKey"),
        Key(1073742055, "RGuiKey"),
        Key(1073742081, "ModeKey"),
        Key(1073742082, "AudioNextKey"),
        Key(1073742083, "AudioPrevKey"),
        Key(1073742084, "AudioStopKey"),
        Key(1073742085, "AudioPlayKey"),
        Key(1073742086, "AudioMuteKey"),
        Key(1073742087, "MediaSelectKey"),
        Key(1073742088, "WwwKey"),
        Key(1073742089, "MailKey"),
        Key(1073742090, "CalculatorKey"),
        Key(1073742091, "ComputerKey"),
        Key(1073742092, "AcSearchKey"),
        Key(1073742093, "AcHomeKey"),
        Key(1073742094, "AcBackKey"),
        Key(1073742095, "AcForwardKey"),
        Key(1073742096, "AcStopKey"),
        Key(1073742097, "AcRefreshKey"),
        Key(1073742098, "AcBookmarksKey"),
        Key(1073742099, "BrightnessDownKey"),
        Key(1073742100, "BrightnessUpKey"),
        Key(1073742101, "DisplaySwitchKey"),
        Key(1073742102, "KbdIllumToggleKey"),
        Key(1073742103, "KbdIllumDownKey"),
        Key(1073742104, "KbdIllumUpKey"),
        Key(1073742105, "EjectKey"),
        Key(1073742106, "SleepKey"),
        ];

        entries.sort();
    unsafe {
        longest_ident = entries.iter().map(|&key| key.ident().len()).max_by(|&i| i).unwrap();
    }
    try!(out.write("// This automatically generated file is used as sdl2::keycode.

use std::hash::Hash;
use std::hash::sip::SipState;

use std::num::FromPrimitive;
use std::num::ToPrimitive;

#[deriving(Eq, TotalEq, Show)]
pub enum KeyCode {
".as_bytes()));
    for &entry in entries.iter() {
        try!(out.write(format!("    {} = {},\n", entry.padded_ident(), entry.code).container_as_bytes()));
    }

    try!(out.write("
}

impl Hash for KeyCode {
   #[inline]
    fn hash(&self, state: &mut SipState) {
	self.code().hash(state);
    }
}

impl KeyCode {
    /// Get the code
    pub fn code(&self) -> i32 {
        match *self {
".as_bytes()));
    for &entry in entries.iter() {
        try!(out.write(format!("            {} => {},\n", entry.padded_ident(), entry.code).container_as_bytes()));
    }
    try!(out.write("
        }
    }
}

impl ToPrimitive for KeyCode {
    /// Equivalent to `self.code()`
".as_bytes()));
    let types = vec!("i64", "u64", "int");
    for primitive_type in types.iter() {
        try!(out.write(format!("fn to_{}(&self) -> Option<{}> \\{
            Some(self.code() as {})
        \\}\n", *primitive_type, *primitive_type, *primitive_type).container_as_bytes()));
    }

try!(out.write("
}

impl FromPrimitive for KeyCode {

    /// Get a *registered* key code.
    ///
    /// This will return UnknownKey if an unknown code is passed.
    ///
    /// For example, `from_int(13)` will return `ReturnKey`.
".as_bytes()));
    for primitive_type in types.iter() {
        try!(out.write(format!("
    fn from_{}(n: {}) -> Option<KeyCode> \\{
        match n \\{
", *primitive_type, *primitive_type).container_as_bytes()));
        for &entry in entries.iter() {
            try!(out.write(format!("            {} => Some({}),\n", entry.code, entry.ident()).container_as_bytes()));
        }
        try!(out.write("
                _   => { Some(UnknownKey) }
            }
        }\n".as_bytes()));
    }

try!(out.write("
}".as_bytes()));
	try!(out.flush());
    Ok(())
}
