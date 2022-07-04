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
#[doc = "Field `tx_stalled` reader - Transaction Stalled Int Enable"]
pub type TX_STALLED_R = crate::BitReader<bool>;
#[doc = "Field `tx_stalled` writer - Transaction Stalled Int Enable"]
pub type TX_STALLED_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 0>;
#[doc = "Field `rx_stalled` reader - Results Stalled Int Enable"]
pub type RX_STALLED_R = crate::BitReader<bool>;
#[doc = "Field `rx_stalled` writer - Results Stalled Int Enable"]
pub type RX_STALLED_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 1>;
#[doc = "Field `tx_ready` reader - Transaction Ready Int Enable"]
pub type TX_READY_R = crate::BitReader<bool>;
#[doc = "Field `tx_ready` writer - Transaction Ready Int Enable"]
pub type TX_READY_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 2>;
#[doc = "Field `rx_done` reader - Results Done Int Enable"]
pub type RX_DONE_R = crate::BitReader<bool>;
#[doc = "Field `rx_done` writer - Results Done Int Enable"]
pub type RX_DONE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 3>;
#[doc = "Field `tx_fifo_ae` reader - TXFIFO Almost Empty Int Enable"]
pub type TX_FIFO_AE_R = crate::BitReader<bool>;
#[doc = "Field `tx_fifo_ae` writer - TXFIFO Almost Empty Int Enable"]
pub type TX_FIFO_AE_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 4>;
#[doc = "Field `rx_fifo_af` reader - RXFIFO Almost Full Int Enable"]
pub type RX_FIFO_AF_R = crate::BitReader<bool>;
#[doc = "Field `rx_fifo_af` writer - RXFIFO Almost Full Int Enable"]
pub type RX_FIFO_AF_W<'a> = crate::BitWriter<'a, u32, INTEN_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Transaction Stalled Int Enable"]
    #[inline(always)]
    pub fn tx_stalled(&self) -> TX_STALLED_R {
        TX_STALLED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Results Stalled Int Enable"]
    #[inline(always)]
    pub fn rx_stalled(&self) -> RX_STALLED_R {
        RX_STALLED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transaction Ready Int Enable"]
    #[inline(always)]
    pub fn tx_ready(&self) -> TX_READY_R {
        TX_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Results Done Int Enable"]
    #[inline(always)]
    pub fn rx_done(&self) -> RX_DONE_R {
        RX_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXFIFO Almost Empty Int Enable"]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXFIFO Almost Full Int Enable"]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transaction Stalled Int Enable"]
    #[inline(always)]
    pub fn tx_stalled(&mut self) -> TX_STALLED_W {
        TX_STALLED_W::new(self)
    }
    #[doc = "Bit 1 - Results Stalled Int Enable"]
    #[inline(always)]
    pub fn rx_stalled(&mut self) -> RX_STALLED_W {
        RX_STALLED_W::new(self)
    }
    #[doc = "Bit 2 - Transaction Ready Int Enable"]
    #[inline(always)]
    pub fn tx_ready(&mut self) -> TX_READY_W {
        TX_READY_W::new(self)
    }
    #[doc = "Bit 3 - Results Done Int Enable"]
    #[inline(always)]
    pub fn rx_done(&mut self) -> RX_DONE_W {
        RX_DONE_W::new(self)
    }
    #[doc = "Bit 4 - TXFIFO Almost Empty Int Enable"]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W::new(self)
    }
    #[doc = "Bit 5 - RXFIFO Almost Full Int Enable"]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Interrupt Enable/Disable Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
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
