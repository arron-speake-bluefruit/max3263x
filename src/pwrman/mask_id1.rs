#[doc = "Register `MASK_ID1` reader"]
pub struct R(crate::R<MASK_ID1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_ID1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_ID1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_ID1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK_ID1` writer"]
pub struct W(crate::W<MASK_ID1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_ID1_SPEC>;
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
impl From<crate::W<MASK_ID1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_ID1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mask_id` reader - Mask ID\\[58:28\\]"]
pub struct MASK_ID_R(crate::FieldReader<u32>);
impl MASK_ID_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        MASK_ID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_ID_R {
    type Target = crate::FieldReader<u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mask_id_enable` reader - Enable Mask ID"]
pub struct MASK_ID_ENABLE_R(crate::FieldReader<bool>);
impl MASK_ID_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASK_ID_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_ID_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mask_id_enable` writer - Enable Mask ID"]
pub struct MASK_ID_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_ID_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:30 - Mask ID\\[58:28\\]"]
    #[inline(always)]
    pub fn mask_id(&self) -> MASK_ID_R {
        MASK_ID_R::new((self.bits & 0x7fff_ffff) as u32)
    }
    #[doc = "Bit 31 - Enable Mask ID"]
    #[inline(always)]
    pub fn mask_id_enable(&self) -> MASK_ID_ENABLE_R {
        MASK_ID_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Enable Mask ID"]
    #[inline(always)]
    pub fn mask_id_enable(&mut self) -> MASK_ID_ENABLE_W {
        MASK_ID_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask ID Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask_id1](index.html) module"]
pub struct MASK_ID1_SPEC;
impl crate::RegisterSpec for MASK_ID1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mask_id1::R](R) reader structure"]
impl crate::Readable for MASK_ID1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask_id1::W](W) writer structure"]
impl crate::Writable for MASK_ID1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASK_ID1 to value 0"]
impl crate::Resettable for MASK_ID1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
