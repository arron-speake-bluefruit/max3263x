#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `v1_2_warning` reader - 1.2V Warning Monitor Int Enable"]
pub struct V1_2_WARNING_R(crate::FieldReader<bool>);
impl V1_2_WARNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        V1_2_WARNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V1_2_WARNING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `v1_2_warning` writer - 1.2V Warning Monitor Int Enable"]
pub struct V1_2_WARNING_W<'a> {
    w: &'a mut W,
}
impl<'a> V1_2_WARNING_W<'a> {
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
#[doc = "Field `v1_8_warning` reader - 1.8V Warning Monitor Int Enable"]
pub struct V1_8_WARNING_R(crate::FieldReader<bool>);
impl V1_8_WARNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        V1_8_WARNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for V1_8_WARNING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `v1_8_warning` writer - 1.8V Warning Monitor Int Enable"]
pub struct V1_8_WARNING_W<'a> {
    w: &'a mut W,
}
impl<'a> V1_8_WARNING_W<'a> {
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
#[doc = "Field `rtc_warning` reader - RTC Warning Monitor Int Enable"]
pub struct RTC_WARNING_R(crate::FieldReader<bool>);
impl RTC_WARNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_WARNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_WARNING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_warning` writer - RTC Warning Monitor Int Enable"]
pub struct RTC_WARNING_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_WARNING_W<'a> {
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
#[doc = "Field `vdda_warning` reader - VDDA Warning Monitor Int Enable"]
pub struct VDDA_WARNING_R(crate::FieldReader<bool>);
impl VDDA_WARNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDA_WARNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDA_WARNING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vdda_warning` writer - VDDA Warning Monitor Int Enable"]
pub struct VDDA_WARNING_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDA_WARNING_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `vddb_warning` reader - VDDB Warning Monitor Int Enable"]
pub struct VDDB_WARNING_R(crate::FieldReader<bool>);
impl VDDB_WARNING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VDDB_WARNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDB_WARNING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vddb_warning` writer - VDDB Warning Monitor Int Enable"]
pub struct VDDB_WARNING_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDB_WARNING_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 1.2V Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn v1_2_warning(&self) -> V1_2_WARNING_R {
        V1_2_WARNING_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1.8V Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn v1_8_warning(&self) -> V1_8_WARNING_R {
        V1_8_WARNING_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RTC Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn rtc_warning(&self) -> RTC_WARNING_R {
        RTC_WARNING_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VDDA Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn vdda_warning(&self) -> VDDA_WARNING_R {
        VDDA_WARNING_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VDDB Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn vddb_warning(&self) -> VDDB_WARNING_R {
        VDDB_WARNING_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1.2V Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn v1_2_warning(&mut self) -> V1_2_WARNING_W {
        V1_2_WARNING_W { w: self }
    }
    #[doc = "Bit 1 - 1.8V Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn v1_8_warning(&mut self) -> V1_8_WARNING_W {
        V1_8_WARNING_W { w: self }
    }
    #[doc = "Bit 2 - RTC Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn rtc_warning(&mut self) -> RTC_WARNING_W {
        RTC_WARNING_W { w: self }
    }
    #[doc = "Bit 3 - VDDA Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn vdda_warning(&mut self) -> VDDA_WARNING_W {
        VDDA_WARNING_W { w: self }
    }
    #[doc = "Bit 4 - VDDB Warning Monitor Int Enable"]
    #[inline(always)]
    pub fn vddb_warning(&mut self) -> VDDB_WARNING_W {
        VDDB_WARNING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable/Disable Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
