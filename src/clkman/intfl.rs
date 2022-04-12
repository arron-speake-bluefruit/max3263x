#[doc = "Register `INTFL` reader"]
pub struct R(crate::R<INTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL` writer"]
pub struct W(crate::W<INTFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL_SPEC>;
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
impl From<crate::W<INTFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `crypto_stable` reader - Crypto Oscillator Stable Interrupt Flag"]
pub struct CRYPTO_STABLE_R(crate::FieldReader<bool, bool>);
impl CRYPTO_STABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_STABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_STABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crypto_stable` writer - Crypto Oscillator Stable Interrupt Flag"]
pub struct CRYPTO_STABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_STABLE_W<'a> {
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
#[doc = "Field `sys_ro_stable` reader - System Oscillator Stable Interrupt Flag"]
pub struct SYS_RO_STABLE_R(crate::FieldReader<bool, bool>);
impl SYS_RO_STABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SYS_RO_STABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYS_RO_STABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sys_ro_stable` writer - System Oscillator Stable Interrupt Flag"]
pub struct SYS_RO_STABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> SYS_RO_STABLE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Crypto Oscillator Stable Interrupt Flag"]
    #[inline(always)]
    pub fn crypto_stable(&self) -> CRYPTO_STABLE_R {
        CRYPTO_STABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - System Oscillator Stable Interrupt Flag"]
    #[inline(always)]
    pub fn sys_ro_stable(&self) -> SYS_RO_STABLE_R {
        SYS_RO_STABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Crypto Oscillator Stable Interrupt Flag"]
    #[inline(always)]
    pub fn crypto_stable(&mut self) -> CRYPTO_STABLE_W {
        CRYPTO_STABLE_W { w: self }
    }
    #[doc = "Bit 1 - System Oscillator Stable Interrupt Flag"]
    #[inline(always)]
    pub fn sys_ro_stable(&mut self) -> SYS_RO_STABLE_W {
        SYS_RO_STABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl::R](R) reader structure"]
impl crate::Readable for INTFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl::W](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
