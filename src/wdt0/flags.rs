#[doc = "Register `FLAGS` reader"]
pub struct R(crate::R<FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLAGS` writer"]
pub struct W(crate::W<FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLAGS_SPEC>;
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
impl From<crate::W<FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `timeout` reader - Watchdog Timeout Interrupt Flag"]
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
#[doc = "Field `timeout` writer - Watchdog Timeout Interrupt Flag"]
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `pre_win` reader - Watchdog Pre-Window Clear Interrupt Flag"]
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
#[doc = "Field `pre_win` writer - Watchdog Pre-Window Clear Interrupt Flag"]
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `reset_out` reader - Watchdog Reset Flag"]
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
#[doc = "Field `reset_out` writer - Watchdog Reset Flag"]
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog Pre-Window Clear Interrupt Flag"]
    #[inline(always)]
    pub fn pre_win(&self) -> PRE_WIN_R {
        PRE_WIN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog Reset Flag"]
    #[inline(always)]
    pub fn reset_out(&self) -> RESET_OUT_R {
        RESET_OUT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Watchdog Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W {
        TIMEOUT_W { w: self }
    }
    #[doc = "Bit 1 - Watchdog Pre-Window Clear Interrupt Flag"]
    #[inline(always)]
    pub fn pre_win(&mut self) -> PRE_WIN_W {
        PRE_WIN_W { w: self }
    }
    #[doc = "Bit 2 - Watchdog Reset Flag"]
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
#[doc = "WDT0 - Watchdog Interrupt and Reset Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](index.html) module"]
pub struct FLAGS_SPEC;
impl crate::RegisterSpec for FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flags::R](R) reader structure"]
impl crate::Readable for FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flags::W](W) writer structure"]
impl crate::Writable for FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLAGS to value 0"]
impl crate::Resettable for FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
