#[doc = "Register `CLK_GATE_CTRL1` reader"]
pub struct R(crate::R<CLK_GATE_CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_GATE_CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_GATE_CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_GATE_CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_GATE_CTRL1` writer"]
pub struct W(crate::W<CLK_GATE_CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_GATE_CTRL1_SPEC>;
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
impl From<crate::W<CLK_GATE_CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_GATE_CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `watchdog1_clk_gater` reader - Clock Gating Control for Watchdog Timer 1"]
pub struct WATCHDOG1_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl WATCHDOG1_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WATCHDOG1_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WATCHDOG1_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `watchdog1_clk_gater` writer - Clock Gating Control for Watchdog Timer 1"]
pub struct WATCHDOG1_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> WATCHDOG1_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "Field `gpio_clk_gater` reader - Clock Gating Control for GPIO Ports"]
pub struct GPIO_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl GPIO_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        GPIO_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio_clk_gater` writer - Clock Gating Control for GPIO Ports"]
pub struct GPIO_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 2)) | ((value as u32 & 3) << 2);
        self.w
    }
}
#[doc = "Field `timer0_clk_gater` reader - Clock Gating Control for Timer/Counter Module 0"]
pub struct TIMER0_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl TIMER0_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER0_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER0_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer0_clk_gater` writer - Clock Gating Control for Timer/Counter Module 0"]
pub struct TIMER0_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER0_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `timer1_clk_gater` reader - Clock Gating Control for Timer/Counter Module 1"]
pub struct TIMER1_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl TIMER1_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER1_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER1_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer1_clk_gater` writer - Clock Gating Control for Timer/Counter Module 1"]
pub struct TIMER1_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER1_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `timer2_clk_gater` reader - Clock Gating Control for Timer/Counter Module 2"]
pub struct TIMER2_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl TIMER2_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER2_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER2_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer2_clk_gater` writer - Clock Gating Control for Timer/Counter Module 2"]
pub struct TIMER2_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER2_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 8)) | ((value as u32 & 3) << 8);
        self.w
    }
}
#[doc = "Field `timer3_clk_gater` reader - Clock Gating Control for Timer/Counter Module 3"]
pub struct TIMER3_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl TIMER3_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER3_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER3_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer3_clk_gater` writer - Clock Gating Control for Timer/Counter Module 3"]
pub struct TIMER3_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER3_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 10)) | ((value as u32 & 3) << 10);
        self.w
    }
}
#[doc = "Field `timer4_clk_gater` reader - Clock Gating Control for Timer/Counter Module 4"]
pub struct TIMER4_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl TIMER4_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER4_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER4_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer4_clk_gater` writer - Clock Gating Control for Timer/Counter Module 4"]
pub struct TIMER4_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER4_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 12)) | ((value as u32 & 3) << 12);
        self.w
    }
}
#[doc = "Field `timer5_clk_gater` reader - Clock Gating Control for Timer/Counter Module 5"]
pub struct TIMER5_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl TIMER5_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TIMER5_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TIMER5_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `timer5_clk_gater` writer - Clock Gating Control for Timer/Counter Module 5"]
pub struct TIMER5_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMER5_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `pulsetrain_clk_gater` reader - Clock Gating Control for Pulse Train Generators"]
pub struct PULSETRAIN_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl PULSETRAIN_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PULSETRAIN_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PULSETRAIN_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pulsetrain_clk_gater` writer - Clock Gating Control for Pulse Train Generators"]
pub struct PULSETRAIN_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> PULSETRAIN_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `uart0_clk_gater` reader - Clock Gating Control for UART 0"]
pub struct UART0_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl UART0_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART0_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART0_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart0_clk_gater` writer - Clock Gating Control for UART 0"]
pub struct UART0_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `uart1_clk_gater` reader - Clock Gating Control for UART 1"]
pub struct UART1_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl UART1_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART1_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART1_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart1_clk_gater` writer - Clock Gating Control for UART 1"]
pub struct UART1_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> UART1_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 20)) | ((value as u32 & 3) << 20);
        self.w
    }
}
#[doc = "Field `uart2_clk_gater` reader - Clock Gating Control for UART 2"]
pub struct UART2_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl UART2_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART2_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART2_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart2_clk_gater` writer - Clock Gating Control for UART 2"]
pub struct UART2_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> UART2_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 22)) | ((value as u32 & 3) << 22);
        self.w
    }
}
#[doc = "Field `uart3_clk_gater` reader - Clock Gating Control for UART 3"]
pub struct UART3_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl UART3_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        UART3_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART3_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart3_clk_gater` writer - Clock Gating Control for UART 3"]
pub struct UART3_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> UART3_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 24)) | ((value as u32 & 3) << 24);
        self.w
    }
}
#[doc = "Field `i2cm0_clk_gater` reader - Clock Gating Control for I2C Master 0"]
pub struct I2CM0_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl I2CM0_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2CM0_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CM0_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2cm0_clk_gater` writer - Clock Gating Control for I2C Master 0"]
pub struct I2CM0_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CM0_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 26)) | ((value as u32 & 3) << 26);
        self.w
    }
}
#[doc = "Field `i2cm1_clk_gater` reader - Clock Gating Control for I2C Master 1"]
pub struct I2CM1_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl I2CM1_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2CM1_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CM1_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2cm1_clk_gater` writer - Clock Gating Control for I2C Master 1"]
pub struct I2CM1_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CM1_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 28)) | ((value as u32 & 3) << 28);
        self.w
    }
}
#[doc = "Field `i2cm2_clk_gater` reader - Clock Gating Control for I2C Master 2"]
pub struct I2CM2_CLK_GATER_R(crate::FieldReader<u8, u8>);
impl I2CM2_CLK_GATER_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        I2CM2_CLK_GATER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for I2CM2_CLK_GATER_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `i2cm2_clk_gater` writer - Clock Gating Control for I2C Master 2"]
pub struct I2CM2_CLK_GATER_W<'a> {
    w: &'a mut W,
}
impl<'a> I2CM2_CLK_GATER_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 30)) | ((value as u32 & 3) << 30);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Clock Gating Control for Watchdog Timer 1"]
    #[inline(always)]
    pub fn watchdog1_clk_gater(&self) -> WATCHDOG1_CLK_GATER_R {
        WATCHDOG1_CLK_GATER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Clock Gating Control for GPIO Ports"]
    #[inline(always)]
    pub fn gpio_clk_gater(&self) -> GPIO_CLK_GATER_R {
        GPIO_CLK_GATER_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0_clk_gater(&self) -> TIMER0_CLK_GATER_R {
        TIMER0_CLK_GATER_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1_clk_gater(&self) -> TIMER1_CLK_GATER_R {
        TIMER1_CLK_GATER_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Clock Gating Control for Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2_clk_gater(&self) -> TIMER2_CLK_GATER_R {
        TIMER2_CLK_GATER_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Clock Gating Control for Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3_clk_gater(&self) -> TIMER3_CLK_GATER_R {
        TIMER3_CLK_GATER_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Clock Gating Control for Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4_clk_gater(&self) -> TIMER4_CLK_GATER_R {
        TIMER4_CLK_GATER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Clock Gating Control for Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5_clk_gater(&self) -> TIMER5_CLK_GATER_R {
        TIMER5_CLK_GATER_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Clock Gating Control for Pulse Train Generators"]
    #[inline(always)]
    pub fn pulsetrain_clk_gater(&self) -> PULSETRAIN_CLK_GATER_R {
        PULSETRAIN_CLK_GATER_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - Clock Gating Control for UART 0"]
    #[inline(always)]
    pub fn uart0_clk_gater(&self) -> UART0_CLK_GATER_R {
        UART0_CLK_GATER_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Clock Gating Control for UART 1"]
    #[inline(always)]
    pub fn uart1_clk_gater(&self) -> UART1_CLK_GATER_R {
        UART1_CLK_GATER_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - Clock Gating Control for UART 2"]
    #[inline(always)]
    pub fn uart2_clk_gater(&self) -> UART2_CLK_GATER_R {
        UART2_CLK_GATER_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - Clock Gating Control for UART 3"]
    #[inline(always)]
    pub fn uart3_clk_gater(&self) -> UART3_CLK_GATER_R {
        UART3_CLK_GATER_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Clock Gating Control for I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0_clk_gater(&self) -> I2CM0_CLK_GATER_R {
        I2CM0_CLK_GATER_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Clock Gating Control for I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1_clk_gater(&self) -> I2CM1_CLK_GATER_R {
        I2CM1_CLK_GATER_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - Clock Gating Control for I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2_clk_gater(&self) -> I2CM2_CLK_GATER_R {
        I2CM2_CLK_GATER_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock Gating Control for Watchdog Timer 1"]
    #[inline(always)]
    pub fn watchdog1_clk_gater(&mut self) -> WATCHDOG1_CLK_GATER_W {
        WATCHDOG1_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 2:3 - Clock Gating Control for GPIO Ports"]
    #[inline(always)]
    pub fn gpio_clk_gater(&mut self) -> GPIO_CLK_GATER_W {
        GPIO_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 4:5 - Clock Gating Control for Timer/Counter Module 0"]
    #[inline(always)]
    pub fn timer0_clk_gater(&mut self) -> TIMER0_CLK_GATER_W {
        TIMER0_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 6:7 - Clock Gating Control for Timer/Counter Module 1"]
    #[inline(always)]
    pub fn timer1_clk_gater(&mut self) -> TIMER1_CLK_GATER_W {
        TIMER1_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 8:9 - Clock Gating Control for Timer/Counter Module 2"]
    #[inline(always)]
    pub fn timer2_clk_gater(&mut self) -> TIMER2_CLK_GATER_W {
        TIMER2_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 10:11 - Clock Gating Control for Timer/Counter Module 3"]
    #[inline(always)]
    pub fn timer3_clk_gater(&mut self) -> TIMER3_CLK_GATER_W {
        TIMER3_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 12:13 - Clock Gating Control for Timer/Counter Module 4"]
    #[inline(always)]
    pub fn timer4_clk_gater(&mut self) -> TIMER4_CLK_GATER_W {
        TIMER4_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 14:15 - Clock Gating Control for Timer/Counter Module 5"]
    #[inline(always)]
    pub fn timer5_clk_gater(&mut self) -> TIMER5_CLK_GATER_W {
        TIMER5_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 16:17 - Clock Gating Control for Pulse Train Generators"]
    #[inline(always)]
    pub fn pulsetrain_clk_gater(&mut self) -> PULSETRAIN_CLK_GATER_W {
        PULSETRAIN_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 18:19 - Clock Gating Control for UART 0"]
    #[inline(always)]
    pub fn uart0_clk_gater(&mut self) -> UART0_CLK_GATER_W {
        UART0_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 20:21 - Clock Gating Control for UART 1"]
    #[inline(always)]
    pub fn uart1_clk_gater(&mut self) -> UART1_CLK_GATER_W {
        UART1_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 22:23 - Clock Gating Control for UART 2"]
    #[inline(always)]
    pub fn uart2_clk_gater(&mut self) -> UART2_CLK_GATER_W {
        UART2_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 24:25 - Clock Gating Control for UART 3"]
    #[inline(always)]
    pub fn uart3_clk_gater(&mut self) -> UART3_CLK_GATER_W {
        UART3_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 26:27 - Clock Gating Control for I2C Master 0"]
    #[inline(always)]
    pub fn i2cm0_clk_gater(&mut self) -> I2CM0_CLK_GATER_W {
        I2CM0_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 28:29 - Clock Gating Control for I2C Master 1"]
    #[inline(always)]
    pub fn i2cm1_clk_gater(&mut self) -> I2CM1_CLK_GATER_W {
        I2CM1_CLK_GATER_W { w: self }
    }
    #[doc = "Bits 30:31 - Clock Gating Control for I2C Master 2"]
    #[inline(always)]
    pub fn i2cm2_clk_gater(&mut self) -> I2CM2_CLK_GATER_W {
        I2CM2_CLK_GATER_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Dynamic Clock Gating Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_gate_ctrl1](index.html) module"]
pub struct CLK_GATE_CTRL1_SPEC;
impl crate::RegisterSpec for CLK_GATE_CTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_gate_ctrl1::R](R) reader structure"]
impl crate::Readable for CLK_GATE_CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_gate_ctrl1::W](W) writer structure"]
impl crate::Writable for CLK_GATE_CTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_GATE_CTRL1 to value 0"]
impl crate::Resettable for CLK_GATE_CTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
