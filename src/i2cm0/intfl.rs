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
#[doc = "Field `tx_done` reader - Transaction Done Int Status"]
pub struct TX_DONE_R(crate::FieldReader<bool, bool>);
impl TX_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_done` writer - Transaction Done Int Status"]
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
#[doc = "Field `tx_nacked` reader - Transaction NACKed Int Status"]
pub struct TX_NACKED_R(crate::FieldReader<bool, bool>);
impl TX_NACKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_NACKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_NACKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_nacked` writer - Transaction NACKed Int Status"]
pub struct TX_NACKED_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_NACKED_W<'a> {
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
#[doc = "Field `tx_lost_arbitr` reader - Transaction Lost Arbitration Int Status"]
pub struct TX_LOST_ARBITR_R(crate::FieldReader<bool, bool>);
impl TX_LOST_ARBITR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_LOST_ARBITR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_LOST_ARBITR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_lost_arbitr` writer - Transaction Lost Arbitration Int Status"]
pub struct TX_LOST_ARBITR_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LOST_ARBITR_W<'a> {
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
#[doc = "Field `tx_timeout` reader - Transaction Timed Out Int Status"]
pub struct TX_TIMEOUT_R(crate::FieldReader<bool, bool>);
impl TX_TIMEOUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_TIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_TIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_timeout` writer - Transaction Timed Out Int Status"]
pub struct TX_TIMEOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_TIMEOUT_W<'a> {
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
#[doc = "Field `tx_fifo_empty` reader - Transaction FIFO Empty Int Status"]
pub struct TX_FIFO_EMPTY_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_empty` writer - Transaction FIFO Empty Int Status"]
pub struct TX_FIFO_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_EMPTY_W<'a> {
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
#[doc = "Field `tx_fifo_3q_empty` reader - Transaction FIFO 3Q Empty Int Status"]
pub struct TX_FIFO_3Q_EMPTY_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_3Q_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_3Q_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_3Q_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_3q_empty` writer - Transaction FIFO 3Q Empty Int Status"]
pub struct TX_FIFO_3Q_EMPTY_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_3Q_EMPTY_W<'a> {
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
#[doc = "Field `rx_fifo_not_empty` reader - Results FIFO Not Empty Int Status"]
pub struct RX_FIFO_NOT_EMPTY_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_NOT_EMPTY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_NOT_EMPTY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_NOT_EMPTY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_not_empty` writer - Results FIFO Not Empty Int Status"]
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `rx_fifo_2q_full` reader - Results FIFO 2Q Full Int Status"]
pub struct RX_FIFO_2Q_FULL_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_2Q_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_2Q_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_2Q_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_2q_full` writer - Results FIFO 2Q Full Int Status"]
pub struct RX_FIFO_2Q_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_2Q_FULL_W<'a> {
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
#[doc = "Field `rx_fifo_3q_full` reader - Results FIFO 3Q Full Int Status"]
pub struct RX_FIFO_3Q_FULL_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_3Q_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_3Q_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_3Q_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_3q_full` writer - Results FIFO 3Q Full Int Status"]
pub struct RX_FIFO_3Q_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_3Q_FULL_W<'a> {
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
#[doc = "Field `rx_fifo_full` reader - Results FIFO Full Int Status"]
pub struct RX_FIFO_FULL_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_FULL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_FULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_FULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_full` writer - Results FIFO Full Int Status"]
pub struct RX_FIFO_FULL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_FULL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Transaction Done Int Status"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transaction NACKed Int Status"]
    #[inline(always)]
    pub fn tx_nacked(&self) -> TX_NACKED_R {
        TX_NACKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Lost Arbitration Int Status"]
    #[inline(always)]
    pub fn tx_lost_arbitr(&self) -> TX_LOST_ARBITR_R {
        TX_LOST_ARBITR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transaction Timed Out Int Status"]
    #[inline(always)]
    pub fn tx_timeout(&self) -> TX_TIMEOUT_R {
        TX_TIMEOUT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transaction FIFO Empty Int Status"]
    #[inline(always)]
    pub fn tx_fifo_empty(&self) -> TX_FIFO_EMPTY_R {
        TX_FIFO_EMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transaction FIFO 3Q Empty Int Status"]
    #[inline(always)]
    pub fn tx_fifo_3q_empty(&self) -> TX_FIFO_3Q_EMPTY_R {
        TX_FIFO_3Q_EMPTY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Results FIFO Not Empty Int Status"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&self) -> RX_FIFO_NOT_EMPTY_R {
        RX_FIFO_NOT_EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Results FIFO 2Q Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_2q_full(&self) -> RX_FIFO_2Q_FULL_R {
        RX_FIFO_2Q_FULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Results FIFO 3Q Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_3q_full(&self) -> RX_FIFO_3Q_FULL_R {
        RX_FIFO_3Q_FULL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Results FIFO Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_full(&self) -> RX_FIFO_FULL_R {
        RX_FIFO_FULL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Done Int Status"]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TX_DONE_W {
        TX_DONE_W { w: self }
    }
    #[doc = "Bit 1 - Transaction NACKed Int Status"]
    #[inline(always)]
    pub fn tx_nacked(&mut self) -> TX_NACKED_W {
        TX_NACKED_W { w: self }
    }
    #[doc = "Bit 2 - Transaction Lost Arbitration Int Status"]
    #[inline(always)]
    pub fn tx_lost_arbitr(&mut self) -> TX_LOST_ARBITR_W {
        TX_LOST_ARBITR_W { w: self }
    }
    #[doc = "Bit 3 - Transaction Timed Out Int Status"]
    #[inline(always)]
    pub fn tx_timeout(&mut self) -> TX_TIMEOUT_W {
        TX_TIMEOUT_W { w: self }
    }
    #[doc = "Bit 4 - Transaction FIFO Empty Int Status"]
    #[inline(always)]
    pub fn tx_fifo_empty(&mut self) -> TX_FIFO_EMPTY_W {
        TX_FIFO_EMPTY_W { w: self }
    }
    #[doc = "Bit 5 - Transaction FIFO 3Q Empty Int Status"]
    #[inline(always)]
    pub fn tx_fifo_3q_empty(&mut self) -> TX_FIFO_3Q_EMPTY_W {
        TX_FIFO_3Q_EMPTY_W { w: self }
    }
    #[doc = "Bit 6 - Results FIFO Not Empty Int Status"]
    #[inline(always)]
    pub fn rx_fifo_not_empty(&mut self) -> RX_FIFO_NOT_EMPTY_W {
        RX_FIFO_NOT_EMPTY_W { w: self }
    }
    #[doc = "Bit 7 - Results FIFO 2Q Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_2q_full(&mut self) -> RX_FIFO_2Q_FULL_W {
        RX_FIFO_2Q_FULL_W { w: self }
    }
    #[doc = "Bit 8 - Results FIFO 3Q Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_3q_full(&mut self) -> RX_FIFO_3Q_FULL_W {
        RX_FIFO_3Q_FULL_W { w: self }
    }
    #[doc = "Bit 9 - Results FIFO Full Int Status"]
    #[inline(always)]
    pub fn rx_fifo_full(&mut self) -> RX_FIFO_FULL_W {
        RX_FIFO_FULL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intfl](index.html) module"]
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
