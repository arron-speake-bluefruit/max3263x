#[doc = "Register `GEN_CTRL` reader"]
pub struct R(crate::R<GEN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN_CTRL` writer"]
pub struct W(crate::W<GEN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN_CTRL_SPEC>;
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
impl From<crate::W<GEN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPI Slave Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SPI_SLAVE_EN_A {
    #[doc = "0: Disable SPI Slave"]
    DISABLED = 0,
    #[doc = "1: Enable SPI Slave"]
    ENABLED = 1,
}
impl From<SPI_SLAVE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: SPI_SLAVE_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `spi_slave_en` reader - SPI Slave Enable"]
pub struct SPI_SLAVE_EN_R(crate::FieldReader<bool>);
impl SPI_SLAVE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SPI_SLAVE_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPI_SLAVE_EN_A {
        match self.bits {
            false => SPI_SLAVE_EN_A::DISABLED,
            true => SPI_SLAVE_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SPI_SLAVE_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SPI_SLAVE_EN_A::ENABLED
    }
}
impl core::ops::Deref for SPI_SLAVE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_slave_en` writer - SPI Slave Enable"]
pub struct SPI_SLAVE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_SLAVE_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_SLAVE_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable SPI Slave"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPI_SLAVE_EN_A::DISABLED)
    }
    #[doc = "Enable SPI Slave"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPI_SLAVE_EN_A::ENABLED)
    }
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
#[doc = "TX FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_EN_A {
    #[doc = "0: Disable SPI Slave TX FIFO"]
    DISABLED = 0,
    #[doc = "1: Enable SPI Slave TX FIFO"]
    ENABLED = 1,
}
impl From<TX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tx_fifo_en` reader - TX FIFO Enable"]
pub struct TX_FIFO_EN_R(crate::FieldReader<bool>);
impl TX_FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_EN_A {
        match self.bits {
            false => TX_FIFO_EN_A::DISABLED,
            true => TX_FIFO_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TX_FIFO_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TX_FIFO_EN_A::ENABLED
    }
}
impl core::ops::Deref for TX_FIFO_EN_R {
    type Target = crate::FieldReader<bool>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable SPI Slave TX FIFO"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_FIFO_EN_A::DISABLED)
    }
    #[doc = "Enable SPI Slave TX FIFO"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_FIFO_EN_A::ENABLED)
    }
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
#[doc = "SPI RX FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_EN_A {
    #[doc = "0: Disable SPI Slave RX FIFO"]
    DISABLED = 0,
    #[doc = "1: Enable SPI Slave RX FIFO"]
    ENABLED = 1,
}
impl From<RX_FIFO_EN_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rx_fifo_en` reader - SPI RX FIFO Enable"]
pub struct RX_FIFO_EN_R(crate::FieldReader<bool>);
impl RX_FIFO_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_EN_A {
        match self.bits {
            false => RX_FIFO_EN_A::DISABLED,
            true => RX_FIFO_EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RX_FIFO_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RX_FIFO_EN_A::ENABLED
    }
}
impl core::ops::Deref for RX_FIFO_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_en` writer - SPI RX FIFO Enable"]
pub struct RX_FIFO_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable SPI Slave RX FIFO"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RX_FIFO_EN_A::DISABLED)
    }
    #[doc = "Enable SPI Slave RX FIFO"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RX_FIFO_EN_A::ENABLED)
    }
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
#[doc = "Width of SPI Slave Data Transfers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DATA_WIDTH_A {
    #[doc = "0: 1-bit Wide"]
    X1 = 0,
    #[doc = "1: 2-bit Wide/Dual"]
    X2 = 1,
    #[doc = "2: 4-bit Wide/Quad"]
    X4 = 2,
    #[doc = "3: Reserved for future use. Do not use."]
    INVALID = 3,
}
impl From<DATA_WIDTH_A> for u8 {
    #[inline(always)]
    fn from(variant: DATA_WIDTH_A) -> Self {
        variant as _
    }
}
#[doc = "Field `data_width` reader - Width of SPI Slave Data Transfers"]
pub struct DATA_WIDTH_R(crate::FieldReader<u8>);
impl DATA_WIDTH_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DATA_WIDTH_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATA_WIDTH_A {
        match self.bits {
            0 => DATA_WIDTH_A::X1,
            1 => DATA_WIDTH_A::X2,
            2 => DATA_WIDTH_A::X4,
            3 => DATA_WIDTH_A::INVALID,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `X1`"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        **self == DATA_WIDTH_A::X1
    }
    #[doc = "Checks if the value of the field is `X2`"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        **self == DATA_WIDTH_A::X2
    }
    #[doc = "Checks if the value of the field is `X4`"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        **self == DATA_WIDTH_A::X4
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        **self == DATA_WIDTH_A::INVALID
    }
}
impl core::ops::Deref for DATA_WIDTH_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `data_width` writer - Width of SPI Slave Data Transfers"]
pub struct DATA_WIDTH_W<'a> {
    w: &'a mut W,
}
impl<'a> DATA_WIDTH_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DATA_WIDTH_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "1-bit Wide"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::X1)
    }
    #[doc = "2-bit Wide/Dual"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::X2)
    }
    #[doc = "4-bit Wide/Quad"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::X4)
    }
    #[doc = "Reserved for future use. Do not use."]
    #[inline(always)]
    pub fn invalid(self) -> &'a mut W {
        self.variant(DATA_WIDTH_A::INVALID)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 4)) | ((value as u32 & 3) << 4);
        self.w
    }
}
#[doc = "Field `spi_mode` reader - Defines Clock Polarity (bit 17) and Clock Phase (bit 16), collectively referred to as SPI Mode."]
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
#[doc = "Field `spi_mode` writer - Defines Clock Polarity (bit 17) and Clock Phase (bit 16), collectively referred to as SPI Mode."]
pub struct SPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "Invert TX Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_CLK_INVERT_A {
    #[doc = "0: No Effect"]
    NO_EFFECT = 0,
    #[doc = "1: Inverts the TX transmit clock such that outgoing data is updated on the opposite clock edge from that specified by spi_mode. Effectively, this inverts the value of the Clock Polarity bit from the value specified in spi_mode."]
    INVERT = 1,
}
impl From<TX_CLK_INVERT_A> for bool {
    #[inline(always)]
    fn from(variant: TX_CLK_INVERT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tx_clk_invert` reader - Invert TX Clock"]
