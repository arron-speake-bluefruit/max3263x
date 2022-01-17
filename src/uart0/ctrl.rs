#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `uart_en` reader - UART Enable"]
pub struct UART_EN_R(crate::FieldReader<bool, bool>);
impl UART_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        UART_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UART_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `uart_en` writer - UART Enable"]
pub struct UART_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_EN_W<'a> {
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
#[doc = "Field `rx_fifo_en` reader - RX FIFO Enable"]
pub struct RX_FIFO_EN_R(crate::FieldReader<bool, bool>);
impl RX_FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_en` writer - RX FIFO Enable"]
pub struct RX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_EN_W<'a> {
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
#[doc = "Field `tx_fifo_en` reader - TX FIFO Enable"]
pub struct TX_FIFO_EN_R(crate::FieldReader<bool, bool>);
impl TX_FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_FIFO_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_en` writer - TX FIFO Enable"]
pub struct TX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_EN_W<'a> {
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
#[doc = "Field `data_size` reader - Data Size"]
pub struct DATA_SIZE_R(crate::FieldReader<u8, u8>);
impl DATA_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATA_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `data_size` writer - Data Size"]
pub struct DATA_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `extra_stop` reader - Extra Stop Enable"]
pub struct EXTRA_STOP_R(crate::FieldReader<bool, bool>);
impl EXTRA_STOP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EXTRA_STOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EXTRA_STOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `extra_stop` writer - Extra Stop Enable"]
pub struct EXTRA_STOP_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTRA_STOP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `parity` reader - Parity Mode"]
pub struct PARITY_R(crate::FieldReader<u8, u8>);
impl PARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARITY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `parity` writer - Parity Mode"]
pub struct PARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> PARITY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `cts_en` reader - CTS Enable"]
pub struct CTS_EN_R(crate::FieldReader<bool, bool>);
impl CTS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cts_en` writer - CTS Enable"]
pub struct CTS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `cts_polarity` reader - CTS Polarity"]
pub struct CTS_POLARITY_R(crate::FieldReader<bool, bool>);
impl CTS_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CTS_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTS_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cts_polarity` writer - CTS Polarity"]
pub struct CTS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> CTS_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `rts_en` reader - RTS Enable"]
pub struct RTS_EN_R(crate::FieldReader<bool, bool>);
impl RTS_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rts_en` writer - RTS Enable"]
pub struct RTS_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `rts_polarity` reader - RTS Polarity"]
pub struct RTS_POLARITY_R(crate::FieldReader<bool, bool>);
impl RTS_POLARITY_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTS_POLARITY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_POLARITY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rts_polarity` writer - RTS Polarity"]
pub struct RTS_POLARITY_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_POLARITY_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `rts_level` reader - RX FIFO LTE Level for RTS Assert"]
pub struct RTS_LEVEL_R(crate::FieldReader<u8, u8>);
impl RTS_LEVEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RTS_LEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTS_LEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rts_level` writer - RX FIFO LTE Level for RTS Assert"]
pub struct RTS_LEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RTS_LEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 20)) | ((value as u32 & 0x3f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_en(&self) -> UART_EN_R {
        UART_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - TX FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 4:5 - Data Size"]
    #[inline(always)]
    pub fn data_size(&self) -> DATA_SIZE_R {
        DATA_SIZE_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 8 - Extra Stop Enable"]
    #[inline(always)]
    pub fn extra_stop(&self) -> EXTRA_STOP_R {
        EXTRA_STOP_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Parity Mode"]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bit 16 - CTS Enable"]
    #[inline(always)]
    pub fn cts_en(&self) -> CTS_EN_R {
        CTS_EN_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - CTS Polarity"]
    #[inline(always)]
    pub fn cts_polarity(&self) -> CTS_POLARITY_R {
        CTS_POLARITY_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - RTS Enable"]
    #[inline(always)]
    pub fn rts_en(&self) -> RTS_EN_R {
        RTS_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - RTS Polarity"]
    #[inline(always)]
    pub fn rts_polarity(&self) -> RTS_POLARITY_R {
        RTS_POLARITY_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:25 - RX FIFO LTE Level for RTS Assert"]
    #[inline(always)]
    pub fn rts_level(&self) -> RTS_LEVEL_R {
        RTS_LEVEL_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_en(&mut self) -> UART_EN_W {
        UART_EN_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W {
        RX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 2 - TX FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W {
        TX_FIFO_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Data Size"]
    #[inline(always)]
    pub fn data_size(&mut self) -> DATA_SIZE_W {
        DATA_SIZE_W { w: self }
    }
    #[doc = "Bit 8 - Extra Stop Enable"]
    #[inline(always)]
    pub fn extra_stop(&mut self) -> EXTRA_STOP_W {
        EXTRA_STOP_W { w: self }
    }
    #[doc = "Bits 12:13 - Parity Mode"]
    #[inline(always)]
    pub fn parity(&mut self) -> PARITY_W {
        PARITY_W { w: self }
    }
    #[doc = "Bit 16 - CTS Enable"]
    #[inline(always)]
    pub fn cts_en(&mut self) -> CTS_EN_W {
        CTS_EN_W { w: self }
    }
    #[doc = "Bit 17 - CTS Polarity"]
    #[inline(always)]
    pub fn cts_polarity(&mut self) -> CTS_POLARITY_W {
        CTS_POLARITY_W { w: self }
    }
    #[doc = "Bit 18 - RTS Enable"]
    #[inline(always)]
    pub fn rts_en(&mut self) -> RTS_EN_W {
        RTS_EN_W { w: self }
    }
    #[doc = "Bit 19 - RTS Polarity"]
    #[inline(always)]
    pub fn rts_polarity(&mut self) -> RTS_POLARITY_W {
        RTS_POLARITY_W { w: self }
    }
    #[doc = "Bits 20:25 - RX FIFO LTE Level for RTS Assert"]
    #[inline(always)]
    pub fn rts_level(&mut self) -> RTS_LEVEL_W {
        RTS_LEVEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
