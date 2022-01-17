#[doc = "Register `OSC_CTRL` reader"]
pub struct R(crate::R<OSC_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CTRL` writer"]
pub struct W(crate::W<OSC_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CTRL_SPEC>;
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
impl From<crate::W<OSC_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `osc_bypass` reader - Bypass RTC oscillator"]
pub struct OSC_BYPASS_R(crate::FieldReader<bool, bool>);
impl OSC_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `osc_bypass` writer - Bypass RTC oscillator"]
pub struct OSC_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_BYPASS_W<'a> {
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
#[doc = "Field `osc_disable_r` reader - if osc_disable_sel = 1, this will hold the RTC in reset."]
pub struct OSC_DISABLE_R_R(crate::FieldReader<bool, bool>);
impl OSC_DISABLE_R_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_DISABLE_R_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_DISABLE_R_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `osc_disable_r` writer - if osc_disable_sel = 1, this will hold the RTC in reset."]
pub struct OSC_DISABLE_R_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_DISABLE_R_W<'a> {
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
#[doc = "Select RTC Oscillator Disable Control Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OSC_DISABLE_SEL_A {
    #[doc = "0: PowerSequencer controls the reset state of the RTC"]
    PWRSEQ_CONTROL = 0,
    #[doc = "1: RTC reset controlled by osc_disable_r bit"]
    RTC_DOMAIN_CONTROL = 1,
}
impl From<OSC_DISABLE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: OSC_DISABLE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `osc_disable_sel` reader - Select RTC Oscillator Disable Control Source"]
pub struct OSC_DISABLE_SEL_R(crate::FieldReader<bool, OSC_DISABLE_SEL_A>);
impl OSC_DISABLE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_DISABLE_SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OSC_DISABLE_SEL_A {
        match self.bits {
            false => OSC_DISABLE_SEL_A::PWRSEQ_CONTROL,
            true => OSC_DISABLE_SEL_A::RTC_DOMAIN_CONTROL,
        }
    }
    #[doc = "Checks if the value of the field is `PWRSEQ_CONTROL`"]
    #[inline(always)]
    pub fn is_pwr_seq_control(&self) -> bool {
        **self == OSC_DISABLE_SEL_A::PWRSEQ_CONTROL
    }
    #[doc = "Checks if the value of the field is `RTC_DOMAIN_CONTROL`"]
    #[inline(always)]
    pub fn is_rtc_domain_control(&self) -> bool {
        **self == OSC_DISABLE_SEL_A::RTC_DOMAIN_CONTROL
    }
}
impl core::ops::Deref for OSC_DISABLE_SEL_R {
    type Target = crate::FieldReader<bool, OSC_DISABLE_SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `osc_disable_sel` writer - Select RTC Oscillator Disable Control Source"]
pub struct OSC_DISABLE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OSC_DISABLE_SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OSC_DISABLE_SEL_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PowerSequencer controls the reset state of the RTC"]
    #[inline(always)]
    pub fn pwr_seq_control(self) -> &'a mut W {
        self.variant(OSC_DISABLE_SEL_A::PWRSEQ_CONTROL)
    }
    #[doc = "RTC reset controlled by osc_disable_r bit"]
    #[inline(always)]
    pub fn rtc_domain_control(self) -> &'a mut W {
        self.variant(OSC_DISABLE_SEL_A::RTC_DOMAIN_CONTROL)
    }
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
#[doc = "Field `osc_disable_o` reader - Reset RTC Oscillator"]
pub struct OSC_DISABLE_O_R(crate::FieldReader<bool, bool>);
impl OSC_DISABLE_O_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_DISABLE_O_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_DISABLE_O_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bypass RTC oscillator"]
    #[inline(always)]
    pub fn osc_bypass(&self) -> OSC_BYPASS_R {
        OSC_BYPASS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - if osc_disable_sel = 1, this will hold the RTC in reset."]
    #[inline(always)]
    pub fn osc_disable_r(&self) -> OSC_DISABLE_R_R {
        OSC_DISABLE_R_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Select RTC Oscillator Disable Control Source"]
    #[inline(always)]
    pub fn osc_disable_sel(&self) -> OSC_DISABLE_SEL_R {
        OSC_DISABLE_SEL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Reset RTC Oscillator"]
    #[inline(always)]
    pub fn osc_disable_o(&self) -> OSC_DISABLE_O_R {
        OSC_DISABLE_O_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bypass RTC oscillator"]
    #[inline(always)]
    pub fn osc_bypass(&mut self) -> OSC_BYPASS_W {
        OSC_BYPASS_W { w: self }
    }
    #[doc = "Bit 1 - if osc_disable_sel = 1, this will hold the RTC in reset."]
    #[inline(always)]
    pub fn osc_disable_r(&mut self) -> OSC_DISABLE_R_W {
        OSC_DISABLE_R_W { w: self }
    }
    #[doc = "Bit 2 - Select RTC Oscillator Disable Control Source"]
    #[inline(always)]
    pub fn osc_disable_sel(&mut self) -> OSC_DISABLE_SEL_W {
        OSC_DISABLE_SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Oscillator Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_ctrl](index.html) module"]
pub struct OSC_CTRL_SPEC;
impl crate::RegisterSpec for OSC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_ctrl::R](R) reader structure"]
impl crate::Readable for OSC_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_ctrl::W](W) writer structure"]
impl crate::Writable for OSC_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSC_CTRL to value 0"]
impl crate::Resettable for OSC_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
