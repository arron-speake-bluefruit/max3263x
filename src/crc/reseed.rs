#[doc = "Register `RESEED` reader"]
pub struct R(crate::R<RESEED_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESEED_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESEED_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESEED_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESEED` writer"]
pub struct W(crate::W<RESEED_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESEED_SPEC>;
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
impl From<crate::W<RESEED_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESEED_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `crc16` reader - Reseed CRC16 Generator"]
pub struct CRC16_R(crate::FieldReader<bool>);
impl CRC16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC16_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crc16` writer - Reseed CRC16 Generator"]
pub struct CRC16_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC16_W<'a> {
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
#[doc = "Field `crc32` reader - Reseed CRC32 Generator"]
pub struct CRC32_R(crate::FieldReader<bool>);
impl CRC32_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC32_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crc32` writer - Reseed CRC32 Generator"]
pub struct CRC32_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC32_W<'a> {
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
#[doc = "Field `rev_endian16` reader - Reverse Endianness for CRC16"]
pub struct REV_ENDIAN16_R(crate::FieldReader<bool>);
impl REV_ENDIAN16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REV_ENDIAN16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_ENDIAN16_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rev_endian16` writer - Reverse Endianness for CRC16"]
pub struct REV_ENDIAN16_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_ENDIAN16_W<'a> {
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
#[doc = "Field `rev_endian32` reader - Reverse Endianness for CRC32"]
pub struct REV_ENDIAN32_R(crate::FieldReader<bool>);
impl REV_ENDIAN32_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        REV_ENDIAN32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_ENDIAN32_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rev_endian32` writer - Reverse Endianness for CRC32"]
pub struct REV_ENDIAN32_W<'a> {
    w: &'a mut W,
}
impl<'a> REV_ENDIAN32_W<'a> {
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
#[doc = "Field `ccitt_mode` reader - CRC16-CCITT Mode"]
pub struct CCITT_MODE_R(crate::FieldReader<bool>);
impl CCITT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CCITT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCITT_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ccitt_mode` writer - CRC16-CCITT Mode"]
pub struct CCITT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CCITT_MODE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Reseed CRC16 Generator"]
    #[inline(always)]
    pub fn crc16(&self) -> CRC16_R {
        CRC16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reseed CRC32 Generator"]
    #[inline(always)]
    pub fn crc32(&self) -> CRC32_R {
        CRC32_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Reverse Endianness for CRC16"]
    #[inline(always)]
    pub fn rev_endian16(&self) -> REV_ENDIAN16_R {
        REV_ENDIAN16_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reverse Endianness for CRC32"]
    #[inline(always)]
    pub fn rev_endian32(&self) -> REV_ENDIAN32_R {
        REV_ENDIAN32_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - CRC16-CCITT Mode"]
    #[inline(always)]
    pub fn ccitt_mode(&self) -> CCITT_MODE_R {
        CCITT_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reseed CRC16 Generator"]
    #[inline(always)]
    pub fn crc16(&mut self) -> CRC16_W {
        CRC16_W { w: self }
    }
    #[doc = "Bit 1 - Reseed CRC32 Generator"]
    #[inline(always)]
    pub fn crc32(&mut self) -> CRC32_W {
        CRC32_W { w: self }
    }
    #[doc = "Bit 4 - Reverse Endianness for CRC16"]
    #[inline(always)]
    pub fn rev_endian16(&mut self) -> REV_ENDIAN16_W {
        REV_ENDIAN16_W { w: self }
    }
    #[doc = "Bit 5 - Reverse Endianness for CRC32"]
    #[inline(always)]
    pub fn rev_endian32(&mut self) -> REV_ENDIAN32_W {
        REV_ENDIAN32_W { w: self }
    }
    #[doc = "Bit 8 - CRC16-CCITT Mode"]
    #[inline(always)]
    pub fn ccitt_mode(&mut self) -> CCITT_MODE_W {
        CCITT_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CRC-16/CRC-32 Reseed Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reseed](index.html) module"]
pub struct RESEED_SPEC;
impl crate::RegisterSpec for RESEED_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reseed::R](R) reader structure"]
impl crate::Readable for RESEED_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reseed::W](W) writer structure"]
impl crate::Writable for RESEED_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESEED to value 0"]
impl crate::Resettable for RESEED_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
