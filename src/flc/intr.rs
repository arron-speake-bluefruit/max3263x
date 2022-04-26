#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `finished_if` reader - Flash Write/Erase Operation Finished"]
pub struct FINISHED_IF_R(crate::FieldReader<bool>);
impl FINISHED_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FINISHED_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINISHED_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `finished_if` writer - Flash Write/Erase Operation Finished"]
pub struct FINISHED_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> FINISHED_IF_W<'a> {
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
#[doc = "Field `failed_if` reader - Flash Operation Failed"]
pub struct FAILED_IF_R(crate::FieldReader<bool>);
impl FAILED_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAILED_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAILED_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `failed_if` writer - Flash Operation Failed"]
pub struct FAILED_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> FAILED_IF_W<'a> {
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
#[doc = "Field `finished_ie` reader - Flash Write/Erase Operation Finished Interrupt Enable"]
pub struct FINISHED_IE_R(crate::FieldReader<bool>);
impl FINISHED_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FINISHED_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINISHED_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `finished_ie` writer - Flash Write/Erase Operation Finished Interrupt Enable"]
pub struct FINISHED_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FINISHED_IE_W<'a> {
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
#[doc = "Field `failed_ie` reader - Flash Operation Failed Interrupt Enable"]
pub struct FAILED_IE_R(crate::FieldReader<bool>);
impl FAILED_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAILED_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAILED_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `failed_ie` writer - Flash Operation Failed Interrupt Enable"]
pub struct FAILED_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> FAILED_IE_W<'a> {
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
#[doc = "Field `fail_flags` reader - Flash Operation Failure Details"]
pub struct FAIL_FLAGS_R(crate::FieldReader<u16>);
impl FAIL_FLAGS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        FAIL_FLAGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAIL_FLAGS_R {
    type Target = crate::FieldReader<u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Flash Write/Erase Operation Finished"]
    #[inline(always)]
    pub fn finished_if(&self) -> FINISHED_IF_R {
        FINISHED_IF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Operation Failed"]
    #[inline(always)]
    pub fn failed_if(&self) -> FAILED_IF_R {
        FAILED_IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Flash Write/Erase Operation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn finished_ie(&self) -> FINISHED_IE_R {
        FINISHED_IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Flash Operation Failed Interrupt Enable"]
    #[inline(always)]
    pub fn failed_ie(&self) -> FAILED_IE_R {
        FAILED_IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Flash Operation Failure Details"]
    #[inline(always)]
    pub fn fail_flags(&self) -> FAIL_FLAGS_R {
        FAIL_FLAGS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Write/Erase Operation Finished"]
    #[inline(always)]
    pub fn finished_if(&mut self) -> FINISHED_IF_W {
        FINISHED_IF_W { w: self }
    }
    #[doc = "Bit 1 - Flash Operation Failed"]
    #[inline(always)]
    pub fn failed_if(&mut self) -> FAILED_IF_W {
        FAILED_IF_W { w: self }
    }
    #[doc = "Bit 8 - Flash Write/Erase Operation Finished Interrupt Enable"]
    #[inline(always)]
    pub fn finished_ie(&mut self) -> FINISHED_IE_W {
        FINISHED_IE_W { w: self }
    }
    #[doc = "Bit 9 - Flash Operation Failed Interrupt Enable"]
    #[inline(always)]
    pub fn failed_ie(&mut self) -> FAILED_IE_W {
        FAILED_IE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Interrupt Flags and Enable/Disable 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
