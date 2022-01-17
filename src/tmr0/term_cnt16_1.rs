#[doc = "Register `TERM_CNT16_1` reader"]
pub struct R(crate::R<TERM_CNT16_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TERM_CNT16_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TERM_CNT16_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TERM_CNT16_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TERM_CNT16_1` writer"]
pub struct W(crate::W<TERM_CNT16_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TERM_CNT16_1_SPEC>;
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
impl From<crate::W<TERM_CNT16_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TERM_CNT16_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `term_count` reader - Terminal Count Setting"]
pub struct TERM_COUNT_R(crate::FieldReader<u16, u16>);
impl TERM_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TERM_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TERM_COUNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `term_count` writer - Terminal Count Setting"]
pub struct TERM_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TERM_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Terminal Count Setting"]
    #[inline(always)]
    pub fn term_count(&self) -> TERM_COUNT_R {
        TERM_COUNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Terminal Count Setting"]
    #[inline(always)]
    pub fn term_count(&mut self) -> TERM_COUNT_W {
        TERM_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "16 bit\\]
Terminal Count Setting, 16-bit Timer 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [term_cnt16_1](index.html) module"]
pub struct TERM_CNT16_1_SPEC;
impl crate::RegisterSpec for TERM_CNT16_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [term_cnt16_1::R](R) reader structure"]
impl crate::Readable for TERM_CNT16_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [term_cnt16_1::W](W) writer structure"]
impl crate::Writable for TERM_CNT16_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TERM_CNT16_1 to value 0"]
impl crate::Resettable for TERM_CNT16_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
