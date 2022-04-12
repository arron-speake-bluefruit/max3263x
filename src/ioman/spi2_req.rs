#[doc = "Register `SPI2_REQ` reader"]
pub struct R(crate::R<SPI2_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI2_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI2_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI2_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI2_REQ` writer"]
pub struct W(crate::W<SPI2_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI2_REQ_SPEC>;
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
impl From<crate::W<SPI2_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI2_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mapping_req` reader - SPI Master 2 I/O Mapping Select"]
pub struct MAPPING_REQ_R(crate::FieldReader<bool, bool>);
impl MAPPING_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MAPPING_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAPPING_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mapping_req` writer - SPI Master 2 I/O Mapping Select"]
pub struct MAPPING_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> MAPPING_REQ_W<'a> {
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
#[doc = "Field `core_io_req` reader - SPI Master 2 Core I/O Request"]
pub struct CORE_IO_REQ_R(crate::FieldReader<bool, bool>);
impl CORE_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CORE_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CORE_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `core_io_req` writer - SPI Master 2 Core I/O Request"]
pub struct CORE_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CORE_IO_REQ_W<'a> {
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
#[doc = "Field `ss0_io_req` reader - SPI Master 2 SS\\[0\\]
I/O Request"]
pub struct SS0_IO_REQ_R(crate::FieldReader<bool, bool>);
impl SS0_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS0_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS0_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss0_io_req` writer - SPI Master 2 SS\\[0\\]
I/O Request"]
pub struct SS0_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SS0_IO_REQ_W<'a> {
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
#[doc = "Field `ss1_io_req` reader - SPI Master 2 SS\\[1\\]
I/O Request"]
pub struct SS1_IO_REQ_R(crate::FieldReader<bool, bool>);
impl SS1_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS1_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS1_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss1_io_req` writer - SPI Master 2 SS\\[1\\]
I/O Request"]
pub struct SS1_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SS1_IO_REQ_W<'a> {
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
#[doc = "Field `ss2_io_req` reader - SPI Master 2 SS\\[2\\]
I/O Request"]
pub struct SS2_IO_REQ_R(crate::FieldReader<bool, bool>);
impl SS2_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS2_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS2_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss2_io_req` writer - SPI Master 2 SS\\[2\\]
I/O Request"]
pub struct SS2_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SS2_IO_REQ_W<'a> {
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
#[doc = "Field `sr0_io_req` reader - SPI Master 2 SR\\[0\\]
I/O Request"]
pub struct SR0_IO_REQ_R(crate::FieldReader<bool, bool>);
impl SR0_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SR0_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR0_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sr0_io_req` writer - SPI Master 2 SR\\[0\\]
I/O Request"]
pub struct SR0_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SR0_IO_REQ_W<'a> {
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
#[doc = "Field `sr1_io_req` reader - SPI Master 2 SR\\[1\\]
I/O Request"]
pub struct SR1_IO_REQ_R(crate::FieldReader<bool, bool>);
impl SR1_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SR1_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SR1_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sr1_io_req` writer - SPI Master 2 SR\\[1\\]
I/O Request"]
pub struct SR1_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> SR1_IO_REQ_W<'a> {
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
#[doc = "Field `quad_io_req` reader - SPI Master 2 Quad I/O Request"]
pub struct QUAD_IO_REQ_R(crate::FieldReader<bool, bool>);
impl QUAD_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        QUAD_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QUAD_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `quad_io_req` writer - SPI Master 2 Quad I/O Request"]
pub struct QUAD_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> QUAD_IO_REQ_W<'a> {
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
#[doc = "Field `fast_mode` reader - SPI Master 2 Fast Mode Request"]
pub struct FAST_MODE_R(crate::FieldReader<bool, bool>);
impl FAST_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAST_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAST_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fast_mode` writer - SPI Master 2 Fast Mode Request"]
pub struct FAST_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_MODE_W<'a> {
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
    #[doc = "Bit 0 - SPI Master 2 I/O Mapping Select"]
    #[inline(always)]
    pub fn mapping_req(&self) -> MAPPING_REQ_R {
        MAPPING_REQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - SPI Master 2 Core I/O Request"]
    #[inline(always)]
    pub fn core_io_req(&self) -> CORE_IO_REQ_R {
        CORE_IO_REQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI Master 2 SS\\[0\\]
I/O Request"]
    #[inline(always)]
    pub fn ss0_io_req(&self) -> SS0_IO_REQ_R {
        SS0_IO_REQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI Master 2 SS\\[1\\]
I/O Request"]
    #[inline(always)]
    pub fn ss1_io_req(&self) -> SS1_IO_REQ_R {
        SS1_IO_REQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI Master 2 SS\\[2\\]
I/O Request"]
    #[inline(always)]
    pub fn ss2_io_req(&self) -> SS2_IO_REQ_R {
        SS2_IO_REQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - SPI Master 2 SR\\[0\\]
I/O Request"]
    #[inline(always)]
    pub fn sr0_io_req(&self) -> SR0_IO_REQ_R {
        SR0_IO_REQ_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPI Master 2 SR\\[1\\]
I/O Request"]
    #[inline(always)]
    pub fn sr1_io_req(&self) -> SR1_IO_REQ_R {
        SR1_IO_REQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI Master 2 Quad I/O Request"]
    #[inline(always)]
    pub fn quad_io_req(&self) -> QUAD_IO_REQ_R {
        QUAD_IO_REQ_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - SPI Master 2 Fast Mode Request"]
    #[inline(always)]
    pub fn fast_mode(&self) -> FAST_MODE_R {
        FAST_MODE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Master 2 I/O Mapping Select"]
    #[inline(always)]
    pub fn mapping_req(&mut self) -> MAPPING_REQ_W {
        MAPPING_REQ_W { w: self }
    }
    #[doc = "Bit 4 - SPI Master 2 Core I/O Request"]
    #[inline(always)]
    pub fn core_io_req(&mut self) -> CORE_IO_REQ_W {
        CORE_IO_REQ_W { w: self }
    }
    #[doc = "Bit 8 - SPI Master 2 SS\\[0\\]
I/O Request"]
    #[inline(always)]
    pub fn ss0_io_req(&mut self) -> SS0_IO_REQ_W {
        SS0_IO_REQ_W { w: self }
    }
    #[doc = "Bit 9 - SPI Master 2 SS\\[1\\]
I/O Request"]
    #[inline(always)]
    pub fn ss1_io_req(&mut self) -> SS1_IO_REQ_W {
        SS1_IO_REQ_W { w: self }
    }
    #[doc = "Bit 10 - SPI Master 2 SS\\[2\\]
I/O Request"]
    #[inline(always)]
    pub fn ss2_io_req(&mut self) -> SS2_IO_REQ_W {
        SS2_IO_REQ_W { w: self }
    }
    #[doc = "Bit 16 - SPI Master 2 SR\\[0\\]
I/O Request"]
    #[inline(always)]
    pub fn sr0_io_req(&mut self) -> SR0_IO_REQ_W {
        SR0_IO_REQ_W { w: self }
    }
    #[doc = "Bit 17 - SPI Master 2 SR\\[1\\]
I/O Request"]
    #[inline(always)]
    pub fn sr1_io_req(&mut self) -> SR1_IO_REQ_W {
        SR1_IO_REQ_W { w: self }
    }
    #[doc = "Bit 20 - SPI Master 2 Quad I/O Request"]
    #[inline(always)]
    pub fn quad_io_req(&mut self) -> QUAD_IO_REQ_W {
        QUAD_IO_REQ_W { w: self }
    }
    #[doc = "Bit 24 - SPI Master 2 Fast Mode Request"]
    #[inline(always)]
    pub fn fast_mode(&mut self) -> FAST_MODE_W {
        FAST_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master 2 I/O Mode Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi2_req](index.html) module"]
pub struct SPI2_REQ_SPEC;
impl crate::RegisterSpec for SPI2_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi2_req::R](R) reader structure"]
impl crate::Readable for SPI2_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi2_req::W](W) writer structure"]
impl crate::Writable for SPI2_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI2_REQ to value 0"]
impl crate::Resettable for SPI2_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
