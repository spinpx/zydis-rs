//! Binary instruction decoding.

use gen::*;
use status::ZydisResult;
use std::mem::uninitialized;

pub struct Decoder {
    decoder: ZydisDecoder,
}

impl Decoder {
    pub fn new_ex(
        machine_mode: ZydisMachineModes,
        address_width: ZydisAddressWidths,
        granularity: ZydisDecodeGranularities,
    ) -> ZydisResult<Decoder> {
        unsafe {
            let mut decoder = uninitialized();
            check!(
                ZydisDecoderInitEx(
                    &mut decoder,
                    machine_mode as _,
                    address_width as _,
                    granularity as _
                ),
                Decoder { decoder }
            )
        }
    }

    pub fn new(
        machine_mode: ZydisMachineModes,
        address_width: ZydisAddressWidths,
    ) -> ZydisResult<Decoder> {
        Decoder::new_ex(
            machine_mode,
            address_width,
            ZYDIS_DECODE_GRANULARITY_DEFAULT,
        )
    }

    /// Decodes a binary instruction to `ZydisDecodedInstruction`, taking
    /// additional flags.
    ///
    /// # Examples
    ///
    /// ```
    /// static INT3: &'static [u8] = &[0xCCu8];
    /// let mut decoder = zydis::Decoder::new(
    ///     zydis::gen::ZYDIS_MACHINE_MODE_LONG_64,
    ///     zydis::gen::ZYDIS_ADDRESS_WIDTH_64
    /// ).unwrap();
    /// let info = decoder.decode(INT3, 0x00400000).unwrap();
    /// assert_eq!(info.mnemonic as u32, zydis::gen::ZYDIS_MNEMONIC_INT3);
    /// ```
    pub fn decode(
        &self,
        buffer: &[u8],
        instruction_pointer: u64,
    ) -> ZydisResult<ZydisDecodedInstruction> {
        unsafe {
            let mut info: ZydisDecodedInstruction = uninitialized();
            check!(
                ZydisDecoderDecodeBuffer(
                    &self.decoder,
                    buffer.as_ptr() as _,
                    buffer.len(),
                    instruction_pointer,
                    &mut info
                ),
                info
            )
        }
    }
}
