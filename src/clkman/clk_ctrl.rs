#[doc = "Register `CLK_CTRL` reader"]
pub struct R(crate::R<CLK_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CTRL` writer"]
pub struct W(crate::W<CLK_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CTRL_SPEC>;
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
impl From<crate::W<CLK_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `system_source_select` reader - System Clock Source Select"]
pub struct SYSTEM_SOURCE_SELECT_R(crate::FieldReader<u8, u8>);
impl SYSTEM_SOURCE_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SYSTEM_SOURCE_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SYSTEM_SOURCE_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `system_source_select` writer - System Clock Source Select"]
pub struct SYSTEM_SOURCE_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> SYSTEM_SOURCE_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `usb_clock_enable` reader - USB Clock Enable"]
pub struct USB_CLOCK_ENABLE_R(crate::FieldReader<bool, bool>);
impl USB_CLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_CLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_CLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_clock_enable` writer - USB Clock Enable"]
pub struct USB_CLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_CLOCK_ENABLE_W<'a> {
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
#[doc = "Field `usb_clock_select` reader - USB Clock Select"]
pub struct USB_CLOCK_SELECT_R(crate::FieldReader<bool, bool>);
impl USB_CLOCK_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_CLOCK_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_CLOCK_SELECT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb_clock_select` writer - USB Clock Select"]
pub struct USB_CLOCK_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_CLOCK_SELECT_W<'a> {
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
#[doc = "Field `crypto_clock_enable` reader - Crypto Clock Enable"]
pub struct CRYPTO_CLOCK_ENABLE_R(crate::FieldReader<bool, bool>);
impl CRYPTO_CLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_CLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_CLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crypto_clock_enable` writer - Crypto Clock Enable"]
pub struct CRYPTO_CLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_CLOCK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
#[doc = "Field `rtos_mode` reader - Enable RTOS Mode for SysTick Timers"]
pub struct RTOS_MODE_R(crate::FieldReader<bool, bool>);
impl RTOS_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTOS_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTOS_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtos_mode` writer - Enable RTOS Mode for SysTick Timers"]
pub struct RTOS_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTOS_MODE_W<'a> {
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
#[doc = "Field `cpu_dynamic_clock` reader - Enable CPU Dynamic Clock Gating"]
pub struct CPU_DYNAMIC_CLOCK_R(crate::FieldReader<bool, bool>);
impl CPU_DYNAMIC_CLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_DYNAMIC_CLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_DYNAMIC_CLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cpu_dynamic_clock` writer - Enable CPU Dynamic Clock Gating"]
pub struct CPU_DYNAMIC_CLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_DYNAMIC_CLOCK_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `wdt0_clock_enable` reader - Watchdog 0 Clock Enable"]
pub struct WDT0_CLOCK_ENABLE_R(crate::FieldReader<bool, bool>);
impl WDT0_CLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT0_CLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT0_CLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wdt0_clock_enable` writer - Watchdog 0 Clock Enable"]
pub struct WDT0_CLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT0_CLOCK_ENABLE_W<'a> {
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
#[doc = "Field `wdt0_clock_select` reader - Watchdog 0 Clock Source Select"]
pub struct WDT0_CLOCK_SELECT_R(crate::FieldReader<u8, u8>);
impl WDT0_CLOCK_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT0_CLOCK_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT0_CLOCK_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wdt0_clock_select` writer - Watchdog 0 Clock Source Select"]
pub struct WDT0_CLOCK_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT0_CLOCK_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 17)) | ((value as u32 & 3) << 17);
        self.w
    }
}
#[doc = "Field `wdt1_clock_enable` reader - Watchdog 1 Clock Enable"]
pub struct WDT1_CLOCK_ENABLE_R(crate::FieldReader<bool, bool>);
impl WDT1_CLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WDT1_CLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT1_CLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wdt1_clock_enable` writer - Watchdog 1 Clock Enable"]
pub struct WDT1_CLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_CLOCK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `wdt1_clock_select` reader - Watchdog 1 Clock Source Select"]
pub struct WDT1_CLOCK_SELECT_R(crate::FieldReader<u8, u8>);
impl WDT1_CLOCK_SELECT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WDT1_CLOCK_SELECT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT1_CLOCK_SELECT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wdt1_clock_select` writer - Watchdog 1 Clock Source Select"]
pub struct WDT1_CLOCK_SELECT_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT1_CLOCK_SELECT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 21)) | ((value as u32 & 3) << 21);
        self.w
    }
}
#[doc = "Field `adc_clock_enable` reader - ADC Clock Enable"]
pub struct ADC_CLOCK_ENABLE_R(crate::FieldReader<bool, bool>);
impl ADC_CLOCK_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CLOCK_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLOCK_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_clock_enable` writer - ADC Clock Enable"]
pub struct ADC_CLOCK_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLOCK_ENABLE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System Clock Source Select"]
    #[inline(always)]
    pub fn system_source_select(&self) -> SYSTEM_SOURCE_SELECT_R {
        SYSTEM_SOURCE_SELECT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - USB Clock Enable"]
    #[inline(always)]
    pub fn usb_clock_enable(&self) -> USB_CLOCK_ENABLE_R {
        USB_CLOCK_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB Clock Select"]
    #[inline(always)]
    pub fn usb_clock_select(&self) -> USB_CLOCK_SELECT_R {
        USB_CLOCK_SELECT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Crypto Clock Enable"]
    #[inline(always)]
    pub fn crypto_clock_enable(&self) -> CRYPTO_CLOCK_ENABLE_R {
        CRYPTO_CLOCK_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable RTOS Mode for SysTick Timers"]
    #[inline(always)]
    pub fn rtos_mode(&self) -> RTOS_MODE_R {
        RTOS_MODE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable CPU Dynamic Clock Gating"]
    #[inline(always)]
    pub fn cpu_dynamic_clock(&self) -> CPU_DYNAMIC_CLOCK_R {
        CPU_DYNAMIC_CLOCK_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Watchdog 0 Clock Enable"]
    #[inline(always)]
    pub fn wdt0_clock_enable(&self) -> WDT0_CLOCK_ENABLE_R {
        WDT0_CLOCK_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Watchdog 0 Clock Source Select"]
    #[inline(always)]
    pub fn wdt0_clock_select(&self) -> WDT0_CLOCK_SELECT_R {
        WDT0_CLOCK_SELECT_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 20 - Watchdog 1 Clock Enable"]
    #[inline(always)]
    pub fn wdt1_clock_enable(&self) -> WDT1_CLOCK_ENABLE_R {
        WDT1_CLOCK_ENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Watchdog 1 Clock Source Select"]
    #[inline(always)]
    pub fn wdt1_clock_select(&self) -> WDT1_CLOCK_SELECT_R {
        WDT1_CLOCK_SELECT_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - ADC Clock Enable"]
    #[inline(always)]
    pub fn adc_clock_enable(&self) -> ADC_CLOCK_ENABLE_R {
        ADC_CLOCK_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System Clock Source Select"]
    #[inline(always)]
    pub fn system_source_select(&mut self) -> SYSTEM_SOURCE_SELECT_W {
        SYSTEM_SOURCE_SELECT_W { w: self }
    }
    #[doc = "Bit 4 - USB Clock Enable"]
    #[inline(always)]
    pub fn usb_clock_enable(&mut self) -> USB_CLOCK_ENABLE_W {
        USB_CLOCK_ENABLE_W { w: self }
    }
    #[doc = "Bit 5 - USB Clock Select"]
    #[inline(always)]
    pub fn usb_clock_select(&mut self) -> USB_CLOCK_SELECT_W {
        USB_CLOCK_SELECT_W { w: self }
    }
    #[doc = "Bit 8 - Crypto Clock Enable"]
    #[inline(always)]
    pub fn crypto_clock_enable(&mut self) -> CRYPTO_CLOCK_ENABLE_W {
        CRYPTO_CLOCK_ENABLE_W { w: self }
    }
    #[doc = "Bit 12 - Enable RTOS Mode for SysTick Timers"]
    #[inline(always)]
    pub fn rtos_mode(&mut self) -> RTOS_MODE_W {
        RTOS_MODE_W { w: self }
    }
    #[doc = "Bit 13 - Enable CPU Dynamic Clock Gating"]
    #[inline(always)]
    pub fn cpu_dynamic_clock(&mut self) -> CPU_DYNAMIC_CLOCK_W {
        CPU_DYNAMIC_CLOCK_W { w: self }
    }
    #[doc = "Bit 16 - Watchdog 0 Clock Enable"]
    #[inline(always)]
    pub fn wdt0_clock_enable(&mut self) -> WDT0_CLOCK_ENABLE_W {
        WDT0_CLOCK_ENABLE_W { w: self }
    }
    #[doc = "Bits 17:18 - Watchdog 0 Clock Source Select"]
    #[inline(always)]
    pub fn wdt0_clock_select(&mut self) -> WDT0_CLOCK_SELECT_W {
        WDT0_CLOCK_SELECT_W { w: self }
    }
    #[doc = "Bit 20 - Watchdog 1 Clock Enable"]
    #[inline(always)]
    pub fn wdt1_clock_enable(&mut self) -> WDT1_CLOCK_ENABLE_W {
        WDT1_CLOCK_ENABLE_W { w: self }
    }
    #[doc = "Bits 21:22 - Watchdog 1 Clock Source Select"]
    #[inline(always)]
    pub fn wdt1_clock_select(&mut self) -> WDT1_CLOCK_SELECT_W {
        WDT1_CLOCK_SELECT_W { w: self }
    }
    #[doc = "Bit 24 - ADC Clock Enable"]
    #[inline(always)]
    pub fn adc_clock_enable(&mut self) -> ADC_CLOCK_ENABLE_W {
        ADC_CLOCK_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_ctrl](index.html) module"]
pub struct CLK_CTRL_SPEC;
impl crate::RegisterSpec for CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_ctrl::R](R) reader structure"]
impl crate::Readable for CLK_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_ctrl::W](W) writer structure"]
impl crate::Writable for CLK_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CTRL to value 0"]
impl crate::Resettable for CLK_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
