#[doc = "Register `REG4` reader"]
pub struct R(crate::R<REG4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG4` writer"]
pub struct W(crate::W<REG4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG4_SPEC>;
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
impl From<crate::W<REG4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_tm_ps_2_gpio` reader - Internal Use Only"]
pub struct PWR_TM_PS_2_GPIO_R(crate::FieldReader<bool, bool>);
impl PWR_TM_PS_2_GPIO_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_TM_PS_2_GPIO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TM_PS_2_GPIO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_tm_ps_2_gpio` writer - Internal Use Only"]
pub struct PWR_TM_PS_2_GPIO_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TM_PS_2_GPIO_W<'a> {
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
#[doc = "Field `pwr_tm_fast_timers` reader - Internal Use Only"]
pub struct PWR_TM_FAST_TIMERS_R(crate::FieldReader<bool, bool>);
impl PWR_TM_FAST_TIMERS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_TM_FAST_TIMERS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TM_FAST_TIMERS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_tm_fast_timers` writer - Internal Use Only"]
pub struct PWR_TM_FAST_TIMERS_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TM_FAST_TIMERS_W<'a> {
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
#[doc = "Field `pwr_usb_dis_comp` reader - Internal Use Only"]
pub struct PWR_USB_DIS_COMP_R(crate::FieldReader<bool, bool>);
impl PWR_USB_DIS_COMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_USB_DIS_COMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_USB_DIS_COMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_usb_dis_comp` writer - Internal Use Only"]
pub struct PWR_USB_DIS_COMP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_USB_DIS_COMP_W<'a> {
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
#[doc = "Field `pwr_ro_tstclk_en` reader - Internal Use Only"]
pub struct PWR_RO_TSTCLK_EN_R(crate::FieldReader<bool, bool>);
impl PWR_RO_TSTCLK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_RO_TSTCLK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_RO_TSTCLK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_ro_tstclk_en` writer - Internal Use Only"]
pub struct PWR_RO_TSTCLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_RO_TSTCLK_EN_W<'a> {
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
#[doc = "Field `pwr_nr_clk_gate_en` reader - Internal Use Only"]
pub struct PWR_NR_CLK_GATE_EN_R(crate::FieldReader<bool, bool>);
impl PWR_NR_CLK_GATE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_NR_CLK_GATE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_NR_CLK_GATE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_nr_clk_gate_en` writer - Internal Use Only"]
pub struct PWR_NR_CLK_GATE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_NR_CLK_GATE_EN_W<'a> {
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
#[doc = "Field `pwr_ext_clk_in_en` reader - Internal Use Only"]
pub struct PWR_EXT_CLK_IN_EN_R(crate::FieldReader<bool, bool>);
impl PWR_EXT_CLK_IN_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_EXT_CLK_IN_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_EXT_CLK_IN_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_ext_clk_in_en` writer - Internal Use Only"]
pub struct PWR_EXT_CLK_IN_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_EXT_CLK_IN_EN_W<'a> {
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
#[doc = "Field `pwr_pseq_32k_en` reader - Internal Use Only"]
pub struct PWR_PSEQ_32K_EN_R(crate::FieldReader<bool, bool>);
impl PWR_PSEQ_32K_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_PSEQ_32K_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_PSEQ_32K_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_pseq_32k_en` writer - Internal Use Only"]
pub struct PWR_PSEQ_32K_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_PSEQ_32K_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_tm_ps_2_gpio(&self) -> PWR_TM_PS_2_GPIO_R {
        PWR_TM_PS_2_GPIO_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_tm_fast_timers(&self) -> PWR_TM_FAST_TIMERS_R {
        PWR_TM_FAST_TIMERS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_usb_dis_comp(&self) -> PWR_USB_DIS_COMP_R {
        PWR_USB_DIS_COMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_ro_tstclk_en(&self) -> PWR_RO_TSTCLK_EN_R {
        PWR_RO_TSTCLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_nr_clk_gate_en(&self) -> PWR_NR_CLK_GATE_EN_R {
        PWR_NR_CLK_GATE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_ext_clk_in_en(&self) -> PWR_EXT_CLK_IN_EN_R {
        PWR_EXT_CLK_IN_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_pseq_32k_en(&self) -> PWR_PSEQ_32K_EN_R {
        PWR_PSEQ_32K_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_tm_ps_2_gpio(&mut self) -> PWR_TM_PS_2_GPIO_W {
        PWR_TM_PS_2_GPIO_W { w: self }
    }
    #[doc = "Bit 1 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_tm_fast_timers(&mut self) -> PWR_TM_FAST_TIMERS_W {
        PWR_TM_FAST_TIMERS_W { w: self }
    }
    #[doc = "Bit 3 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_usb_dis_comp(&mut self) -> PWR_USB_DIS_COMP_W {
        PWR_USB_DIS_COMP_W { w: self }
    }
    #[doc = "Bit 4 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_ro_tstclk_en(&mut self) -> PWR_RO_TSTCLK_EN_W {
        PWR_RO_TSTCLK_EN_W { w: self }
    }
    #[doc = "Bit 5 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_nr_clk_gate_en(&mut self) -> PWR_NR_CLK_GATE_EN_W {
        PWR_NR_CLK_GATE_EN_W { w: self }
    }
    #[doc = "Bit 6 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_ext_clk_in_en(&mut self) -> PWR_EXT_CLK_IN_EN_W {
        PWR_EXT_CLK_IN_EN_W { w: self }
    }
    #[doc = "Bit 7 - Internal Use Only"]
    #[inline(always)]
    pub fn pwr_pseq_32k_en(&mut self) -> PWR_PSEQ_32K_EN_W {
        PWR_PSEQ_32K_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 4 (Internal Test Only)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg4](index.html) module"]
pub struct REG4_SPEC;
impl crate::RegisterSpec for REG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg4::R](R) reader structure"]
impl crate::Readable for REG4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg4::W](W) writer structure"]
impl crate::Writable for REG4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG4 to value 0"]
impl crate::Resettable for REG4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
