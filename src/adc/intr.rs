#[doc = "Register `INTR` reader"]
pub struct R(crate::R<INTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTR` writer"]
pub struct W(crate::W<INTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTR_SPEC>;
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
impl From<crate::W<INTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `adc_done_ie` reader - ADC Done Interrupt Enable"]
pub struct ADC_DONE_IE_R(crate::FieldReader<bool>);
impl ADC_DONE_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DONE_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DONE_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_done_ie` writer - ADC Done Interrupt Enable"]
pub struct ADC_DONE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DONE_IE_W<'a> {
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
#[doc = "Field `adc_ref_ready_ie` reader - ADC Reference Ready Interrupt Enable"]
pub struct ADC_REF_READY_IE_R(crate::FieldReader<bool>);
impl ADC_REF_READY_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_REF_READY_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_REF_READY_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_ref_ready_ie` writer - ADC Reference Ready Interrupt Enable"]
pub struct ADC_REF_READY_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REF_READY_IE_W<'a> {
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
#[doc = "Field `adc_hi_limit_ie` reader - ADC Hi Limit Monitor Interrupt Enable"]
pub struct ADC_HI_LIMIT_IE_R(crate::FieldReader<bool>);
impl ADC_HI_LIMIT_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_HI_LIMIT_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_HI_LIMIT_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_hi_limit_ie` writer - ADC Hi Limit Monitor Interrupt Enable"]
pub struct ADC_HI_LIMIT_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_HI_LIMIT_IE_W<'a> {
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
#[doc = "Field `adc_lo_limit_ie` reader - ADC Lo Limit Monitor Interrupt Enable"]
pub struct ADC_LO_LIMIT_IE_R(crate::FieldReader<bool>);
impl ADC_LO_LIMIT_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_LO_LIMIT_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_LO_LIMIT_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_lo_limit_ie` writer - ADC Lo Limit Monitor Interrupt Enable"]
pub struct ADC_LO_LIMIT_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_LO_LIMIT_IE_W<'a> {
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
#[doc = "Field `adc_overflow_ie` reader - ADC Overflow Interrupt Enable"]
pub struct ADC_OVERFLOW_IE_R(crate::FieldReader<bool>);
impl ADC_OVERFLOW_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_OVERFLOW_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OVERFLOW_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_overflow_ie` writer - ADC Overflow Interrupt Enable"]
pub struct ADC_OVERFLOW_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OVERFLOW_IE_W<'a> {
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
#[doc = "Field `ro_cal_done_ie` reader - RO Cal Done Interrupt Enable"]
pub struct RO_CAL_DONE_IE_R(crate::FieldReader<bool>);
impl RO_CAL_DONE_IE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RO_CAL_DONE_IE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_CAL_DONE_IE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_cal_done_ie` writer - RO Cal Done Interrupt Enable"]
pub struct RO_CAL_DONE_IE_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_CAL_DONE_IE_W<'a> {
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
#[doc = "Field `adc_done_if` reader - ADC Done Interrupt Flag"]
pub struct ADC_DONE_IF_R(crate::FieldReader<bool>);
impl ADC_DONE_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_DONE_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_DONE_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_done_if` writer - ADC Done Interrupt Flag"]
pub struct ADC_DONE_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_DONE_IF_W<'a> {
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
#[doc = "Field `adc_ref_ready_if` reader - ADC Reference Ready Interrupt Flag"]
pub struct ADC_REF_READY_IF_R(crate::FieldReader<bool>);
impl ADC_REF_READY_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_REF_READY_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_REF_READY_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_ref_ready_if` writer - ADC Reference Ready Interrupt Flag"]
pub struct ADC_REF_READY_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_REF_READY_IF_W<'a> {
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
#[doc = "Field `adc_hi_limit_if` reader - ADC Hi Limit Monitor Interrupt Flag"]
pub struct ADC_HI_LIMIT_IF_R(crate::FieldReader<bool>);
impl ADC_HI_LIMIT_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_HI_LIMIT_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_HI_LIMIT_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_hi_limit_if` writer - ADC Hi Limit Monitor Interrupt Flag"]
pub struct ADC_HI_LIMIT_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_HI_LIMIT_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `adc_lo_limit_if` reader - ADC Lo Limit Monitor Interrupt Flag"]
pub struct ADC_LO_LIMIT_IF_R(crate::FieldReader<bool>);
impl ADC_LO_LIMIT_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_LO_LIMIT_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_LO_LIMIT_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_lo_limit_if` writer - ADC Lo Limit Monitor Interrupt Flag"]
pub struct ADC_LO_LIMIT_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_LO_LIMIT_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `adc_overflow_if` reader - ADC Overflow Interrupt Flag"]
pub struct ADC_OVERFLOW_IF_R(crate::FieldReader<bool>);
impl ADC_OVERFLOW_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_OVERFLOW_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_OVERFLOW_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `adc_overflow_if` writer - ADC Overflow Interrupt Flag"]
pub struct ADC_OVERFLOW_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC_OVERFLOW_IF_W<'a> {
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
#[doc = "Field `ro_cal_done_if` reader - RO Cal Done Interrupt Flag"]
pub struct RO_CAL_DONE_IF_R(crate::FieldReader<bool>);
impl RO_CAL_DONE_IF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RO_CAL_DONE_IF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RO_CAL_DONE_IF_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ro_cal_done_if` writer - RO Cal Done Interrupt Flag"]
pub struct RO_CAL_DONE_IF_W<'a> {
    w: &'a mut W,
}
impl<'a> RO_CAL_DONE_IF_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `adc_int_pending` reader - ADC Interrupt Pending Status"]
pub struct ADC_INT_PENDING_R(crate::FieldReader<bool>);
impl ADC_INT_PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ADC_INT_PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADC_INT_PENDING_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn adc_done_ie(&self) -> ADC_DONE_IE_R {
        ADC_DONE_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ref_ready_ie(&self) -> ADC_REF_READY_IE_R {
        ADC_REF_READY_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn adc_hi_limit_ie(&self) -> ADC_HI_LIMIT_IE_R {
        ADC_HI_LIMIT_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn adc_lo_limit_ie(&self) -> ADC_LO_LIMIT_IE_R {
        ADC_LO_LIMIT_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn adc_overflow_ie(&self) -> ADC_OVERFLOW_IE_R {
        ADC_OVERFLOW_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RO Cal Done Interrupt Enable"]
    #[inline(always)]
    pub fn ro_cal_done_ie(&self) -> RO_CAL_DONE_IE_R {
        RO_CAL_DONE_IE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn adc_done_if(&self) -> ADC_DONE_IF_R {
        ADC_DONE_IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn adc_ref_ready_if(&self) -> ADC_REF_READY_IF_R {
        ADC_REF_READY_IF_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn adc_hi_limit_if(&self) -> ADC_HI_LIMIT_IF_R {
        ADC_HI_LIMIT_IF_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn adc_lo_limit_if(&self) -> ADC_LO_LIMIT_IF_R {
        ADC_LO_LIMIT_IF_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adc_overflow_if(&self) -> ADC_OVERFLOW_IF_R {
        ADC_OVERFLOW_IF_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RO Cal Done Interrupt Flag"]
    #[inline(always)]
    pub fn ro_cal_done_if(&self) -> RO_CAL_DONE_IF_R {
        RO_CAL_DONE_IF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC Interrupt Pending Status"]
    #[inline(always)]
    pub fn adc_int_pending(&self) -> ADC_INT_PENDING_R {
        ADC_INT_PENDING_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Done Interrupt Enable"]
    #[inline(always)]
    pub fn adc_done_ie(&mut self) -> ADC_DONE_IE_W {
        ADC_DONE_IE_W { w: self }
    }
    #[doc = "Bit 1 - ADC Reference Ready Interrupt Enable"]
    #[inline(always)]
    pub fn adc_ref_ready_ie(&mut self) -> ADC_REF_READY_IE_W {
        ADC_REF_READY_IE_W { w: self }
    }
    #[doc = "Bit 2 - ADC Hi Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn adc_hi_limit_ie(&mut self) -> ADC_HI_LIMIT_IE_W {
        ADC_HI_LIMIT_IE_W { w: self }
    }
    #[doc = "Bit 3 - ADC Lo Limit Monitor Interrupt Enable"]
    #[inline(always)]
    pub fn adc_lo_limit_ie(&mut self) -> ADC_LO_LIMIT_IE_W {
        ADC_LO_LIMIT_IE_W { w: self }
    }
    #[doc = "Bit 4 - ADC Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn adc_overflow_ie(&mut self) -> ADC_OVERFLOW_IE_W {
        ADC_OVERFLOW_IE_W { w: self }
    }
    #[doc = "Bit 5 - RO Cal Done Interrupt Enable"]
    #[inline(always)]
    pub fn ro_cal_done_ie(&mut self) -> RO_CAL_DONE_IE_W {
        RO_CAL_DONE_IE_W { w: self }
    }
    #[doc = "Bit 16 - ADC Done Interrupt Flag"]
    #[inline(always)]
    pub fn adc_done_if(&mut self) -> ADC_DONE_IF_W {
        ADC_DONE_IF_W { w: self }
    }
    #[doc = "Bit 17 - ADC Reference Ready Interrupt Flag"]
    #[inline(always)]
    pub fn adc_ref_ready_if(&mut self) -> ADC_REF_READY_IF_W {
        ADC_REF_READY_IF_W { w: self }
    }
    #[doc = "Bit 18 - ADC Hi Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn adc_hi_limit_if(&mut self) -> ADC_HI_LIMIT_IF_W {
        ADC_HI_LIMIT_IF_W { w: self }
    }
    #[doc = "Bit 19 - ADC Lo Limit Monitor Interrupt Flag"]
    #[inline(always)]
    pub fn adc_lo_limit_if(&mut self) -> ADC_LO_LIMIT_IF_W {
        ADC_LO_LIMIT_IF_W { w: self }
    }
    #[doc = "Bit 20 - ADC Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn adc_overflow_if(&mut self) -> ADC_OVERFLOW_IF_W {
        ADC_OVERFLOW_IF_W { w: self }
    }
    #[doc = "Bit 21 - RO Cal Done Interrupt Flag"]
    #[inline(always)]
    pub fn ro_cal_done_if(&mut self) -> RO_CAL_DONE_IF_W {
        RO_CAL_DONE_IF_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intr](index.html) module"]
pub struct INTR_SPEC;
impl crate::RegisterSpec for INTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [intr::R](R) reader structure"]
impl crate::Readable for INTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [intr::W](W) writer structure"]
impl crate::Writable for INTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTR to value 0"]
impl crate::Resettable for INTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
