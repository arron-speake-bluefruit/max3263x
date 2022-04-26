#[doc = "Register `OUT_OWNER` reader"]
pub struct R(crate::R<OUT_OWNER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_OWNER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_OWNER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_OWNER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OUT_OWNER` writer"]
pub struct W(crate::W<OUT_OWNER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OUT_OWNER_SPEC>;
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
impl From<crate::W<OUT_OWNER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OUT_OWNER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `buf0_owner` reader - Owner for OUT Buffer 0 for Endpoints"]
pub struct BUF0_OWNER_R(crate::FieldReader<u8>);
impl BUF0_OWNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUF0_OWNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF0_OWNER_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf0_owner` writer - Owner for OUT Buffer 0 for Endpoints"]
pub struct BUF0_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF0_OWNER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `buf1_owner` reader - Owner for OUT Buffer 1 for Endpoints"]
pub struct BUF1_OWNER_R(crate::FieldReader<u8>);
impl BUF1_OWNER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BUF1_OWNER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF1_OWNER_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf1_owner` writer - Owner for OUT Buffer 1 for Endpoints"]
pub struct BUF1_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF1_OWNER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | ((value as u32 & 0xff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Owner for OUT Buffer 0 for Endpoints"]
    #[inline(always)]
    pub fn buf0_owner(&self) -> BUF0_OWNER_R {
        BUF0_OWNER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Owner for OUT Buffer 1 for Endpoints"]
    #[inline(always)]
    pub fn buf1_owner(&self) -> BUF1_OWNER_R {
        BUF1_OWNER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Owner for OUT Buffer 0 for Endpoints"]
    #[inline(always)]
    pub fn buf0_owner(&mut self) -> BUF0_OWNER_W {
        BUF0_OWNER_W { w: self }
    }
    #[doc = "Bits 16:23 - Owner for OUT Buffer 1 for Endpoints"]
    #[inline(always)]
    pub fn buf1_owner(&mut self) -> BUF1_OWNER_W {
        BUF1_OWNER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB OUT Endpoint Buffer Owner Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_owner](index.html) module"]
pub struct OUT_OWNER_SPEC;
impl crate::RegisterSpec for OUT_OWNER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_owner::R](R) reader structure"]
impl crate::Readable for OUT_OWNER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [out_owner::W](W) writer structure"]
impl crate::Writable for OUT_OWNER_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OUT_OWNER to value 0"]
impl crate::Resettable for OUT_OWNER_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
