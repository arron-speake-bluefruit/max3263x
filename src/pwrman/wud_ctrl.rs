#[doc = "Register `WUD_CTRL` reader"]
pub struct R(crate::R<WUD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_CTRL` writer"]
pub struct W(crate::W<WUD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_CTRL_SPEC>;
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
impl From<crate::W<WUD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pad_select` reader - Wake-Up Pad Select"]
pub struct PAD_SELECT_R(crate::FieldReader<u8>);
impl PAD_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_SELECT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_select` writer - Wake-Up Pad Select"]
pub struct PAD_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `pad_mode` reader - Wake-Up Pad Signal Mode"]
pub struct PAD_MODE_R(crate::FieldReader<u8>);
impl PAD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAD_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pad_mode` writer - Wake-Up Pad Signal Mode"]
pub struct PAD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAD_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `clear_all` reader - Clear All WUD Pad States"]
pub struct CLEAR_ALL_R(crate::FieldReader<bool>);
impl CLEAR_ALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLEAR_ALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLEAR_ALL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clear_all` writer - Clear All WUD Pad States"]
pub struct CLEAR_ALL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_ALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `ctrl_enable` reader - Enable WUD Control Modification"]
pub struct CTRL_ENABLE_R(crate::FieldReader<bool>);
impl CTRL_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTRL_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTRL_ENABLE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ctrl_enable` writer - Enable WUD Control Modification"]
pub struct CTRL_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CTRL_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - Wake-Up Pad Select"]
    #[inline(always)]
    pub fn pad_select(&self) -> PAD_SELECT_R {
        PAD_SELECT_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Wake-Up Pad Signal Mode"]
    #[inline(always)]
    pub fn pad_mode(&self) -> PAD_MODE_R {
        PAD_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - Clear All WUD Pad States"]
    #[inline(always)]
    pub fn clear_all(&self) -> CLEAR_ALL_R {
        CLEAR_ALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable WUD Control Modification"]
    #[inline(always)]
    pub fn ctrl_enable(&self) -> CTRL_ENABLE_R {
        CTRL_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Wake-Up Pad Select"]
    #[inline(always)]
    pub fn pad_select(&mut self) -> PAD_SELECT_W {
        PAD_SELECT_W { w: self }
    }
    #[doc = "Bits 8:9 - Wake-Up Pad Signal Mode"]
    #[inline(always)]
    pub fn pad_mode(&mut self) -> PAD_MODE_W {
        PAD_MODE_W { w: self }
    }
    #[doc = "Bit 12 - Clear All WUD Pad States"]
    #[inline(always)]
    pub fn clear_all(&mut self) -> CLEAR_ALL_W {
        CLEAR_ALL_W { w: self }
    }
    #[doc = "Bit 16 - Enable WUD Control Modification"]
    #[inline(always)]
    pub fn ctrl_enable(&mut self) -> CTRL_ENABLE_W {
        CTRL_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-Up Detect Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_ctrl](index.html) module"]
pub struct WUD_CTRL_SPEC;
impl crate::RegisterSpec for WUD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_ctrl::R](R) reader structure"]
impl crate::Readable for WUD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_ctrl::W](W) writer structure"]
impl crate::Writable for WUD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_CTRL to value 0"]
impl crate::Resettable for WUD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
