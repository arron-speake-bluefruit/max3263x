#[doc = "Register `ENABLE` reader"]
pub struct R(crate::R<ENABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ENABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ENABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ENABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ENABLE` writer"]
pub struct W(crate::W<ENABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ENABLE_SPEC>;
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
impl From<crate::W<ENABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ENABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timeout` reader - Enable Watchdog Interrupt"]
pub struct TIMEOUT_R(crate::FieldReader<bool, bool>);
impl TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timeout` writer - Enable Watchdog Interrupt"]
pub struct TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMEOUT_W<'a> {
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
#[doc = "Field `pre_win` reader - Enable Watchdog Pre-Window Reset Interrupt"]
pub struct PRE_WIN_R(crate::FieldReader<bool, bool>);
impl PRE_WIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRE_WIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRE_WIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pre_win` writer - Enable Watchdog Pre-Window Reset Interrupt"]
pub struct PRE_WIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PRE_WIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `reset_out` reader - Enable Watchdog Reset Output"]
pub struct RESET_OUT_R(crate::FieldReader<bool, bool>);
impl RESET_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RESET_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESET_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `reset_out` writer - Enable Watchdog Reset Output"]
pub struct RESET_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RESET_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable Watchdog Interrupt"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enable Watchdog Pre-Window Reset Interrupt"]
    #[inline(always)]
    pub fn pre_win(&self) -> PRE_WIN_R {
        PRE_WIN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Enable Watchdog Reset Output"]
    #[inline(always)]
    pub fn reset_out(&self) -> RESET_OUT_R {
        RESET_OUT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Watchdog Interrupt"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 1 - Enable Watchdog Pre-Window Reset Interrupt"]
    #[inline(always)]
    pub fn pre_win(&mut self) -> PRE_WIN_W {
        PRE_WIN_W { w: self }
    }
    #[doc = "Bit 2 - Enable Watchdog Reset Output"]
    #[inline(always)]
    pub fn reset_out(&mut self) -> RESET_OUT_W {
        RESET_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "WDT0 - Interrupt/Reset Enable/Disable Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [enable](index.html) module"]
pub struct ENABLE_SPEC;
impl crate::RegisterSpec for ENABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [enable::R](R) reader structure"]
impl crate::Readable for ENABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [enable::W](W) writer structure"]
impl crate::Writable for ENABLE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ENABLE to value 0"]
impl crate::Resettable for ENABLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
