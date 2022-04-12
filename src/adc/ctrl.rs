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
#[doc = "Field `cpu_adc_start` reader - Start ADC Conversion"]
pub struct CPU_ADC_START_R(crate::FieldReader<bool, bool>);
impl CPU_ADC_START_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CPU_ADC_START_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_ADC_START_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cpu_adc_start` writer - Start ADC Conversion"]
pub struct CPU_ADC_START_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_ADC_START_W<'a> {
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
#[doc = "Field `adc_pu` reader - ADC Power Up"]
pub struct ADC_PU_R(crate::FieldReader<bool, bool>);
impl ADC_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_pu` writer - ADC Power Up"]
pub struct ADC_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_PU_W<'a> {
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
#[doc = "Field `buf_pu` reader - ADC Input Buffer Power Up"]
pub struct BUF_PU_R(crate::FieldReader<bool, bool>);
impl BUF_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_pu` writer - ADC Input Buffer Power Up"]
pub struct BUF_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_PU_W<'a> {
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
#[doc = "Field `adc_refbuf_pu` reader - ADC Reference Buffer Power Up"]
pub struct ADC_REFBUF_PU_R(crate::FieldReader<bool, bool>);
impl ADC_REFBUF_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_REFBUF_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_REFBUF_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_refbuf_pu` writer - ADC Reference Buffer Power Up"]
pub struct ADC_REFBUF_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REFBUF_PU_W<'a> {
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
#[doc = "Field `adc_chgpump_pu` reader - ADC Charge Pump Power Up"]
pub struct ADC_CHGPUMP_PU_R(crate::FieldReader<bool, bool>);
impl ADC_CHGPUMP_PU_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CHGPUMP_PU_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CHGPUMP_PU_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_chgpump_pu` writer - ADC Charge Pump Power Up"]
pub struct ADC_CHGPUMP_PU_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHGPUMP_PU_W<'a> {
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
#[doc = "Field `buf_chop_dis` reader - ADC Input Buffer Chop Disable (INTERNAL ONLY)"]
pub struct BUF_CHOP_DIS_R(crate::FieldReader<bool, bool>);
impl BUF_CHOP_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_CHOP_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_CHOP_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_chop_dis` writer - ADC Input Buffer Chop Disable (INTERNAL ONLY)"]
pub struct BUF_CHOP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_CHOP_DIS_W<'a> {
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
#[doc = "Field `buf_pump_dis` reader - Disable Use of Charge Pump Output by Input Buffer (INTERNAL)"]
pub struct BUF_PUMP_DIS_R(crate::FieldReader<bool, bool>);
impl BUF_PUMP_DIS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_PUMP_DIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_PUMP_DIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_pump_dis` writer - Disable Use of Charge Pump Output by Input Buffer (INTERNAL)"]
pub struct BUF_PUMP_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_PUMP_DIS_W<'a> {
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
#[doc = "Field `buf_bypass` reader - Bypass Input Buffer"]
pub struct BUF_BYPASS_R(crate::FieldReader<bool, bool>);
impl BUF_BYPASS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_BYPASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_BYPASS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_bypass` writer - Bypass Input Buffer"]
pub struct BUF_BYPASS_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_BYPASS_W<'a> {
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
#[doc = "Field `adc_refscl` reader - ADC Reference Scale"]
pub struct ADC_REFSCL_R(crate::FieldReader<bool, bool>);
impl ADC_REFSCL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_REFSCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_REFSCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_refscl` writer - ADC Reference Scale"]
pub struct ADC_REFSCL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REFSCL_W<'a> {
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
#[doc = "Field `adc_scale` reader - ADC Scale"]
pub struct ADC_SCALE_R(crate::FieldReader<bool, bool>);
impl ADC_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_SCALE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_scale` writer - ADC Scale"]
pub struct ADC_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_SCALE_W<'a> {
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
#[doc = "Field `adc_refsel` reader - ADC Reference (VRef) Select (INTERNAL ONLY)"]
pub struct ADC_REFSEL_R(crate::FieldReader<bool, bool>);
impl ADC_REFSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_REFSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_REFSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_refsel` writer - ADC Reference (VRef) Select (INTERNAL ONLY)"]
pub struct ADC_REFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REFSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `adc_clk_en` reader - ADC Clock Enable"]
pub struct ADC_CLK_EN_R(crate::FieldReader<bool, bool>);
impl ADC_CLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_CLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_clk_en` writer - ADC Clock Enable"]
pub struct ADC_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `adc_chsel` reader - ADC Channel Select"]
pub struct ADC_CHSEL_R(crate::FieldReader<u8, u8>);
impl ADC_CHSEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ADC_CHSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_CHSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_chsel` writer - ADC Channel Select"]
pub struct ADC_CHSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_CHSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 12)) | ((value as u32 & 0x0f) << 12);
        self.w
    }
}
#[doc = "Field `adc_xref` reader - Enable Use of ADC External Reference"]
pub struct ADC_XREF_R(crate::FieldReader<bool, bool>);
impl ADC_XREF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_XREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_XREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_xref` writer - Enable Use of ADC External Reference"]
pub struct ADC_XREF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_XREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `adc_dataalign` reader - ADC Data Alignment Select"]
pub struct ADC_DATAALIGN_R(crate::FieldReader<bool, bool>);
impl ADC_DATAALIGN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DATAALIGN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DATAALIGN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_dataalign` writer - ADC Data Alignment Select"]
pub struct ADC_DATAALIGN_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DATAALIGN_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `afe_pwr_up_dly` reader - Delay from ADC Powerup Until ADC Ready Asserted"]
pub struct AFE_PWR_UP_DLY_R(crate::FieldReader<u8, u8>);
impl AFE_PWR_UP_DLY_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        AFE_PWR_UP_DLY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AFE_PWR_UP_DLY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `afe_pwr_up_dly` writer - Delay from ADC Powerup Until ADC Ready Asserted"]
pub struct AFE_PWR_UP_DLY_W<'a> {
    w: &'a mut W,
}
impl<'a> AFE_PWR_UP_DLY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | ((value as u32 & 0xff) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    pub fn cpu_adc_start(&self) -> CPU_ADC_START_R {
        CPU_ADC_START_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    pub fn adc_pu(&self) -> ADC_PU_R {
        ADC_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Input Buffer Power Up"]
    #[inline(always)]
    pub fn buf_pu(&self) -> BUF_PU_R {
        BUF_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    pub fn adc_refbuf_pu(&self) -> ADC_REFBUF_PU_R {
        ADC_REFBUF_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Charge Pump Power Up"]
    #[inline(always)]
    pub fn adc_chgpump_pu(&self) -> ADC_CHGPUMP_PU_R {
        ADC_CHGPUMP_PU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC Input Buffer Chop Disable (INTERNAL ONLY)"]
    #[inline(always)]
    pub fn buf_chop_dis(&self) -> BUF_CHOP_DIS_R {
        BUF_CHOP_DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Use of Charge Pump Output by Input Buffer (INTERNAL)"]
    #[inline(always)]
    pub fn buf_pump_dis(&self) -> BUF_PUMP_DIS_R {
        BUF_PUMP_DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bypass Input Buffer"]
    #[inline(always)]
    pub fn buf_bypass(&self) -> BUF_BYPASS_R {
        BUF_BYPASS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    pub fn adc_refscl(&self) -> ADC_REFSCL_R {
        ADC_REFSCL_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    pub fn adc_scale(&self) -> ADC_SCALE_R {
        ADC_SCALE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC Reference (VRef) Select (INTERNAL ONLY)"]
    #[inline(always)]
    pub fn adc_refsel(&self) -> ADC_REFSEL_R {
        ADC_REFSEL_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    pub fn adc_clk_en(&self) -> ADC_CLK_EN_R {
        ADC_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - ADC Channel Select"]
    #[inline(always)]
    pub fn adc_chsel(&self) -> ADC_CHSEL_R {
        ADC_CHSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Enable Use of ADC External Reference"]
    #[inline(always)]
    pub fn adc_xref(&self) -> ADC_XREF_R {
        ADC_XREF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC Data Alignment Select"]
    #[inline(always)]
    pub fn adc_dataalign(&self) -> ADC_DATAALIGN_R {
        ADC_DATAALIGN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Delay from ADC Powerup Until ADC Ready Asserted"]
    #[inline(always)]
    pub fn afe_pwr_up_dly(&self) -> AFE_PWR_UP_DLY_R {
        AFE_PWR_UP_DLY_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start ADC Conversion"]
    #[inline(always)]
    pub fn cpu_adc_start(&mut self) -> CPU_ADC_START_W {
        CPU_ADC_START_W { w: self }
    }
    #[doc = "Bit 1 - ADC Power Up"]
    #[inline(always)]
    pub fn adc_pu(&mut self) -> ADC_PU_W {
        ADC_PU_W { w: self }
    }
    #[doc = "Bit 2 - ADC Input Buffer Power Up"]
    #[inline(always)]
    pub fn buf_pu(&mut self) -> BUF_PU_W {
        BUF_PU_W { w: self }
    }
    #[doc = "Bit 3 - ADC Reference Buffer Power Up"]
    #[inline(always)]
    pub fn adc_refbuf_pu(&mut self) -> ADC_REFBUF_PU_W {
        ADC_REFBUF_PU_W { w: self }
    }
    #[doc = "Bit 4 - ADC Charge Pump Power Up"]
    #[inline(always)]
    pub fn adc_chgpump_pu(&mut self) -> ADC_CHGPUMP_PU_W {
        ADC_CHGPUMP_PU_W { w: self }
    }
    #[doc = "Bit 5 - ADC Input Buffer Chop Disable (INTERNAL ONLY)"]
    #[inline(always)]
    pub fn buf_chop_dis(&mut self) -> BUF_CHOP_DIS_W {
        BUF_CHOP_DIS_W { w: self }
    }
    #[doc = "Bit 6 - Disable Use of Charge Pump Output by Input Buffer (INTERNAL)"]
    #[inline(always)]
    pub fn buf_pump_dis(&mut self) -> BUF_PUMP_DIS_W {
        BUF_PUMP_DIS_W { w: self }
    }
    #[doc = "Bit 7 - Bypass Input Buffer"]
    #[inline(always)]
    pub fn buf_bypass(&mut self) -> BUF_BYPASS_W {
        BUF_BYPASS_W { w: self }
    }
    #[doc = "Bit 8 - ADC Reference Scale"]
    #[inline(always)]
    pub fn adc_refscl(&mut self) -> ADC_REFSCL_W {
        ADC_REFSCL_W { w: self }
    }
    #[doc = "Bit 9 - ADC Scale"]
    #[inline(always)]
    pub fn adc_scale(&mut self) -> ADC_SCALE_W {
        ADC_SCALE_W { w: self }
    }
    #[doc = "Bit 10 - ADC Reference (VRef) Select (INTERNAL ONLY)"]
    #[inline(always)]
    pub fn adc_refsel(&mut self) -> ADC_REFSEL_W {
        ADC_REFSEL_W { w: self }
    }
    #[doc = "Bit 11 - ADC Clock Enable"]
    #[inline(always)]
    pub fn adc_clk_en(&mut self) -> ADC_CLK_EN_W {
        ADC_CLK_EN_W { w: self }
    }
    #[doc = "Bits 12:15 - ADC Channel Select"]
    #[inline(always)]
    pub fn adc_chsel(&mut self) -> ADC_CHSEL_W {
        ADC_CHSEL_W { w: self }
    }
    #[doc = "Bit 16 - Enable Use of ADC External Reference"]
    #[inline(always)]
    pub fn adc_xref(&mut self) -> ADC_XREF_W {
        ADC_XREF_W { w: self }
    }
    #[doc = "Bit 17 - ADC Data Alignment Select"]
    #[inline(always)]
    pub fn adc_dataalign(&mut self) -> ADC_DATAALIGN_W {
        ADC_DATAALIGN_W { w: self }
    }
    #[doc = "Bits 24:31 - Delay from ADC Powerup Until ADC Ready Asserted"]
    #[inline(always)]
    pub fn afe_pwr_up_dly(&mut self) -> AFE_PWR_UP_DLY_W {
        AFE_PWR_UP_DLY_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
