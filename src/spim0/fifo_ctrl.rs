#[doc = "Register `FIFO_CTRL` reader"]
pub struct R(crate::R<FIFO_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FIFO_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FIFO_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FIFO_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FIFO_CTRL` writer"]
pub struct W(crate::W<FIFO_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FIFO_CTRL_SPEC>;
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
impl From<crate::W<FIFO_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FIFO_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `tx_fifo_ae_lvl` reader - Transaction FIFO AE Level"]
pub struct TX_FIFO_AE_LVL_R(crate::FieldReader<u8>);
impl TX_FIFO_AE_LVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_AE_LVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_AE_LVL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_ae_lvl` writer - Transaction FIFO AE Level"]
pub struct TX_FIFO_AE_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_AE_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `tx_fifo_used` reader - Transaction FIFO Used"]
pub struct TX_FIFO_USED_R(crate::FieldReader<u8>);
impl TX_FIFO_USED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TX_FIFO_USED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_USED_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_af_lvl` reader - Results FIFO AF Level"]
pub struct RX_FIFO_AF_LVL_R(crate::FieldReader<u8>);
impl RX_FIFO_AF_LVL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_AF_LVL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_AF_LVL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_af_lvl` writer - Results FIFO AF Level"]
pub struct RX_FIFO_AF_LVL_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_AF_LVL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 16)) | ((value as u32 & 0x1f) << 16);
        self.w
    }
}
#[doc = "Field `rx_fifo_used` reader - Results FIFO Used"]
pub struct RX_FIFO_USED_R(crate::FieldReader<u8>);
impl RX_FIFO_USED_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_USED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_USED_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - Transaction FIFO AE Level"]
    #[inline(always)]
    pub fn tx_fifo_ae_lvl(&self) -> TX_FIFO_AE_LVL_R {
        TX_FIFO_AE_LVL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Transaction FIFO Used"]
    #[inline(always)]
    pub fn tx_fifo_used(&self) -> TX_FIFO_USED_R {
        TX_FIFO_USED_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Results FIFO AF Level"]
    #[inline(always)]
    pub fn rx_fifo_af_lvl(&self) -> RX_FIFO_AF_LVL_R {
        RX_FIFO_AF_LVL_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:29 - Results FIFO Used"]
    #[inline(always)]
    pub fn rx_fifo_used(&self) -> RX_FIFO_USED_R {
        RX_FIFO_USED_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Transaction FIFO AE Level"]
    #[inline(always)]
    pub fn tx_fifo_ae_lvl(&mut self) -> TX_FIFO_AE_LVL_W {
        TX_FIFO_AE_LVL_W { w: self }
    }
    #[doc = "Bits 16:20 - Results FIFO AF Level"]
    #[inline(always)]
    pub fn rx_fifo_af_lvl(&mut self) -> RX_FIFO_AF_LVL_W {
        RX_FIFO_AF_LVL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master FIFO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fifo_ctrl](index.html) module"]
pub struct FIFO_CTRL_SPEC;
impl crate::RegisterSpec for FIFO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fifo_ctrl::R](R) reader structure"]
impl crate::Readable for FIFO_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [fifo_ctrl::W](W) writer structure"]
impl crate::Writable for FIFO_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FIFO_CTRL to value 0"]
impl crate::Resettable for FIFO_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
