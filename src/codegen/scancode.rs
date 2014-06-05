use std::io::{IoResult,Writer};
use std::path::BytesContainer;
use super::get_writer;

struct ScanCode {
    code: uint,
    ident: &'static str,
}

impl PartialOrd for ScanCode {
    fn lt (&self, other: &ScanCode) -> bool {
        if self.code < other.code {
            true
        } else {
            false
        }
    }
}

impl PartialEq for ScanCode {
    fn eq (&self, other: &ScanCode) -> bool {
        if self.code == other.code {
            true
        } else {
            false
        }
    }
}

impl Ord for ScanCode {
    fn cmp(&self, other: &ScanCode) -> Ordering {
        if self.code < other.code {
            Less
        } else if self.code > other.code {
            Greater
        } else { Equal }
    }
}
impl Eq for ScanCode {
}

#[allow(non_snake_case_functions)]
fn ScanCode(code: uint, ident: &'static str) -> ScanCode {
    ScanCode { code: code, ident: ident }
}

impl ScanCode {
    fn ident(&self) -> String {
        self.ident.to_string()
    }

    fn padded_ident(&self) -> String {
        self.ident().append(" ".repeat(unsafe { longest_ident } - self.ident().len()).as_slice())
    }

}

static mut longest_ident: uint = 0;

