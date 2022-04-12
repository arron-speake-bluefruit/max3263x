#[doc = "Register `MARGIN_CTRL` reader"]
pub struct R(crate::R<MARGIN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MARGIN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MARGIN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MARGIN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MARGIN_CTRL` writer"]
pub struct W(crate::W<MARGIN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MARGIN_CTRL_SPEC>;
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
impl From<crate::W<MARGIN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MARGIN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `extra_margin` reader - Extra Margin Adjustment"]
pub struct EXTRA_MARGIN_R(crate::FieldReader<u8, u8>);
impl EXTRA_MARGIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTRA_MARGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRA_MARGIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `extra_margin` writer - Extra Margin Adjustment"]
pub struct EXTRA_MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRA_MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `extra_write_margin` reader - Extra Write Margin Adjustment"]
pub struct EXTRA_WRITE_MARGIN_R(crate::FieldReader<u8, u8>);
impl EXTRA_WRITE_MARGIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EXTRA_WRITE_MARGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRA_WRITE_MARGIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `extra_write_margin` writer - Extra Write Margin Adjustment"]
pub struct EXTRA_WRITE_MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRA_WRITE_MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 3)) | ((value as u32 & 3) << 3);
        self.w
    }
}
#[doc = "Field `write_assist_en` reader - Write Assist Enable"]
pub struct WRITE_ASSIST_EN_R(crate::FieldReader<bool, bool>);
impl WRITE_ASSIST_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_ASSIST_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_ASSIST_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `write_assist_en` writer - Write Assist Enable"]
pub struct WRITE_ASSIST_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ASSIST_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `write_assist_margin` reader - Write Assist Margin Adjustment"]
pub struct WRITE_ASSIST_MARGIN_R(crate::FieldReader<u8, u8>);
impl WRITE_ASSIST_MARGIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WRITE_ASSIST_MARGIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_ASSIST_MARGIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `write_assist_margin` writer - Write Assist Margin Adjustment"]
pub struct WRITE_ASSIST_MARGIN_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_ASSIST_MARGIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Extra Margin Adjustment"]
    #[inline(always)]
    pub fn extra_margin(&self) -> EXTRA_MARGIN_R {
        EXTRA_MARGIN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Extra Write Margin Adjustment"]
    #[inline(always)]
    pub fn extra_write_margin(&self) -> EXTRA_WRITE_MARGIN_R {
        EXTRA_WRITE_MARGIN_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Write Assist Enable"]
    #[inline(always)]
    pub fn write_assist_en(&self) -> WRITE_ASSIST_EN_R {
        WRITE_ASSIST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Write Assist Margin Adjustment"]
    #[inline(always)]
    pub fn write_assist_margin(&self) -> WRITE_ASSIST_MARGIN_R {
        WRITE_ASSIST_MARGIN_R::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Extra Margin Adjustment"]
    #[inline(always)]
    pub fn extra_margin(&mut self) -> EXTRA_MARGIN_W {
        EXTRA_MARGIN_W { w: self }
    }
    #[doc = "Bits 3:4 - Extra Write Margin Adjustment"]
    #[inline(always)]
    pub fn extra_write_margin(&mut self) -> EXTRA_WRITE_MARGIN_W {
        EXTRA_WRITE_MARGIN_W { w: self }
    }
    #[doc = "Bit 5 - Write Assist Enable"]
    #[inline(always)]
    pub fn write_assist_en(&mut self) -> WRITE_ASSIST_EN_W {
        WRITE_ASSIST_EN_W { w: self }
    }
    #[doc = "Bits 6:7 - Write Assist Margin Adjustment"]
    #[inline(always)]
    pub fn write_assist_margin(&mut self) -> WRITE_ASSIST_MARGIN_W {
        WRITE_ASSIST_MARGIN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Margin Adjustment\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [margin_ctrl](index.html) module"]
pub struct MARGIN_CTRL_SPEC;
impl crate::RegisterSpec for MARGIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [margin_ctrl::R](R) reader structure"]
impl crate::Readable for MARGIN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [margin_ctrl::W](W) writer structure"]
impl crate::Writable for MARGIN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MARGIN_CTRL to value 0"]
impl crate::Resettable for MARGIN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
