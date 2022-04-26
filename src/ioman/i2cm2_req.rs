#[doc = "Register `I2CM2_REQ` reader"]
pub struct R(crate::R<I2CM2_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2CM2_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2CM2_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2CM2_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2CM2_REQ` writer"]
pub struct W(crate::W<I2CM2_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2CM2_REQ_SPEC>;
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
impl From<crate::W<I2CM2_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2CM2_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mapping_req` reader - I2C Master 2 I/O Request"]
pub struct MAPPING_REQ_R(crate::FieldReader<bool>);
impl MAPPING_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAPPING_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAPPING_REQ_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mapping_req` writer - I2C Master 2 I/O Request"]
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - I2C Master 2 I/O Request"]
    #[inline(always)]
    pub fn mapping_req(&self) -> MAPPING_REQ_R {
        MAPPING_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - I2C Master 2 I/O Request"]
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
#[doc = "I2C Master 2 I/O Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2cm2_req](index.html) module"]
pub struct I2CM2_REQ_SPEC;
impl crate::RegisterSpec for I2CM2_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2cm2_req::R](R) reader structure"]
impl crate::Readable for I2CM2_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2cm2_req::W](W) writer structure"]
impl crate::Writable for I2CM2_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets I2CM2_REQ to value 0"]
impl crate::Resettable for I2CM2_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
