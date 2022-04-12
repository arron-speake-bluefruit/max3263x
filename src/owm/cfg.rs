#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `long_line_mode` reader - Long Line Mode"]
pub struct LONG_LINE_MODE_R(crate::FieldReader<bool, bool>);
impl LONG_LINE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LONG_LINE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LONG_LINE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `long_line_mode` writer - Long Line Mode"]
pub struct LONG_LINE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LONG_LINE_MODE_W<'a> {
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
#[doc = "Field `force_pres_det` reader - Force Line During Presence Detect"]
pub struct FORCE_PRES_DET_R(crate::FieldReader<bool, bool>);
impl FORCE_PRES_DET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FORCE_PRES_DET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FORCE_PRES_DET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `force_pres_det` writer - Force Line During Presence Detect"]
pub struct FORCE_PRES_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> FORCE_PRES_DET_W<'a> {
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
#[doc = "Field `bit_bang_en` reader - Bit Bang Enable"]
pub struct BIT_BANG_EN_R(crate::FieldReader<bool, bool>);
impl BIT_BANG_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT_BANG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_BANG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bit_bang_en` writer - Bit Bang Enable"]
pub struct BIT_BANG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_BANG_EN_W<'a> {
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
#[doc = "Field `ext_pullup_mode` reader - Provide an extra output to control an external pullup."]
pub struct EXT_PULLUP_MODE_R(crate::FieldReader<bool, bool>);
impl EXT_PULLUP_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_PULLUP_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_PULLUP_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ext_pullup_mode` writer - Provide an extra output to control an external pullup."]
pub struct EXT_PULLUP_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_PULLUP_MODE_W<'a> {
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
#[doc = "Field `ext_pullup_enable` reader - Enable External Pullup"]
pub struct EXT_PULLUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl EXT_PULLUP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXT_PULLUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXT_PULLUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ext_pullup_enable` writer - Enable External Pullup"]
pub struct EXT_PULLUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> EXT_PULLUP_ENABLE_W<'a> {
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
#[doc = "Field `single_bit_mode` reader - Enable Single Bit TX/RX Mode"]
pub struct SINGLE_BIT_MODE_R(crate::FieldReader<bool, bool>);
impl SINGLE_BIT_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_BIT_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_BIT_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `single_bit_mode` writer - Enable Single Bit TX/RX Mode"]
pub struct SINGLE_BIT_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SINGLE_BIT_MODE_W<'a> {
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
#[doc = "Field `overdrive` reader - Enables overdrive speed for 1-Wire operations."]
pub struct OVERDRIVE_R(crate::FieldReader<bool, bool>);
impl OVERDRIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERDRIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERDRIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `overdrive` writer - Enables overdrive speed for 1-Wire operations."]
pub struct OVERDRIVE_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERDRIVE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `int_pullup_enable` reader - Enable internal pullup."]
pub struct INT_PULLUP_ENABLE_R(crate::FieldReader<bool, bool>);
impl INT_PULLUP_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_PULLUP_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_PULLUP_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `int_pullup_enable` writer - Enable internal pullup."]
pub struct INT_PULLUP_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_PULLUP_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Long Line Mode"]
    #[inline(always)]
    pub fn long_line_mode(&self) -> LONG_LINE_MODE_R {
        LONG_LINE_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force Line During Presence Detect"]
    #[inline(always)]
    pub fn force_pres_det(&self) -> FORCE_PRES_DET_R {
        FORCE_PRES_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Enable"]
    #[inline(always)]
    pub fn bit_bang_en(&self) -> BIT_BANG_EN_R {
        BIT_BANG_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Provide an extra output to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&self) -> EXT_PULLUP_MODE_R {
        EXT_PULLUP_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable External Pullup"]
    #[inline(always)]
    pub fn ext_pullup_enable(&self) -> EXT_PULLUP_ENABLE_R {
        EXT_PULLUP_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode"]
    #[inline(always)]
    pub fn single_bit_mode(&self) -> SINGLE_BIT_MODE_R {
        SINGLE_BIT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&self) -> OVERDRIVE_R {
        OVERDRIVE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable internal pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&self) -> INT_PULLUP_ENABLE_R {
        INT_PULLUP_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Long Line Mode"]
    #[inline(always)]
    pub fn long_line_mode(&mut self) -> LONG_LINE_MODE_W {
        LONG_LINE_MODE_W { w: self }
    }
    #[doc = "Bit 1 - Force Line During Presence Detect"]
    #[inline(always)]
    pub fn force_pres_det(&mut self) -> FORCE_PRES_DET_W {
        FORCE_PRES_DET_W { w: self }
    }
    #[doc = "Bit 2 - Bit Bang Enable"]
    #[inline(always)]
    pub fn bit_bang_en(&mut self) -> BIT_BANG_EN_W {
        BIT_BANG_EN_W { w: self }
    }
    #[doc = "Bit 3 - Provide an extra output to control an external pullup."]
    #[inline(always)]
    pub fn ext_pullup_mode(&mut self) -> EXT_PULLUP_MODE_W {
        EXT_PULLUP_MODE_W { w: self }
    }
    #[doc = "Bit 4 - Enable External Pullup"]
    #[inline(always)]
    pub fn ext_pullup_enable(&mut self) -> EXT_PULLUP_ENABLE_W {
        EXT_PULLUP_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - Enable Single Bit TX/RX Mode"]
    #[inline(always)]
    pub fn single_bit_mode(&mut self) -> SINGLE_BIT_MODE_W {
        SINGLE_BIT_MODE_W { w: self }
    }
    #[doc = "Bit 6 - Enables overdrive speed for 1-Wire operations."]
    #[inline(always)]
    pub fn overdrive(&mut self) -> OVERDRIVE_W {
        OVERDRIVE_W { w: self }
    }
    #[doc = "Bit 7 - Enable internal pullup."]
    #[inline(always)]
    pub fn int_pullup_enable(&mut self) -> INT_PULLUP_ENABLE_W {
        INT_PULLUP_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
