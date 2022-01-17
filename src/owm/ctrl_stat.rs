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
#[doc = "Field `start_ow_reset` reader - Start OW Reset"]
pub struct START_OW_RESET_R(crate::FieldReader<bool, bool>);
impl START_OW_RESET_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        START_OW_RESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for START_OW_RESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `start_ow_reset` writer - Start OW Reset"]
pub struct START_OW_RESET_W<'a> {
    w: &'a mut W,
}
impl<'a> START_OW_RESET_W<'a> {
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
#[doc = "Field `sra_mode` reader - SRA Mode"]
pub struct SRA_MODE_R(crate::FieldReader<bool, bool>);
impl SRA_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRA_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRA_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sra_mode` writer - SRA Mode"]
pub struct SRA_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SRA_MODE_W<'a> {
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
#[doc = "Field `bit_bang_oe` reader - Bit Bang Output Enable"]
pub struct BIT_BANG_OE_R(crate::FieldReader<bool, bool>);
impl BIT_BANG_OE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BIT_BANG_OE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BIT_BANG_OE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bit_bang_oe` writer - Bit Bang Output Enable"]
pub struct BIT_BANG_OE_W<'a> {
    w: &'a mut W,
}
impl<'a> BIT_BANG_OE_W<'a> {
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
#[doc = "Field `ow_input` reader - OW Input State"]
pub struct OW_INPUT_R(crate::FieldReader<bool, bool>);
impl OW_INPUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OW_INPUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OW_INPUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `presence_detect` reader - Presence Pulse Detected"]
pub struct PRESENCE_DETECT_R(crate::FieldReader<bool, bool>);
impl PRESENCE_DETECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRESENCE_DETECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESENCE_DETECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `presence_detect` writer - Presence Pulse Detected"]
pub struct PRESENCE_DETECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESENCE_DETECT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start OW Reset"]
    #[inline(always)]
    pub fn start_ow_reset(&self) -> START_OW_RESET_R {
        START_OW_RESET_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - SRA Mode"]
    #[inline(always)]
    pub fn sra_mode(&self) -> SRA_MODE_R {
        SRA_MODE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bit Bang Output Enable"]
    #[inline(always)]
    pub fn bit_bang_oe(&self) -> BIT_BANG_OE_R {
        BIT_BANG_OE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OW Input State"]
    #[inline(always)]
    pub fn ow_input(&self) -> OW_INPUT_R {
        OW_INPUT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Presence Pulse Detected"]
    #[inline(always)]
    pub fn presence_detect(&self) -> PRESENCE_DETECT_R {
        PRESENCE_DETECT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start OW Reset"]
    #[inline(always)]
    pub fn start_ow_reset(&mut self) -> START_OW_RESET_W {
        START_OW_RESET_W { w: self }
    }
    #[doc = "Bit 1 - SRA Mode"]
    #[inline(always)]
    pub fn sra_mode(&mut self) -> SRA_MODE_W {
        SRA_MODE_W { w: self }
    }
    #[doc = "Bit 2 - Bit Bang Output Enable"]
    #[inline(always)]
    pub fn bit_bang_oe(&mut self) -> BIT_BANG_OE_W {
        BIT_BANG_OE_W { w: self }
    }
    #[doc = "Bit 7 - Presence Pulse Detected"]
    #[inline(always)]
    pub fn presence_detect(&mut self) -> PRESENCE_DETECT_W {
        PRESENCE_DETECT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "1-Wire Master Control/Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_stat](index.html) module"]
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
