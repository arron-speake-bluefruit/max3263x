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
#[doc = "Field `comp0` reader - RTC Compare 0 Interrupt Status"]
pub struct COMP0_R(crate::FieldReader<bool, bool>);
impl COMP0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `comp0` writer - RTC Compare 0 Interrupt Status"]
pub struct COMP0_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP0_W<'a> {
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
#[doc = "Field `comp1` reader - RTC Compare 1 Interrupt Status"]
pub struct COMP1_R(crate::FieldReader<bool, bool>);
impl COMP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `comp1` writer - RTC Compare 1 Interrupt Status"]
pub struct COMP1_W<'a> {
    w: &'a mut W,
}
impl<'a> COMP1_W<'a> {
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
#[doc = "Field `prescale_comp` reader - RTC Prescale Compare Int Status"]
pub struct PRESCALE_COMP_R(crate::FieldReader<bool, bool>);
impl PRESCALE_COMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRESCALE_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALE_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `prescale_comp` writer - RTC Prescale Compare Int Status"]
pub struct PRESCALE_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCALE_COMP_W<'a> {
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
#[doc = "Field `overflow` reader - RTC Overflow Interrupt Status"]
pub struct OVERFLOW_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `overflow` writer - RTC Overflow Interrupt Status"]
pub struct OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> OVERFLOW_W<'a> {
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
#[doc = "Field `trim` reader - RTC Trim Interrupt Status"]
pub struct TRIM_R(crate::FieldReader<bool, bool>);
impl TRIM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim` writer - RTC Trim Interrupt Status"]
pub struct TRIM_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `comp0_flag_a` reader - RTC Compare 0 4kHz Flag"]
pub struct COMP0_FLAG_A_R(crate::FieldReader<bool, bool>);
impl COMP0_FLAG_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP0_FLAG_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP0_FLAG_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `comp1_flag_a` reader - RTC Compare 1 4kHz Flag"]
pub struct COMP1_FLAG_A_R(crate::FieldReader<bool, bool>);
impl COMP1_FLAG_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        COMP1_FLAG_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COMP1_FLAG_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `prescl_flag_a` reader - RTC Prescale Compare 4kHz Flag"]
pub struct PRESCL_FLAG_A_R(crate::FieldReader<bool, bool>);
impl PRESCL_FLAG_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRESCL_FLAG_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCL_FLAG_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `overflow_flag_a` reader - RTC Overflow 4kHz Flag"]
pub struct OVERFLOW_FLAG_A_R(crate::FieldReader<bool, bool>);
impl OVERFLOW_FLAG_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OVERFLOW_FLAG_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OVERFLOW_FLAG_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_flag_a` reader - RTC Trim Event 4kHz Flag"]
pub struct TRIM_FLAG_A_R(crate::FieldReader<bool, bool>);
impl TRIM_FLAG_A_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_FLAG_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_FLAG_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `async_clr_flags` writer - Asynchronous RTC Flag Clear"]
pub struct ASYNC_CLR_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNC_CLR_FLAGS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RTC Compare 0 Interrupt Status"]
    #[inline(always)]
    pub fn comp0(&self) -> COMP0_R {
        COMP0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RTC Compare 1 Interrupt Status"]
    #[inline(always)]
    pub fn comp1(&self) -> COMP1_R {
        COMP1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Prescale Compare Int Status"]
    #[inline(always)]
    pub fn prescale_comp(&self) -> PRESCALE_COMP_R {
        PRESCALE_COMP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RTC Overflow Interrupt Status"]
    #[inline(always)]
    pub fn overflow(&self) -> OVERFLOW_R {
        OVERFLOW_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - RTC Trim Interrupt Status"]
    #[inline(always)]
    pub fn trim(&self) -> TRIM_R {
        TRIM_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RTC Compare 0 4kHz Flag"]
    #[inline(always)]
    pub fn comp0_flag_a(&self) -> COMP0_FLAG_A_R {
        COMP0_FLAG_A_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - RTC Compare 1 4kHz Flag"]
    #[inline(always)]
    pub fn comp1_flag_a(&self) -> COMP1_FLAG_A_R {
        COMP1_FLAG_A_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - RTC Prescale Compare 4kHz Flag"]
    #[inline(always)]
    pub fn prescl_flag_a(&self) -> PRESCL_FLAG_A_R {
        PRESCL_FLAG_A_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RTC Overflow 4kHz Flag"]
    #[inline(always)]
    pub fn overflow_flag_a(&self) -> OVERFLOW_FLAG_A_R {
        OVERFLOW_FLAG_A_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RTC Trim Event 4kHz Flag"]
    #[inline(always)]
    pub fn trim_flag_a(&self) -> TRIM_FLAG_A_R {
        TRIM_FLAG_A_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Compare 0 Interrupt Status"]
    #[inline(always)]
    pub fn comp0(&mut self) -> COMP0_W {
        COMP0_W { w: self }
    }
    #[doc = "Bit 1 - RTC Compare 1 Interrupt Status"]
    #[inline(always)]
    pub fn comp1(&mut self) -> COMP1_W {
        COMP1_W { w: self }
    }
    #[doc = "Bit 2 - RTC Prescale Compare Int Status"]
    #[inline(always)]
    pub fn prescale_comp(&mut self) -> PRESCALE_COMP_W {
        PRESCALE_COMP_W { w: self }
    }
    #[doc = "Bit 3 - RTC Overflow Interrupt Status"]
    #[inline(always)]
    pub fn overflow(&mut self) -> OVERFLOW_W {
        OVERFLOW_W { w: self }
    }
    #[doc = "Bit 4 - RTC Trim Interrupt Status"]
    #[inline(always)]
    pub fn trim(&mut self) -> TRIM_W {
        TRIM_W { w: self }
    }
    #[doc = "Bit 31 - Asynchronous RTC Flag Clear"]
    #[inline(always)]
    pub fn async_clr_flags(&mut self) -> ASYNC_CLR_FLAGS_W {
        ASYNC_CLR_FLAGS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CPU Interrupt and RTC Domain Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](index.html) module"]
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
