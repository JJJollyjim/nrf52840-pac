#[doc = "Reader of register POFCON"]
pub type R = crate::R<u32, super::POFCON>;
#[doc = "Writer for register POFCON"]
pub type W = crate::W<u32, super::POFCON>;
#[doc = "Register POFCON `reset()`'s with value 0"]
impl crate::ResetValue for super::POFCON {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable or disable power failure warning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum POF_A {
    #[doc = "0: Disable"]
    DISABLED,
    #[doc = "1: Enable"]
    ENABLED,
}
impl From<POF_A> for bool {
    #[inline(always)]
    fn from(variant: POF_A) -> Self {
        match variant {
            POF_A::DISABLED => false,
            POF_A::ENABLED => true,
        }
    }
}
#[doc = "Reader of field `POF`"]
pub type POF_R = crate::R<bool, POF_A>;
impl POF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POF_A {
        match self.bits {
            false => POF_A::DISABLED,
            true => POF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == POF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == POF_A::ENABLED
    }
}
#[doc = "Write proxy for field `POF`"]
pub struct POF_W<'a> {
    w: &'a mut W,
}
impl<'a> POF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(POF_A::DISABLED)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(POF_A::ENABLED)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Power-fail comparator threshold setting. This setting applies both for normal voltage mode (supply connected to both VDD and VDDH) and high voltage mode (supply connected to VDDH only). Values 0-3 set threshold below 1.7 V and should not be used as brown out detection will be activated before power failure warning on such low voltages.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRESHOLD_A {
    #[doc = "4: Set threshold to 1.7 V"]
    V17,
    #[doc = "5: Set threshold to 1.8 V"]
    V18,
    #[doc = "6: Set threshold to 1.9 V"]
    V19,
    #[doc = "7: Set threshold to 2.0 V"]
    V20,
    #[doc = "8: Set threshold to 2.1 V"]
    V21,
    #[doc = "9: Set threshold to 2.2 V"]
    V22,
    #[doc = "10: Set threshold to 2.3 V"]
    V23,
    #[doc = "11: Set threshold to 2.4 V"]
    V24,
    #[doc = "12: Set threshold to 2.5 V"]
    V25,
    #[doc = "13: Set threshold to 2.6 V"]
    V26,
    #[doc = "14: Set threshold to 2.7 V"]
    V27,
    #[doc = "15: Set threshold to 2.8 V"]
    V28,
}
impl From<THRESHOLD_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLD_A) -> Self {
        match variant {
            THRESHOLD_A::V17 => 4,
            THRESHOLD_A::V18 => 5,
            THRESHOLD_A::V19 => 6,
            THRESHOLD_A::V20 => 7,
            THRESHOLD_A::V21 => 8,
            THRESHOLD_A::V22 => 9,
            THRESHOLD_A::V23 => 10,
            THRESHOLD_A::V24 => 11,
            THRESHOLD_A::V25 => 12,
            THRESHOLD_A::V26 => 13,
            THRESHOLD_A::V27 => 14,
            THRESHOLD_A::V28 => 15,
        }
    }
}
#[doc = "Reader of field `THRESHOLD`"]
pub type THRESHOLD_R = crate::R<u8, THRESHOLD_A>;
impl THRESHOLD_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, THRESHOLD_A> {
        use crate::Variant::*;
        match self.bits {
            4 => Val(THRESHOLD_A::V17),
            5 => Val(THRESHOLD_A::V18),
            6 => Val(THRESHOLD_A::V19),
            7 => Val(THRESHOLD_A::V20),
            8 => Val(THRESHOLD_A::V21),
            9 => Val(THRESHOLD_A::V22),
            10 => Val(THRESHOLD_A::V23),
            11 => Val(THRESHOLD_A::V24),
            12 => Val(THRESHOLD_A::V25),
            13 => Val(THRESHOLD_A::V26),
            14 => Val(THRESHOLD_A::V27),
            15 => Val(THRESHOLD_A::V28),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `V17`"]
    #[inline(always)]
    pub fn is_v17(&self) -> bool {
        *self == THRESHOLD_A::V17
    }
    #[doc = "Checks if the value of the field is `V18`"]
    #[inline(always)]
    pub fn is_v18(&self) -> bool {
        *self == THRESHOLD_A::V18
    }
    #[doc = "Checks if the value of the field is `V19`"]
    #[inline(always)]
    pub fn is_v19(&self) -> bool {
        *self == THRESHOLD_A::V19
    }
    #[doc = "Checks if the value of the field is `V20`"]
    #[inline(always)]
    pub fn is_v20(&self) -> bool {
        *self == THRESHOLD_A::V20
    }
    #[doc = "Checks if the value of the field is `V21`"]
    #[inline(always)]
    pub fn is_v21(&self) -> bool {
        *self == THRESHOLD_A::V21
    }
    #[doc = "Checks if the value of the field is `V22`"]
    #[inline(always)]
    pub fn is_v22(&self) -> bool {
        *self == THRESHOLD_A::V22
    }
    #[doc = "Checks if the value of the field is `V23`"]
    #[inline(always)]
    pub fn is_v23(&self) -> bool {
        *self == THRESHOLD_A::V23
    }
    #[doc = "Checks if the value of the field is `V24`"]
    #[inline(always)]
    pub fn is_v24(&self) -> bool {
        *self == THRESHOLD_A::V24
    }
    #[doc = "Checks if the value of the field is `V25`"]
    #[inline(always)]
    pub fn is_v25(&self) -> bool {
        *self == THRESHOLD_A::V25
    }
    #[doc = "Checks if the value of the field is `V26`"]
    #[inline(always)]
    pub fn is_v26(&self) -> bool {
        *self == THRESHOLD_A::V26
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLD_A::V27
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline(always)]
    pub fn is_v28(&self) -> bool {
        *self == THRESHOLD_A::V28
    }
}
#[doc = "Write proxy for field `THRESHOLD`"]
pub struct THRESHOLD_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLD_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THRESHOLD_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Set threshold to 1.7 V"]
    #[inline(always)]
    pub fn v17(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V17)
    }
    #[doc = "Set threshold to 1.8 V"]
    #[inline(always)]
    pub fn v18(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V18)
    }
    #[doc = "Set threshold to 1.9 V"]
    #[inline(always)]
    pub fn v19(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V19)
    }
    #[doc = "Set threshold to 2.0 V"]
    #[inline(always)]
    pub fn v20(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V20)
    }
    #[doc = "Set threshold to 2.1 V"]
    #[inline(always)]
    pub fn v21(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V21)
    }
    #[doc = "Set threshold to 2.2 V"]
    #[inline(always)]
    pub fn v22(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V22)
    }
    #[doc = "Set threshold to 2.3 V"]
    #[inline(always)]
    pub fn v23(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V23)
    }
    #[doc = "Set threshold to 2.4 V"]
    #[inline(always)]
    pub fn v24(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V24)
    }
    #[doc = "Set threshold to 2.5 V"]
    #[inline(always)]
    pub fn v25(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V25)
    }
    #[doc = "Set threshold to 2.6 V"]
    #[inline(always)]
    pub fn v26(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V26)
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn v28(self) -> &'a mut W {
        self.variant(THRESHOLD_A::V28)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 1)) | (((value as u32) & 0x0f) << 1);
        self.w
    }
}
#[doc = "Power-fail comparator threshold setting for high voltage mode (supply connected to VDDH only). This setting does not apply for normal voltage mode (supply connected to both VDD and VDDH).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum THRESHOLDVDDH_A {
    #[doc = "0: Set threshold to 2.7 V"]
    V27,
    #[doc = "1: Set threshold to 2.8 V"]
    V28,
    #[doc = "2: Set threshold to 2.9 V"]
    V29,
    #[doc = "3: Set threshold to 3.0 V"]
    V30,
    #[doc = "4: Set threshold to 3.1 V"]
    V31,
    #[doc = "5: Set threshold to 3.2 V"]
    V32,
    #[doc = "6: Set threshold to 3.3 V"]
    V33,
    #[doc = "7: Set threshold to 3.4 V"]
    V34,
    #[doc = "8: Set threshold to 3.5 V"]
    V35,
    #[doc = "9: Set threshold to 3.6 V"]
    V36,
    #[doc = "10: Set threshold to 3.7 V"]
    V37,
    #[doc = "11: Set threshold to 3.8 V"]
    V38,
    #[doc = "12: Set threshold to 3.9 V"]
    V39,
    #[doc = "13: Set threshold to 4.0 V"]
    V40,
    #[doc = "14: Set threshold to 4.1 V"]
    V41,
    #[doc = "15: Set threshold to 4.2 V"]
    V42,
}
impl From<THRESHOLDVDDH_A> for u8 {
    #[inline(always)]
    fn from(variant: THRESHOLDVDDH_A) -> Self {
        match variant {
            THRESHOLDVDDH_A::V27 => 0,
            THRESHOLDVDDH_A::V28 => 1,
            THRESHOLDVDDH_A::V29 => 2,
            THRESHOLDVDDH_A::V30 => 3,
            THRESHOLDVDDH_A::V31 => 4,
            THRESHOLDVDDH_A::V32 => 5,
            THRESHOLDVDDH_A::V33 => 6,
            THRESHOLDVDDH_A::V34 => 7,
            THRESHOLDVDDH_A::V35 => 8,
            THRESHOLDVDDH_A::V36 => 9,
            THRESHOLDVDDH_A::V37 => 10,
            THRESHOLDVDDH_A::V38 => 11,
            THRESHOLDVDDH_A::V39 => 12,
            THRESHOLDVDDH_A::V40 => 13,
            THRESHOLDVDDH_A::V41 => 14,
            THRESHOLDVDDH_A::V42 => 15,
        }
    }
}
#[doc = "Reader of field `THRESHOLDVDDH`"]
pub type THRESHOLDVDDH_R = crate::R<u8, THRESHOLDVDDH_A>;
impl THRESHOLDVDDH_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> THRESHOLDVDDH_A {
        match self.bits {
            0 => THRESHOLDVDDH_A::V27,
            1 => THRESHOLDVDDH_A::V28,
            2 => THRESHOLDVDDH_A::V29,
            3 => THRESHOLDVDDH_A::V30,
            4 => THRESHOLDVDDH_A::V31,
            5 => THRESHOLDVDDH_A::V32,
            6 => THRESHOLDVDDH_A::V33,
            7 => THRESHOLDVDDH_A::V34,
            8 => THRESHOLDVDDH_A::V35,
            9 => THRESHOLDVDDH_A::V36,
            10 => THRESHOLDVDDH_A::V37,
            11 => THRESHOLDVDDH_A::V38,
            12 => THRESHOLDVDDH_A::V39,
            13 => THRESHOLDVDDH_A::V40,
            14 => THRESHOLDVDDH_A::V41,
            15 => THRESHOLDVDDH_A::V42,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `V27`"]
    #[inline(always)]
    pub fn is_v27(&self) -> bool {
        *self == THRESHOLDVDDH_A::V27
    }
    #[doc = "Checks if the value of the field is `V28`"]
    #[inline(always)]
    pub fn is_v28(&self) -> bool {
        *self == THRESHOLDVDDH_A::V28
    }
    #[doc = "Checks if the value of the field is `V29`"]
    #[inline(always)]
    pub fn is_v29(&self) -> bool {
        *self == THRESHOLDVDDH_A::V29
    }
    #[doc = "Checks if the value of the field is `V30`"]
    #[inline(always)]
    pub fn is_v30(&self) -> bool {
        *self == THRESHOLDVDDH_A::V30
    }
    #[doc = "Checks if the value of the field is `V31`"]
    #[inline(always)]
    pub fn is_v31(&self) -> bool {
        *self == THRESHOLDVDDH_A::V31
    }
    #[doc = "Checks if the value of the field is `V32`"]
    #[inline(always)]
    pub fn is_v32(&self) -> bool {
        *self == THRESHOLDVDDH_A::V32
    }
    #[doc = "Checks if the value of the field is `V33`"]
    #[inline(always)]
    pub fn is_v33(&self) -> bool {
        *self == THRESHOLDVDDH_A::V33
    }
    #[doc = "Checks if the value of the field is `V34`"]
    #[inline(always)]
    pub fn is_v34(&self) -> bool {
        *self == THRESHOLDVDDH_A::V34
    }
    #[doc = "Checks if the value of the field is `V35`"]
    #[inline(always)]
    pub fn is_v35(&self) -> bool {
        *self == THRESHOLDVDDH_A::V35
    }
    #[doc = "Checks if the value of the field is `V36`"]
    #[inline(always)]
    pub fn is_v36(&self) -> bool {
        *self == THRESHOLDVDDH_A::V36
    }
    #[doc = "Checks if the value of the field is `V37`"]
    #[inline(always)]
    pub fn is_v37(&self) -> bool {
        *self == THRESHOLDVDDH_A::V37
    }
    #[doc = "Checks if the value of the field is `V38`"]
    #[inline(always)]
    pub fn is_v38(&self) -> bool {
        *self == THRESHOLDVDDH_A::V38
    }
    #[doc = "Checks if the value of the field is `V39`"]
    #[inline(always)]
    pub fn is_v39(&self) -> bool {
        *self == THRESHOLDVDDH_A::V39
    }
    #[doc = "Checks if the value of the field is `V40`"]
    #[inline(always)]
    pub fn is_v40(&self) -> bool {
        *self == THRESHOLDVDDH_A::V40
    }
    #[doc = "Checks if the value of the field is `V41`"]
    #[inline(always)]
    pub fn is_v41(&self) -> bool {
        *self == THRESHOLDVDDH_A::V41
    }
    #[doc = "Checks if the value of the field is `V42`"]
    #[inline(always)]
    pub fn is_v42(&self) -> bool {
        *self == THRESHOLDVDDH_A::V42
    }
}
#[doc = "Write proxy for field `THRESHOLDVDDH`"]
pub struct THRESHOLDVDDH_W<'a> {
    w: &'a mut W,
}
impl<'a> THRESHOLDVDDH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: THRESHOLDVDDH_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Set threshold to 2.7 V"]
    #[inline(always)]
    pub fn v27(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V27)
    }
    #[doc = "Set threshold to 2.8 V"]
    #[inline(always)]
    pub fn v28(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V28)
    }
    #[doc = "Set threshold to 2.9 V"]
    #[inline(always)]
    pub fn v29(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V29)
    }
    #[doc = "Set threshold to 3.0 V"]
    #[inline(always)]
    pub fn v30(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V30)
    }
    #[doc = "Set threshold to 3.1 V"]
    #[inline(always)]
    pub fn v31(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V31)
    }
    #[doc = "Set threshold to 3.2 V"]
    #[inline(always)]
    pub fn v32(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V32)
    }
    #[doc = "Set threshold to 3.3 V"]
    #[inline(always)]
    pub fn v33(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V33)
    }
    #[doc = "Set threshold to 3.4 V"]
    #[inline(always)]
    pub fn v34(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V34)
    }
    #[doc = "Set threshold to 3.5 V"]
    #[inline(always)]
    pub fn v35(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V35)
    }
    #[doc = "Set threshold to 3.6 V"]
    #[inline(always)]
    pub fn v36(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V36)
    }
    #[doc = "Set threshold to 3.7 V"]
    #[inline(always)]
    pub fn v37(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V37)
    }
    #[doc = "Set threshold to 3.8 V"]
    #[inline(always)]
    pub fn v38(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V38)
    }
    #[doc = "Set threshold to 3.9 V"]
    #[inline(always)]
    pub fn v39(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V39)
    }
    #[doc = "Set threshold to 4.0 V"]
    #[inline(always)]
    pub fn v40(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V40)
    }
    #[doc = "Set threshold to 4.1 V"]
    #[inline(always)]
    pub fn v41(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V41)
    }
    #[doc = "Set threshold to 4.2 V"]
    #[inline(always)]
    pub fn v42(self) -> &'a mut W {
        self.variant(THRESHOLDVDDH_A::V42)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable power failure warning"]
    #[inline(always)]
    pub fn pof(&self) -> POF_R {
        POF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting. This setting applies both for normal voltage mode (supply connected to both VDD and VDDH) and high voltage mode (supply connected to VDDH only). Values 0-3 set threshold below 1.7 V and should not be used as brown out detection will be activated before power failure warning on such low voltages."]
    #[inline(always)]
    pub fn threshold(&self) -> THRESHOLD_R {
        THRESHOLD_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Power-fail comparator threshold setting for high voltage mode (supply connected to VDDH only). This setting does not apply for normal voltage mode (supply connected to both VDD and VDDH)."]
    #[inline(always)]
    pub fn thresholdvddh(&self) -> THRESHOLDVDDH_R {
        THRESHOLDVDDH_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable power failure warning"]
    #[inline(always)]
    pub fn pof(&mut self) -> POF_W {
        POF_W { w: self }
    }
    #[doc = "Bits 1:4 - Power-fail comparator threshold setting. This setting applies both for normal voltage mode (supply connected to both VDD and VDDH) and high voltage mode (supply connected to VDDH only). Values 0-3 set threshold below 1.7 V and should not be used as brown out detection will be activated before power failure warning on such low voltages."]
    #[inline(always)]
    pub fn threshold(&mut self) -> THRESHOLD_W {
        THRESHOLD_W { w: self }
    }
    #[doc = "Bits 8:11 - Power-fail comparator threshold setting for high voltage mode (supply connected to VDDH only). This setting does not apply for normal voltage mode (supply connected to both VDD and VDDH)."]
    #[inline(always)]
    pub fn thresholdvddh(&mut self) -> THRESHOLDVDDH_W {
        THRESHOLDVDDH_W { w: self }
    }
}
