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
#[doc = "Field `enable` reader - RTC Timer Enable"]
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
#[doc = "Field `enable` writer - RTC Timer Enable"]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `clear` writer - RTC Timer Clear Bit"]
pub struct CLEAR_W<'a> {
    w: &'a mut W,
}
impl<'a> CLEAR_W<'a> {
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
#[doc = "Field `pending` reader - RTC Transaction Pending"]
pub struct PENDING_R(crate::FieldReader<bool, bool>);
impl PENDING_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PENDING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PENDING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `use_async_flags` reader - Use Async RTC Flags"]
pub struct USE_ASYNC_FLAGS_R(crate::FieldReader<bool, bool>);
impl USE_ASYNC_FLAGS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        USE_ASYNC_FLAGS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USE_ASYNC_FLAGS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `use_async_flags` writer - Use Async RTC Flags"]
pub struct USE_ASYNC_FLAGS_W<'a> {
    w: &'a mut W,
}
impl<'a> USE_ASYNC_FLAGS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `aggressive_rst` reader - Use Aggressive Reset Mode"]
pub struct AGGRESSIVE_RST_R(crate::FieldReader<bool, bool>);
impl AGGRESSIVE_RST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AGGRESSIVE_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AGGRESSIVE_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `aggressive_rst` writer - Use Aggressive Reset Mode"]
pub struct AGGRESSIVE_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> AGGRESSIVE_RST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `en_active` reader - Enable RTC in Active Modes"]
pub struct EN_ACTIVE_R(crate::FieldReader<bool, bool>);
impl EN_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `osc_goto_low_active` reader - osc_goto_low_r transaction is pending"]
pub struct OSC_GOTO_LOW_ACTIVE_R(crate::FieldReader<bool, bool>);
impl OSC_GOTO_LOW_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_GOTO_LOW_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_GOTO_LOW_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `osc_frce_sm_en_active` reader - osc_force_mode transaction is pending"]
pub struct OSC_FRCE_SM_EN_ACTIVE_R(crate::FieldReader<bool, bool>);
impl OSC_FRCE_SM_EN_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_FRCE_SM_EN_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_FRCE_SM_EN_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `osc_frce_st_active` reader - osc_force_state transaction is pending"]
pub struct OSC_FRCE_ST_ACTIVE_R(crate::FieldReader<bool, bool>);
impl OSC_FRCE_ST_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        OSC_FRCE_ST_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OSC_FRCE_ST_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `set_active` reader - timer_set_active"]
pub struct SET_ACTIVE_R(crate::FieldReader<bool, bool>);
impl SET_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SET_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SET_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `clr_active` reader - RTC clear is pending"]
pub struct CLR_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CLR_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CLR_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLR_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rollover_clr_active` reader - rollover clr is pending"]
pub struct ROLLOVER_CLR_ACTIVE_R(crate::FieldReader<bool, bool>);
impl ROLLOVER_CLR_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        ROLLOVER_CLR_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ROLLOVER_CLR_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `prescale_cmpr0_active` reader - prescale cmpr0 is pending"]
pub struct PRESCALE_CMPR0_ACTIVE_R(crate::FieldReader<bool, bool>);
impl PRESCALE_CMPR0_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRESCALE_CMPR0_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALE_CMPR0_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `prescale_update_active` reader - prescale update transaction is pending"]
pub struct PRESCALE_UPDATE_ACTIVE_R(crate::FieldReader<bool, bool>);
impl PRESCALE_UPDATE_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PRESCALE_UPDATE_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRESCALE_UPDATE_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmpr1_clr_active` reader - cmpr1 clear transaction is pending"]
pub struct CMPR1_CLR_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CMPR1_CLR_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR1_CLR_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR1_CLR_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `cmpr0_clr_active` reader - cmpr0 clear transaction is pending"]
pub struct CMPR0_CLR_ACTIVE_R(crate::FieldReader<bool, bool>);
impl CMPR0_CLR_ACTIVE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CMPR0_CLR_ACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMPR0_CLR_ACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - RTC Transaction Pending"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Use Async RTC Flags"]
    #[inline(always)]
    pub fn use_async_flags(&self) -> USE_ASYNC_FLAGS_R {
        USE_ASYNC_FLAGS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Use Aggressive Reset Mode"]
    #[inline(always)]
    pub fn aggressive_rst(&self) -> AGGRESSIVE_RST_R {
        AGGRESSIVE_RST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable RTC in Active Modes"]
    #[inline(always)]
    pub fn en_active(&self) -> EN_ACTIVE_R {
        EN_ACTIVE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - osc_goto_low_r transaction is pending"]
    #[inline(always)]
    pub fn osc_goto_low_active(&self) -> OSC_GOTO_LOW_ACTIVE_R {
        OSC_GOTO_LOW_ACTIVE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - osc_force_mode transaction is pending"]
    #[inline(always)]
    pub fn osc_frce_sm_en_active(&self) -> OSC_FRCE_SM_EN_ACTIVE_R {
        OSC_FRCE_SM_EN_ACTIVE_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - osc_force_state transaction is pending"]
    #[inline(always)]
    pub fn osc_frce_st_active(&self) -> OSC_FRCE_ST_ACTIVE_R {
        OSC_FRCE_ST_ACTIVE_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - timer_set_active"]
    #[inline(always)]
    pub fn set_active(&self) -> SET_ACTIVE_R {
        SET_ACTIVE_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - RTC clear is pending"]
    #[inline(always)]
    pub fn clr_active(&self) -> CLR_ACTIVE_R {
        CLR_ACTIVE_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - rollover clr is pending"]
    #[inline(always)]
    pub fn rollover_clr_active(&self) -> ROLLOVER_CLR_ACTIVE_R {
        ROLLOVER_CLR_ACTIVE_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - prescale cmpr0 is pending"]
    #[inline(always)]
    pub fn prescale_cmpr0_active(&self) -> PRESCALE_CMPR0_ACTIVE_R {
        PRESCALE_CMPR0_ACTIVE_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - prescale update transaction is pending"]
    #[inline(always)]
    pub fn prescale_update_active(&self) -> PRESCALE_UPDATE_ACTIVE_R {
        PRESCALE_UPDATE_ACTIVE_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - cmpr1 clear transaction is pending"]
    #[inline(always)]
    pub fn cmpr1_clr_active(&self) -> CMPR1_CLR_ACTIVE_R {
        CMPR1_CLR_ACTIVE_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - cmpr0 clear transaction is pending"]
    #[inline(always)]
    pub fn cmpr0_clr_active(&self) -> CMPR0_CLR_ACTIVE_R {
        CMPR0_CLR_ACTIVE_R::new(((self.bits >> 26) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC Timer Enable"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bit 1 - RTC Timer Clear Bit"]
    #[inline(always)]
    pub fn clear(&mut self) -> CLEAR_W {
        CLEAR_W { w: self }
    }
    #[doc = "Bit 3 - Use Async RTC Flags"]
    #[inline(always)]
    pub fn use_async_flags(&mut self) -> USE_ASYNC_FLAGS_W {
        USE_ASYNC_FLAGS_W { w: self }
    }
    #[doc = "Bit 4 - Use Aggressive Reset Mode"]
    #[inline(always)]
    pub fn aggressive_rst(&mut self) -> AGGRESSIVE_RST_W {
        AGGRESSIVE_RST_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
