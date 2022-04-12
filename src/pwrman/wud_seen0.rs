#[doc = "Register `WUD_SEEN0` reader"]
pub struct R(crate::R<WUD_SEEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_SEEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_SEEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_SEEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_SEEN0` writer"]
pub struct W(crate::W<WUD_SEEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_SEEN0_SPEC>;
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
impl From<crate::W<WUD_SEEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_SEEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpio0` reader - Wake-Up Detect Status for P0.0"]
pub struct GPIO0_R(crate::FieldReader<bool, bool>);
impl GPIO0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio1` reader - Wake-Up Detect Status for P0.1"]
pub struct GPIO1_R(crate::FieldReader<bool, bool>);
impl GPIO1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio2` reader - Wake-Up Detect Status for P0.2"]
pub struct GPIO2_R(crate::FieldReader<bool, bool>);
impl GPIO2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio3` reader - Wake-Up Detect Status for P0.3"]
pub struct GPIO3_R(crate::FieldReader<bool, bool>);
impl GPIO3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio4` reader - Wake-Up Detect Status for P0.4"]
pub struct GPIO4_R(crate::FieldReader<bool, bool>);
impl GPIO4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio5` reader - Wake-Up Detect Status for P0.5"]
pub struct GPIO5_R(crate::FieldReader<bool, bool>);
impl GPIO5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio6` reader - Wake-Up Detect Status for P0.6"]
pub struct GPIO6_R(crate::FieldReader<bool, bool>);
impl GPIO6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio7` reader - Wake-Up Detect Status for P0.7"]
pub struct GPIO7_R(crate::FieldReader<bool, bool>);
impl GPIO7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio8` reader - Wake-Up Detect Status for P1.0"]
pub struct GPIO8_R(crate::FieldReader<bool, bool>);
impl GPIO8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio9` reader - Wake-Up Detect Status for P1.1"]
pub struct GPIO9_R(crate::FieldReader<bool, bool>);
impl GPIO9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio10` reader - Wake-Up Detect Status for P1.2"]
pub struct GPIO10_R(crate::FieldReader<bool, bool>);
impl GPIO10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio11` reader - Wake-Up Detect Status for P1.3"]
pub struct GPIO11_R(crate::FieldReader<bool, bool>);
impl GPIO11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio12` reader - Wake-Up Detect Status for P1.4"]
pub struct GPIO12_R(crate::FieldReader<bool, bool>);
impl GPIO12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio13` reader - Wake-Up Detect Status for P1.5"]
pub struct GPIO13_R(crate::FieldReader<bool, bool>);
impl GPIO13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio14` reader - Wake-Up Detect Status for P1.6"]
pub struct GPIO14_R(crate::FieldReader<bool, bool>);
impl GPIO14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio15` reader - Wake-Up Detect Status for P1.7"]
pub struct GPIO15_R(crate::FieldReader<bool, bool>);
impl GPIO15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio16` reader - Wake-Up Detect Status for P2.0"]
pub struct GPIO16_R(crate::FieldReader<bool, bool>);
impl GPIO16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio17` reader - Wake-Up Detect Status for P2.1"]
pub struct GPIO17_R(crate::FieldReader<bool, bool>);
impl GPIO17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio18` reader - Wake-Up Detect Status for P2.2"]
pub struct GPIO18_R(crate::FieldReader<bool, bool>);
impl GPIO18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio19` reader - Wake-Up Detect Status for P2.3"]
pub struct GPIO19_R(crate::FieldReader<bool, bool>);
impl GPIO19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio20` reader - Wake-Up Detect Status for P2.4"]
pub struct GPIO20_R(crate::FieldReader<bool, bool>);
impl GPIO20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio21` reader - Wake-Up Detect Status for P2.5"]
pub struct GPIO21_R(crate::FieldReader<bool, bool>);
impl GPIO21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio22` reader - Wake-Up Detect Status for P2.6"]
pub struct GPIO22_R(crate::FieldReader<bool, bool>);
impl GPIO22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio23` reader - Wake-Up Detect Status for P2.7"]
pub struct GPIO23_R(crate::FieldReader<bool, bool>);
impl GPIO23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio24` reader - Wake-Up Detect Status for P3.0"]
pub struct GPIO24_R(crate::FieldReader<bool, bool>);
impl GPIO24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio25` reader - Wake-Up Detect Status for P3.1"]
pub struct GPIO25_R(crate::FieldReader<bool, bool>);
impl GPIO25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio26` reader - Wake-Up Detect Status for P3.2"]
pub struct GPIO26_R(crate::FieldReader<bool, bool>);
impl GPIO26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio27` reader - Wake-Up Detect Status for P3.3"]
pub struct GPIO27_R(crate::FieldReader<bool, bool>);
impl GPIO27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio28` reader - Wake-Up Detect Status for P3.4"]
pub struct GPIO28_R(crate::FieldReader<bool, bool>);
impl GPIO28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio29` reader - Wake-Up Detect Status for P3.5"]
pub struct GPIO29_R(crate::FieldReader<bool, bool>);
impl GPIO29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio30` reader - Wake-Up Detect Status for P3.6"]
pub struct GPIO30_R(crate::FieldReader<bool, bool>);
impl GPIO30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio31` reader - Wake-Up Detect Status for P3.7"]
pub struct GPIO31_R(crate::FieldReader<bool, bool>);
impl GPIO31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up Detect Status for P0.0"]
    #[inline(always)]
    pub fn gpio0(&self) -> GPIO0_R {
        GPIO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake-Up Detect Status for P0.1"]
    #[inline(always)]
    pub fn gpio1(&self) -> GPIO1_R {
        GPIO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Detect Status for P0.2"]
    #[inline(always)]
    pub fn gpio2(&self) -> GPIO2_R {
        GPIO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wake-Up Detect Status for P0.3"]
    #[inline(always)]
    pub fn gpio3(&self) -> GPIO3_R {
        GPIO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Detect Status for P0.4"]
    #[inline(always)]
    pub fn gpio4(&self) -> GPIO4_R {
        GPIO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wake-Up Detect Status for P0.5"]
    #[inline(always)]
    pub fn gpio5(&self) -> GPIO5_R {
        GPIO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Detect Status for P0.6"]
    #[inline(always)]
    pub fn gpio6(&self) -> GPIO6_R {
        GPIO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Wake-Up Detect Status for P0.7"]
    #[inline(always)]
    pub fn gpio7(&self) -> GPIO7_R {
        GPIO7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Wake-Up Detect Status for P1.0"]
    #[inline(always)]
    pub fn gpio8(&self) -> GPIO8_R {
        GPIO8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Wake-Up Detect Status for P1.1"]
    #[inline(always)]
    pub fn gpio9(&self) -> GPIO9_R {
        GPIO9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wake-Up Detect Status for P1.2"]
    #[inline(always)]
    pub fn gpio10(&self) -> GPIO10_R {
        GPIO10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wake-Up Detect Status for P1.3"]
    #[inline(always)]
    pub fn gpio11(&self) -> GPIO11_R {
        GPIO11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Wake-Up Detect Status for P1.4"]
    #[inline(always)]
    pub fn gpio12(&self) -> GPIO12_R {
        GPIO12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Wake-Up Detect Status for P1.5"]
    #[inline(always)]
    pub fn gpio13(&self) -> GPIO13_R {
        GPIO13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wake-Up Detect Status for P1.6"]
    #[inline(always)]
    pub fn gpio14(&self) -> GPIO14_R {
        GPIO14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Wake-Up Detect Status for P1.7"]
    #[inline(always)]
    pub fn gpio15(&self) -> GPIO15_R {
        GPIO15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Wake-Up Detect Status for P2.0"]
    #[inline(always)]
    pub fn gpio16(&self) -> GPIO16_R {
        GPIO16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Wake-Up Detect Status for P2.1"]
    #[inline(always)]
    pub fn gpio17(&self) -> GPIO17_R {
        GPIO17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wake-Up Detect Status for P2.2"]
    #[inline(always)]
    pub fn gpio18(&self) -> GPIO18_R {
        GPIO18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Wake-Up Detect Status for P2.3"]
    #[inline(always)]
    pub fn gpio19(&self) -> GPIO19_R {
        GPIO19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Wake-Up Detect Status for P2.4"]
    #[inline(always)]
    pub fn gpio20(&self) -> GPIO20_R {
        GPIO20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Wake-Up Detect Status for P2.5"]
    #[inline(always)]
    pub fn gpio21(&self) -> GPIO21_R {
        GPIO21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Wake-Up Detect Status for P2.6"]
    #[inline(always)]
    pub fn gpio22(&self) -> GPIO22_R {
        GPIO22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Wake-Up Detect Status for P2.7"]
    #[inline(always)]
    pub fn gpio23(&self) -> GPIO23_R {
        GPIO23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Wake-Up Detect Status for P3.0"]
    #[inline(always)]
    pub fn gpio24(&self) -> GPIO24_R {
        GPIO24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Wake-Up Detect Status for P3.1"]
    #[inline(always)]
    pub fn gpio25(&self) -> GPIO25_R {
        GPIO25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Wake-Up Detect Status for P3.2"]
    #[inline(always)]
    pub fn gpio26(&self) -> GPIO26_R {
        GPIO26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Wake-Up Detect Status for P3.3"]
    #[inline(always)]
    pub fn gpio27(&self) -> GPIO27_R {
        GPIO27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wake-Up Detect Status for P3.4"]
    #[inline(always)]
    pub fn gpio28(&self) -> GPIO28_R {
        GPIO28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Wake-Up Detect Status for P3.5"]
    #[inline(always)]
    pub fn gpio29(&self) -> GPIO29_R {
        GPIO29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wake-Up Detect Status for P3.6"]
    #[inline(always)]
    pub fn gpio30(&self) -> GPIO30_R {
        GPIO30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Wake-Up Detect Status for P3.7"]
    #[inline(always)]
    pub fn gpio31(&self) -> GPIO31_R {
        GPIO31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Wake-up Detect Status for P0/P1/P2/P3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_seen0](index.html) module"]
pub struct WUD_SEEN0_SPEC;
impl crate::RegisterSpec for WUD_SEEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_seen0::R](R) reader structure"]
impl crate::Readable for WUD_SEEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_seen0::W](W) writer structure"]
impl crate::Writable for WUD_SEEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_SEEN0 to value 0"]
impl crate::Resettable for WUD_SEEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
