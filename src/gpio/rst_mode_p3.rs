#[doc = "Register `RST_MODE_P3` reader"]
pub struct R(crate::R<RST_MODE_P3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_MODE_P3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_MODE_P3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_MODE_P3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_MODE_P3` writer"]
pub struct W(crate::W<RST_MODE_P3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_MODE_P3_SPEC>;
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
impl From<crate::W<RST_MODE_P3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_MODE_P3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P3.0 Default Output Drive Mode"]
pub struct PIN0_R(crate::FieldReader<u8, u8>);
impl PIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin0` writer - P3.0 Default Output Drive Mode"]
pub struct PIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `pin1` reader - P3.1 Default Output Drive Mode"]
pub struct PIN1_R(crate::FieldReader<u8, u8>);
impl PIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin1` writer - P3.1 Default Output Drive Mode"]
pub struct PIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `pin2` reader - P3.2 Default Output Drive Mode"]
pub struct PIN2_R(crate::FieldReader<u8, u8>);
impl PIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin2` writer - P3.2 Default Output Drive Mode"]
pub struct PIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `pin3` reader - P3.3 Default Output Drive Mode"]
pub struct PIN3_R(crate::FieldReader<u8, u8>);
impl PIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN3_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin3` writer - P3.3 Default Output Drive Mode"]
pub struct PIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "Field `pin4` reader - P3.4 Default Output Drive Mode"]
pub struct PIN4_R(crate::FieldReader<u8, u8>);
impl PIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin4` writer - P3.4 Default Output Drive Mode"]
pub struct PIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Field `pin5` reader - P3.5 Default Output Drive Mode"]
pub struct PIN5_R(crate::FieldReader<u8, u8>);
impl PIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin5` writer - P3.5 Default Output Drive Mode"]
pub struct PIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `pin6` reader - P3.6 Default Output Drive Mode"]
pub struct PIN6_R(crate::FieldReader<u8, u8>);
impl PIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN6_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin6` writer - P3.6 Default Output Drive Mode"]
pub struct PIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "Field `pin7` reader - P3.7 Default Output Drive Mode"]
pub struct PIN7_R(crate::FieldReader<u8, u8>);
impl PIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PIN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin7` writer - P3.7 Default Output Drive Mode"]
pub struct PIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - P3.0 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - P3.1 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - P3.2 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - P3.3 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - P3.4 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - P3.5 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - P3.6 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - P3.7 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - P3.0 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W { w: self }
    }
    #[doc = "Bits 4:6 - P3.1 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W { w: self }
    }
    #[doc = "Bits 8:10 - P3.2 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W { w: self }
    }
    #[doc = "Bits 12:14 - P3.3 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W { w: self }
    }
    #[doc = "Bits 16:18 - P3.4 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W { w: self }
    }
    #[doc = "Bits 20:22 - P3.5 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W { w: self }
    }
    #[doc = "Bits 24:26 - P3.6 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W { w: self }
    }
    #[doc = "Bits 28:30 - P3.7 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W {
        PIN7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port P3 Default (Power-On Reset) Output Drive Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_mode_p3](index.html) module"]
pub struct RST_MODE_P3_SPEC;
impl crate::RegisterSpec for RST_MODE_P3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_mode_p3::R](R) reader structure"]
impl crate::Readable for RST_MODE_P3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_mode_p3::W](W) writer structure"]
impl crate::Writable for RST_MODE_P3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_MODE_P3 to value 0"]
impl crate::Resettable for RST_MODE_P3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
