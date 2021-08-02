#[doc = "Register `SDIOCLKCTRL` writer"]
pub struct W(crate::W<SDIOCLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SDIOCLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SDIOCLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SDIOCLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCLK_DRV_PHASE_AW {
    #[doc = "0: 0 degree shift."]
    ENUM_0_DEG = 0,
    #[doc = "1: 90 degree shift."]
    ENUM_90_DEG = 1,
    #[doc = "2: 180 degree shift."]
    ENUM_180_DEG = 2,
    #[doc = "3: 270 degree shift."]
    ENUM_270_DEG = 3,
}
impl From<CCLK_DRV_PHASE_AW> for u8 {
    #[inline(always)]
    fn from(variant: CCLK_DRV_PHASE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CCLK_DRV_PHASE` writer - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
pub struct CCLK_DRV_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_DRV_PHASE_AW) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_AW::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_AW::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_AW::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_DRV_PHASE_AW::ENUM_270_DEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CCLK_SAMPLE_PHASE_AW {
    #[doc = "0: 0 degree shift."]
    ENUM_0_DEG = 0,
    #[doc = "1: 90 degree shift."]
    ENUM_90_DEG = 1,
    #[doc = "2: 180 degree shift."]
    ENUM_180_DEG = 2,
    #[doc = "3: 270 degree shift."]
    ENUM_270_DEG = 3,
}
impl From<CCLK_SAMPLE_PHASE_AW> for u8 {
    #[inline(always)]
    fn from(variant: CCLK_SAMPLE_PHASE_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CCLK_SAMPLE_PHASE` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub struct CCLK_SAMPLE_PHASE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_PHASE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_SAMPLE_PHASE_AW) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "0 degree shift."]
    #[inline(always)]
    pub fn enum_0_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_AW::ENUM_0_DEG)
    }
    #[doc = "90 degree shift."]
    #[inline(always)]
    pub fn enum_90_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_AW::ENUM_90_DEG)
    }
    #[doc = "180 degree shift."]
    #[inline(always)]
    pub fn enum_180_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_AW::ENUM_180_DEG)
    }
    #[doc = "270 degree shift."]
    #[inline(always)]
    pub fn enum_270_deg(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_PHASE_AW::ENUM_270_DEG)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PHASE_ACTIVE_AW {
    #[doc = "0: Bypassed."]
    BYPASSED = 0,
    #[doc = "1: Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    PH_SHIFT = 1,
}
impl From<PHASE_ACTIVE_AW> for bool {
    #[inline(always)]
    fn from(variant: PHASE_ACTIVE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PHASE_ACTIVE` writer - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
pub struct PHASE_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> PHASE_ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PHASE_ACTIVE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Bypassed."]
    #[inline(always)]
    pub fn bypassed(self) -> &'a mut W {
        self.variant(PHASE_ACTIVE_AW::BYPASSED)
    }
    #[doc = "Activates phase shift logic. When active, the clock divider is active and phase delays are enabled."]
    #[inline(always)]
    pub fn ph_shift(self) -> &'a mut W {
        self.variant(PHASE_ACTIVE_AW::PH_SHIFT)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `CCLK_DRV_DELAY` writer - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
pub struct CCLK_DRV_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Enables drive delay, as controlled by the CCLK_DRV_DELAY field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_DRV_DELAY_ACTIVE_AW {
    #[doc = "0: Disable drive delay."]
    DISABLE = 0,
    #[doc = "1: Enable drive delay."]
    ENABLE = 1,
}
impl From<CCLK_DRV_DELAY_ACTIVE_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLK_DRV_DELAY_ACTIVE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_DRV_DELAY_ACTIVE` writer - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
pub struct CCLK_DRV_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_DRV_DELAY_ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_DRV_DELAY_ACTIVE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable drive delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVE_AW::DISABLE)
    }
    #[doc = "Enable drive delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_DRV_DELAY_ACTIVE_AW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | ((value as u32 & 0x01) << 23);
        self.w
    }
}
#[doc = "Field `CCLK_SAMPLE_DELAY` writer - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
pub struct CCLK_SAMPLE_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
#[doc = "Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CCLK_SAMPLE_DELAY_ACTIVE_AW {
    #[doc = "0: Disables sample delay."]
    DISABLE = 0,
    #[doc = "1: Enables sample delay."]
    ENABLE = 1,
}
impl From<CCLK_SAMPLE_DELAY_ACTIVE_AW> for bool {
    #[inline(always)]
    fn from(variant: CCLK_SAMPLE_DELAY_ACTIVE_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLK_SAMPLE_DELAY_ACTIVE` writer - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
pub struct CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCLK_SAMPLE_DELAY_ACTIVE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CCLK_SAMPLE_DELAY_ACTIVE_AW) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disables sample delay."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVE_AW::DISABLE)
    }
    #[doc = "Enables sample delay."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(CCLK_SAMPLE_DELAY_ACTIVE_AW::ENABLE)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:1 - Programmable delay value by which cclk_in_drv is phase-shifted with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_phase(&mut self) -> CCLK_DRV_PHASE_W {
        CCLK_DRV_PHASE_W { w: self }
    }
    #[doc = "Bits 2:3 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_phase(&mut self) -> CCLK_SAMPLE_PHASE_W {
        CCLK_SAMPLE_PHASE_W { w: self }
    }
    #[doc = "Bit 7 - Enables the delays CCLK_DRV_PHASE and CCLK_SAMPLE_PHASE."]
    #[inline(always)]
    pub fn phase_active(&mut self) -> PHASE_ACTIVE_W {
        PHASE_ACTIVE_W { w: self }
    }
    #[doc = "Bits 16:20 - Programmable delay value by which cclk_in_drv is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_drv_delay(&mut self) -> CCLK_DRV_DELAY_W {
        CCLK_DRV_DELAY_W { w: self }
    }
    #[doc = "Bit 23 - Enables drive delay, as controlled by the CCLK_DRV_DELAY field."]
    #[inline(always)]
    pub fn cclk_drv_delay_active(&mut self) -> CCLK_DRV_DELAY_ACTIVE_W {
        CCLK_DRV_DELAY_ACTIVE_W { w: self }
    }
    #[doc = "Bits 24:28 - Programmable delay value by which cclk_in_sample is delayed with regard to cclk_in."]
    #[inline(always)]
    pub fn cclk_sample_delay(&mut self) -> CCLK_SAMPLE_DELAY_W {
        CCLK_SAMPLE_DELAY_W { w: self }
    }
    #[doc = "Bit 31 - Enables sample delay, as controlled by the CCLK_SAMPLE_DELAY field."]
    #[inline(always)]
    pub fn cclk_sample_delay_active(&mut self) -> CCLK_SAMPLE_DELAY_ACTIVE_W {
        CCLK_SAMPLE_DELAY_ACTIVE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO CCLKIN phase and delay control\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sdioclkctrl](index.html) module"]
pub struct SDIOCLKCTRL_SPEC;
impl crate::RegisterSpec for SDIOCLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [sdioclkctrl::W](W) writer structure"]
impl crate::Writable for SDIOCLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SDIOCLKCTRL to value 0"]
impl crate::Resettable for SDIOCLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
