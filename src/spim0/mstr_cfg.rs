#[doc = "Register `MSTR_CFG` reader"]
pub struct R(crate::R<MSTR_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSTR_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSTR_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSTR_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSTR_CFG` writer"]
pub struct W(crate::W<MSTR_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSTR_CFG_SPEC>;
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
impl From<crate::W<MSTR_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSTR_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slave_sel` reader - SPI Slave Select"]
pub struct SLAVE_SEL_R(crate::FieldReader<u8>);
impl SLAVE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_SEL_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slave_sel` writer - SPI Slave Select"]
pub struct SLAVE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !7) | (value as u32 & 7);
        self.w
    }
}
#[doc = "Field `three_wire_mode` reader - 3-Wire Mode"]
pub struct THREE_WIRE_MODE_R(crate::FieldReader<bool>);
impl THREE_WIRE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        THREE_WIRE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for THREE_WIRE_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `three_wire_mode` writer - 3-Wire Mode"]
pub struct THREE_WIRE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> THREE_WIRE_MODE_W<'a> {
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
#[doc = "Field `spi_mode` reader - SPI Mode"]
pub struct SPI_MODE_R(crate::FieldReader<u8>);
impl SPI_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPI_MODE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_mode` writer - SPI Mode"]
pub struct SPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `page_size` reader - Page Size"]
pub struct PAGE_SIZE_R(crate::FieldReader<u8>);
impl PAGE_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PAGE_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE_SIZE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `page_size` writer - Page Size"]
pub struct PAGE_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 6)) | ((value as u32 & 3) << 6);
        self.w
    }
}
#[doc = "Field `sck_hi_clk` reader - SCK High Clocks"]
pub struct SCK_HI_CLK_R(crate::FieldReader<u8>);
impl SCK_HI_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCK_HI_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_HI_CLK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sck_hi_clk` writer - SCK High Clocks"]
pub struct SCK_HI_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_HI_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `sck_lo_clk` reader - SCK Low Clocks"]
pub struct SCK_LO_CLK_R(crate::FieldReader<u8>);
impl SCK_LO_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCK_LO_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_LO_CLK_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sck_lo_clk` writer - SCK Low Clocks"]
pub struct SCK_LO_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> SCK_LO_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `act_delay` reader - SS Active Timing"]
pub struct ACT_DELAY_R(crate::FieldReader<u8>);
impl ACT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_DELAY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `act_delay` writer - SS Active Timing"]
pub struct ACT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Field `inact_delay` reader - SS Inactive Timing"]
pub struct INACT_DELAY_R(crate::FieldReader<u8>);
impl INACT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INACT_DELAY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INACT_DELAY_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `inact_delay` writer - SS Inactive Timing"]
pub struct INACT_DELAY_W<'a> {
    w: &'a mut W,
}
impl<'a> INACT_DELAY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `sdio_sample_point` reader - SDIO Sample Point"]
pub struct SDIO_SAMPLE_POINT_R(crate::FieldReader<u8>);
impl SDIO_SAMPLE_POINT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SDIO_SAMPLE_POINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SDIO_SAMPLE_POINT_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sdio_sample_point` writer - SDIO Sample Point"]
pub struct SDIO_SAMPLE_POINT_W<'a> {
    w: &'a mut W,
}
impl<'a> SDIO_SAMPLE_POINT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SPI Slave Select"]
    #[inline(always)]
    pub fn slave_sel(&self) -> SLAVE_SEL_R {
        SLAVE_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3-Wire Mode"]
    #[inline(always)]
    pub fn three_wire_mode(&self) -> THREE_WIRE_MODE_R {
        THREE_WIRE_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SPI_MODE_R {
        SPI_MODE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Page Size"]
    #[inline(always)]
    pub fn page_size(&self) -> PAGE_SIZE_R {
        PAGE_SIZE_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline(always)]
    pub fn sck_hi_clk(&self) -> SCK_HI_CLK_R {
        SCK_HI_CLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline(always)]
    pub fn sck_lo_clk(&self) -> SCK_LO_CLK_R {
        SCK_LO_CLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline(always)]
    pub fn act_delay(&self) -> ACT_DELAY_R {
        ACT_DELAY_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline(always)]
    pub fn inact_delay(&self) -> INACT_DELAY_R {
        INACT_DELAY_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23 - SDIO Sample Point"]
    #[inline(always)]
    pub fn sdio_sample_point(&self) -> SDIO_SAMPLE_POINT_R {
        SDIO_SAMPLE_POINT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPI Slave Select"]
    #[inline(always)]
    pub fn slave_sel(&mut self) -> SLAVE_SEL_W {
        SLAVE_SEL_W { w: self }
    }
    #[doc = "Bit 3 - 3-Wire Mode"]
    #[inline(always)]
    pub fn three_wire_mode(&mut self) -> THREE_WIRE_MODE_W {
        THREE_WIRE_MODE_W { w: self }
    }
    #[doc = "Bits 4:5 - SPI Mode"]
    #[inline(always)]
    pub fn spi_mode(&mut self) -> SPI_MODE_W {
        SPI_MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Page Size"]
    #[inline(always)]
    pub fn page_size(&mut self) -> PAGE_SIZE_W {
        PAGE_SIZE_W { w: self }
    }
    #[doc = "Bits 8:11 - SCK High Clocks"]
    #[inline(always)]
    pub fn sck_hi_clk(&mut self) -> SCK_HI_CLK_W {
        SCK_HI_CLK_W { w: self }
    }
    #[doc = "Bits 12:15 - SCK Low Clocks"]
    #[inline(always)]
    pub fn sck_lo_clk(&mut self) -> SCK_LO_CLK_W {
        SCK_LO_CLK_W { w: self }
    }
    #[doc = "Bits 16:17 - SS Active Timing"]
    #[inline(always)]
    pub fn act_delay(&mut self) -> ACT_DELAY_W {
        ACT_DELAY_W { w: self }
    }
    #[doc = "Bits 18:19 - SS Inactive Timing"]
    #[inline(always)]
    pub fn inact_delay(&mut self) -> INACT_DELAY_W {
        INACT_DELAY_W { w: self }
    }
    #[doc = "Bits 20:23 - SDIO Sample Point"]
    #[inline(always)]
    pub fn sdio_sample_point(&mut self) -> SDIO_SAMPLE_POINT_W {
        SDIO_SAMPLE_POINT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Master Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mstr_cfg](index.html) module"]
pub struct MSTR_CFG_SPEC;
impl crate::RegisterSpec for MSTR_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mstr_cfg::R](R) reader structure"]
impl crate::Readable for MSTR_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mstr_cfg::W](W) writer structure"]
impl crate::Writable for MSTR_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSTR_CFG to value 0"]
impl crate::Resettable for MSTR_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
