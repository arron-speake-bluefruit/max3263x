#[doc = "Register `SPCL_CTRL` reader"]
pub struct R(crate::R<SPCL_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPCL_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPCL_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPCL_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPCL_CTRL` writer"]
pub struct W(crate::W<SPCL_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPCL_CTRL_SPEC>;
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
impl From<crate::W<SPCL_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPCL_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ss_sample_mode` reader - SS Sample Mode"]
pub struct SS_SAMPLE_MODE_R(crate::FieldReader<bool>);
impl SS_SAMPLE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_SAMPLE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_SAMPLE_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_sample_mode` writer - SS Sample Mode"]
pub struct SS_SAMPLE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_SAMPLE_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `miso_fc_en` reader - SDIO(1) to SR(0) Mode"]
pub struct MISO_FC_EN_R(crate::FieldReader<bool>);
impl MISO_FC_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MISO_FC_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MISO_FC_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `miso_fc_en` writer - SDIO(1) to SR(0) Mode"]
pub struct MISO_FC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> MISO_FC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `ss_sa_sdio_out` reader - SDIO Active Output Value"]
pub struct SS_SA_SDIO_OUT_R(crate::FieldReader<u8>);
impl SS_SA_SDIO_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SS_SA_SDIO_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_SA_SDIO_OUT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_sa_sdio_out` writer - SDIO Active Output Value"]
pub struct SS_SA_SDIO_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_SA_SDIO_OUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `ss_sa_sdio_dr_en` reader - SDIO Active Drive Mode"]
pub struct SS_SA_SDIO_DR_EN_R(crate::FieldReader<u8>);
impl SS_SA_SDIO_DR_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SS_SA_SDIO_DR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_SA_SDIO_DR_EN_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_sa_sdio_dr_en` writer - SDIO Active Drive Mode"]
pub struct SS_SA_SDIO_DR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_SA_SDIO_DR_EN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SS Sample Mode"]
    #[inline(always)]
    pub fn ss_sample_mode(&self) -> SS_SAMPLE_MODE_R {
        SS_SAMPLE_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDIO(1) to SR(0) Mode"]
    #[inline(always)]
    pub fn miso_fc_en(&self) -> MISO_FC_EN_R {
        MISO_FC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:7 - SDIO Active Output Value"]
    #[inline(always)]
    pub fn ss_sa_sdio_out(&self) -> SS_SA_SDIO_OUT_R {
        SS_SA_SDIO_OUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - SDIO Active Drive Mode"]
    #[inline(always)]
    pub fn ss_sa_sdio_dr_en(&self) -> SS_SA_SDIO_DR_EN_R {
        SS_SA_SDIO_DR_EN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SS Sample Mode"]
    #[inline(always)]
    pub fn ss_sample_mode(&mut self) -> SS_SAMPLE_MODE_W {
        SS_SAMPLE_MODE_W { w: self }
    }
    #[doc = "Bit 1 - SDIO(1) to SR(0) Mode"]
    #[inline(always)]
    pub fn miso_fc_en(&mut self) -> MISO_FC_EN_W {
        MISO_FC_EN_W { w: self }
    }
    #[doc = "Bits 4:7 - SDIO Active Output Value"]
    #[inline(always)]
    pub fn ss_sa_sdio_out(&mut self) -> SS_SA_SDIO_OUT_W {
        SS_SA_SDIO_OUT_W { w: self }
    }
    #[doc = "Bits 8:11 - SDIO Active Drive Mode"]
    #[inline(always)]
    pub fn ss_sa_sdio_dr_en(&mut self) -> SS_SA_SDIO_DR_EN_W {
        SS_SA_SDIO_DR_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Special Mode Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spcl_ctrl](index.html) module"]
pub struct SPCL_CTRL_SPEC;
impl crate::RegisterSpec for SPCL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spcl_ctrl::R](R) reader structure"]
impl crate::Readable for SPCL_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spcl_ctrl::W](W) writer structure"]
impl crate::Writable for SPCL_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPCL_CTRL to value 0"]
impl crate::Resettable for SPCL_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
