#[doc = "Register `TRIM_CALC` reader"]
pub struct R(crate::R<TRIM_CALC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_CALC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_CALC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_CALC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_CALC` writer"]
pub struct W(crate::W<TRIM_CALC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_CALC_SPEC>;
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
impl From<crate::W<TRIM_CALC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_CALC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `trim_clk_sel` reader - Trim Clock Select"]
pub struct TRIM_CLK_SEL_R(crate::FieldReader<bool, bool>);
impl TRIM_CLK_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_CLK_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_CLK_SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_clk_sel` writer - Trim Clock Select"]
pub struct TRIM_CLK_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_CLK_SEL_W<'a> {
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
#[doc = "Field `trim_calc_start` reader - Start Trim Calculation"]
pub struct TRIM_CALC_START_R(crate::FieldReader<bool, bool>);
impl TRIM_CALC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_CALC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_CALC_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_calc_start` writer - Start Trim Calculation"]
pub struct TRIM_CALC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_CALC_START_W<'a> {
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
#[doc = "Field `trim_calc_completed` reader - Trim Calculation Completed"]
pub struct TRIM_CALC_COMPLETED_R(crate::FieldReader<bool, bool>);
impl TRIM_CALC_COMPLETED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_CALC_COMPLETED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_CALC_COMPLETED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_enable` reader - Trim Logic Enable"]
pub struct TRIM_ENABLE_R(crate::FieldReader<bool, bool>);
impl TRIM_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_enable` writer - Trim Logic Enable"]
pub struct TRIM_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `trim_calc_results` reader - Trim Calculation Results"]
pub struct TRIM_CALC_RESULTS_R(crate::FieldReader<u16, u16>);
impl TRIM_CALC_RESULTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u16) -> Self {
        TRIM_CALC_RESULTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_CALC_RESULTS_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Trim Clock Select"]
    #[inline(always)]
    pub fn trim_clk_sel(&self) -> TRIM_CLK_SEL_R {
        TRIM_CLK_SEL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Trim Calculation"]
    #[inline(always)]
    pub fn trim_calc_start(&self) -> TRIM_CALC_START_R {
        TRIM_CALC_START_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Trim Calculation Completed"]
    #[inline(always)]
    pub fn trim_calc_completed(&self) -> TRIM_CALC_COMPLETED_R {
        TRIM_CALC_COMPLETED_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Trim Logic Enable"]
    #[inline(always)]
    pub fn trim_enable(&self) -> TRIM_ENABLE_R {
        TRIM_ENABLE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - Trim Calculation Results"]
    #[inline(always)]
    pub fn trim_calc_results(&self) -> TRIM_CALC_RESULTS_R {
        TRIM_CALC_RESULTS_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Trim Clock Select"]
    #[inline(always)]
    pub fn trim_clk_sel(&mut self) -> TRIM_CLK_SEL_W {
        TRIM_CLK_SEL_W { w: self }
    }
    #[doc = "Bit 1 - Start Trim Calculation"]
    #[inline(always)]
    pub fn trim_calc_start(&mut self) -> TRIM_CALC_START_W {
        TRIM_CALC_START_W { w: self }
    }
    #[doc = "Bit 3 - Trim Logic Enable"]
    #[inline(always)]
    pub fn trim_enable(&mut self) -> TRIM_ENABLE_W {
        TRIM_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Trim Calculation Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_calc](index.html) module"]
pub struct TRIM_CALC_SPEC;
impl crate::RegisterSpec for TRIM_CALC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_calc::R](R) reader structure"]
impl crate::Readable for TRIM_CALC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_calc::W](W) writer structure"]
impl crate::Writable for TRIM_CALC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_CALC to value 0"]
impl crate::Resettable for TRIM_CALC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
