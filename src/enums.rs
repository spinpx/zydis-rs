//! Contains enum definitions and some utility functions on them.
#![allow(non_camel_case_types)]

mod generated;

pub use self::generated::*;
use super::ffi;
use bitflags::bitflags;
use core::fmt;
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};

/// Maximum length of an instruction in bytes.
pub const MAX_INSTRUCTION_LENGTH: usize = 15;

/// Maximum number of operands (visible and hidden).
pub const MAX_OPERAND_COUNT: usize = 10;

/// Maximum number of visible operands.
pub const MAX_OPERAND_COUNT_VISIBLE: usize = 5;

/// Maximum number of instruction segments.
pub(crate) const MAX_INSTRUCTION_SEGMENT_COUNT: usize = 9;

/// Maximum number of encoder operands.
pub const ENCODER_MAX_OPERANDS: usize = 5;

impl Mnemonic {
    /// Returns the static string corresponding to this mnemonic.
    ///
    /// # Examples
    /// ```
    /// use zydis::Mnemonic;
    /// let str = Mnemonic::CMOVP.static_string().unwrap();
    /// assert_eq!("cmovp", str);
    /// ```
    pub fn static_string(self) -> Option<&'static str> {
        unsafe { check_string!(ffi::ZydisMnemonicGetString(self)) }
    }

    #[doc(hidden)]
    #[deprecated(since = "4.0.0", note = "use `static_string()` instead")]
    pub fn get_string(self) -> Option<&'static str> {
        self.static_string()
    }
}

impl fmt::Display for Mnemonic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.static_string().ok_or(fmt::Error)?)
    }
}

pub type RegisterWidth = u16;

impl Register {
    /// Returns the ID of this register.
    ///
    /// # Examples
    /// ```
    /// use zydis::Register;
    /// assert_eq!(0, Register::RAX.id());
    /// ```
    pub fn id(self) -> u8 {
        unsafe { ffi::ZydisRegisterGetId(self) as u8 }
    }

    #[doc(hidden)]
    #[deprecated(since = "4.0.0", note = "use `id()` instead")]
    pub fn get_id(self) -> u8 {
        self.id()
    }

    /// Returns the register-class of this register.
    ///
    /// # Examples
    /// ```
    /// use zydis::{Register, RegisterClass};
    ///
    /// let class = Register::ECX.class();
    /// assert_eq!(RegisterClass::GPR32, class);
    /// ```
    pub fn class(self) -> RegisterClass {
        unsafe { ffi::ZydisRegisterGetClass(self) }
    }

    #[doc(hidden)]
    #[deprecated(since = "4.0.0", note = "use `class()` instead")]
    pub fn get_class(self) -> RegisterClass {
        self.class()
    }

    /// Returns the textual representation of this register.
    ///
    /// # Examples
    /// ```
    /// use zydis::Register;
    ///
    /// let str = Register::EAX.static_string().unwrap();
    /// assert_eq!("eax", str);
    /// ```
    pub fn static_string(self) -> Option<&'static str> {
        unsafe { check_string!(ffi::ZydisRegisterGetString(self)) }
    }

    #[doc(hidden)]
    #[deprecated(since = "4.0.0", note = "use `static_string()` instead")]
    pub fn get_string(self) -> Option<&'static str> {
        self.static_string()
    }

    /// Returns the width of this register, in bits.
    ///
    /// # Examples
    /// ```
    /// use zydis::{MachineMode, Register};
    ///
    /// let width = Register::DR0.width(MachineMode::LEGACY_32);
    /// assert_eq!(32, width);
    /// ```
    pub fn width(self, mode: MachineMode) -> RegisterWidth {
        unsafe { ffi::ZydisRegisterGetWidth(mode, self) }
    }

    #[doc(hidden)]
    #[deprecated(since = "4.0.0", note = "use `width()` instead")]
    pub fn get_width(self, mode: MachineMode) -> RegisterWidth {
        self.width(mode)
    }

    /// Returns the largest enclosing register of the given register.
    ///
    /// # Examples
    /// ```
    /// use zydis::{MachineMode, Register};
    ///
    /// let reg = Register::EAX.largest_enclosing(MachineMode::LONG_64);
    /// assert_eq!(reg, Register::RAX);
    /// ```
    pub fn largest_enclosing(self, mode: MachineMode) -> Register {
        unsafe { ffi::ZydisRegisterGetLargestEnclosing(mode, self) }
    }

    #[doc(hidden)]
    #[deprecated(since = "4.0.0", note = "use `largest_enclosing()` instead")]
    pub fn get_largest_enclosing(self, mode: MachineMode) -> Register {
        self.largest_enclosing(mode)
    }
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.static_string().ok_or(fmt::Error)?)
    }
}

