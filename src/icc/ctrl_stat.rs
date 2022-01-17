#[doc = "Register `CTRL_STAT` reader"]
pub struct R(crate::R<CTRL_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_STAT` writer"]
pub struct W(crate::W<CTRL_STAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_STAT_SPEC>;
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
impl From<crate::W<CTRL_STAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_STAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - Cache Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable` writer - Cache Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `ready` reader - Cache Ready Status"]
pub struct READY_R(crate::FieldReader<bool, bool>);
impl READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 16 - Cache Ready Status"]
    #[inline(always)]
    pub fn ready(&self) -> READY_R {
        READY_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cache Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control and Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_stat](index.html) module"]
pub struct CTRL_STAT_SPEC;
impl crate::RegisterSpec for CTRL_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_stat::R](R) reader structure"]
impl crate::Readable for CTRL_STAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_stat::W](W) writer structure"]
impl crate::Writable for CTRL_STAT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL_STAT to value 0"]
impl crate::Resettable for CTRL_STAT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
