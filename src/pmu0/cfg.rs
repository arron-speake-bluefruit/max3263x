#[doc = "Register `CFG` reader"]
pub struct R(crate::R<CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFG` writer"]
pub struct W(crate::W<CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFG_SPEC>;
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
impl From<crate::W<CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `enable` reader - PMU Channel Enable"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `enable` writer - PMU Channel Enable"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `ll_stopped` reader - Linked List Engine Status"]
pub struct LL_STOPPED_R(crate::FieldReader<bool, bool>);
impl LL_STOPPED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        LL_STOPPED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LL_STOPPED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ll_stopped` writer - Linked List Engine Status"]
pub struct LL_STOPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> LL_STOPPED_W<'a> {
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
#[doc = "Field `manual` reader - Manual Mode Enable"]
pub struct MANUAL_R(crate::FieldReader<bool, bool>);
impl MANUAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MANUAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MANUAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `manual` writer - Manual Mode Enable"]
pub struct MANUAL_W<'a> {
    w: &'a mut W,
}
impl<'a> MANUAL_W<'a> {
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
#[doc = "Field `bus_error` reader - AHB Bus Error Interrupt Flag"]
pub struct BUS_ERROR_R(crate::FieldReader<bool, bool>);
impl BUS_ERROR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUS_ERROR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUS_ERROR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bus_error` writer - AHB Bus Error Interrupt Flag"]
pub struct BUS_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUS_ERROR_W<'a> {
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
#[doc = "Field `to_stat` reader - AHB Bus Timeout Interrupt Flag"]
pub struct TO_STAT_R(crate::FieldReader<bool, bool>);
impl TO_STAT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TO_STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `to_stat` writer - AHB Bus Timeout Interrupt Flag"]
pub struct TO_STAT_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_STAT_W<'a> {
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
#[doc = "Field `to_sel` reader - Time Out Interval Select"]
pub struct TO_SEL_R(crate::FieldReader<u8, u8>);
impl TO_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        TO_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TO_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `to_sel` writer - Time Out Interval Select"]
pub struct TO_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> TO_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(7 << 11)) | ((value as u32 & 7) << 11);
        self.w
    }
}
#[doc = "Field `ps_sel` reader - Time Out Interval Prescale Select"]
pub struct PS_SEL_R(crate::FieldReader<u8, u8>);
impl PS_SEL_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        PS_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PS_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ps_sel` writer - Time Out Interval Prescale Select"]
pub struct PS_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PS_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(3 << 14)) | ((value as u32 & 3) << 14);
        self.w
    }
}
#[doc = "Field `interrupt` reader - Descriptor Interrupt Flag"]
pub struct INTERRUPT_R(crate::FieldReader<bool, bool>);
impl INTERRUPT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INTERRUPT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTERRUPT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `interrupt` writer - Descriptor Interrupt Flag"]
pub struct INTERRUPT_W<'a> {
    w: &'a mut W,
}
impl<'a> INTERRUPT_W<'a> {
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
#[doc = "Field `int_en` reader - PMU Channel Interrupt Enable"]
pub struct INT_EN_R(crate::FieldReader<bool, bool>);
impl INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `int_en` writer - PMU Channel Interrupt Enable"]
pub struct INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> INT_EN_W<'a> {
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
#[doc = "Field `burst_size` reader - DMA Maximum Burst Size"]
pub struct BURST_SIZE_R(crate::FieldReader<u8, u8>);
impl BURST_SIZE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        BURST_SIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BURST_SIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `burst_size` writer - DMA Maximum Burst Size"]
pub struct BURST_SIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> BURST_SIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x1f << 24)) | ((value as u32 & 0x1f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - PMU Channel Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Linked List Engine Status"]
    #[inline(always)]
    pub fn ll_stopped(&self) -> LL_STOPPED_R {
        LL_STOPPED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Manual Mode Enable"]
    #[inline(always)]
    pub fn manual(&self) -> MANUAL_R {
        MANUAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn bus_error(&self) -> BUS_ERROR_R {
        BUS_ERROR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - AHB Bus Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn to_stat(&self) -> TO_STAT_R {
        TO_STAT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 11:13 - Time Out Interval Select"]
    #[inline(always)]
    pub fn to_sel(&self) -> TO_SEL_R {
        TO_SEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 14:15 - Time Out Interval Prescale Select"]
    #[inline(always)]
    pub fn ps_sel(&self) -> PS_SEL_R {
        PS_SEL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Descriptor Interrupt Flag"]
    #[inline(always)]
    pub fn interrupt(&self) -> INTERRUPT_R {
        INTERRUPT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PMU Channel Interrupt Enable"]
    #[inline(always)]
    pub fn int_en(&self) -> INT_EN_R {
        INT_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 24:28 - DMA Maximum Burst Size"]
    #[inline(always)]
    pub fn burst_size(&self) -> BURST_SIZE_R {
        BURST_SIZE_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PMU Channel Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 2 - Linked List Engine Status"]
    #[inline(always)]
    pub fn ll_stopped(&mut self) -> LL_STOPPED_W {
        LL_STOPPED_W { w: self }
    }
    #[doc = "Bit 3 - Manual Mode Enable"]
    #[inline(always)]
    pub fn manual(&mut self) -> MANUAL_W {
        MANUAL_W { w: self }
    }
    #[doc = "Bit 4 - AHB Bus Error Interrupt Flag"]
    #[inline(always)]
    pub fn bus_error(&mut self) -> BUS_ERROR_W {
        BUS_ERROR_W { w: self }
    }
    #[doc = "Bit 6 - AHB Bus Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn to_stat(&mut self) -> TO_STAT_W {
        TO_STAT_W { w: self }
    }
    #[doc = "Bits 11:13 - Time Out Interval Select"]
    #[inline(always)]
    pub fn to_sel(&mut self) -> TO_SEL_W {
        TO_SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - Time Out Interval Prescale Select"]
    #[inline(always)]
    pub fn ps_sel(&mut self) -> PS_SEL_W {
        PS_SEL_W { w: self }
    }
    #[doc = "Bit 16 - Descriptor Interrupt Flag"]
    #[inline(always)]
    pub fn interrupt(&mut self) -> INTERRUPT_W {
        INTERRUPT_W { w: self }
    }
    #[doc = "Bit 17 - PMU Channel Interrupt Enable"]
    #[inline(always)]
    pub fn int_en(&mut self) -> INT_EN_W {
        INT_EN_W { w: self }
    }
    #[doc = "Bits 24:28 - DMA Maximum Burst Size"]
    #[inline(always)]
    pub fn burst_size(&mut self) -> BURST_SIZE_W {
        BURST_SIZE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG_SPEC;
impl crate::RegisterSpec for CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfg::R](R) reader structure"]
impl crate::Readable for CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfg::W](W) writer structure"]
impl crate::Writable for CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFG to value 0"]
impl crate::Resettable for CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
