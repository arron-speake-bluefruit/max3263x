#[doc = "Register `OWM_ACK` reader"]
pub struct R(crate::R<OWM_ACK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OWM_ACK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OWM_ACK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OWM_ACK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OWM_ACK` writer"]
pub struct W(crate::W<OWM_ACK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OWM_ACK_SPEC>;
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
impl From<crate::W<OWM_ACK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OWM_ACK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mapping_ack` reader - 1-Wire Line I/O Acknowledge"]
pub struct MAPPING_ACK_R(crate::FieldReader<bool, bool>);
impl MAPPING_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAPPING_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAPPING_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mapping_ack` writer - 1-Wire Line I/O Acknowledge"]
pub struct MAPPING_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> MAPPING_ACK_W<'a> {
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
#[doc = "Field `epu_io_ack` reader - External Pullup Control Line I/O Acknowledge"]
pub struct EPU_IO_ACK_R(crate::FieldReader<bool, bool>);
impl EPU_IO_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EPU_IO_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPU_IO_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `epu_io_ack` writer - External Pullup Control Line I/O Acknowledge"]
pub struct EPU_IO_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EPU_IO_ACK_W<'a> {
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
impl R {
    #[doc = "Bit 4 - 1-Wire Line I/O Acknowledge"]
    #[inline(always)]
    pub fn mapping_ack(&self) -> MAPPING_ACK_R {
        MAPPING_ACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - External Pullup Control Line I/O Acknowledge"]
    #[inline(always)]
    pub fn epu_io_ack(&self) -> EPU_IO_ACK_R {
        EPU_IO_ACK_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - 1-Wire Line I/O Acknowledge"]
    #[inline(always)]
    pub fn mapping_ack(&mut self) -> MAPPING_ACK_W {
        MAPPING_ACK_W { w: self }
    }
    #[doc = "Bit 5 - External Pullup Control Line I/O Acknowledge"]
    #[inline(always)]
    pub fn epu_io_ack(&mut self) -> EPU_IO_ACK_W {
        EPU_IO_ACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Master I/O Mode Acknowledge\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [owm_ack](index.html) module"]
pub struct OWM_ACK_SPEC;
impl crate::RegisterSpec for OWM_ACK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [owm_ack::R](R) reader structure"]
impl crate::Readable for OWM_ACK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [owm_ack::W](W) writer structure"]
impl crate::Writable for OWM_ACK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OWM_ACK to value 0"]
impl crate::Resettable for OWM_ACK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
