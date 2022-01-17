#[doc = "Register `DMA_ERR_INT` reader"]
pub struct R(crate::R<DMA_ERR_INT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_ERR_INT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_ERR_INT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_ERR_INT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_ERR_INT` writer"]
pub struct W(crate::W<DMA_ERR_INT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_ERR_INT_SPEC>;
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
impl From<crate::W<DMA_ERR_INT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_ERR_INT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dma_err0` reader - Endpoint 0 DMA Error Interrupt Flag"]
pub struct DMA_ERR0_R(crate::FieldReader<bool, bool>);
impl DMA_ERR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err0` writer - Endpoint 0 DMA Error Interrupt Flag"]
pub struct DMA_ERR0_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR0_W<'a> {
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
#[doc = "Field `dma_err1` reader - Endpoint 1 DMA Error Interrupt Flag"]
pub struct DMA_ERR1_R(crate::FieldReader<bool, bool>);
impl DMA_ERR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err1` writer - Endpoint 1 DMA Error Interrupt Flag"]
pub struct DMA_ERR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR1_W<'a> {
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
#[doc = "Field `dma_err2` reader - Endpoint 2 DMA Error Interrupt Flag"]
pub struct DMA_ERR2_R(crate::FieldReader<bool, bool>);
impl DMA_ERR2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err2` writer - Endpoint 2 DMA Error Interrupt Flag"]
pub struct DMA_ERR2_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR2_W<'a> {
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
#[doc = "Field `dma_err3` reader - Endpoint 3 DMA Error Interrupt Flag"]
pub struct DMA_ERR3_R(crate::FieldReader<bool, bool>);
impl DMA_ERR3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err3` writer - Endpoint 3 DMA Error Interrupt Flag"]
pub struct DMA_ERR3_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR3_W<'a> {
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
#[doc = "Field `dma_err4` reader - Endpoint 4 DMA Error Interrupt Flag"]
pub struct DMA_ERR4_R(crate::FieldReader<bool, bool>);
impl DMA_ERR4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err4` writer - Endpoint 4 DMA Error Interrupt Flag"]
pub struct DMA_ERR4_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR4_W<'a> {
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
#[doc = "Field `dma_err5` reader - Endpoint 5 DMA Error Interrupt Flag"]
pub struct DMA_ERR5_R(crate::FieldReader<bool, bool>);
impl DMA_ERR5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err5` writer - Endpoint 5 DMA Error Interrupt Flag"]
pub struct DMA_ERR5_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR5_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `dma_err6` reader - Endpoint 6 DMA Error Interrupt Flag"]
pub struct DMA_ERR6_R(crate::FieldReader<bool, bool>);
impl DMA_ERR6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err6` writer - Endpoint 6 DMA Error Interrupt Flag"]
pub struct DMA_ERR6_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR6_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `dma_err7` reader - Endpoint 7 DMA Error Interrupt Flag"]
pub struct DMA_ERR7_R(crate::FieldReader<bool, bool>);
impl DMA_ERR7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err7` writer - Endpoint 7 DMA Error Interrupt Flag"]
pub struct DMA_ERR7_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Endpoint 0 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err0(&self) -> DMA_ERR0_R {
        DMA_ERR0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Endpoint 1 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err1(&self) -> DMA_ERR1_R {
        DMA_ERR1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Endpoint 2 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err2(&self) -> DMA_ERR2_R {
        DMA_ERR2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Endpoint 3 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err3(&self) -> DMA_ERR3_R {
        DMA_ERR3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint 4 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err4(&self) -> DMA_ERR4_R {
        DMA_ERR4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint 5 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err5(&self) -> DMA_ERR5_R {
        DMA_ERR5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint 6 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err6(&self) -> DMA_ERR6_R {
        DMA_ERR6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Endpoint 7 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err7(&self) -> DMA_ERR7_R {
        DMA_ERR7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint 0 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err0(&mut self) -> DMA_ERR0_W {
        DMA_ERR0_W { w: self }
    }
    #[doc = "Bit 1 - Endpoint 1 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err1(&mut self) -> DMA_ERR1_W {
        DMA_ERR1_W { w: self }
    }
    #[doc = "Bit 2 - Endpoint 2 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err2(&mut self) -> DMA_ERR2_W {
        DMA_ERR2_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint 3 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err3(&mut self) -> DMA_ERR3_W {
        DMA_ERR3_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint 4 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err4(&mut self) -> DMA_ERR4_W {
        DMA_ERR4_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint 5 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err5(&mut self) -> DMA_ERR5_W {
        DMA_ERR5_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint 6 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err6(&mut self) -> DMA_ERR6_W {
        DMA_ERR6_W { w: self }
    }
    #[doc = "Bit 7 - Endpoint 7 DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err7(&mut self) -> DMA_ERR7_W {
        DMA_ERR7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB DMA Error Interrupt\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_err_int](index.html) module"]
pub struct DMA_ERR_INT_SPEC;
impl crate::RegisterSpec for DMA_ERR_INT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_err_int::R](R) reader structure"]
impl crate::Readable for DMA_ERR_INT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_err_int::W](W) writer structure"]
impl crate::Writable for DMA_ERR_INT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMA_ERR_INT to value 0"]
impl crate::Resettable for DMA_ERR_INT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
