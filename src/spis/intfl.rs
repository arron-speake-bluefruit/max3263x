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
#[doc = "Field `tx_fifo_ae` reader - TX FIFO Almost Empty"]
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
#[doc = "Field `tx_fifo_ae` writer - TX FIFO Almost Empty"]
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `rx_fifo_af` reader - RX FIFO Almost Full"]
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
#[doc = "Field `rx_fifo_af` writer - RX FIFO Almost Full"]
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `tx_no_data` reader - TX FIFO Empty"]
pub struct TX_NO_DATA_R(crate::FieldReader<bool, bool>);
impl TX_NO_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_NO_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_NO_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_no_data` writer - TX FIFO Empty"]
pub struct TX_NO_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_NO_DATA_W<'a> {
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
#[doc = "Field `rx_lost_data` reader - RX FIFO Overflow"]
pub struct RX_LOST_DATA_R(crate::FieldReader<bool, bool>);
impl RX_LOST_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_LOST_DATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_LOST_DATA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_lost_data` writer - RX FIFO Overflow"]
pub struct RX_LOST_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LOST_DATA_W<'a> {
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
#[doc = "Field `ss_asserted` reader - Slave Select Asserted"]
pub struct SS_ASSERTED_R(crate::FieldReader<bool, bool>);
impl SS_ASSERTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_ASSERTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_ASSERTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_asserted` writer - Slave Select Asserted"]
pub struct SS_ASSERTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_ASSERTED_W<'a> {
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
#[doc = "Field `ss_deasserted` reader - Slave Select Deasserted"]
pub struct SS_DEASSERTED_R(crate::FieldReader<bool, bool>);
impl SS_DEASSERTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_DEASSERTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SS_DEASSERTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_deasserted` writer - Slave Select Deasserted"]
pub struct SS_DEASSERTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_DEASSERTED_W<'a> {
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
impl R {
    #[doc = "Bit 0 - TX FIFO Almost Empty"]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Almost Full"]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Empty"]
    #[inline(always)]
    pub fn tx_no_data(&self) -> TX_NO_DATA_R {
        TX_NO_DATA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Overflow"]
    #[inline(always)]
    pub fn rx_lost_data(&self) -> RX_LOST_DATA_R {
        RX_LOST_DATA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Slave Select Asserted"]
    #[inline(always)]
    pub fn ss_asserted(&self) -> SS_ASSERTED_R {
        SS_ASSERTED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Slave Select Deasserted"]
    #[inline(always)]
    pub fn ss_deasserted(&self) -> SS_DEASSERTED_R {
        SS_DEASSERTED_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Almost Empty"]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Almost Full"]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Empty"]
    #[inline(always)]
    pub fn tx_no_data(&mut self) -> TX_NO_DATA_W {
        TX_NO_DATA_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO Overflow"]
    #[inline(always)]
    pub fn rx_lost_data(&mut self) -> RX_LOST_DATA_W {
        RX_LOST_DATA_W { w: self }
    }
    #[doc = "Bit 5 - Slave Select Asserted"]
    #[inline(always)]
    pub fn ss_asserted(&mut self) -> SS_ASSERTED_W {
        SS_ASSERTED_W { w: self }
    }
    #[doc = "Bit 6 - Slave Select Deasserted"]
    #[inline(always)]
    pub fn ss_deasserted(&mut self) -> SS_DEASSERTED_W {
        SS_DEASSERTED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Slave Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
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
