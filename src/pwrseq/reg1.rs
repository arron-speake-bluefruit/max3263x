#[doc = "Register `REG1` reader"]
pub struct R(crate::R<REG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG1` writer"]
pub struct W(crate::W<REG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG1_SPEC>;
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
impl From<crate::W<REG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_clr_io_event_latch` reader - Clear all GPIO Event Seen Latches"]
pub struct PWR_CLR_IO_EVENT_LATCH_R(crate::FieldReader<bool>);
impl PWR_CLR_IO_EVENT_LATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_CLR_IO_EVENT_LATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_CLR_IO_EVENT_LATCH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_clr_io_event_latch` writer - Clear all GPIO Event Seen Latches"]
pub struct PWR_CLR_IO_EVENT_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_CLR_IO_EVENT_LATCH_W<'a> {
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
#[doc = "Field `pwr_clr_io_cfg_latch` reader - Clear all GPIO Configuration Latches"]
pub struct PWR_CLR_IO_CFG_LATCH_R(crate::FieldReader<bool>);
impl PWR_CLR_IO_CFG_LATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_CLR_IO_CFG_LATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_CLR_IO_CFG_LATCH_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_clr_io_cfg_latch` writer - Clear all GPIO Configuration Latches"]
pub struct PWR_CLR_IO_CFG_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_CLR_IO_CFG_LATCH_W<'a> {
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
#[doc = "Field `pwr_mbus_gate` reader - Freeze GPIO MBus State"]
pub struct PWR_MBUS_GATE_R(crate::FieldReader<bool>);
impl PWR_MBUS_GATE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_MBUS_GATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_MBUS_GATE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_mbus_gate` writer - Freeze GPIO MBus State"]
pub struct PWR_MBUS_GATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_MBUS_GATE_W<'a> {
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
#[doc = "Field `pwr_discharge_en` reader - Enable Flash Discharge During Powerfail Event"]
pub struct PWR_DISCHARGE_EN_R(crate::FieldReader<bool>);
impl PWR_DISCHARGE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_DISCHARGE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_DISCHARGE_EN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_discharge_en` writer - Enable Flash Discharge During Powerfail Event"]
pub struct PWR_DISCHARGE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_DISCHARGE_EN_W<'a> {
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
#[doc = "Field `pwr_tvdd12_well` reader - TVDD12 Well Switch"]
pub struct PWR_TVDD12_WELL_R(crate::FieldReader<bool>);
impl PWR_TVDD12_WELL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_TVDD12_WELL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TVDD12_WELL_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_tvdd12_well` writer - TVDD12 Well Switch"]
pub struct PWR_TVDD12_WELL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TVDD12_WELL_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Clear all GPIO Event Seen Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_event_latch(&self) -> PWR_CLR_IO_EVENT_LATCH_R {
        PWR_CLR_IO_EVENT_LATCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear all GPIO Configuration Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_cfg_latch(&self) -> PWR_CLR_IO_CFG_LATCH_R {
        PWR_CLR_IO_CFG_LATCH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Freeze GPIO MBus State"]
    #[inline(always)]
    pub fn pwr_mbus_gate(&self) -> PWR_MBUS_GATE_R {
        PWR_MBUS_GATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Flash Discharge During Powerfail Event"]
    #[inline(always)]
    pub fn pwr_discharge_en(&self) -> PWR_DISCHARGE_EN_R {
        PWR_DISCHARGE_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TVDD12 Well Switch"]
    #[inline(always)]
    pub fn pwr_tvdd12_well(&self) -> PWR_TVDD12_WELL_R {
        PWR_TVDD12_WELL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear all GPIO Event Seen Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_event_latch(&mut self) -> PWR_CLR_IO_EVENT_LATCH_W {
        PWR_CLR_IO_EVENT_LATCH_W { w: self }
    }
    #[doc = "Bit 1 - Clear all GPIO Configuration Latches"]
    #[inline(always)]
    pub fn pwr_clr_io_cfg_latch(&mut self) -> PWR_CLR_IO_CFG_LATCH_W {
        PWR_CLR_IO_CFG_LATCH_W { w: self }
    }
    #[doc = "Bit 2 - Freeze GPIO MBus State"]
    #[inline(always)]
    pub fn pwr_mbus_gate(&mut self) -> PWR_MBUS_GATE_W {
        PWR_MBUS_GATE_W { w: self }
    }
    #[doc = "Bit 3 - Enable Flash Discharge During Powerfail Event"]
    #[inline(always)]
    pub fn pwr_discharge_en(&mut self) -> PWR_DISCHARGE_EN_W {
        PWR_DISCHARGE_EN_W { w: self }
    }
    #[doc = "Bit 4 - TVDD12 Well Switch"]
    #[inline(always)]
    pub fn pwr_tvdd12_well(&mut self) -> PWR_TVDD12_WELL_W {
        PWR_TVDD12_WELL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg1](index.html) module"]
pub struct REG1_SPEC;
impl crate::RegisterSpec for REG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg1::R](R) reader structure"]
impl crate::Readable for REG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg1::W](W) writer structure"]
impl crate::Writable for REG1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG1 to value 0"]
impl crate::Resettable for REG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
