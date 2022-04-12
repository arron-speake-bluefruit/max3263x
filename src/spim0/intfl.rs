#[doc = "Register `INTFL` reader"]
pub struct R(crate::R<INTFL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTFL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTFL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTFL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTFL` writer"]
pub struct W(crate::W<INTFL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTFL_SPEC>;
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
impl From<crate::W<INTFL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTFL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_stalled` reader - Transaction Stalled Int Status"]
pub struct TX_STALLED_R(crate::FieldReader<bool, bool>);
impl TX_STALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_STALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_STALLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_stalled` writer - Transaction Stalled Int Status"]
pub struct TX_STALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_STALLED_W<'a> {
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
#[doc = "Field `rx_stalled` reader - Results Stalled Int Status"]
pub struct RX_STALLED_R(crate::FieldReader<bool, bool>);
impl RX_STALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_STALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_STALLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_stalled` writer - Results Stalled Int Status"]
pub struct RX_STALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_STALLED_W<'a> {
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
#[doc = "Field `tx_ready` reader - Transaction Ready Int Status"]
pub struct TX_READY_R(crate::FieldReader<bool, bool>);
impl TX_READY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_READY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_READY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_ready` writer - Transaction Ready Int Status"]
pub struct TX_READY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_READY_W<'a> {
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
#[doc = "Field `rx_done` reader - Results Done Int Status"]
pub struct RX_DONE_R(crate::FieldReader<bool, bool>);
impl RX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_done` writer - Results Done Int Status"]
pub struct RX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DONE_W<'a> {
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
#[doc = "Field `tx_fifo_ae` reader - TXFIFO Almost Empty Int Status"]
pub struct TX_FIFO_AE_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_AE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_AE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_ae` writer - TXFIFO Almost Empty Int Status"]
pub struct TX_FIFO_AE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_AE_W<'a> {
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
#[doc = "Field `rx_fifo_af` reader - RXFIFO Almost Full Int Status"]
pub struct RX_FIFO_AF_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_AF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_AF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_AF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_af` writer - RXFIFO Almost Full Int Status"]
pub struct RX_FIFO_AF_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_AF_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transaction Stalled Int Status"]
    #[inline(always)]
    pub fn tx_stalled(&self) -> TX_STALLED_R {
        TX_STALLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Results Stalled Int Status"]
    #[inline(always)]
    pub fn rx_stalled(&self) -> RX_STALLED_R {
        RX_STALLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Ready Int Status"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Results Done Int Status"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXFIFO Almost Empty Int Status"]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO Almost Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Stalled Int Status"]
    #[inline(always)]
    pub fn tx_stalled(&mut self) -> TX_STALLED_W {
        TX_STALLED_W { w: self }
    }
    #[doc = "Bit 1 - Results Stalled Int Status"]
    #[inline(always)]
    pub fn rx_stalled(&mut self) -> RX_STALLED_W {
        RX_STALLED_W { w: self }
    }
    #[doc = "Bit 2 - Transaction Ready Int Status"]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W { w: self }
    }
    #[doc = "Bit 3 - Results Done Int Status"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RX_DONE_W {
        RX_DONE_W { w: self }
    }
    #[doc = "Bit 4 - TXFIFO Almost Empty Int Status"]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W { w: self }
    }
    #[doc = "Bit 5 - RXFIFO Almost Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
pub struct INTFL_SPEC;
impl crate::RegisterSpec for INTFL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intfl::R](R) reader structure"]
impl crate::Readable for INTFL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intfl::W](W) writer structure"]
impl crate::Writable for INTFL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTFL to value 0"]
impl crate::Resettable for INTFL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
