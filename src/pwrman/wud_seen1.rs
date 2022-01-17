#[doc = "Register `WUD_SEEN1` reader"]
pub struct R(crate::R<WUD_SEEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_SEEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_SEEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_SEEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_SEEN1` writer"]
pub struct W(crate::W<WUD_SEEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_SEEN1_SPEC>;
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
impl From<crate::W<WUD_SEEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_SEEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `gpio32` reader - Wake-Up Detect Status for P4.0"]
pub struct GPIO32_R(crate::FieldReader<bool, bool>);
impl GPIO32_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO32_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO32_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio33` reader - Wake-Up Detect Status for P4.1"]
pub struct GPIO33_R(crate::FieldReader<bool, bool>);
impl GPIO33_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO33_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO33_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio34` reader - Wake-Up Detect Status for P4.2"]
pub struct GPIO34_R(crate::FieldReader<bool, bool>);
impl GPIO34_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO34_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio35` reader - Wake-Up Detect Status for P4.3"]
pub struct GPIO35_R(crate::FieldReader<bool, bool>);
impl GPIO35_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO35_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO35_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio36` reader - Wake-Up Detect Status for P4.4"]
pub struct GPIO36_R(crate::FieldReader<bool, bool>);
impl GPIO36_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO36_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO36_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio37` reader - Wake-Up Detect Status for P4.5"]
pub struct GPIO37_R(crate::FieldReader<bool, bool>);
impl GPIO37_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO37_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO37_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio38` reader - Wake-Up Detect Status for P4.6"]
pub struct GPIO38_R(crate::FieldReader<bool, bool>);
impl GPIO38_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO38_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO38_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio39` reader - Wake-Up Detect Status for P4.7"]
pub struct GPIO39_R(crate::FieldReader<bool, bool>);
impl GPIO39_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO39_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO39_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio40` reader - Wake-Up Detect Status for P5.0"]
pub struct GPIO40_R(crate::FieldReader<bool, bool>);
impl GPIO40_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO40_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO40_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio41` reader - Wake-Up Detect Status for P5.1"]
pub struct GPIO41_R(crate::FieldReader<bool, bool>);
impl GPIO41_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO41_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO41_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio42` reader - Wake-Up Detect Status for P5.2"]
pub struct GPIO42_R(crate::FieldReader<bool, bool>);
impl GPIO42_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO42_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO42_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio43` reader - Wake-Up Detect Status for P5.3"]
pub struct GPIO43_R(crate::FieldReader<bool, bool>);
impl GPIO43_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO43_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO43_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio44` reader - Wake-Up Detect Status for P5.4"]
pub struct GPIO44_R(crate::FieldReader<bool, bool>);
impl GPIO44_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO44_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO44_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio45` reader - Wake-Up Detect Status for P5.5"]
pub struct GPIO45_R(crate::FieldReader<bool, bool>);
impl GPIO45_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO45_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO45_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio46` reader - Wake-Up Detect Status for P5.6"]
pub struct GPIO46_R(crate::FieldReader<bool, bool>);
impl GPIO46_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO46_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO46_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio47` reader - Wake-Up Detect Status for P5.7"]
pub struct GPIO47_R(crate::FieldReader<bool, bool>);
impl GPIO47_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO47_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO47_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `gpio48` reader - Wake-Up Detect Status for P6.0"]
pub struct GPIO48_R(crate::FieldReader<bool, bool>);
impl GPIO48_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        GPIO48_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GPIO48_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Wake-Up Detect Status for P4.0"]
    #[inline(always)]
    pub fn gpio32(&self) -> GPIO32_R {
        GPIO32_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Wake-Up Detect Status for P4.1"]
    #[inline(always)]
    pub fn gpio33(&self) -> GPIO33_R {
        GPIO33_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-Up Detect Status for P4.2"]
    #[inline(always)]
    pub fn gpio34(&self) -> GPIO34_R {
        GPIO34_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Wake-Up Detect Status for P4.3"]
    #[inline(always)]
    pub fn gpio35(&self) -> GPIO35_R {
        GPIO35_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Detect Status for P4.4"]
    #[inline(always)]
    pub fn gpio36(&self) -> GPIO36_R {
        GPIO36_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Wake-Up Detect Status for P4.5"]
    #[inline(always)]
    pub fn gpio37(&self) -> GPIO37_R {
        GPIO37_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Wake-Up Detect Status for P4.6"]
    #[inline(always)]
    pub fn gpio38(&self) -> GPIO38_R {
        GPIO38_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Wake-Up Detect Status for P4.7"]
    #[inline(always)]
    pub fn gpio39(&self) -> GPIO39_R {
        GPIO39_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Wake-Up Detect Status for P5.0"]
    #[inline(always)]
    pub fn gpio40(&self) -> GPIO40_R {
        GPIO40_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Wake-Up Detect Status for P5.1"]
    #[inline(always)]
    pub fn gpio41(&self) -> GPIO41_R {
        GPIO41_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Wake-Up Detect Status for P5.2"]
    #[inline(always)]
    pub fn gpio42(&self) -> GPIO42_R {
        GPIO42_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Wake-Up Detect Status for P5.3"]
    #[inline(always)]
    pub fn gpio43(&self) -> GPIO43_R {
        GPIO43_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Wake-Up Detect Status for P5.4"]
    #[inline(always)]
    pub fn gpio44(&self) -> GPIO44_R {
        GPIO44_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Wake-Up Detect Status for P5.5"]
    #[inline(always)]
    pub fn gpio45(&self) -> GPIO45_R {
        GPIO45_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Wake-Up Detect Status for P5.6"]
    #[inline(always)]
    pub fn gpio46(&self) -> GPIO46_R {
        GPIO46_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Wake-Up Detect Status for P5.7"]
    #[inline(always)]
    pub fn gpio47(&self) -> GPIO47_R {
        GPIO47_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Wake-Up Detect Status for P6.0"]
    #[inline(always)]
    pub fn gpio48(&self) -> GPIO48_R {
        GPIO48_R::new(((self.bits >> 16) & 0x01) != 0)
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
#[doc = "Wake-up Detect Status for P4/P5/P6/P7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_seen1](index.html) module"]
pub struct WUD_SEEN1_SPEC;
impl crate::RegisterSpec for WUD_SEEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_seen1::R](R) reader structure"]
impl crate::Readable for WUD_SEEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_seen1::W](W) writer structure"]
impl crate::Writable for WUD_SEEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_SEEN1 to value 0"]
impl crate::Resettable for WUD_SEEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
