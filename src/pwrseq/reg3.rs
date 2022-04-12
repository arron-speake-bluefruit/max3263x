#[doc = "Register `REG3` reader"]
pub struct R(crate::R<REG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG3` writer"]
pub struct W(crate::W<REG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG3_SPEC>;
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
impl From<crate::W<REG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_rosel` reader - Relaxation Oscillator Stable Timeout"]
pub struct PWR_ROSEL_R(crate::FieldReader<u8, u8>);
impl PWR_ROSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_ROSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_ROSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_rosel` writer - Relaxation Oscillator Stable Timeout"]
pub struct PWR_ROSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_ROSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `pwr_fltrrosel` reader - Window of time power must be valid before entering Run mode."]
pub struct PWR_FLTRROSEL_R(crate::FieldReader<u8, u8>);
impl PWR_FLTRROSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_FLTRROSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_FLTRROSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_fltrrosel` writer - Window of time power must be valid before entering Run mode."]
pub struct PWR_FLTRROSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_FLTRROSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 3)) | ((value as u32 & 7) << 3);
        self.w
    }
}
#[doc = "Field `pwr_svm_clk_mux` reader - SVM Clock Mux"]
pub struct PWR_SVM_CLK_MUX_R(crate::FieldReader<u8, u8>);
impl PWR_SVM_CLK_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_SVM_CLK_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SVM_CLK_MUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_svm_clk_mux` writer - SVM Clock Mux"]
pub struct PWR_SVM_CLK_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SVM_CLK_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `pwr_ro_clk_mux` reader - Relaxation Clock Mux"]
pub struct PWR_RO_CLK_MUX_R(crate::FieldReader<u8, u8>);
impl PWR_RO_CLK_MUX_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_RO_CLK_MUX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_RO_CLK_MUX_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_ro_clk_mux` writer - Relaxation Clock Mux"]
pub struct PWR_RO_CLK_MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_RO_CLK_MUX_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `pwr_failsel` reader - Timeout before rebooting during PowerFail/BootFail events."]
pub struct PWR_FAILSEL_R(crate::FieldReader<u8, u8>);
impl PWR_FAILSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PWR_FAILSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_FAILSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_failsel` writer - Timeout before rebooting during PowerFail/BootFail events."]
pub struct PWR_FAILSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_FAILSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 10)) | ((value as u32 & 7) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Relaxation Oscillator Stable Timeout"]
    #[inline(always)]
    pub fn pwr_rosel(&self) -> PWR_ROSEL_R {
        PWR_ROSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Window of time power must be valid before entering Run mode."]
    #[inline(always)]
    pub fn pwr_fltrrosel(&self) -> PWR_FLTRROSEL_R {
        PWR_FLTRROSEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - SVM Clock Mux"]
    #[inline(always)]
    pub fn pwr_svm_clk_mux(&self) -> PWR_SVM_CLK_MUX_R {
        PWR_SVM_CLK_MUX_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Relaxation Clock Mux"]
    #[inline(always)]
    pub fn pwr_ro_clk_mux(&self) -> PWR_RO_CLK_MUX_R {
        PWR_RO_CLK_MUX_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - Timeout before rebooting during PowerFail/BootFail events."]
    #[inline(always)]
    pub fn pwr_failsel(&self) -> PWR_FAILSEL_R {
        PWR_FAILSEL_R::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Relaxation Oscillator Stable Timeout"]
    #[inline(always)]
    pub fn pwr_rosel(&mut self) -> PWR_ROSEL_W {
        PWR_ROSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Window of time power must be valid before entering Run mode."]
    #[inline(always)]
    pub fn pwr_fltrrosel(&mut self) -> PWR_FLTRROSEL_W {
        PWR_FLTRROSEL_W { w: self }
    }
    #[doc = "Bits 6:7 - SVM Clock Mux"]
    #[inline(always)]
    pub fn pwr_svm_clk_mux(&mut self) -> PWR_SVM_CLK_MUX_W {
        PWR_SVM_CLK_MUX_W { w: self }
    }
    #[doc = "Bits 8:9 - Relaxation Clock Mux"]
    #[inline(always)]
    pub fn pwr_ro_clk_mux(&mut self) -> PWR_RO_CLK_MUX_W {
        PWR_RO_CLK_MUX_W { w: self }
    }
    #[doc = "Bits 10:12 - Timeout before rebooting during PowerFail/BootFail events."]
    #[inline(always)]
    pub fn pwr_failsel(&mut self) -> PWR_FAILSEL_W {
        PWR_FAILSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3](index.html) module"]
pub struct REG3_SPEC;
impl crate::RegisterSpec for REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg3::R](R) reader structure"]
impl crate::Readable for REG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg3::W](W) writer structure"]
impl crate::Writable for REG3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG3 to value 0"]
impl crate::Resettable for REG3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
