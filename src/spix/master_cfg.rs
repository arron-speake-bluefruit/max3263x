#[doc = "Register `MASTER_CFG` reader"]
pub struct R(crate::R<MASTER_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASTER_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASTER_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASTER_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASTER_CFG` writer"]
pub struct W(crate::W<MASTER_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASTER_CFG_SPEC>;
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
impl From<crate::W<MASTER_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASTER_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "SPIX Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI_MODE_A {
    #[doc = "0: SCK is active high, data is sampled on clock rising edge."]
    SCK_HI_SAMPLE_RISING = 0,
    #[doc = "3: SCK is active low, data is sampled on clock rising edge."]
    SCK_LO_SAMPLE_FALLING = 3,
}
impl From<SPI_MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI_MODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `spi_mode` reader - SPIX Mode"]
pub struct SPI_MODE_R(crate::FieldReader<u8, SPI_MODE_A>);
impl SPI_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPI_MODE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI_MODE_A> {
        match self.bits {
            0 => Some(SPI_MODE_A::SCK_HI_SAMPLE_RISING),
            3 => Some(SPI_MODE_A::SCK_LO_SAMPLE_FALLING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SCK_HI_SAMPLE_RISING`"]
    #[inline(always)]
    pub fn is_sck_hi_sample_rising(&self) -> bool {
        **self == SPI_MODE_A::SCK_HI_SAMPLE_RISING
    }
    #[doc = "Checks if the value of the field is `SCK_LO_SAMPLE_FALLING`"]
    #[inline(always)]
    pub fn is_sck_lo_sample_falling(&self) -> bool {
        **self == SPI_MODE_A::SCK_LO_SAMPLE_FALLING
    }
}
impl core::ops::Deref for SPI_MODE_R {
    type Target = crate::FieldReader<u8, SPI_MODE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spi_mode` writer - SPIX Mode"]
pub struct SPI_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI_MODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI_MODE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SCK is active high, data is sampled on clock rising edge."]
    #[inline(always)]
    pub fn sck_hi_sample_rising(self) -> &'a mut W {
        self.variant(SPI_MODE_A::SCK_HI_SAMPLE_RISING)
    }
    #[doc = "SCK is active low, data is sampled on clock rising edge."]
    #[inline(always)]
    pub fn sck_lo_sample_falling(self) -> &'a mut W {
        self.variant(SPI_MODE_A::SCK_LO_SAMPLE_FALLING)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !3) | (value as u32 & 3);
        self.w
    }
}
#[doc = "SPIX Slave Select Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_ACT_LO_A {
    #[doc = "0: Enabled slave select (SS) is active high."]
    ACTIVE_HIGH = 0,
    #[doc = "1: Enabled slave select (SS) is active low."]
    ACTIVE_LOW = 1,
}
impl From<SS_ACT_LO_A> for bool {
    #[inline(always)]
    fn from(variant: SS_ACT_LO_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ss_act_lo` reader - SPIX Slave Select Polarity"]
pub struct SS_ACT_LO_R(crate::FieldReader<bool, SS_ACT_LO_A>);
impl SS_ACT_LO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_ACT_LO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_ACT_LO_A {
        match self.bits {
            false => SS_ACT_LO_A::ACTIVE_HIGH,
            true => SS_ACT_LO_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        **self == SS_ACT_LO_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        **self == SS_ACT_LO_A::ACTIVE_LOW
    }
}
impl core::ops::Deref for SS_ACT_LO_R {
    type Target = crate::FieldReader<bool, SS_ACT_LO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_act_lo` writer - SPIX Slave Select Polarity"]
pub struct SS_ACT_LO_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_ACT_LO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_ACT_LO_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Enabled slave select (SS) is active high."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(SS_ACT_LO_A::ACTIVE_HIGH)
    }
    #[doc = "Enabled slave select (SS) is active low."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(SS_ACT_LO_A::ACTIVE_LOW)
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
#[doc = "Alternate Timing Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ALT_TIMING_EN_A {
    #[doc = "0: Alternate timing is disabled."]
    DISABLED = 0,
    #[doc = "1: Alternate timing will be enabled automatically when needed."]
    ENABLED_AS_NEEDED = 1,
}
impl From<ALT_TIMING_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ALT_TIMING_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `alt_timing_en` reader - Alternate Timing Mode Enable"]
pub struct ALT_TIMING_EN_R(crate::FieldReader<bool, ALT_TIMING_EN_A>);
impl ALT_TIMING_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ALT_TIMING_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALT_TIMING_EN_A {
        match self.bits {
            false => ALT_TIMING_EN_A::DISABLED,
            true => ALT_TIMING_EN_A::ENABLED_AS_NEEDED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == ALT_TIMING_EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED_AS_NEEDED`"]
    #[inline(always)]
    pub fn is_enabled_as_needed(&self) -> bool {
        **self == ALT_TIMING_EN_A::ENABLED_AS_NEEDED
    }
}
impl core::ops::Deref for ALT_TIMING_EN_R {
    type Target = crate::FieldReader<bool, ALT_TIMING_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `alt_timing_en` writer - Alternate Timing Mode Enable"]
pub struct ALT_TIMING_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_TIMING_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ALT_TIMING_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Alternate timing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALT_TIMING_EN_A::DISABLED)
    }
    #[doc = "Alternate timing will be enabled automatically when needed."]
    #[inline(always)]
    pub fn enabled_as_needed(self) -> &'a mut W {
        self.variant(ALT_TIMING_EN_A::ENABLED_AS_NEEDED)
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `slave_sel` reader - SPIX Slave Select"]
pub struct SLAVE_SEL_R(crate::FieldReader<u8, u8>);
impl SLAVE_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slave_sel` writer - SPIX Slave Select"]
pub struct SLAVE_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 4)) | ((value as u32 & 7) << 4);
        self.w
    }
}
#[doc = "Field `sck_hi_clk` reader - SCK High Clocks"]
pub struct SCK_HI_CLK_R(crate::FieldReader<u8, u8>);
impl SCK_HI_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCK_HI_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_HI_CLK_R {
    type Target = crate::FieldReader<u8, u8>;
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
pub struct SCK_LO_CLK_R(crate::FieldReader<u8, u8>);
impl SCK_LO_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SCK_LO_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCK_LO_CLK_R {
    type Target = crate::FieldReader<u8, u8>;
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
#[doc = "SS Active Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ACT_DELAY_A {
    #[doc = "0: No SS Active timing delay enabled."]
    OFF = 0,
    #[doc = "1: SS Active timing delay of 2 SPIX module clock cycles."]
    FOR_2_MOD_CLK = 1,
    #[doc = "2: SS Active timing delay of 4 SPIX module clock cycles."]
    FOR_4_MOD_CLK = 2,
    #[doc = "3: SS Active timing delay of 8 SPIX module clock cycles."]
    FOR_8_MOD_CLK = 3,
}
impl From<ACT_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: ACT_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `act_delay` reader - SS Active Timing"]
pub struct ACT_DELAY_R(crate::FieldReader<u8, ACT_DELAY_A>);
impl ACT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ACT_DELAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACT_DELAY_A {
        match self.bits {
            0 => ACT_DELAY_A::OFF,
            1 => ACT_DELAY_A::FOR_2_MOD_CLK,
            2 => ACT_DELAY_A::FOR_4_MOD_CLK,
            3 => ACT_DELAY_A::FOR_8_MOD_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == ACT_DELAY_A::OFF
    }
    #[doc = "Checks if the value of the field is `FOR_2_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_2_mod_clk(&self) -> bool {
        **self == ACT_DELAY_A::FOR_2_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_4_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_4_mod_clk(&self) -> bool {
        **self == ACT_DELAY_A::FOR_4_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_8_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_8_mod_clk(&self) -> bool {
        **self == ACT_DELAY_A::FOR_8_MOD_CLK
    }
}
impl core::ops::Deref for ACT_DELAY_R {
    type Target = crate::FieldReader<u8, ACT_DELAY_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ACT_DELAY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No SS Active timing delay enabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(ACT_DELAY_A::OFF)
    }
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    #[inline(always)]
    pub fn for_2_mod_clk(self) -> &'a mut W {
        self.variant(ACT_DELAY_A::FOR_2_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    #[inline(always)]
    pub fn for_4_mod_clk(self) -> &'a mut W {
        self.variant(ACT_DELAY_A::FOR_4_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    #[inline(always)]
    pub fn for_8_mod_clk(self) -> &'a mut W {
        self.variant(ACT_DELAY_A::FOR_8_MOD_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 16)) | ((value as u32 & 3) << 16);
        self.w
    }
}
#[doc = "SS Inactive Timing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INACT_DELAY_A {
    #[doc = "0: No SS Active timing delay enabled."]
    OFF = 0,
    #[doc = "1: SS Active timing delay of 2 SPIX module clock cycles."]
    FOR_2_MOD_CLK = 1,
    #[doc = "2: SS Active timing delay of 4 SPIX module clock cycles."]
    FOR_4_MOD_CLK = 2,
    #[doc = "3: SS Active timing delay of 8 SPIX module clock cycles."]
    FOR_8_MOD_CLK = 3,
}
impl From<INACT_DELAY_A> for u8 {
    #[inline(always)]
    fn from(variant: INACT_DELAY_A) -> Self {
        variant as _
    }
}
#[doc = "Field `inact_delay` reader - SS Inactive Timing"]
pub struct INACT_DELAY_R(crate::FieldReader<u8, INACT_DELAY_A>);
impl INACT_DELAY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        INACT_DELAY_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INACT_DELAY_A {
        match self.bits {
            0 => INACT_DELAY_A::OFF,
            1 => INACT_DELAY_A::FOR_2_MOD_CLK,
            2 => INACT_DELAY_A::FOR_4_MOD_CLK,
            3 => INACT_DELAY_A::FOR_8_MOD_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == INACT_DELAY_A::OFF
    }
    #[doc = "Checks if the value of the field is `FOR_2_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_2_mod_clk(&self) -> bool {
        **self == INACT_DELAY_A::FOR_2_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_4_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_4_mod_clk(&self) -> bool {
        **self == INACT_DELAY_A::FOR_4_MOD_CLK
    }
    #[doc = "Checks if the value of the field is `FOR_8_MOD_CLK`"]
    #[inline(always)]
    pub fn is_for_8_mod_clk(&self) -> bool {
        **self == INACT_DELAY_A::FOR_8_MOD_CLK
    }
}
impl core::ops::Deref for INACT_DELAY_R {
    type Target = crate::FieldReader<u8, INACT_DELAY_A>;
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
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INACT_DELAY_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "No SS Active timing delay enabled."]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(INACT_DELAY_A::OFF)
    }
    #[doc = "SS Active timing delay of 2 SPIX module clock cycles."]
    #[inline(always)]
    pub fn for_2_mod_clk(self) -> &'a mut W {
        self.variant(INACT_DELAY_A::FOR_2_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 4 SPIX module clock cycles."]
    #[inline(always)]
    pub fn for_4_mod_clk(self) -> &'a mut W {
        self.variant(INACT_DELAY_A::FOR_4_MOD_CLK)
    }
    #[doc = "SS Active timing delay of 8 SPIX module clock cycles."]
    #[inline(always)]
    pub fn for_8_mod_clk(self) -> &'a mut W {
        self.variant(INACT_DELAY_A::FOR_8_MOD_CLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 18)) | ((value as u32 & 3) << 18);
        self.w
    }
}
#[doc = "Field `alt_sck_hi_clk` reader - Alt SCK High Clocks"]
pub struct ALT_SCK_HI_CLK_R(crate::FieldReader<u8, u8>);
impl ALT_SCK_HI_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ALT_SCK_HI_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALT_SCK_HI_CLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `alt_sck_hi_clk` writer - Alt SCK High Clocks"]
pub struct ALT_SCK_HI_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_SCK_HI_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `alt_sck_lo_clk` reader - Alt SCK Low Clocks"]
pub struct ALT_SCK_LO_CLK_R(crate::FieldReader<u8, u8>);
impl ALT_SCK_LO_CLK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ALT_SCK_LO_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALT_SCK_LO_CLK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `alt_sck_lo_clk` writer - Alt SCK Low Clocks"]
pub struct ALT_SCK_LO_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> ALT_SCK_LO_CLK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - SPIX Mode"]
    #[inline(always)]
    pub fn spi_mode(&self) -> SPI_MODE_R {
        SPI_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - SPIX Slave Select Polarity"]
    #[inline(always)]
    pub fn ss_act_lo(&self) -> SS_ACT_LO_R {
        SS_ACT_LO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Alternate Timing Mode Enable"]
    #[inline(always)]
    pub fn alt_timing_en(&self) -> ALT_TIMING_EN_R {
        ALT_TIMING_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - SPIX Slave Select"]
    #[inline(always)]
    pub fn slave_sel(&self) -> SLAVE_SEL_R {
        SLAVE_SEL_R::new(((self.bits >> 4) & 7) as u8)
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
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline(always)]
    pub fn alt_sck_hi_clk(&self) -> ALT_SCK_HI_CLK_R {
        ALT_SCK_HI_CLK_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline(always)]
    pub fn alt_sck_lo_clk(&self) -> ALT_SCK_LO_CLK_R {
        ALT_SCK_LO_CLK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPIX Mode"]
    #[inline(always)]
    pub fn spi_mode(&mut self) -> SPI_MODE_W {
        SPI_MODE_W { w: self }
    }
    #[doc = "Bit 2 - SPIX Slave Select Polarity"]
    #[inline(always)]
    pub fn ss_act_lo(&mut self) -> SS_ACT_LO_W {
        SS_ACT_LO_W { w: self }
    }
    #[doc = "Bit 3 - Alternate Timing Mode Enable"]
    #[inline(always)]
    pub fn alt_timing_en(&mut self) -> ALT_TIMING_EN_W {
        ALT_TIMING_EN_W { w: self }
    }
    #[doc = "Bits 4:6 - SPIX Slave Select"]
    #[inline(always)]
    pub fn slave_sel(&mut self) -> SLAVE_SEL_W {
        SLAVE_SEL_W { w: self }
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
    #[doc = "Bits 20:23 - Alt SCK High Clocks"]
    #[inline(always)]
    pub fn alt_sck_hi_clk(&mut self) -> ALT_SCK_HI_CLK_W {
        ALT_SCK_HI_CLK_W { w: self }
    }
    #[doc = "Bits 24:27 - Alt SCK Low Clocks"]
    #[inline(always)]
    pub fn alt_sck_lo_clk(&mut self) -> ALT_SCK_LO_CLK_W {
        ALT_SCK_LO_CLK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIX Master Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master_cfg](index.html) module"]
pub struct MASTER_CFG_SPEC;
impl crate::RegisterSpec for MASTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [master_cfg::R](R) reader structure"]
impl crate::Readable for MASTER_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [master_cfg::W](W) writer structure"]
impl crate::Writable for MASTER_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MASTER_CFG to value 0"]
impl crate::Resettable for MASTER_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
