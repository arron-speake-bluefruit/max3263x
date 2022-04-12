#[doc = "Register `MAWS` reader"]
pub struct R(crate::R<MAWS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAWS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAWS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAWS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAWS` writer"]
pub struct W(crate::W<MAWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAWS_SPEC>;
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
impl From<crate::W<MAWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `modlen` reader - MAA Word Size"]
pub struct MODLEN_R(crate::FieldReader<u16, u16>);
impl MODLEN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        MODLEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODLEN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `modlen` writer - MAA Word Size"]
pub struct MODLEN_W<'a> {
    w: &'a mut W,
}
impl<'a> MODLEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `byteswap` reader - Big Endian Byte Mode"]
pub struct BYTESWAP_R(crate::FieldReader<bool, bool>);
impl BYTESWAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTESWAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTESWAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byteswap` writer - Big Endian Byte Mode"]
pub struct BYTESWAP_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTESWAP_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - MAA Word Size"]
    #[inline(always)]
    pub fn modlen(&self) -> MODLEN_R {
        MODLEN_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - Big Endian Byte Mode"]
    #[inline(always)]
    pub fn byteswap(&self) -> BYTESWAP_R {
        BYTESWAP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - MAA Word Size"]
    #[inline(always)]
    pub fn modlen(&mut self) -> MODLEN_W {
        MODLEN_W { w: self }
    }
    #[doc = "Bit 15 - Big Endian Byte Mode"]
    #[inline(always)]
    pub fn byteswap(&mut self) -> BYTESWAP_W {
        BYTESWAP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAA Word (Operand) Size, Big/Little Endian Mode Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [maws](index.html) module"]
pub struct MAWS_SPEC;
impl crate::RegisterSpec for MAWS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [maws::R](R) reader structure"]
impl crate::Readable for MAWS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [maws::W](W) writer structure"]
impl crate::Writable for MAWS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAWS to value 0"]
impl crate::Resettable for MAWS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
