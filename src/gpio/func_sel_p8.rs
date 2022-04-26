#[doc = "Register `FUNC_SEL_P8` reader"]
pub struct R(crate::R<FUNC_SEL_P8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_SEL_P8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_SEL_P8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_SEL_P8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC_SEL_P8` writer"]
pub struct W(crate::W<FUNC_SEL_P8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_SEL_P8_SPEC>;
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
impl From<crate::W<FUNC_SEL_P8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_SEL_P8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P8.0 Output Function Select"]
pub struct PIN0_R(crate::FieldReader<u8>);
impl PIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN0_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin0` writer - P8.0 Output Function Select"]
pub struct PIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `pin1` reader - P8.1 Output Function Select"]
pub struct PIN1_R(crate::FieldReader<u8>);
impl PIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN1_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin1` writer - P8.1 Output Function Select"]
pub struct PIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - P8.0 Output Function Select"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - P8.1 Output Function Select"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - P8.0 Output Function Select"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W { w: self }
    }
    #[doc = "Bits 4:7 - P8.1 Output Function Select"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port P8 GPIO Function Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_sel_p8](index.html) module"]
pub struct FUNC_SEL_P8_SPEC;
impl crate::RegisterSpec for FUNC_SEL_P8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_sel_p8::R](R) reader structure"]
impl crate::Readable for FUNC_SEL_P8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_sel_p8::W](W) writer structure"]
impl crate::Writable for FUNC_SEL_P8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC_SEL_P8 to value 0"]
impl crate::Resettable for FUNC_SEL_P8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
