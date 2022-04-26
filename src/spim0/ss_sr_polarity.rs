#[doc = "Register `SS_SR_POLARITY` reader"]
pub struct R(crate::R<SS_SR_POLARITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SS_SR_POLARITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SS_SR_POLARITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SS_SR_POLARITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SS_SR_POLARITY` writer"]
pub struct W(crate::W<SS_SR_POLARITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SS_SR_POLARITY_SPEC>;
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
impl From<crate::W<SS_SR_POLARITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SS_SR_POLARITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ss_polarity` reader - SS Signal Polarity"]
pub struct SS_POLARITY_R(crate::FieldReader<u8>);
impl SS_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SS_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_POLARITY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_polarity` writer - SS Signal Polarity"]
pub struct SS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_POLARITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `fc_polarity` reader - SR Signal Polarity \\[FC Polarity\\]"]
pub struct FC_POLARITY_R(crate::FieldReader<u8>);
impl FC_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FC_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FC_POLARITY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fc_polarity` writer - SR Signal Polarity \\[FC Polarity\\]"]
pub struct FC_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> FC_POLARITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - SS Signal Polarity"]
    #[inline(always)]
    pub fn ss_polarity(&self) -> SS_POLARITY_R {
        SS_POLARITY_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SR Signal Polarity \\[FC Polarity\\]"]
    #[inline(always)]
    pub fn fc_polarity(&self) -> FC_POLARITY_R {
        FC_POLARITY_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SS Signal Polarity"]
    #[inline(always)]
    pub fn ss_polarity(&mut self) -> SS_POLARITY_W {
        SS_POLARITY_W { w: self }
    }
    #[doc = "Bits 8:15 - SR Signal Polarity \\[FC Polarity\\]"]
    #[inline(always)]
    pub fn fc_polarity(&mut self) -> FC_POLARITY_W {
        FC_POLARITY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Polarity Control for SS and SR Signals\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ss_sr_polarity](index.html) module"]
pub struct SS_SR_POLARITY_SPEC;
impl crate::RegisterSpec for SS_SR_POLARITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ss_sr_polarity::R](R) reader structure"]
impl crate::Readable for SS_SR_POLARITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ss_sr_polarity::W](W) writer structure"]
impl crate::Writable for SS_SR_POLARITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SS_SR_POLARITY to value 0"]
impl crate::Resettable for SS_SR_POLARITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
