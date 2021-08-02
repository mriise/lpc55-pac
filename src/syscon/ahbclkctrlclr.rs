#[doc = "Register `AHBCLKCTRLCLR[%s]` reader"]
pub struct R(crate::R<AHBCLKCTRLCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHBCLKCTRLCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHBCLKCTRLCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHBCLKCTRLCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHBCLKCTRLCLR[%s]` writer"]
pub struct W(crate::W<AHBCLKCTRLCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHBCLKCTRLCLR_SPEC>;
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
impl From<crate::W<AHBCLKCTRLCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHBCLKCTRLCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Data array value"]
pub struct DATA_R(crate::FieldReader<u32, u32>);
impl DATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATA` writer - Data array value"]
pub struct DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data array value"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral reset control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahbclkctrlclr](index.html) module"]
pub struct AHBCLKCTRLCLR_SPEC;
impl crate::RegisterSpec for AHBCLKCTRLCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahbclkctrlclr::R](R) reader structure"]
impl crate::Readable for AHBCLKCTRLCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahbclkctrlclr::W](W) writer structure"]
impl crate::Writable for AHBCLKCTRLCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AHBCLKCTRLCLR[%s]
to value 0"]
impl crate::Resettable for AHBCLKCTRLCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