pub fn generate(output_dir: &Path) -> IoResult<()> {
    let mut out = get_writer(output_dir, "scancode.rs");
    let mut entries = [
        ScanCode(0, "UnknownScanCode"),
        ScanCode(4, "AScanCode"),
        ScanCode(5, "BScanCode"),
        ScanCode(6, "CScanCode"),
        ScanCode(7, "DScanCode"),
        ScanCode(8, "EScanCode"),
        ScanCode(9, "FScanCode"),
        ScanCode(10, "GScanCode"),
        ScanCode(11, "HScanCode"),
        ScanCode(12, "IScanCode"),
        ScanCode(13, "JScanCode"),
        ScanCode(14, "KScanCode"),
        ScanCode(15, "LScanCode"),
        ScanCode(16, "MScanCode"),
        ScanCode(17, "NScanCode"),
        ScanCode(18, "OScanCode"),
        ScanCode(19, "PScanCode"),
        ScanCode(20, "QScanCode"),
        ScanCode(21, "RScanCode"),
        ScanCode(22, "SScanCode"),
        ScanCode(23, "TScanCode"),
        ScanCode(24, "UScanCode"),
        ScanCode(25, "VScanCode"),
        ScanCode(26, "WScanCode"),
        ScanCode(27, "XScanCode"),
        ScanCode(28, "YScanCode"),
        ScanCode(29, "ZScanCode"),
        ScanCode(30, "Num1ScanCode"),
        ScanCode(31, "Num2ScanCode"),
        ScanCode(32, "Num3ScanCode"),
        ScanCode(33, "Num4ScanCode"),
        ScanCode(34, "Num5ScanCode"),
        ScanCode(35, "Num6ScanCode"),
        ScanCode(36, "Num7ScanCode"),
        ScanCode(37, "Num8ScanCode"),
        ScanCode(38, "Num9ScanCode"),
        ScanCode(39, "Num0ScanCode"),
        ScanCode(40, "ReturnScanCode"),
        ScanCode(41, "EscapeScanCode"),
        ScanCode(42, "BackspaceScanCode"),
        ScanCode(43, "TabScanCode"),
        ScanCode(44, "SpaceScanCode"),
        ScanCode(45, "MinusScanCode"),
        ScanCode(46, "EqualsScanCode"),
        ScanCode(47, "LeftBracketScanCode"),
        ScanCode(48, "RightBracketScanCode"),
        ScanCode(49, "BackslashScanCode"),
        ScanCode(50, "NonUsHashScanCode"),
        ScanCode(51, "SemicolonScanCode"),
        ScanCode(52, "ApostropheScanCode"),
        ScanCode(53, "GraveScanCode"),
        ScanCode(54, "CommaScanCode"),
        ScanCode(55, "PeriodScanCode"),
        ScanCode(56, "SlashScanCode"),
        ScanCode(57, "CapsLockScanCode"),
        ScanCode(58, "F1ScanCode"),
        ScanCode(59, "F2ScanCode"),
        ScanCode(60, "F3ScanCode"),
        ScanCode(61, "F4ScanCode"),
        ScanCode(62, "F5ScanCode"),
        ScanCode(63, "F6ScanCode"),
        ScanCode(64, "F7ScanCode"),
        ScanCode(65, "F8ScanCode"),
        ScanCode(66, "F9ScanCode"),
        ScanCode(67, "F10ScanCode"),
        ScanCode(68, "F11ScanCode"),
        ScanCode(69, "F12ScanCode"),
        ScanCode(70, "PrintScreenScanCode"),
        ScanCode(71, "ScrollLockScanCode"),
        ScanCode(72, "PauseScanCode"),
        ScanCode(73, "InsertScanCode"),
        ScanCode(74, "HomeScanCode"),
        ScanCode(75, "PageUpScanCode"),
        ScanCode(76, "DeleteScanCode"),
        ScanCode(77, "EndScanCode"),
        ScanCode(78, "PageDownScanCode"),
        ScanCode(79, "RightScanCode"),
        ScanCode(80, "LeftScanCode"),
        ScanCode(81, "DownScanCode"),
        ScanCode(82, "UpScanCode"),
        ScanCode(83, "NumLockClearScanCode"),
        ScanCode(84, "KpDivideScanCode"),
        ScanCode(85, "KpMultiplyScanCode"),
        ScanCode(86, "KpMinusScanCode"),
        ScanCode(87, "KpPlusScanCode"),
        ScanCode(88, "KpEnterScanCode"),
        ScanCode(89, "Kp1ScanCode"),
        ScanCode(90, "Kp2ScanCode"),
        ScanCode(91, "Kp3ScanCode"),
        ScanCode(92, "Kp4ScanCode"),
        ScanCode(93, "Kp5ScanCode"),
        ScanCode(94, "Kp6ScanCode"),
        ScanCode(95, "Kp7ScanCode"),
        ScanCode(96, "Kp8ScanCode"),
        ScanCode(97, "Kp9ScanCode"),
        ScanCode(98, "Kp0ScanCode"),
        ScanCode(99, "KpPeriodScanCode"),
        ScanCode(100, "NonUsBackslashScanCode"),
        ScanCode(101, "ApplicationScanCode"),
        ScanCode(102, "PowerScanCode"),
        ScanCode(103, "KpEqualsScanCode"),
        ScanCode(104, "F13ScanCode"),
        ScanCode(105, "F14ScanCode"),
        ScanCode(106, "F15ScanCode"),
        ScanCode(107, "F16ScanCode"),
        ScanCode(108, "F17ScanCode"),
        ScanCode(109, "F18ScanCode"),
        ScanCode(110, "F19ScanCode"),
        ScanCode(111, "F20ScanCode"),
        ScanCode(112, "F21ScanCode"),
        ScanCode(113, "F22ScanCode"),
        ScanCode(114, "F23ScanCode"),
        ScanCode(115, "F24ScanCode"),
        ScanCode(116, "ExecuteScanCode"),
        ScanCode(117, "HelpScanCode"),
        ScanCode(118, "MenuScanCode"),
        ScanCode(119, "SelectScanCode"),
        ScanCode(120, "StopScanCode"),
        ScanCode(121, "AgainScanCode"),
        ScanCode(122, "UndoScanCode"),
        ScanCode(123, "CutScanCode"),
        ScanCode(124, "CopyScanCode"),
        ScanCode(125, "PasteScanCode"),
        ScanCode(126, "FindScanCode"),
        ScanCode(127, "MuteScanCode"),
        ScanCode(128, "VolumeUpScanCode"),
        ScanCode(129, "VolumeDownScanCode"),
        ScanCode(133, "KpCommaScanCode"),
        ScanCode(134, "KpEqualsAS400ScanCode"),
        ScanCode(135, "International1ScanCode"),
        ScanCode(136, "International2ScanCode"),
        ScanCode(137, "International3ScanCode"),
        ScanCode(138, "International4ScanCode"),
        ScanCode(139, "International5ScanCode"),
        ScanCode(140, "International6ScanCode"),
        ScanCode(141, "International7ScanCode"),
        ScanCode(142, "International8ScanCode"),
        ScanCode(143, "International9ScanCode"),
        ScanCode(144, "Lang1ScanCode"),
        ScanCode(145, "Lang2ScanCode"),
        ScanCode(146, "Lang3ScanCode"),
        ScanCode(147, "Lang4ScanCode"),
        ScanCode(148, "Lang5ScanCode"),
        ScanCode(149, "Lang6ScanCode"),
        ScanCode(150, "Lang7ScanCode"),
        ScanCode(151, "Lang8ScanCode"),
        ScanCode(152, "Lang9ScanCode"),
        ScanCode(153, "AltEraseScanCode"),
        ScanCode(154, "SysReqScanCode"),
        ScanCode(155, "CancelScanCode"),
        ScanCode(156, "ClearScanCode"),
        ScanCode(157, "PriorScanCode"),
        ScanCode(158, "Return2ScanCode"),
        ScanCode(159, "SeparatorScanCode"),
        ScanCode(160, "OutScanCode"),
        ScanCode(161, "OperScanCode"),
        ScanCode(162, "ClearAgainScanCode"),
        ScanCode(163, "CrseScanCode"),
        ScanCode(164, "ExseLScanCode"),
        ScanCode(176, "Kp00ScanCode"),
        ScanCode(177, "Kp000ScanCode"),
        ScanCode(178, "ThousandsSeparatorScanCode"),
        ScanCode(179, "DecimalSeparatorScanCode"),
        ScanCode(180, "CurrencyUnitScanCode"),
        ScanCode(181, "CurrencySubUnitScanCode"),
        ScanCode(182, "KpLeftParenScanCode"),
        ScanCode(183, "KpRightParenScanCode"),
        ScanCode(184, "KpLeftBraceScanCode"),
        ScanCode(185, "KpRightBraceScanCode"),
        ScanCode(186, "KpTabScanCode"),
        ScanCode(187, "KpBackspaceScanCode"),
        ScanCode(188, "KpAScanCode"),
        ScanCode(189, "KpBScanCode"),
        ScanCode(190, "KpCScanCode"),
        ScanCode(191, "KpDScanCode"),
        ScanCode(192, "KpEScanCode"),
        ScanCode(193, "KpFScanCode"),
        ScanCode(194, "KpXorScanCode"),
        ScanCode(195, "KpPowerScanCode"),
        ScanCode(196, "KpPercentScanCode"),
        ScanCode(197, "KpLessScanCode"),
        ScanCode(198, "KpGreaterScanCode"),
        ScanCode(199, "KpAmpersandScanCode"),
        ScanCode(200, "KpDblAmpersandScanCode"),
        ScanCode(201, "KpVerticalBarScanCode"),
        ScanCode(202, "KpDblVerticalBarScanCode"),
        ScanCode(203, "KpColonScanCode"),
        ScanCode(204, "KpHashScanCode"),
        ScanCode(205, "KpSpaceScanCode"),
        ScanCode(206, "KpAtScanCode"),
        ScanCode(207, "KpExclamScanCode"),
        ScanCode(208, "KpMemStoreScanCode"),
        ScanCode(209, "KpMemRecallScanCode"),
        ScanCode(210, "KpMemClearScanCode"),
        ScanCode(211, "KpMemAddScanCode"),
        ScanCode(212, "KpMemSubtractScanCode"),
        ScanCode(213, "KpMemMultiplyScanCode"),
        ScanCode(214, "KpMemDivideScanCode"),
        ScanCode(215, "KpPlusMinusScanCode"),
        ScanCode(216, "KpClearScanCode"),
        ScanCode(217, "KpClearEntryScanCode"),
        ScanCode(218, "KpBinaryScanCode"),
        ScanCode(219, "KpOoctalScanCode"),
        ScanCode(220, "KpDecimalScanCode"),
        ScanCode(221, "KpHexadecimalScanCode"),
        ScanCode(224, "LCtrlScanCode"),
        ScanCode(225, "LShiftScanCode"),
        ScanCode(226, "LAltScanCode"),
        ScanCode(227, "LGuiScanCode"),
        ScanCode(228, "RCtrlScanCode"),
        ScanCode(229, "RShiftScanCode"),
        ScanCode(230, "RAltScanCode"),
        ScanCode(231, "RGuiScanCode"),
        ScanCode(257, "ModeScanCode"),
        ScanCode(258, "AudioNextScanCode"),
        ScanCode(259, "AudioPrevScanCode"),
        ScanCode(260, "AudioStopScanCode"),
        ScanCode(261, "AudioPlayScanCode"),
        ScanCode(262, "AudioMuteScanCode"),
        ScanCode(263, "MediaSelectScanCode"),
        ScanCode(264, "WwwScanCode"),
        ScanCode(265, "MailScanCode"),
        ScanCode(266, "CalculatorScanCode"),
        ScanCode(267, "ComputerScanCode"),
        ScanCode(268, "AcSearchScanCode"),
        ScanCode(269, "AcHomeScanCode"),
        ScanCode(270, "AcBackScanCode"),
        ScanCode(271, "AcForwardScanCode"),
        ScanCode(272, "AcStopScanCode"),
        ScanCode(273, "AcRefreshScanCode"),
        ScanCode(274, "AcBookmarksScanCode"),
        ScanCode(275, "BrightnessDownScanCode"),
        ScanCode(276, "BrightnessUpScanCode"),
        ScanCode(277, "DisplaySwitchScanCode"),
        ScanCode(278, "KbdIllumToggleScanCode"),
        ScanCode(279, "KbdIllumDownScanCode"),
        ScanCode(280, "KbdIllumUpScanCode"),
        ScanCode(281, "EjectScanCode"),
        ScanCode(282, "SleepScanCode"),
        ScanCode(283, "App1ScanCode"),
        ScanCode(284, "App2ScanCode"),

        ScanCode(512, "NumScanCode"),
        ];

        entries.sort();
    unsafe {
        longest_ident = entries.iter().map(|&key| key.ident().len()).max_by(|&i| i).unwrap();
    }
    try!(out.write("// This automatically generated file is used as sdl2::scancode.

use std::hash::Hash;
use std::hash::sip::SipState;

use std::num::FromPrimitive;
use std::num::ToPrimitive;

#[deriving(PartialEq, Eq, Show)]
pub enum ScanCode {
".as_bytes()));
    for &entry in entries.iter() {
        try!(out.write(format!("    {} = {},\n", entry.padded_ident(), entry.code).container_as_bytes()));
    }

    try!(out.write("
}

impl Hash for ScanCode {
	#[inline]
	fn hash(&self, state: &mut SipState) {
		self.code().hash(state);
	}
}

impl ScanCode {
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

impl ToPrimitive for ScanCode {

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

impl FromPrimitive for ScanCode {

    /// Get a *registered* scan code.
    ///
    /// This will return UnknownScanCode if an unknown code is passed.
    ///
    /// For example, `from_int(4)` will return `AScanCode`.
".as_bytes()));

	    for primitive_type in types.iter() {
        try!(out.write(format!("
    fn from_{}(n: {}) -> Option<ScanCode> \\{
        match n \\{
", *primitive_type, *primitive_type).container_as_bytes()));

        for &entry in entries.iter() {
            try!(out.write(format!("            {} => Some({}),\n", entry.code, entry.ident()).container_as_bytes()));
        }

        try!(out.write("
                _   => { Some(UnknownScanCode) }
            }
        }\n".as_bytes()));
    }

try!(out.write("
}".as_bytes()));

    try!(out.flush());
    Ok(())
}
