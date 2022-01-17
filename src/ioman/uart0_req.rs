#[doc = "Register `UART0_REQ` reader"]
pub struct R(crate::R<UART0_REQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UART0_REQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UART0_REQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UART0_REQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UART0_REQ` writer"]
pub struct W(crate::W<UART0_REQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UART0_REQ_SPEC>;
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
impl From<crate::W<UART0_REQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UART0_REQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `io_map` reader - UART0 TX/RX I/O Mapping Select"]
pub struct IO_MAP_R(crate::FieldReader<bool, bool>);
impl IO_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_MAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_map` writer - UART0 TX/RX I/O Mapping Select"]
pub struct IO_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_MAP_W<'a> {
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
#[doc = "Field `cts_map` reader - UART0 CTS I/O Mapping Select"]
pub struct CTS_MAP_R(crate::FieldReader<bool, bool>);
impl CTS_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_MAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cts_map` writer - UART0 CTS I/O Mapping Select"]
pub struct CTS_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_MAP_W<'a> {
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
#[doc = "Field `rts_map` reader - UART0 RTS I/O Mapping Select"]
pub struct RTS_MAP_R(crate::FieldReader<bool, bool>);
impl RTS_MAP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_MAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_MAP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rts_map` writer - UART0 RTS I/O Mapping Select"]
pub struct RTS_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_MAP_W<'a> {
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
#[doc = "Field `io_req` reader - UART0 TX/RX I/O Request"]
pub struct IO_REQ_R(crate::FieldReader<bool, bool>);
impl IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `io_req` writer - UART0 TX/RX I/O Request"]
pub struct IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> IO_REQ_W<'a> {
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
#[doc = "Field `cts_io_req` reader - UART0 CTS I/O Request"]
pub struct CTS_IO_REQ_R(crate::FieldReader<bool, bool>);
impl CTS_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cts_io_req` writer - UART0 CTS I/O Request"]
pub struct CTS_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_IO_REQ_W<'a> {
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
#[doc = "Field `rts_io_req` reader - UART0 RTS I/O Request"]
pub struct RTS_IO_REQ_R(crate::FieldReader<bool, bool>);
impl RTS_IO_REQ_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_IO_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_IO_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rts_io_req` writer - UART0 RTS I/O Request"]
pub struct RTS_IO_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_IO_REQ_W<'a> {
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
impl R {
    #[doc = "Bit 0 - UART0 TX/RX I/O Mapping Select"]
    #[inline(always)]
    pub fn io_map(&self) -> IO_MAP_R {
        IO_MAP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - UART0 CTS I/O Mapping Select"]
    #[inline(always)]
    pub fn cts_map(&self) -> CTS_MAP_R {
        CTS_MAP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART0 RTS I/O Mapping Select"]
    #[inline(always)]
    pub fn rts_map(&self) -> RTS_MAP_R {
        RTS_MAP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - UART0 TX/RX I/O Request"]
    #[inline(always)]
    pub fn io_req(&self) -> IO_REQ_R {
        IO_REQ_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - UART0 CTS I/O Request"]
    #[inline(always)]
    pub fn cts_io_req(&self) -> CTS_IO_REQ_R {
        CTS_IO_REQ_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - UART0 RTS I/O Request"]
    #[inline(always)]
    pub fn rts_io_req(&self) -> RTS_IO_REQ_R {
        RTS_IO_REQ_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 TX/RX I/O Mapping Select"]
    #[inline(always)]
    pub fn io_map(&mut self) -> IO_MAP_W {
        IO_MAP_W { w: self }
    }
    #[doc = "Bit 1 - UART0 CTS I/O Mapping Select"]
    #[inline(always)]
    pub fn cts_map(&mut self) -> CTS_MAP_W {
        CTS_MAP_W { w: self }
    }
    #[doc = "Bit 2 - UART0 RTS I/O Mapping Select"]
    #[inline(always)]
    pub fn rts_map(&mut self) -> RTS_MAP_W {
        RTS_MAP_W { w: self }
    }
    #[doc = "Bit 4 - UART0 TX/RX I/O Request"]
    #[inline(always)]
    pub fn io_req(&mut self) -> IO_REQ_W {
        IO_REQ_W { w: self }
    }
    #[doc = "Bit 5 - UART0 CTS I/O Request"]
    #[inline(always)]
    pub fn cts_io_req(&mut self) -> CTS_IO_REQ_W {
        CTS_IO_REQ_W { w: self }
    }
    #[doc = "Bit 6 - UART0 RTS I/O Request"]
    #[inline(always)]
    pub fn rts_io_req(&mut self) -> RTS_IO_REQ_W {
        RTS_IO_REQ_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART0 I/O Mode Request\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uart0_req](index.html) module"]
pub struct UART0_REQ_SPEC;
impl crate::RegisterSpec for UART0_REQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [uart0_req::R](R) reader structure"]
impl crate::Readable for UART0_REQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uart0_req::W](W) writer structure"]
impl crate::Writable for UART0_REQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UART0_REQ to value 0"]
impl crate::Resettable for UART0_REQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
