#[doc = "Register `TIMER1CAPTSEL[%s]` reader"]
pub struct R(crate::R<TIMER1CAPTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1CAPTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1CAPTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1CAPTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER1CAPTSEL[%s]` writer"]
pub struct W(crate::W<TIMER1CAPTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER1CAPTSEL_SPEC>;
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
impl From<crate::W<TIMER1CAPTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER1CAPTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Input number to TIMER1 capture inputs 0 to 4\n\nValue on reset: 31"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CAPTSEL_A {
    #[doc = "0: CT_INP0 function selected from IOCON register"]
    VAL0 = 0,
    #[doc = "1: CT_INP1 function selected from IOCON register"]
    VAL1 = 1,
    #[doc = "2: CT_INP2 function selected from IOCON register"]
    VAL2 = 2,
    #[doc = "3: CT_INP3 function selected from IOCON register"]
    VAL3 = 3,
    #[doc = "4: CT_INP4 function selected from IOCON register"]
    VAL4 = 4,
    #[doc = "5: CT_INP5 function selected from IOCON register"]
    VAL5 = 5,
    #[doc = "6: CT_INP6 function selected from IOCON register"]
    VAL6 = 6,
    #[doc = "7: CT_INP7 function selected from IOCON register"]
    VAL7 = 7,
    #[doc = "8: CT_INP8 function selected from IOCON register"]
    VAL8 = 8,
    #[doc = "9: CT_INP9 function selected from IOCON register"]
    VAL9 = 9,
    #[doc = "10: CT_INP10 function selected from IOCON register"]
    VAL10 = 10,
    #[doc = "11: CT_INP11 function selected from IOCON register"]
    VAL11 = 11,
    #[doc = "12: CT_INP12 function selected from IOCON register"]
    VAL12 = 12,
    #[doc = "13: CT_INP13 function selected from IOCON register"]
    VAL13 = 13,
    #[doc = "14: CT_INP14 function selected from IOCON register"]
    VAL14 = 14,
    #[doc = "15: CT_INP15 function selected from IOCON register"]
    VAL15 = 15,
    #[doc = "16: CT_INP16 function selected from IOCON register"]
    VAL16 = 16,
    #[doc = "17: None"]
    VAL17 = 17,
    #[doc = "18: None"]
    VAL18 = 18,
    #[doc = "19: None"]
    VAL19 = 19,
    #[doc = "20: USB0_FRAME_TOGGLE"]
    VAL20 = 20,
    #[doc = "21: USB1_FRAME_TOGGLE"]
    VAL21 = 21,
    #[doc = "22: COMP_OUTPUT output from analog comparator"]
    VAL22 = 22,
    #[doc = "23: I2S_SHARED_WS\\[0\\]
output from I2S pin sharing"]
    VAL23 = 23,
    #[doc = "24: I2S_SHARED_WS\\[1\\]
output from I2S pin sharing"]
    VAL24 = 24,
    #[doc = "25: None"]
    VAL25 = 25,
    #[doc = "26: None"]
    VAL26 = 26,
    #[doc = "27: None"]
    VAL27 = 27,
    #[doc = "28: None"]
    VAL28 = 28,
    #[doc = "29: None"]
    VAL29 = 29,
    #[doc = "30: None"]
    VAL30 = 30,
    #[doc = "31: None"]
    VAL31 = 31,
}
impl From<CAPTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CAPTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CAPTSEL` reader - Input number to TIMER1 capture inputs 0 to 4"]
pub struct CAPTSEL_R(crate::FieldReader<u8, CAPTSEL_A>);
impl CAPTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CAPTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTSEL_A {
        match self.bits {
            0 => CAPTSEL_A::VAL0,
            1 => CAPTSEL_A::VAL1,
            2 => CAPTSEL_A::VAL2,
            3 => CAPTSEL_A::VAL3,
            4 => CAPTSEL_A::VAL4,
            5 => CAPTSEL_A::VAL5,
            6 => CAPTSEL_A::VAL6,
            7 => CAPTSEL_A::VAL7,
            8 => CAPTSEL_A::VAL8,
            9 => CAPTSEL_A::VAL9,
            10 => CAPTSEL_A::VAL10,
            11 => CAPTSEL_A::VAL11,
            12 => CAPTSEL_A::VAL12,
            13 => CAPTSEL_A::VAL13,
            14 => CAPTSEL_A::VAL14,
            15 => CAPTSEL_A::VAL15,
            16 => CAPTSEL_A::VAL16,
            17 => CAPTSEL_A::VAL17,
            18 => CAPTSEL_A::VAL18,
            19 => CAPTSEL_A::VAL19,
            20 => CAPTSEL_A::VAL20,
            21 => CAPTSEL_A::VAL21,
            22 => CAPTSEL_A::VAL22,
            23 => CAPTSEL_A::VAL23,
            24 => CAPTSEL_A::VAL24,
            25 => CAPTSEL_A::VAL25,
            26 => CAPTSEL_A::VAL26,
            27 => CAPTSEL_A::VAL27,
            28 => CAPTSEL_A::VAL28,
            29 => CAPTSEL_A::VAL29,
            30 => CAPTSEL_A::VAL30,
            31 => CAPTSEL_A::VAL31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `VAL0`"]
    #[inline(always)]
    pub fn is_val0(&self) -> bool {
        **self == CAPTSEL_A::VAL0
    }
    #[doc = "Checks if the value of the field is `VAL1`"]
    #[inline(always)]
    pub fn is_val1(&self) -> bool {
        **self == CAPTSEL_A::VAL1
    }
    #[doc = "Checks if the value of the field is `VAL2`"]
    #[inline(always)]
    pub fn is_val2(&self) -> bool {
        **self == CAPTSEL_A::VAL2
    }
    #[doc = "Checks if the value of the field is `VAL3`"]
    #[inline(always)]
    pub fn is_val3(&self) -> bool {
        **self == CAPTSEL_A::VAL3
    }
    #[doc = "Checks if the value of the field is `VAL4`"]
    #[inline(always)]
    pub fn is_val4(&self) -> bool {
        **self == CAPTSEL_A::VAL4
    }
    #[doc = "Checks if the value of the field is `VAL5`"]
    #[inline(always)]
    pub fn is_val5(&self) -> bool {
        **self == CAPTSEL_A::VAL5
    }
    #[doc = "Checks if the value of the field is `VAL6`"]
    #[inline(always)]
    pub fn is_val6(&self) -> bool {
        **self == CAPTSEL_A::VAL6
    }
    #[doc = "Checks if the value of the field is `VAL7`"]
    #[inline(always)]
    pub fn is_val7(&self) -> bool {
        **self == CAPTSEL_A::VAL7
    }
    #[doc = "Checks if the value of the field is `VAL8`"]
    #[inline(always)]
    pub fn is_val8(&self) -> bool {
        **self == CAPTSEL_A::VAL8
    }
    #[doc = "Checks if the value of the field is `VAL9`"]
    #[inline(always)]
    pub fn is_val9(&self) -> bool {
        **self == CAPTSEL_A::VAL9
    }
    #[doc = "Checks if the value of the field is `VAL10`"]
    #[inline(always)]
    pub fn is_val10(&self) -> bool {
        **self == CAPTSEL_A::VAL10
    }
    #[doc = "Checks if the value of the field is `VAL11`"]
    #[inline(always)]
    pub fn is_val11(&self) -> bool {
        **self == CAPTSEL_A::VAL11
    }
    #[doc = "Checks if the value of the field is `VAL12`"]
    #[inline(always)]
    pub fn is_val12(&self) -> bool {
        **self == CAPTSEL_A::VAL12
    }
    #[doc = "Checks if the value of the field is `VAL13`"]
    #[inline(always)]
    pub fn is_val13(&self) -> bool {
        **self == CAPTSEL_A::VAL13
    }
    #[doc = "Checks if the value of the field is `VAL14`"]
    #[inline(always)]
    pub fn is_val14(&self) -> bool {
        **self == CAPTSEL_A::VAL14
    }
    #[doc = "Checks if the value of the field is `VAL15`"]
    #[inline(always)]
    pub fn is_val15(&self) -> bool {
        **self == CAPTSEL_A::VAL15
    }
    #[doc = "Checks if the value of the field is `VAL16`"]
    #[inline(always)]
    pub fn is_val16(&self) -> bool {
        **self == CAPTSEL_A::VAL16
    }
    #[doc = "Checks if the value of the field is `VAL17`"]
    #[inline(always)]
    pub fn is_val17(&self) -> bool {
        **self == CAPTSEL_A::VAL17
    }
    #[doc = "Checks if the value of the field is `VAL18`"]
    #[inline(always)]
    pub fn is_val18(&self) -> bool {
        **self == CAPTSEL_A::VAL18
    }
    #[doc = "Checks if the value of the field is `VAL19`"]
    #[inline(always)]
    pub fn is_val19(&self) -> bool {
        **self == CAPTSEL_A::VAL19
    }
    #[doc = "Checks if the value of the field is `VAL20`"]
    #[inline(always)]
    pub fn is_val20(&self) -> bool {
        **self == CAPTSEL_A::VAL20
    }
    #[doc = "Checks if the value of the field is `VAL21`"]
    #[inline(always)]
    pub fn is_val21(&self) -> bool {
        **self == CAPTSEL_A::VAL21
    }
    #[doc = "Checks if the value of the field is `VAL22`"]
    #[inline(always)]
    pub fn is_val22(&self) -> bool {
        **self == CAPTSEL_A::VAL22
    }
    #[doc = "Checks if the value of the field is `VAL23`"]
    #[inline(always)]
    pub fn is_val23(&self) -> bool {
        **self == CAPTSEL_A::VAL23
    }
    #[doc = "Checks if the value of the field is `VAL24`"]
    #[inline(always)]
    pub fn is_val24(&self) -> bool {
        **self == CAPTSEL_A::VAL24
    }
    #[doc = "Checks if the value of the field is `VAL25`"]
    #[inline(always)]
    pub fn is_val25(&self) -> bool {
        **self == CAPTSEL_A::VAL25
    }
    #[doc = "Checks if the value of the field is `VAL26`"]
    #[inline(always)]
    pub fn is_val26(&self) -> bool {
        **self == CAPTSEL_A::VAL26
    }
    #[doc = "Checks if the value of the field is `VAL27`"]
    #[inline(always)]
    pub fn is_val27(&self) -> bool {
        **self == CAPTSEL_A::VAL27
    }
    #[doc = "Checks if the value of the field is `VAL28`"]
    #[inline(always)]
    pub fn is_val28(&self) -> bool {
        **self == CAPTSEL_A::VAL28
    }
    #[doc = "Checks if the value of the field is `VAL29`"]
    #[inline(always)]
    pub fn is_val29(&self) -> bool {
        **self == CAPTSEL_A::VAL29
    }
    #[doc = "Checks if the value of the field is `VAL30`"]
    #[inline(always)]
    pub fn is_val30(&self) -> bool {
        **self == CAPTSEL_A::VAL30
    }
    #[doc = "Checks if the value of the field is `VAL31`"]
    #[inline(always)]
    pub fn is_val31(&self) -> bool {
        **self == CAPTSEL_A::VAL31
    }
}
impl core::ops::Deref for CAPTSEL_R {
    type Target = crate::FieldReader<u8, CAPTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAPTSEL` writer - Input number to TIMER1 capture inputs 0 to 4"]
pub struct CAPTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CAPTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAPTSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "CT_INP0 function selected from IOCON register"]
    #[inline(always)]
    pub fn val0(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL0)
    }
    #[doc = "CT_INP1 function selected from IOCON register"]
    #[inline(always)]
    pub fn val1(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL1)
    }
    #[doc = "CT_INP2 function selected from IOCON register"]
    #[inline(always)]
    pub fn val2(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL2)
    }
    #[doc = "CT_INP3 function selected from IOCON register"]
    #[inline(always)]
    pub fn val3(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL3)
    }
    #[doc = "CT_INP4 function selected from IOCON register"]
    #[inline(always)]
    pub fn val4(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL4)
    }
    #[doc = "CT_INP5 function selected from IOCON register"]
    #[inline(always)]
    pub fn val5(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL5)
    }
    #[doc = "CT_INP6 function selected from IOCON register"]
    #[inline(always)]
    pub fn val6(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL6)
    }
    #[doc = "CT_INP7 function selected from IOCON register"]
    #[inline(always)]
    pub fn val7(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL7)
    }
    #[doc = "CT_INP8 function selected from IOCON register"]
    #[inline(always)]
    pub fn val8(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL8)
    }
    #[doc = "CT_INP9 function selected from IOCON register"]
    #[inline(always)]
    pub fn val9(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL9)
    }
    #[doc = "CT_INP10 function selected from IOCON register"]
    #[inline(always)]
    pub fn val10(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL10)
    }
    #[doc = "CT_INP11 function selected from IOCON register"]
    #[inline(always)]
    pub fn val11(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL11)
    }
    #[doc = "CT_INP12 function selected from IOCON register"]
    #[inline(always)]
    pub fn val12(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL12)
    }
    #[doc = "CT_INP13 function selected from IOCON register"]
    #[inline(always)]
    pub fn val13(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL13)
    }
    #[doc = "CT_INP14 function selected from IOCON register"]
    #[inline(always)]
    pub fn val14(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL14)
    }
    #[doc = "CT_INP15 function selected from IOCON register"]
    #[inline(always)]
    pub fn val15(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL15)
    }
    #[doc = "CT_INP16 function selected from IOCON register"]
    #[inline(always)]
    pub fn val16(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL16)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val17(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL17)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val18(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL18)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val19(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL19)
    }
    #[doc = "USB0_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val20(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL20)
    }
    #[doc = "USB1_FRAME_TOGGLE"]
    #[inline(always)]
    pub fn val21(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL21)
    }
    #[doc = "COMP_OUTPUT output from analog comparator"]
    #[inline(always)]
    pub fn val22(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL22)
    }
    #[doc = "I2S_SHARED_WS\\[0\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val23(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL23)
    }
    #[doc = "I2S_SHARED_WS\\[1\\]
output from I2S pin sharing"]
    #[inline(always)]
    pub fn val24(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL24)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val25(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL25)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val26(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL26)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val27(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL27)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val28(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL28)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val29(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL29)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val30(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL30)
    }
    #[doc = "None"]
    #[inline(always)]
    pub fn val31(self) -> &'a mut W {
        self.variant(CAPTSEL_A::VAL31)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - Input number to TIMER1 capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&self) -> CAPTSEL_R {
        CAPTSEL_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Input number to TIMER1 capture inputs 0 to 4"]
    #[inline(always)]
    pub fn captsel(&mut self) -> CAPTSEL_W {
        CAPTSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture select registers for TIMER1 inputs\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1captsel](index.html) module"]
pub struct TIMER1CAPTSEL_SPEC;
impl crate::RegisterSpec for TIMER1CAPTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1captsel::R](R) reader structure"]
impl crate::Readable for TIMER1CAPTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer1captsel::W](W) writer structure"]
impl crate::Writable for TIMER1CAPTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIMER1CAPTSEL[%s]
to value 0x1f"]
impl crate::Resettable for TIMER1CAPTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x1f
    }
}
