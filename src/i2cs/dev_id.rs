#[doc = "Register `DEV_ID` reader"]
pub struct R(crate::R<DEV_ID_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_ID_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_ID_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_ID_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_ID` writer"]
pub struct W(crate::W<DEV_ID_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_ID_SPEC>;
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
impl From<crate::W<DEV_ID_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_ID_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slave_dev_id` reader - Slave Device ID"]
pub struct SLAVE_DEV_ID_R(crate::FieldReader<u16, u16>);
impl SLAVE_DEV_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        SLAVE_DEV_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_DEV_ID_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slave_dev_id` writer - Slave Device ID"]
pub struct SLAVE_DEV_ID_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_DEV_ID_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `ten_bit_id_mode` reader - 10-bit ID Mode"]
pub struct TEN_BIT_ID_MODE_R(crate::FieldReader<bool, bool>);
impl TEN_BIT_ID_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TEN_BIT_ID_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEN_BIT_ID_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ten_bit_id_mode` writer - 10-bit ID Mode"]
pub struct TEN_BIT_ID_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TEN_BIT_ID_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `slave_reset` reader - Slave Reset"]
pub struct SLAVE_RESET_R(crate::FieldReader<bool, bool>);
impl SLAVE_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SLAVE_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slave_reset` writer - Slave Reset"]
pub struct SLAVE_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_RESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave Device ID"]
    #[inline(always)]
    pub fn slave_dev_id(&self) -> SLAVE_DEV_ID_R {
        SLAVE_DEV_ID_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 12 - 10-bit ID Mode"]
    #[inline(always)]
    pub fn ten_bit_id_mode(&self) -> TEN_BIT_ID_MODE_R {
        TEN_BIT_ID_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Slave Reset"]
    #[inline(always)]
    pub fn slave_reset(&self) -> SLAVE_RESET_R {
        SLAVE_RESET_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave Device ID"]
    #[inline(always)]
    pub fn slave_dev_id(&mut self) -> SLAVE_DEV_ID_W {
        SLAVE_DEV_ID_W { w: self }
    }
    #[doc = "Bit 12 - 10-bit ID Mode"]
    #[inline(always)]
    pub fn ten_bit_id_mode(&mut self) -> TEN_BIT_ID_MODE_W {
        TEN_BIT_ID_MODE_W { w: self }
    }
    #[doc = "Bit 14 - Slave Reset"]
    #[inline(always)]
    pub fn slave_reset(&mut self) -> SLAVE_RESET_W {
        SLAVE_RESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Slave Device ID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_id](index.html) module"]
pub struct DEV_ID_SPEC;
impl crate::RegisterSpec for DEV_ID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_id::R](R) reader structure"]
impl crate::Readable for DEV_ID_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_id::W](W) writer structure"]
impl crate::Writable for DEV_ID_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEV_ID to value 0"]
impl crate::Resettable for DEV_ID_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