impl RegisterClass {
    /// Returns the register specified by this register class and `id`.
    ///
    /// # Examples
    /// ```
    /// use zydis::{Register, RegisterClass};
    /// let eax = RegisterClass::GPR32.encode(0);
    /// assert_eq!(Register::EAX, eax);
    /// ```
    pub fn encode(self, id: u8) -> Register {
        unsafe { ffi::ZydisRegisterEncode(self, id) }
    }

    /// Returns the width of the specified register-class.
    pub fn width(self, mode: MachineMode) -> RegisterWidth {
        unsafe { ffi::ZydisRegisterClassGetWidth(mode, self) }
    }

    #[doc(hidden)]
    #[deprecated(since = "4.0.0", note = "use `width()` instead")]
    pub fn get_width(self, mode: MachineMode) -> RegisterWidth {
        self.width(mode)
    }
}

/// The type of a formatter token.
#[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct Token(pub u8);

pub const TOKEN_INVALID: Token = Token(0x0);
pub const TOKEN_WHITESPACE: Token = Token(0x1);
pub const TOKEN_DELIMITER: Token = Token(0x2);
pub const TOKEN_PARENTHESIS_OPEN: Token = Token(0x3);
pub const TOKEN_PARENTHESIS_CLOSE: Token = Token(0x4);
pub const TOKEN_PREFIX: Token = Token(0x5);
pub const TOKEN_MNEMONIC: Token = Token(0x6);
pub const TOKEN_REGISTER: Token = Token(0x7);
pub const TOKEN_ADDRESS_ABS: Token = Token(0x8);
pub const TOKEN_ADDRESS_REL: Token = Token(0x9);
pub const TOKEN_DISPLACEMENT: Token = Token(0xA);
pub const TOKEN_IMMEDIATE: Token = Token(0xB);
pub const TOKEN_TYPECAST: Token = Token(0xC);
pub const TOKEN_DECORATOR: Token = Token(0xD);
pub const TOKEN_SYMBOL: Token = Token(0xE);
/// The base for user defined tokens.
pub const TOKEN_USER: Token = Token(0x80);

static TOKEN_NAMES: [&str; 0xF] = [
    "invalid",
    "whitespace",
    "delimiter",
    "opening parenthesis",
    "closing parenthesis",
    "prefix",
    "mnemonic",
    "register",
    "absolute address",
    "relative address",
    "displacement",
    "immediate",
    "typecast",
    "decorator",
    "symbol",
];

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if usize::from(self.0) < TOKEN_NAMES.len() {
            f.write_str(TOKEN_NAMES[self.0 as usize])
        } else if self.0 >= TOKEN_USER.0 {
            write!(f, "<user token {:02X}>", self.0)
        } else {
            write!(f, "<unknown>")
        }
    }
}

