#[doc = "Register `DATA_BYTE` reader"]
pub struct R(crate::R<DATA_BYTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_BYTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_BYTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_BYTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_BYTE` writer"]
pub struct W(crate::W<DATA_BYTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_BYTE_SPEC>;
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
impl From<crate::W<DATA_BYTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_BYTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `data_field` reader - Data Field"]
pub struct DATA_FIELD_R(crate::FieldReader<u8, u8>);
impl DATA_FIELD_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_FIELD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_FIELD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `data_field` writer - Data Field"]
pub struct DATA_FIELD_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_FIELD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `read_only_fl` reader - Read Only Flag"]
pub struct READ_ONLY_FL_R(crate::FieldReader<bool, bool>);
impl READ_ONLY_FL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READ_ONLY_FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READ_ONLY_FL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `read_only_fl` writer - Read Only Flag"]
pub struct READ_ONLY_FL_W<'a> {
    w: &'a mut W,
}
impl<'a> READ_ONLY_FL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `data_updated_fl` reader - Byte Updated Flag"]
pub struct DATA_UPDATED_FL_R(crate::FieldReader<bool, bool>);
impl DATA_UPDATED_FL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DATA_UPDATED_FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_UPDATED_FL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Data Field"]
    #[inline(always)]
    pub fn data_field(&self) -> DATA_FIELD_R {
        DATA_FIELD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Read Only Flag"]
    #[inline(always)]
    pub fn read_only_fl(&self) -> READ_ONLY_FL_R {
        READ_ONLY_FL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Byte Updated Flag"]
    #[inline(always)]
    pub fn data_updated_fl(&self) -> DATA_UPDATED_FL_R {
        DATA_UPDATED_FL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data Field"]
    #[inline(always)]
    pub fn data_field(&mut self) -> DATA_FIELD_W {
        DATA_FIELD_W { w: self }
    }
    #[doc = "Bit 8 - Read Only Flag"]
    #[inline(always)]
    pub fn read_only_fl(&mut self) -> READ_ONLY_FL_W {
        READ_ONLY_FL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Data Byte\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_byte](index.html) module"]
pub struct DATA_BYTE_SPEC;
impl crate::RegisterSpec for DATA_BYTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_byte::R](R) reader structure"]
impl crate::Readable for DATA_BYTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_byte::W](W) writer structure"]
impl crate::Writable for DATA_BYTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATA_BYTE to value 0"]
impl crate::Resettable for DATA_BYTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
