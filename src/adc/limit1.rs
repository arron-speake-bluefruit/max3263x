#[doc = "Register `LIMIT1` reader"]
pub struct R(crate::R<LIMIT1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LIMIT1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LIMIT1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LIMIT1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LIMIT1` writer"]
pub struct W(crate::W<LIMIT1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LIMIT1_SPEC>;
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
impl From<crate::W<LIMIT1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LIMIT1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ch_lo_limit` reader - Low Limit Threshold"]
pub struct CH_LO_LIMIT_R(crate::FieldReader<u16, u16>);
impl CH_LO_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CH_LO_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_LO_LIMIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_lo_limit` writer - Low Limit Threshold"]
pub struct CH_LO_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_LO_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `ch_hi_limit` reader - High Limit Threshold"]
pub struct CH_HI_LIMIT_R(crate::FieldReader<u16, u16>);
impl CH_HI_LIMIT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        CH_HI_LIMIT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_HI_LIMIT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_hi_limit` writer - High Limit Threshold"]
pub struct CH_HI_LIMIT_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_HI_LIMIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 12)) | ((value as u32 & 0x03ff) << 12);
        self.w
    }
}
#[doc = "Field `ch_sel` reader - ADC Channel Select"]
pub struct CH_SEL_R(crate::FieldReader<u8, u8>);
impl CH_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CH_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_sel` writer - ADC Channel Select"]
pub struct CH_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `ch_lo_limit_en` reader - Low Limit Monitoring Enable"]
pub struct CH_LO_LIMIT_EN_R(crate::FieldReader<bool, bool>);
impl CH_LO_LIMIT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_LO_LIMIT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_LO_LIMIT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_lo_limit_en` writer - Low Limit Monitoring Enable"]
pub struct CH_LO_LIMIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_LO_LIMIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
#[doc = "Field `ch_hi_limit_en` reader - High Limit Monitoring Enable"]
pub struct CH_HI_LIMIT_EN_R(crate::FieldReader<bool, bool>);
impl CH_HI_LIMIT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CH_HI_LIMIT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH_HI_LIMIT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ch_hi_limit_en` writer - High Limit Monitoring Enable"]
pub struct CH_HI_LIMIT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CH_HI_LIMIT_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | ((value as u32 & 0x01) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&self) -> CH_LO_LIMIT_R {
        CH_LO_LIMIT_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&self) -> CH_HI_LIMIT_R {
        CH_HI_LIMIT_R::new(((self.bits >> 12) & 0x03ff) as u16)
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&self) -> CH_SEL_R {
        CH_SEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&self) -> CH_LO_LIMIT_EN_R {
        CH_LO_LIMIT_EN_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&self) -> CH_HI_LIMIT_EN_R {
        CH_HI_LIMIT_EN_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Low Limit Threshold"]
    #[inline(always)]
    pub fn ch_lo_limit(&mut self) -> CH_LO_LIMIT_W {
        CH_LO_LIMIT_W { w: self }
    }
    #[doc = "Bits 12:21 - High Limit Threshold"]
    #[inline(always)]
    pub fn ch_hi_limit(&mut self) -> CH_HI_LIMIT_W {
        CH_HI_LIMIT_W { w: self }
    }
    #[doc = "Bits 24:27 - ADC Channel Select"]
    #[inline(always)]
    pub fn ch_sel(&mut self) -> CH_SEL_W {
        CH_SEL_W { w: self }
    }
    #[doc = "Bit 28 - Low Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_lo_limit_en(&mut self) -> CH_LO_LIMIT_EN_W {
        CH_LO_LIMIT_EN_W { w: self }
    }
    #[doc = "Bit 29 - High Limit Monitoring Enable"]
    #[inline(always)]
    pub fn ch_hi_limit_en(&mut self) -> CH_HI_LIMIT_EN_W {
        CH_HI_LIMIT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Limit 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [limit1](index.html) module"]
pub struct LIMIT1_SPEC;
impl crate::RegisterSpec for LIMIT1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [limit1::R](R) reader structure"]
impl crate::Readable for LIMIT1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [limit1::W](W) writer structure"]
impl crate::Writable for LIMIT1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LIMIT1 to value 0"]
impl crate::Resettable for LIMIT1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