bitflags! {
    /// Describes how an operand is accessed.
    #[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct OperandAction: u32 {
        const READ = 1 << 0;
        const WRITE = 1 << 1;
        const CONDREAD = 1 << 2;
        const CONDWRITE = 1 << 3;

        const READWRITE = Self::READ.bits() | Self::WRITE.bits();
        const CONDREAD_CONDWRITE = Self::CONDREAD.bits() | Self::CONDWRITE.bits();
        const READ_CONDWRITE = Self::READ.bits() | Self::CONDWRITE.bits();
        const CONDREAD_WRITE = Self::CONDREAD.bits() | Self::WRITE.bits();
        const MASK_READ = Self::CONDREAD.bits() | Self::READ.bits();
        const MASK_WRITE = Self::CONDWRITE.bits() | Self::WRITE.bits();
    }

    /// Identifies a CPU flag.
    #[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct CpuFlag: u32 {
        const CF = 1 <<  0;
        const PF = 1 <<  2;
        const AF = 1 <<  4;
        const ZF = 1 <<  6;
        const SF = 1 <<  7;
        const TF = 1 <<  8;
        const IF = 1 <<  9;
        const DF = 1 << 10;
        const OF = 1 << 11;
        const IOPL = 1 << 12;
        const NT = 1 << 14;
        const RF = 1 << 16;
        const VM = 1 << 17;
        const AC = 1 << 18;
        const VIF = 1 << 19;
        const VIP = 1 << 20;
        const ID = 1 << 21;
    }

    /// Identifies an FPU flag.
    #[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct FpuFlag: u32 {
        const C0 = 1 << 0;
        const C1 = 1 << 1;
        const C2 = 1 << 2;
        const C3 = 1 << 3;
    }

    /// Attributes of an operand.
    #[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct OperandAttributes: u8 {
        const IS_MULTISOURCE4 = 1 << 0;
    }

    /// Attributes of an instruction.
    #[cfg_attr(feature = "serialization", derive(Deserialize, Serialize))]
    #[repr(transparent)]
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct InstructionAttributes: u64 {
        const HAS_MODRM = 1 << 0;
        const HAS_SIB = 1 << 1;
        const HAS_REX = 1 << 2;
        const HAS_XOP = 1 << 3;
        const HAS_VEX = 1 << 4;
        const HAS_EVEX = 1 << 5;
        const HAS_MVEX = 1 << 6;
        const IS_RELATIVE = 1 << 7;
        const IS_PRIVILEGED = 1 << 8;

        const CPUFLAG_ACCESS = 1 << 9;
        const CPU_STATE_CR = 1 << 10;
        const CPU_STATE_CW = 1 << 11;
        const FPU_STATE_CR = 1 << 12;
        const FPU_STATE_CW = 1 << 13;
        const XMM_STATE_CR = 1 << 14;
        const XMM_STATE_CW = 1 << 15;

        const ACCEPTS_LOCK = 1 << 16;
        const ACCEPTS_REP = 1 << 17;
        const ACCEPTS_REPE = 1 << 18;
        const ACCEPTS_REPZ = Self::ACCEPTS_REPE.bits();
        const ACCEPTS_REPNE = 1 << 19;
        const ACCEPTS_REPNZ = Self::ACCEPTS_REPNE.bits();
        const ACCEPTS_BND = 1 << 20;
        const ACCEPTS_XACQUIRE = 1 << 21;
        const ACCEPTS_XRELEASE = 1 << 22;
        const ACCEPTS_HLE_WITHOUT_LOCK = 1 << 23;
        const ACCEPTS_BRANCH_HINTS = 1 << 24;
        const ACCEPTS_NOTRACK = 1 << 25;
        const ACCEPTS_SEGMENT = 1 << 26;
        const HAS_LOCK = 1 << 27;
        const HAS_REP = 1 << 28;
        const HAS_REPE = 1 << 29;
        const HAS_REPZ = Self::HAS_REPE.bits();
        const HAS_REPNE = 1 << 30;
        const HAS_REPNZ = Self::HAS_REPNE.bits();
        const HAS_BND = 1 << 31;
        const HAS_XACQUIRE = 1 << 32;
        const HAS_XRELEASE = 1 << 33;
        const HAS_BRANCH_NOT_TAKEN = 1 << 34;
        const HAS_BRANCH_TAKEN = 1 << 35;
        const HAS_NOTRACK = 1 << 36;
        const HAS_SEGMENT_CS = 1 << 37;
        const HAS_SEGMENT_SS = 1 << 38;
        const HAS_SEGMENT_DS = 1 << 39;
        const HAS_SEGMENT_ES = 1 << 40;
        const HAS_SEGMENT_FS = 1 << 41;
        const HAS_SEGMENT_GS = 1 << 42;
        const HAS_SEGMENT
            = Self::HAS_SEGMENT_CS.bits()
            | Self::HAS_SEGMENT_SS.bits()
            | Self::HAS_SEGMENT_DS.bits()
            | Self::HAS_SEGMENT_ES.bits()
            | Self::HAS_SEGMENT_FS.bits()
            | Self::HAS_SEGMENT_GS.bits();
        const HAS_OPERANDSIZE = 1 << 43;
        const HAS_ADDRESSIZE = 1 << 44;
        const HAS_EVEX_B = 1 << 45;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(feature = "full-decoder")]
    fn test_encoding() {
        // TODO: move this test case to decoder?

        use crate::*;
        const CODE: &[u8] = &[0xE8, 0xFB, 0xFF, 0xFF, 0xFF];
        let decoder = Decoder::new32();
        let insn = decoder.decode_first::<AllOperands>(CODE).unwrap().unwrap();
        assert_eq!(insn.operands()[0].encoding, OperandEncoding::JIMM16_32_32);
    }
}
