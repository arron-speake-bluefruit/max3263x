#[doc = "Register `REG5` reader"]
pub struct R(crate::R<REG5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG5` writer"]
pub struct W(crate::W<REG5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG5_SPEC>;
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
impl From<crate::W<REG5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_trim_svm_bg` reader - Power Manager Bandgap trim setting"]
pub struct PWR_TRIM_SVM_BG_R(crate::FieldReader<u16>);
impl PWR_TRIM_SVM_BG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        PWR_TRIM_SVM_BG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TRIM_SVM_BG_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_trim_svm_bg` writer - Power Manager Bandgap trim setting"]
pub struct PWR_TRIM_SVM_BG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TRIM_SVM_BG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | (value as u32 & 0x01ff);
        self.w
    }
}
#[doc = "Field `pwr_trim_bias` reader - Power Manager Bias Current trim setting"]
pub struct PWR_TRIM_BIAS_R(crate::FieldReader<u8>);
impl PWR_TRIM_BIAS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_TRIM_BIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TRIM_BIAS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_trim_bias` writer - Power Manager Bias Current trim setting"]
pub struct PWR_TRIM_BIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TRIM_BIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | ((value as u32 & 0x3f) << 9);
        self.w
    }
}
#[doc = "Field `pwr_trim_retreg` reader - Retention Regulator trim setting"]
pub struct PWR_TRIM_RETREG_R(crate::FieldReader<u8>);
impl PWR_TRIM_RETREG_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_TRIM_RETREG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TRIM_RETREG_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_trim_retreg` writer - Retention Regulator trim setting"]
pub struct PWR_TRIM_RETREG_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TRIM_RETREG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 15)) | ((value as u32 & 0x3f) << 15);
        self.w
    }
}
#[doc = "Field `pwr_rtc_trim` reader - Real Time Clock trim setting"]
pub struct PWR_RTC_TRIM_R(crate::FieldReader<u8>);
impl PWR_RTC_TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_RTC_TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_RTC_TRIM_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_rtc_trim` writer - Real Time Clock trim setting"]
pub struct PWR_RTC_TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_RTC_TRIM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 21)) | ((value as u32 & 0x0f) << 21);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Power Manager Bandgap trim setting"]
    #[inline(always)]
    pub fn pwr_trim_svm_bg(&self) -> PWR_TRIM_SVM_BG_R {
        PWR_TRIM_SVM_BG_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:14 - Power Manager Bias Current trim setting"]
    #[inline(always)]
    pub fn pwr_trim_bias(&self) -> PWR_TRIM_BIAS_R {
        PWR_TRIM_BIAS_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bits 15:20 - Retention Regulator trim setting"]
    #[inline(always)]
    pub fn pwr_trim_retreg(&self) -> PWR_TRIM_RETREG_R {
        PWR_TRIM_RETREG_R::new(((self.bits >> 15) & 0x3f) as u8)
    }
    #[doc = "Bits 21:24 - Real Time Clock trim setting"]
    #[inline(always)]
    pub fn pwr_rtc_trim(&self) -> PWR_RTC_TRIM_R {
        PWR_RTC_TRIM_R::new(((self.bits >> 21) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Power Manager Bandgap trim setting"]
    #[inline(always)]
    pub fn pwr_trim_svm_bg(&mut self) -> PWR_TRIM_SVM_BG_W {
        PWR_TRIM_SVM_BG_W { w: self }
    }
    #[doc = "Bits 9:14 - Power Manager Bias Current trim setting"]
    #[inline(always)]
    pub fn pwr_trim_bias(&mut self) -> PWR_TRIM_BIAS_W {
        PWR_TRIM_BIAS_W { w: self }
    }
    #[doc = "Bits 15:20 - Retention Regulator trim setting"]
    #[inline(always)]
    pub fn pwr_trim_retreg(&mut self) -> PWR_TRIM_RETREG_W {
        PWR_TRIM_RETREG_W { w: self }
    }
    #[doc = "Bits 21:24 - Real Time Clock trim setting"]
    #[inline(always)]
    pub fn pwr_rtc_trim(&mut self) -> PWR_RTC_TRIM_W {
        PWR_RTC_TRIM_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 5 (Trim 0)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg5](index.html) module"]
pub struct REG5_SPEC;
impl crate::RegisterSpec for REG5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg5::R](R) reader structure"]
impl crate::Readable for REG5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg5::W](W) writer structure"]
impl crate::Writable for REG5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG5 to value 0"]
impl crate::Resettable for REG5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
