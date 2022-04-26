#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_done` reader - TX Done Interrupt Enable/Disable"]
pub struct TX_DONE_R(crate::FieldReader<bool>);
impl TX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DONE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_done` writer - TX Done Interrupt Enable/Disable"]
pub struct TX_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DONE_W<'a> {
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
#[doc = "Field `tx_unstalled` reader - TX Unstalled Interrupt Enable/Disable"]
pub struct TX_UNSTALLED_R(crate::FieldReader<bool>);
impl TX_UNSTALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_UNSTALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_UNSTALLED_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_unstalled` writer - TX Unstalled Interrupt Enable/Disable"]
pub struct TX_UNSTALLED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UNSTALLED_W<'a> {
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
#[doc = "Field `tx_fifo_ae` reader - TX FIFO Almost Empty Interrupt Enable/Disable"]
pub struct TX_FIFO_AE_R(crate::FieldReader<bool>);
impl TX_FIFO_AE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_AE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_AE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_ae` writer - TX FIFO Almost Empty Interrupt Enable/Disable"]
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `rx_fifo_not_empty` reader - RX FIFO Not Empty Interrupt Enable/Disable"]
pub struct RX_FIFO_NOT_EMPTY_R(crate::FieldReader<bool>);
impl RX_FIFO_NOT_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_NOT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_NOT_EMPTY_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_not_empty` writer - RX FIFO Not Empty Interrupt Enable/Disable"]
pub struct RX_FIFO_NOT_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_NOT_EMPTY_W<'a> {
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
#[doc = "Field `rx_stalled` reader - RX Stalled Interrupt Enable/Disable"]
pub struct RX_STALLED_R(crate::FieldReader<bool>);
impl RX_STALLED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_STALLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_STALLED_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_stalled` writer - RX Stalled Interrupt Enable/Disable"]
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `rx_fifo_af` reader - RX FIFO Almost Full Interrupt Enable/Disable"]
pub struct RX_FIFO_AF_R(crate::FieldReader<bool>);
impl RX_FIFO_AF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_AF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_AF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_af` writer - RX FIFO Almost Full Interrupt Enable/Disable"]
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
#[doc = "Field `rx_fifo_overflow` reader - RX FIFO Overflow Interrupt Enable/Disable"]
pub struct RX_FIFO_OVERFLOW_R(crate::FieldReader<bool>);
impl RX_FIFO_OVERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_OVERFLOW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_OVERFLOW_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_overflow` writer - RX FIFO Overflow Interrupt Enable/Disable"]
pub struct RX_FIFO_OVERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_OVERFLOW_W<'a> {
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
#[doc = "Field `rx_framing_err` reader - RX Framing Error Interrupt Enable/Disable"]
pub struct RX_FRAMING_ERR_R(crate::FieldReader<bool>);
impl RX_FRAMING_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FRAMING_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FRAMING_ERR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_framing_err` writer - RX Framing Error Interrupt Enable/Disable"]
pub struct RX_FRAMING_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FRAMING_ERR_W<'a> {
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
#[doc = "Field `rx_parity_err` reader - RX Parity Error Interrupt Enable/Disable"]
pub struct RX_PARITY_ERR_R(crate::FieldReader<bool>);
impl RX_PARITY_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_PARITY_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_PARITY_ERR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_parity_err` writer - RX Parity Error Interrupt Enable/Disable"]
pub struct RX_PARITY_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_PARITY_ERR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TX Done Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Unstalled Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn tx_unstalled(&self) -> TX_UNSTALLED_R {
        TX_UNSTALLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Almost Empty Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Not Empty Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTY_R {
        RX_FIFO_NOT_EMPTY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RX Stalled Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_stalled(&self) -> RX_STALLED_R {
        RX_STALLED_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RX FIFO Almost Full Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX FIFO Overflow Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&self) -> RX_FIFO_OVERFLOW_R {
        RX_FIFO_OVERFLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - RX Framing Error Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_framing_err(&self) -> RX_FRAMING_ERR_R {
        RX_FRAMING_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RX Parity Error Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_parity_err(&self) -> RX_PARITY_ERR_R {
        RX_PARITY_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX Done Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 1 - TX Unstalled Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn tx_unstalled(&mut self) -> TX_UNSTALLED_W {
        TX_UNSTALLED_W { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Almost Empty Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO Not Empty Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&mut self) -> RX_FIFO_NOT_EMPTY_W {
        RX_FIFO_NOT_EMPTY_W { w: self }
    }
    #[doc = "Bit 4 - RX Stalled Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_stalled(&mut self) -> RX_STALLED_W {
        RX_STALLED_W { w: self }
    }
    #[doc = "Bit 5 - RX FIFO Almost Full Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W { w: self }
    }
    #[doc = "Bit 6 - RX FIFO Overflow Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_fifo_overflow(&mut self) -> RX_FIFO_OVERFLOW_W {
        RX_FIFO_OVERFLOW_W { w: self }
    }
    #[doc = "Bit 7 - RX Framing Error Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_framing_err(&mut self) -> RX_FRAMING_ERR_W {
        RX_FRAMING_ERR_W { w: self }
    }
    #[doc = "Bit 8 - RX Parity Error Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn rx_parity_err(&mut self) -> RX_PARITY_ERR_W {
        RX_PARITY_ERR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Interrupt Enable/Disable Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
