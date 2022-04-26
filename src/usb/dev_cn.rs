#[doc = "Register `DEV_CN` reader"]
pub struct R(crate::R<DEV_CN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_CN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_CN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_CN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_CN` writer"]
pub struct W(crate::W<DEV_CN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_CN_SPEC>;
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
impl From<crate::W<DEV_CN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_CN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sigrwu` reader - USB Signal Remote Wakeup"]
pub struct SIGRWU_R(crate::FieldReader<bool>);
impl SIGRWU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SIGRWU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIGRWU_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sigrwu` writer - USB Signal Remote Wakeup"]
pub struct SIGRWU_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGRWU_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `connect` reader - Connect to USB"]
pub struct CONNECT_R(crate::FieldReader<bool>);
impl CONNECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CONNECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONNECT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `connect` writer - Connect to USB"]
pub struct CONNECT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONNECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `ulpm` reader - USB Low Power Mode"]
pub struct ULPM_R(crate::FieldReader<bool>);
impl ULPM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ULPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ULPM_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ulpm` writer - USB Low Power Mode"]
pub struct ULPM_W<'a> {
    w: &'a mut W,
}
impl<'a> ULPM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `urst` reader - USB Device Controller Reset"]
pub struct URST_R(crate::FieldReader<bool>);
impl URST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        URST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for URST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `urst` writer - USB Device Controller Reset"]
pub struct URST_W<'a> {
    w: &'a mut W,
}
impl<'a> URST_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `vbgate` reader - VBUS Gate"]
pub struct VBGATE_R(crate::FieldReader<bool>);
impl VBGATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBGATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBGATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vbgate` writer - VBUS Gate"]
pub struct VBGATE_W<'a> {
    w: &'a mut W,
}
impl<'a> VBGATE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `fifo_mode` reader - FIFO Mode"]
pub struct FIFO_MODE_R(crate::FieldReader<bool>);
impl FIFO_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FIFO_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FIFO_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fifo_mode` writer - FIFO Mode"]
pub struct FIFO_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 2 - USB Signal Remote Wakeup"]
    #[inline(always)]
    pub fn sigrwu(&self) -> SIGRWU_R {
        SIGRWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Connect to USB"]
    #[inline(always)]
    pub fn connect(&self) -> CONNECT_R {
        CONNECT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB Low Power Mode"]
    #[inline(always)]
    pub fn ulpm(&self) -> ULPM_R {
        ULPM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB Device Controller Reset"]
    #[inline(always)]
    pub fn urst(&self) -> URST_R {
        URST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VBUS Gate"]
    #[inline(always)]
    pub fn vbgate(&self) -> VBGATE_R {
        VBGATE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - FIFO Mode"]
    #[inline(always)]
    pub fn fifo_mode(&self) -> FIFO_MODE_R {
        FIFO_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - USB Signal Remote Wakeup"]
    #[inline(always)]
    pub fn sigrwu(&mut self) -> SIGRWU_W {
        SIGRWU_W { w: self }
    }
    #[doc = "Bit 3 - Connect to USB"]
    #[inline(always)]
    pub fn connect(&mut self) -> CONNECT_W {
        CONNECT_W { w: self }
    }
    #[doc = "Bit 4 - USB Low Power Mode"]
    #[inline(always)]
    pub fn ulpm(&mut self) -> ULPM_W {
        ULPM_W { w: self }
    }
    #[doc = "Bit 5 - USB Device Controller Reset"]
    #[inline(always)]
    pub fn urst(&mut self) -> URST_W {
        URST_W { w: self }
    }
    #[doc = "Bit 6 - VBUS Gate"]
    #[inline(always)]
    pub fn vbgate(&mut self) -> VBGATE_W {
        VBGATE_W { w: self }
    }
    #[doc = "Bit 9 - FIFO Mode"]
    #[inline(always)]
    pub fn fifo_mode(&mut self) -> FIFO_MODE_W {
        FIFO_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_cn](index.html) module"]
pub struct DEV_CN_SPEC;
impl crate::RegisterSpec for DEV_CN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_cn::R](R) reader structure"]
impl crate::Readable for DEV_CN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_cn::W](W) writer structure"]
impl crate::Writable for DEV_CN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEV_CN to value 0"]
impl crate::Resettable for DEV_CN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
