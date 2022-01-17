#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode` reader - Operating Modes for 32-bit/16-bit Timers"]
pub struct MODE_R(crate::FieldReader<u8, u8>);
impl MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode` writer - Operating Modes for 32-bit/16-bit Timers"]
pub struct MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `tmr2x16` reader - Dual 16-bit Timer Mode"]
pub struct TMR2X16_R(crate::FieldReader<bool, bool>);
impl TMR2X16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TMR2X16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR2X16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tmr2x16` writer - Dual 16-bit Timer Mode"]
pub struct TMR2X16_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR2X16_W<'a> {
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
#[doc = "Field `prescale` reader - Timer Clock Prescale Setting"]
pub struct PRESCALE_R(crate::FieldReader<u8, u8>);
impl PRESCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PRESCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `prescale` writer - Timer Clock Prescale Setting"]
pub struct PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `polarity` reader - Timer I/O Polarity"]
pub struct POLARITY_R(crate::FieldReader<bool, bool>);
impl POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `polarity` writer - Timer I/O Polarity"]
pub struct POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `enable0` reader - Enable 32-bit timer / 16-bit timer 0"]
pub struct ENABLE0_R(crate::FieldReader<bool, bool>);
impl ENABLE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable0` writer - Enable 32-bit timer / 16-bit timer 0"]
pub struct ENABLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `enable1` reader - Enable 16-bit timer 1"]
pub struct ENABLE1_R(crate::FieldReader<bool, bool>);
impl ENABLE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable1` writer - Enable 16-bit timer 1"]
pub struct ENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Operating Modes for 32-bit/16-bit Timers"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - Dual 16-bit Timer Mode"]
    #[inline(always)]
    pub fn tmr2x16(&self) -> TMR2X16_R {
        TMR2X16_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Timer Clock Prescale Setting"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Timer I/O Polarity"]
    #[inline(always)]
    pub fn polarity(&self) -> POLARITY_R {
        POLARITY_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Enable 32-bit timer / 16-bit timer 0"]
    #[inline(always)]
    pub fn enable0(&self) -> ENABLE0_R {
        ENABLE0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Enable 16-bit timer 1"]
    #[inline(always)]
    pub fn enable1(&self) -> ENABLE1_R {
        ENABLE1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Operating Modes for 32-bit/16-bit Timers"]
    #[inline(always)]
    pub fn mode(&mut self) -> MODE_W {
        MODE_W { w: self }
    }
    #[doc = "Bit 3 - Dual 16-bit Timer Mode"]
    #[inline(always)]
    pub fn tmr2x16(&mut self) -> TMR2X16_W {
        TMR2X16_W { w: self }
    }
    #[doc = "Bits 4:7 - Timer Clock Prescale Setting"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W {
        PRESCALE_W { w: self }
    }
    #[doc = "Bit 8 - Timer I/O Polarity"]
    #[inline(always)]
    pub fn polarity(&mut self) -> POLARITY_W {
        POLARITY_W { w: self }
    }
    #[doc = "Bit 12 - Enable 32-bit timer / 16-bit timer 0"]
    #[inline(always)]
    pub fn enable0(&mut self) -> ENABLE0_W {
        ENABLE0_W { w: self }
    }
    #[doc = "Bit 13 - Enable 16-bit timer 1"]
    #[inline(always)]
    pub fn enable1(&mut self) -> ENABLE1_W {
        ENABLE1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
