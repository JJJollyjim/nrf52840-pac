#[doc = "Reader of register LFCLKSRC"]
pub type R = crate::R<u32, super::LFCLKSRC>;
#[doc = "Writer for register LFCLKSRC"]
pub type W = crate::W<u32, super::LFCLKSRC>;
#[doc = "Register LFCLKSRC `reset()`'s with value 0"]
impl crate::ResetValue for super::LFCLKSRC {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SRC_A {
    #[doc = "0: 32.768 kHz RC oscillator (LFRC)"]
    RC,
    #[doc = "1: 32.768 kHz crystal oscillator (LFXO)"]
    XTAL,
    #[doc = "2: 32.768 kHz synthesized from HFCLK (LFSYNT)"]
    SYNTH,
}
impl From<SRC_A> for u8 {
    #[inline(always)]
    fn from(variant: SRC_A) -> Self {
        match variant {
            SRC_A::RC => 0,
            SRC_A::XTAL => 1,
            SRC_A::SYNTH => 2,
        }
    }
}
#[doc = "Reader of field `SRC`"]
pub type SRC_R = crate::R<u8, SRC_A>;
impl SRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SRC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SRC_A::RC),
            1 => Val(SRC_A::XTAL),
            2 => Val(SRC_A::SYNTH),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RC`"]
    #[inline(always)]
    pub fn is_rc(&self) -> bool {
        *self == SRC_A::RC
    }
    #[doc = "Checks if the value of the field is `XTAL`"]
    #[inline(always)]
    pub fn is_xtal(&self) -> bool {
        *self == SRC_A::XTAL
    }
    #[doc = "Checks if the value of the field is `SYNTH`"]
    #[inline(always)]
    pub fn is_synth(&self) -> bool {
        *self == SRC_A::SYNTH
    }
}
#[doc = "Write proxy for field `SRC`"]
pub struct SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> SRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SRC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "32.768 kHz RC oscillator (LFRC)"]
    #[inline(always)]
    pub fn rc(self) -> &'a mut W {
        self.variant(SRC_A::RC)
    }
    #[doc = "32.768 kHz crystal oscillator (LFXO)"]
    #[inline(always)]
    pub fn xtal(self) -> &'a mut W {
        self.variant(SRC_A::XTAL)
    }
    #[doc = "32.768 kHz synthesized from HFCLK (LFSYNT)"]
    #[inline(always)]
    pub fn synth(self) -> &'a mut W {
        self.variant(SRC_A::SYNTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "Enable or disable bypass of LFCLK crystal oscillator with external clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BYPASS_A {
    #[doc = "0: Disable (use with Xtal or low-swing external source)"]
    DISABLED,
    #[doc = "1: Enable (use with rail-to-rail external source)"]
    ENABLED,
}
impl From<BYPASS_A> for bool {
    #[inline(always)]
    fn from(variant: BYPASS_A) -> Self {
        match variant {
            BYPASS_A::DISABLED => false,
            BYPASS_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `BYPASS`"]
pub type BYPASS_R = crate::R<bool, BYPASS_A>;
impl BYPASS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BYPASS_A {
        match self.bits {
            false => BYPASS_A::DISABLED,
            true => BYPASS_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BYPASS_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BYPASS_A::ENABLED
    }
}
#[doc = "Write proxy for field `BYPASS`"]
pub struct BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BYPASS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: BYPASS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable (use with Xtal or low-swing external source)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(BYPASS_A::DISABLED)
    }
    #[doc = "Enable (use with rail-to-rail external source)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(BYPASS_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Enable or disable external source for LFCLK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EXTERNAL_A {
    #[doc = "0: Disable external source (use with Xtal)"]
    DISABLED,
    #[doc = "1: Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    ENABLED,
}
impl From<EXTERNAL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTERNAL_A) -> Self {
        match variant {
            EXTERNAL_A::DISABLED => false,
            EXTERNAL_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `EXTERNAL`"]
pub type EXTERNAL_R = crate::R<bool, EXTERNAL_A>;
impl EXTERNAL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTERNAL_A {
        match self.bits {
            false => EXTERNAL_A::DISABLED,
            true => EXTERNAL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EXTERNAL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EXTERNAL_A::ENABLED
    }
}
#[doc = "Write proxy for field `EXTERNAL`"]
pub struct EXTERNAL_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTERNAL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTERNAL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable external source (use with Xtal)"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EXTERNAL_A::DISABLED)
    }
    #[doc = "Enable use of external source instead of Xtal (SRC needs to be set to Xtal)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EXTERNAL_A::ENABLED)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&self) -> SRC_R {
        SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&self) -> BYPASS_R {
        BYPASS_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source"]
    #[inline(always)]
    pub fn src(&mut self) -> SRC_W {
        SRC_W { w: self }
    }
    #[doc = "Bit 16 - Enable or disable bypass of LFCLK crystal oscillator with external clock source"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BYPASS_W {
        BYPASS_W { w: self }
    }
    #[doc = "Bit 17 - Enable or disable external source for LFCLK"]
    #[inline(always)]
    pub fn external(&mut self) -> EXTERNAL_W {
        EXTERNAL_W { w: self }
    }
}
