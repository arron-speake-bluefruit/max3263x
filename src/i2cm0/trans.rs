#[doc = "Register `TRANS` reader"]
pub struct R(crate::R<TRANS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRANS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRANS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRANS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRANS` writer"]
pub struct W(crate::W<TRANS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRANS_SPEC>;
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
impl From<crate::W<TRANS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRANS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_start` reader - Start Transaction"]
pub struct TX_START_R(crate::FieldReader<bool, bool>);
impl TX_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_start` writer - Start Transaction"]
pub struct TX_START_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_START_W<'a> {
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
#[doc = "Field `tx_in_progress` reader - Transaction In Progress"]
pub struct TX_IN_PROGRESS_R(crate::FieldReader<bool, bool>);
impl TX_IN_PROGRESS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_IN_PROGRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_IN_PROGRESS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_done` reader - Transaction Done"]
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
#[doc = "Field `tx_nacked` reader - Transaction Nacked"]
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
#[doc = "Field `tx_lost_arbitr` reader - Transaction Lost Arbitration"]
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
#[doc = "Field `tx_timeout` reader - Transaction Timed Out"]
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
impl R {
    #[doc = "Bit 0 - Start Transaction"]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transaction In Progress"]
    #[inline(always)]
    pub fn tx_in_progress(&self) -> TX_IN_PROGRESS_R {
        TX_IN_PROGRESS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Done"]
    #[inline(always)]
    pub fn tx_done(&self) -> TX_DONE_R {
        TX_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transaction Nacked"]
    #[inline(always)]
    pub fn tx_nacked(&self) -> TX_NACKED_R {
        TX_NACKED_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transaction Lost Arbitration"]
    #[inline(always)]
    pub fn tx_lost_arbitr(&self) -> TX_LOST_ARBITR_R {
        TX_LOST_ARBITR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transaction Timed Out"]
    #[inline(always)]
    pub fn tx_timeout(&self) -> TX_TIMEOUT_R {
        TX_TIMEOUT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start Transaction"]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W {
        TX_START_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2C Master Transaction Start and Status Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trans](index.html) module"]
pub struct TRANS_SPEC;
impl crate::RegisterSpec for TRANS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trans::R](R) reader structure"]
impl crate::Readable for TRANS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trans::W](W) writer structure"]
impl crate::Writable for TRANS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRANS to value 0"]
impl crate::Resettable for TRANS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
