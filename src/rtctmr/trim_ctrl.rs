#[doc = "Register `TRIM_CTRL` reader"]
pub struct R(crate::R<TRIM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_CTRL` writer"]
pub struct W(crate::W<TRIM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_CTRL_SPEC>;
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
impl From<crate::W<TRIM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `trim_enable_r` reader - Enable RTL Trim of RTC Timer"]
pub struct TRIM_ENABLE_R_R(crate::FieldReader<bool, bool>);
impl TRIM_ENABLE_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_ENABLE_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_ENABLE_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_enable_r` writer - Enable RTL Trim of RTC Timer"]
pub struct TRIM_ENABLE_R_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_ENABLE_R_W<'a> {
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
#[doc = "Field `trim_faster_ovr_r` reader - Force RTC Trim to Faster"]
pub struct TRIM_FASTER_OVR_R_R(crate::FieldReader<bool, bool>);
impl TRIM_FASTER_OVR_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_FASTER_OVR_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_FASTER_OVR_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_faster_ovr_r` writer - Force RTC Trim to Faster"]
pub struct TRIM_FASTER_OVR_R_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_FASTER_OVR_R_W<'a> {
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
#[doc = "Field `trim_slower_r` reader - RTC Trim Direction Status"]
pub struct TRIM_SLOWER_R_R(crate::FieldReader<bool, bool>);
impl TRIM_SLOWER_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_SLOWER_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_SLOWER_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Enable RTL Trim of RTC Timer"]
    #[inline(always)]
    pub fn trim_enable_r(&self) -> TRIM_ENABLE_R_R {
        TRIM_ENABLE_R_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force RTC Trim to Faster"]
    #[inline(always)]
    pub fn trim_faster_ovr_r(&self) -> TRIM_FASTER_OVR_R_R {
        TRIM_FASTER_OVR_R_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Trim Direction Status"]
    #[inline(always)]
    pub fn trim_slower_r(&self) -> TRIM_SLOWER_R_R {
        TRIM_SLOWER_R_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable RTL Trim of RTC Timer"]
    #[inline(always)]
    pub fn trim_enable_r(&mut self) -> TRIM_ENABLE_R_W {
        TRIM_ENABLE_R_W { w: self }
    }
    #[doc = "Bit 1 - Force RTC Trim to Faster"]
    #[inline(always)]
    pub fn trim_faster_ovr_r(&mut self) -> TRIM_FASTER_OVR_R_W {
        TRIM_FASTER_OVR_R_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Timer Trim Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ctrl](index.html) module"]
pub struct TRIM_CTRL_SPEC;
impl crate::RegisterSpec for TRIM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ctrl::R](R) reader structure"]
impl crate::Readable for TRIM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ctrl::W](W) writer structure"]
impl crate::Writable for TRIM_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_CTRL to value 0"]
impl crate::Resettable for TRIM_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
