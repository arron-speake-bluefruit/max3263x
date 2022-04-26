#[doc = "Register `BUF_OVR_INT` reader"]
pub struct R(crate::R<BUF_OVR_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF_OVR_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF_OVR_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF_OVR_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF_OVR_INT` writer"]
pub struct W(crate::W<BUF_OVR_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF_OVR_INT_SPEC>;
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
impl From<crate::W<BUF_OVR_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF_OVR_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `buf_ovr0` reader - Endpoint 0 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR0_R(crate::FieldReader<bool>);
impl BUF_OVR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR0_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr0` writer - Endpoint 0 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR0_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR0_W<'a> {
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
#[doc = "Field `buf_ovr1` reader - Endpoint 1 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR1_R(crate::FieldReader<bool>);
impl BUF_OVR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR1_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr1` writer - Endpoint 1 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR1_W<'a> {
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
#[doc = "Field `buf_ovr2` reader - Endpoint 2 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR2_R(crate::FieldReader<bool>);
impl BUF_OVR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR2_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr2` writer - Endpoint 2 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR2_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR2_W<'a> {
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
#[doc = "Field `buf_ovr3` reader - Endpoint 3 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR3_R(crate::FieldReader<bool>);
impl BUF_OVR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR3_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr3` writer - Endpoint 3 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR3_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR3_W<'a> {
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
#[doc = "Field `buf_ovr4` reader - Endpoint 4 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR4_R(crate::FieldReader<bool>);
impl BUF_OVR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR4_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr4` writer - Endpoint 4 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR4_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR4_W<'a> {
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
#[doc = "Field `buf_ovr5` reader - Endpoint 5 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR5_R(crate::FieldReader<bool>);
impl BUF_OVR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR5_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr5` writer - Endpoint 5 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR5_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR5_W<'a> {
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
#[doc = "Field `buf_ovr6` reader - Endpoint 6 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR6_R(crate::FieldReader<bool>);
impl BUF_OVR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR6_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr6` writer - Endpoint 6 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR6_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR6_W<'a> {
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
#[doc = "Field `buf_ovr7` reader - Endpoint 7 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR7_R(crate::FieldReader<bool>);
impl BUF_OVR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR7_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr7` writer - Endpoint 7 Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR7_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR7_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Endpoint 0 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr0(&self) -> BUF_OVR0_R {
        BUF_OVR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr1(&self) -> BUF_OVR1_R {
        BUF_OVR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr2(&self) -> BUF_OVR2_R {
        BUF_OVR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr3(&self) -> BUF_OVR3_R {
        BUF_OVR3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr4(&self) -> BUF_OVR4_R {
        BUF_OVR4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr5(&self) -> BUF_OVR5_R {
        BUF_OVR5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr6(&self) -> BUF_OVR6_R {
        BUF_OVR6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr7(&self) -> BUF_OVR7_R {
        BUF_OVR7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr0(&mut self) -> BUF_OVR0_W {
        BUF_OVR0_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr1(&mut self) -> BUF_OVR1_W {
        BUF_OVR1_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr2(&mut self) -> BUF_OVR2_W {
        BUF_OVR2_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr3(&mut self) -> BUF_OVR3_W {
        BUF_OVR3_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr4(&mut self) -> BUF_OVR4_W {
        BUF_OVR4_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr5(&mut self) -> BUF_OVR5_W {
        BUF_OVR5_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr6(&mut self) -> BUF_OVR6_W {
        BUF_OVR6_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr7(&mut self) -> BUF_OVR7_W {
        BUF_OVR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Buffer Overflow Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf_ovr_int](index.html) module"]
pub struct BUF_OVR_INT_SPEC;
impl crate::RegisterSpec for BUF_OVR_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf_ovr_int::R](R) reader structure"]
impl crate::Readable for BUF_OVR_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf_ovr_int::W](W) writer structure"]
impl crate::Writable for BUF_OVR_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF_OVR_INT to value 0"]
impl crate::Resettable for BUF_OVR_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
