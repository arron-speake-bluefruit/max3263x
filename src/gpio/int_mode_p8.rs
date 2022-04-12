#[doc = "Register `INT_MODE_P8` reader"]
pub struct R(crate::R<INT_MODE_P8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_MODE_P8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_MODE_P8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_MODE_P8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_MODE_P8` writer"]
pub struct W(crate::W<INT_MODE_P8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_MODE_P8_SPEC>;
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
impl From<crate::W<INT_MODE_P8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_MODE_P8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P8.0 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin0` writer - P8.0 GPIO Interrupt Detection Mode"]
pub struct PIN0_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `pin1` reader - P8.1 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin1` writer - P8.1 GPIO Interrupt Detection Mode"]
pub struct PIN1_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `pin2` reader - P8.2 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin2` writer - P8.2 GPIO Interrupt Detection Mode"]
pub struct PIN2_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 8)) | ((value as u32 & 7) << 8);
        self.w
    }
}
#[doc = "Field `pin3` reader - P8.3 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin3` writer - P8.3 GPIO Interrupt Detection Mode"]
pub struct PIN3_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 12)) | ((value as u32 & 7) << 12);
        self.w
    }
}
#[doc = "Field `pin4` reader - P8.4 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin4` writer - P8.4 GPIO Interrupt Detection Mode"]
pub struct PIN4_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 16)) | ((value as u32 & 7) << 16);
        self.w
    }
}
#[doc = "Field `pin5` reader - P8.5 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin5` writer - P8.5 GPIO Interrupt Detection Mode"]
pub struct PIN5_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 20)) | ((value as u32 & 7) << 20);
        self.w
    }
}
#[doc = "Field `pin6` reader - P8.6 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin6` writer - P8.6 GPIO Interrupt Detection Mode"]
pub struct PIN6_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 24)) | ((value as u32 & 7) << 24);
        self.w
    }
}
#[doc = "Field `pin7` reader - P8.7 GPIO Interrupt Detection Mode"]
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
#[doc = "Field `pin7` writer - P8.7 GPIO Interrupt Detection Mode"]
pub struct PIN7_W<'a> {
    w: &'a mut W,
}
impl<'a> PIN7_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 28)) | ((value as u32 & 7) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - P8.0 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - P8.1 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - P8.2 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - P8.3 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - P8.4 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - P8.5 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - P8.6 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - P8.7 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - P8.0 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W { w: self }
    }
    #[doc = "Bits 4:6 - P8.1 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W { w: self }
    }
    #[doc = "Bits 8:10 - P8.2 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W { w: self }
    }
    #[doc = "Bits 12:14 - P8.3 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W { w: self }
    }
    #[doc = "Bits 16:18 - P8.4 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W { w: self }
    }
    #[doc = "Bits 20:22 - P8.5 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W { w: self }
    }
    #[doc = "Bits 24:26 - P8.6 GPIO Interrupt Detection Mode"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W { w: self }
    }
    #[doc = "Bits 28:30 - P8.7 GPIO Interrupt Detection Mode"]
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
#[doc = "Port P8 Interrupt Detection Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_mode_p8](index.html) module"]
pub struct INT_MODE_P8_SPEC;
impl crate::RegisterSpec for INT_MODE_P8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_mode_p8::R](R) reader structure"]
impl crate::Readable for INT_MODE_P8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_mode_p8::W](W) writer structure"]
impl crate::Writable for INT_MODE_P8_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INT_MODE_P8 to value 0"]
impl crate::Resettable for INT_MODE_P8_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