pub struct TX_CLK_INVERT_R(crate::FieldReader<bool>);
impl TX_CLK_INVERT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_CLK_INVERT_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_CLK_INVERT_A {
        match self.bits {
            false => TX_CLK_INVERT_A::NO_EFFECT,
            true => TX_CLK_INVERT_A::INVERT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        **self == TX_CLK_INVERT_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `INVERT`"]
    #[inline(always)]
    pub fn is_invert(&self) -> bool {
        **self == TX_CLK_INVERT_A::INVERT
    }
}
impl core::ops::Deref for TX_CLK_INVERT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_clk_invert` writer - Invert TX Clock"]
pub struct TX_CLK_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CLK_INVERT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_CLK_INVERT_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "No Effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(TX_CLK_INVERT_A::NO_EFFECT)
    }
    #[doc = "Inverts the TX transmit clock such that outgoing data is updated on the opposite clock edge from that specified by spi_mode. Effectively, this inverts the value of the Clock Polarity bit from the value specified in spi_mode."]
    #[inline(always)]
    pub fn invert(self) -> &'a mut W {
        self.variant(TX_CLK_INVERT_A::INVERT)
    }
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `disable_parking` reader - Disable automatic resetting of SPI Slave on exit from LP Modes"]
pub struct DISABLE_PARKING_R(crate::FieldReader<bool>);
impl DISABLE_PARKING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DISABLE_PARKING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_PARKING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `disable_parking` writer - Disable automatic resetting of SPI Slave on exit from LP Modes"]
pub struct DISABLE_PARKING_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_PARKING_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - SPI Slave Enable"]
    #[inline(always)]
    pub fn spi_slave_en(&self) -> SPI_SLAVE_EN_R {
        SPI_SLAVE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&self) -> TX_FIFO_EN_R {
        TX_FIFO_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SPI RX FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&self) -> RX_FIFO_EN_R {
        RX_FIFO_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Width of SPI Slave Data Transfers"]
    #[inline(always)]
    pub fn data_width(&self) -> DATA_WIDTH_R {
        DATA_WIDTH_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Defines Clock Polarity (bit 17) and Clock Phase (bit 16), collectively referred to as SPI Mode."]
    #[inline(always)]
    pub fn spi_mode(&self) -> SPI_MODE_R {
        SPI_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Invert TX Clock"]
    #[inline(always)]
    pub fn tx_clk_invert(&self) -> TX_CLK_INVERT_R {
        TX_CLK_INVERT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 31 - Disable automatic resetting of SPI Slave on exit from LP Modes"]
    #[inline(always)]
    pub fn disable_parking(&self) -> DISABLE_PARKING_R {
        DISABLE_PARKING_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SPI Slave Enable"]
    #[inline(always)]
    pub fn spi_slave_en(&mut self) -> SPI_SLAVE_EN_W {
        SPI_SLAVE_EN_W { w: self }
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline(always)]
    pub fn tx_fifo_en(&mut self) -> TX_FIFO_EN_W {
        TX_FIFO_EN_W { w: self }
    }
    #[doc = "Bit 2 - SPI RX FIFO Enable"]
    #[inline(always)]
    pub fn rx_fifo_en(&mut self) -> RX_FIFO_EN_W {
        RX_FIFO_EN_W { w: self }
    }
    #[doc = "Bits 4:5 - Width of SPI Slave Data Transfers"]
    #[inline(always)]
    pub fn data_width(&mut self) -> DATA_WIDTH_W {
        DATA_WIDTH_W { w: self }
    }
    #[doc = "Bits 16:17 - Defines Clock Polarity (bit 17) and Clock Phase (bit 16), collectively referred to as SPI Mode."]
    #[inline(always)]
    pub fn spi_mode(&mut self) -> SPI_MODE_W {
        SPI_MODE_W { w: self }
    }
    #[doc = "Bit 20 - Invert TX Clock"]
    #[inline(always)]
    pub fn tx_clk_invert(&mut self) -> TX_CLK_INVERT_W {
        TX_CLK_INVERT_W { w: self }
    }
    #[doc = "Bit 31 - Disable automatic resetting of SPI Slave on exit from LP Modes"]
    #[inline(always)]
    pub fn disable_parking(&mut self) -> DISABLE_PARKING_W {
        DISABLE_PARKING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Slave General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen_ctrl](index.html) module"]
pub struct GEN_CTRL_SPEC;
impl crate::RegisterSpec for GEN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen_ctrl::R](R) reader structure"]
impl crate::Readable for GEN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen_ctrl::W](W) writer structure"]
impl crate::Writable for GEN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN_CTRL to value 0"]
impl crate::Resettable for GEN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
