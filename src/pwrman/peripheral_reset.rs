#[doc = "Register `PERIPHERAL_RESET` reader"]
pub struct R(crate::R<PERIPHERAL_RESET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIPHERAL_RESET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIPHERAL_RESET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIPHERAL_RESET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIPHERAL_RESET` writer"]
pub struct W(crate::W<PERIPHERAL_RESET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIPHERAL_RESET_SPEC>;
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
impl From<crate::W<PERIPHERAL_RESET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIPHERAL_RESET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ssb` reader - Reset SSB"]
pub struct SSB_R(crate::FieldReader<bool, bool>);
impl SSB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SSB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SSB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ssb` writer - Reset SSB"]
pub struct SSB_W<'a> {
    w: &'a mut W,
}
impl<'a> SSB_W<'a> {
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
#[doc = "Field `spix` reader - Reset SPI XIP"]
pub struct SPIX_R(crate::FieldReader<bool, bool>);
impl SPIX_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spix` writer - Reset SPI XIP"]
pub struct SPIX_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIX_W<'a> {
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
#[doc = "Field `pmu` reader - Reset PMU"]
pub struct PMU_R(crate::FieldReader<bool, bool>);
impl PMU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PMU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PMU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pmu` writer - Reset PMU"]
pub struct PMU_W<'a> {
    w: &'a mut W,
}
impl<'a> PMU_W<'a> {
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
#[doc = "Field `usb` reader - Reset USB"]
pub struct USB_R(crate::FieldReader<bool, bool>);
impl USB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `usb` writer - Reset USB"]
pub struct USB_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_W<'a> {
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
#[doc = "Field `crc` reader - Reset CRC"]
pub struct CRC_R(crate::FieldReader<bool, bool>);
impl CRC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crc` writer - Reset CRC"]
pub struct CRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CRC_W<'a> {
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
#[doc = "Field `tpu` reader - Reset TPU"]
pub struct TPU_R(crate::FieldReader<bool, bool>);
impl TPU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TPU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TPU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tpu` writer - Reset TPU"]
pub struct TPU_W<'a> {
    w: &'a mut W,
}
impl<'a> TPU_W<'a> {
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
#[doc = "Field `watchdog0` reader - Reset Watchdog Timer 0"]
pub struct WATCHDOG0_R(crate::FieldReader<bool, bool>);
impl WATCHDOG0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WATCHDOG0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WATCHDOG0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `watchdog0` writer - Reset Watchdog Timer 0"]
pub struct WATCHDOG0_W<'a> {
    w: &'a mut W,
}
impl<'a> WATCHDOG0_W<'a> {
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
#[doc = "Field `gpio` reader - Reset GPIO"]
pub struct GPIO_R(crate::FieldReader<bool, bool>);
impl GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio` writer - Reset GPIO"]
pub struct GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_W<'a> {
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
#[doc = "Field `timer0` reader - Reset Timer/Counter Module 0"]
pub struct TIMER0_R(crate::FieldReader<bool, bool>);
impl TIMER0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer0` writer - Reset Timer/Counter Module 0"]
pub struct TIMER0_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_W<'a> {
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
#[doc = "Field `timer1` reader - Reset Timer/Counter Module 1"]
pub struct TIMER1_R(crate::FieldReader<bool, bool>);
impl TIMER1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer1` writer - Reset Timer/Counter Module 1"]
pub struct TIMER1_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `timer2` reader - Reset Timer/Counter Module 2"]
pub struct TIMER2_R(crate::FieldReader<bool, bool>);
impl TIMER2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer2` writer - Reset Timer/Counter Module 2"]
pub struct TIMER2_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `timer3` reader - Reset Timer/Counter Module 3"]
pub struct TIMER3_R(crate::FieldReader<bool, bool>);
impl TIMER3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer3` writer - Reset Timer/Counter Module 3"]
pub struct TIMER3_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `timer4` reader - Reset Timer/Counter Module 4"]
pub struct TIMER4_R(crate::FieldReader<bool, bool>);
impl TIMER4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer4` writer - Reset Timer/Counter Module 4"]
pub struct TIMER4_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_W<'a> {
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
#[doc = "Field `timer5` reader - Reset Timer/Counter Module 5"]
pub struct TIMER5_R(crate::FieldReader<bool, bool>);
impl TIMER5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TIMER5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer5` writer - Reset Timer/Counter Module 5"]
pub struct TIMER5_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5_W<'a> {
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
#[doc = "Field `pulse_train` reader - Reset All Pulse Trains"]
pub struct PULSE_TRAIN_R(crate::FieldReader<bool, bool>);
impl PULSE_TRAIN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PULSE_TRAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSE_TRAIN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pulse_train` writer - Reset All Pulse Trains"]
pub struct PULSE_TRAIN_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSE_TRAIN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `uart0` reader - Reset UART 0"]
pub struct UART0_R(crate::FieldReader<bool, bool>);
impl UART0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart0` writer - Reset UART 0"]
pub struct UART0_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `uart1` reader - Reset UART 1"]
pub struct UART1_R(crate::FieldReader<bool, bool>);
impl UART1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart1` writer - Reset UART 1"]
pub struct UART1_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_W<'a> {
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
#[doc = "Field `uart2` reader - Reset UART 2"]
pub struct UART2_R(crate::FieldReader<bool, bool>);
impl UART2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart2` writer - Reset UART 2"]
pub struct UART2_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `uart3` reader - Reset UART 3"]
pub struct UART3_R(crate::FieldReader<bool, bool>);
impl UART3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart3` writer - Reset UART 3"]
pub struct UART3_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `i2cm0` reader - Reset I2C Master 0"]
pub struct I2CM0_R(crate::FieldReader<bool, bool>);
impl I2CM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2cm0` writer - Reset I2C Master 0"]
pub struct I2CM0_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `i2cm1` reader - Reset I2C Master 1"]
pub struct I2CM1_R(crate::FieldReader<bool, bool>);
impl I2CM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2cm1` writer - Reset I2C Master 1"]
pub struct I2CM1_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CM1_W<'a> {
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
#[doc = "Field `i2cm2` reader - Reset I2C Master 2"]
pub struct I2CM2_R(crate::FieldReader<bool, bool>);
impl I2CM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2cm2` writer - Reset I2C Master 2"]
pub struct I2CM2_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `i2cs` reader - Reset I2C Slave"]
pub struct I2CS_R(crate::FieldReader<bool, bool>);
impl I2CS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        I2CS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2cs` writer - Reset I2C Slave"]
pub struct I2CS_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CS_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `spim0` reader - Reset SPI Master 0"]
pub struct SPIM0_R(crate::FieldReader<bool, bool>);
impl SPIM0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim0` writer - Reset SPI Master 0"]
pub struct SPIM0_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIM0_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `spim1` reader - Reset SPI Master 1"]
pub struct SPIM1_R(crate::FieldReader<bool, bool>);
impl SPIM1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim1` writer - Reset SPI Master 1"]
pub struct SPIM1_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIM1_W<'a> {
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
#[doc = "Field `spim2` reader - Reset SPI Master 2"]
pub struct SPIM2_R(crate::FieldReader<bool, bool>);
impl SPIM2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIM2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIM2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spim2` writer - Reset SPI Master 2"]
pub struct SPIM2_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIM2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `spib` reader - Reset SPI Bridge"]
pub struct SPIB_R(crate::FieldReader<bool, bool>);
impl SPIB_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPIB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIB_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spib` writer - Reset SPI Bridge"]
pub struct SPIB_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIB_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `owm` reader - Reset 1-Wire Master"]
pub struct OWM_R(crate::FieldReader<bool, bool>);
impl OWM_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OWM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OWM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `owm` writer - Reset 1-Wire Master"]
pub struct OWM_W<'a> {
    w: &'a mut W,
}
impl<'a> OWM_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `adc` reader - Reset ADC"]
pub struct ADC_R(crate::FieldReader<bool, bool>);
impl ADC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc` writer - Reset ADC"]
pub struct ADC_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Reset SSB"]
    #[inline(always)]
    pub fn ssb(&self) -> SSB_R {
        SSB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset SPI XIP"]
    #[inline(always)]
    pub fn spix(&self) -> SPIX_R {
        SPIX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset PMU"]
    #[inline(always)]
    pub fn pmu(&self) -> PMU_R {
        PMU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset USB"]
    #[inline(always)]
    pub fn usb(&self) -> USB_R {
        USB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset CRC"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset TPU"]
    #[inline(always)]
    pub fn tpu(&self) -> TPU_R {
        TPU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset Watchdog Timer 0"]
    #[inline(always)]
    pub fn watchdog0(&self) -> WATCHDOG0_R {
        WATCHDOG0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset GPIO"]
    #[inline(always)]
    pub fn gpio(&self) -> GPIO_R {
        GPIO_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0(&self) -> TIMER0_R {
        TIMER0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1(&self) -> TIMER1_R {
        TIMER1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2(&self) -> TIMER2_R {
        TIMER2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3(&self) -> TIMER3_R {
        TIMER3_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4(&self) -> TIMER4_R {
        TIMER4_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5(&self) -> TIMER5_R {
        TIMER5_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset All Pulse Trains"]
    #[inline(always)]
    pub fn pulse_train(&self) -> PULSE_TRAIN_R {
        PULSE_TRAIN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset UART 0"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset UART 1"]
    #[inline(always)]
    pub fn uart1(&self) -> UART1_R {
        UART1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset UART 2"]
    #[inline(always)]
    pub fn uart2(&self) -> UART2_R {
        UART2_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset UART 3"]
    #[inline(always)]
    pub fn uart3(&self) -> UART3_R {
        UART3_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0(&self) -> I2CM0_R {
        I2CM0_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1(&self) -> I2CM1_R {
        I2CM1_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2(&self) -> I2CM2_R {
        I2CM2_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset I2C Slave"]
    #[inline(always)]
    pub fn i2cs(&self) -> I2CS_R {
        I2CS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset SPI Master 0"]
    #[inline(always)]
    pub fn spim0(&self) -> SPIM0_R {
        SPIM0_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset SPI Master 1"]
    #[inline(always)]
    pub fn spim1(&self) -> SPIM1_R {
        SPIM1_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset SPI Master 2"]
    #[inline(always)]
    pub fn spim2(&self) -> SPIM2_R {
        SPIM2_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset SPI Bridge"]
    #[inline(always)]
    pub fn spib(&self) -> SPIB_R {
        SPIB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset 1-Wire Master"]
    #[inline(always)]
    pub fn owm(&self) -> OWM_R {
        OWM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset ADC"]
    #[inline(always)]
    pub fn adc(&self) -> ADC_R {
        ADC_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset SSB"]
    #[inline(always)]
    pub fn ssb(&mut self) -> SSB_W {
        SSB_W { w: self }
    }
    #[doc = "Bit 1 - Reset SPI XIP"]
    #[inline(always)]
    pub fn spix(&mut self) -> SPIX_W {
        SPIX_W { w: self }
    }
    #[doc = "Bit 2 - Reset PMU"]
    #[inline(always)]
    pub fn pmu(&mut self) -> PMU_W {
        PMU_W { w: self }
    }
    #[doc = "Bit 3 - Reset USB"]
    #[inline(always)]
    pub fn usb(&mut self) -> USB_W {
        USB_W { w: self }
    }
    #[doc = "Bit 4 - Reset CRC"]
    #[inline(always)]
    pub fn crc(&mut self) -> CRC_W {
        CRC_W { w: self }
    }
    #[doc = "Bit 5 - Reset TPU"]
    #[inline(always)]
    pub fn tpu(&mut self) -> TPU_W {
        TPU_W { w: self }
    }
    #[doc = "Bit 6 - Reset Watchdog Timer 0"]
    #[inline(always)]
    pub fn watchdog0(&mut self) -> WATCHDOG0_W {
        WATCHDOG0_W { w: self }
    }
    #[doc = "Bit 7 - Reset GPIO"]
    #[inline(always)]
    pub fn gpio(&mut self) -> GPIO_W {
        GPIO_W { w: self }
    }
    #[doc = "Bit 8 - Reset Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0(&mut self) -> TIMER0_W {
        TIMER0_W { w: self }
    }
    #[doc = "Bit 9 - Reset Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1(&mut self) -> TIMER1_W {
        TIMER1_W { w: self }
    }
    #[doc = "Bit 10 - Reset Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2(&mut self) -> TIMER2_W {
        TIMER2_W { w: self }
    }
    #[doc = "Bit 11 - Reset Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3(&mut self) -> TIMER3_W {
        TIMER3_W { w: self }
    }
    #[doc = "Bit 12 - Reset Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4(&mut self) -> TIMER4_W {
        TIMER4_W { w: self }
    }
    #[doc = "Bit 13 - Reset Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5(&mut self) -> TIMER5_W {
        TIMER5_W { w: self }
    }
    #[doc = "Bit 14 - Reset All Pulse Trains"]
    #[inline(always)]
    pub fn pulse_train(&mut self) -> PULSE_TRAIN_W {
        PULSE_TRAIN_W { w: self }
    }
    #[doc = "Bit 15 - Reset UART 0"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W { w: self }
    }
    #[doc = "Bit 16 - Reset UART 1"]
    #[inline(always)]
    pub fn uart1(&mut self) -> UART1_W {
        UART1_W { w: self }
    }
    #[doc = "Bit 17 - Reset UART 2"]
    #[inline(always)]
    pub fn uart2(&mut self) -> UART2_W {
        UART2_W { w: self }
    }
    #[doc = "Bit 18 - Reset UART 3"]
    #[inline(always)]
    pub fn uart3(&mut self) -> UART3_W {
        UART3_W { w: self }
    }
    #[doc = "Bit 19 - Reset I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0(&mut self) -> I2CM0_W {
        I2CM0_W { w: self }
    }
    #[doc = "Bit 20 - Reset I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1(&mut self) -> I2CM1_W {
        I2CM1_W { w: self }
    }
    #[doc = "Bit 21 - Reset I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2(&mut self) -> I2CM2_W {
        I2CM2_W { w: self }
    }
    #[doc = "Bit 22 - Reset I2C Slave"]
    #[inline(always)]
    pub fn i2cs(&mut self) -> I2CS_W {
        I2CS_W { w: self }
    }
    #[doc = "Bit 23 - Reset SPI Master 0"]
    #[inline(always)]
    pub fn spim0(&mut self) -> SPIM0_W {
        SPIM0_W { w: self }
    }
    #[doc = "Bit 24 - Reset SPI Master 1"]
    #[inline(always)]
    pub fn spim1(&mut self) -> SPIM1_W {
        SPIM1_W { w: self }
    }
    #[doc = "Bit 25 - Reset SPI Master 2"]
    #[inline(always)]
    pub fn spim2(&mut self) -> SPIM2_W {
        SPIM2_W { w: self }
    }
    #[doc = "Bit 26 - Reset SPI Bridge"]
    #[inline(always)]
    pub fn spib(&mut self) -> SPIB_W {
        SPIB_W { w: self }
    }
    #[doc = "Bit 27 - Reset 1-Wire Master"]
    #[inline(always)]
    pub fn owm(&mut self) -> OWM_W {
        OWM_W { w: self }
    }
    #[doc = "Bit 28 - Reset ADC"]
    #[inline(always)]
    pub fn adc(&mut self) -> ADC_W {
        ADC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Peripheral Reset Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peripheral_reset](index.html) module"]
pub struct PERIPHERAL_RESET_SPEC;
impl crate::RegisterSpec for PERIPHERAL_RESET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peripheral_reset::R](R) reader structure"]
impl crate::Readable for PERIPHERAL_RESET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peripheral_reset::W](W) writer structure"]
impl crate::Writable for PERIPHERAL_RESET_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIPHERAL_RESET to value 0"]
impl crate::Resettable for PERIPHERAL_RESET_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
