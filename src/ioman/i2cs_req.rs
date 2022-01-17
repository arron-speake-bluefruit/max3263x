#[doc = "Register `I2CS_REQ` reader"]
pub struct R(crate::R<I2CS_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CS_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CS_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CS_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CS_REQ` writer"]
pub struct W(crate::W<I2CS_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CS_REQ_SPEC>;
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
impl From<crate::W<I2CS_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CS_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_sel` reader - I2C Slave I/O Mapping Select"]
pub struct IO_SEL_R(crate::FieldReader<u8, u8>);
impl IO_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        IO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_sel` writer - I2C Slave I/O Mapping Select"]
pub struct IO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `mapping_req` reader - I2C Slave I/O Request"]
pub struct MAPPING_REQ_R(crate::FieldReader<bool, bool>);
impl MAPPING_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAPPING_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAPPING_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mapping_req` writer - I2C Slave I/O Request"]
pub struct MAPPING_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MAPPING_REQ_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - I2C Slave I/O Mapping Select"]
    #[inline(always)]
    pub fn io_sel(&self) -> IO_SEL_R {
        IO_SEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - I2C Slave I/O Request"]
    #[inline(always)]
    pub fn mapping_req(&self) -> MAPPING_REQ_R {
        MAPPING_REQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - I2C Slave I/O Mapping Select"]
    #[inline(always)]
    pub fn io_sel(&mut self) -> IO_SEL_W {
        IO_SEL_W { w: self }
    }
    #[doc = "Bit 4 - I2C Slave I/O Request"]
    #[inline(always)]
    pub fn mapping_req(&mut self) -> MAPPING_REQ_W {
        MAPPING_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave I/O Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cs_req](index.html) module"]
pub struct I2CS_REQ_SPEC;
impl crate::RegisterSpec for I2CS_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cs_req::R](R) reader structure"]
impl crate::Readable for I2CS_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cs_req::W](W) writer structure"]
impl crate::Writable for I2CS_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2CS_REQ to value 0"]
impl crate::Resettable for I2CS_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
