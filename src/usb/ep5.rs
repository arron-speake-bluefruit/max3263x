#[doc = "Register `EP5` reader"]
pub struct R(crate::R<EP5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP5` writer"]
pub struct W(crate::W<EP5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP5_SPEC>;
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
impl From<crate::W<EP5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ep_dir` reader - Endpoint Direction"]
pub struct EP_DIR_R(crate::FieldReader<u8, u8>);
impl EP_DIR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        EP_DIR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DIR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_dir` writer - Endpoint Direction"]
pub struct EP_DIR_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DIR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `ep_buf2` reader - Endpoint Double Buffered Enable"]
pub struct EP_BUF2_R(crate::FieldReader<bool, bool>);
impl EP_BUF2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_BUF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_BUF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_buf2` writer - Endpoint Double Buffered Enable"]
pub struct EP_BUF2_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_BUF2_W<'a> {
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
#[doc = "Field `ep_int_en` reader - Endpoint Transfer Complete Interrupt Enable"]
pub struct EP_INT_EN_R(crate::FieldReader<bool, bool>);
impl EP_INT_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_INT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_INT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_int_en` writer - Endpoint Transfer Complete Interrupt Enable"]
pub struct EP_INT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_INT_EN_W<'a> {
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
#[doc = "Field `ep_nak_en` reader - Endpoint NAK Interrupt Enable"]
pub struct EP_NAK_EN_R(crate::FieldReader<bool, bool>);
impl EP_NAK_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_NAK_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_NAK_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_nak_en` writer - Endpoint NAK Interrupt Enable"]
pub struct EP_NAK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_NAK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `ep_dt` reader - Endpoint Data Toggle Clear"]
pub struct EP_DT_R(crate::FieldReader<bool, bool>);
impl EP_DT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_DT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_DT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_dt` writer - Endpoint Data Toggle Clear"]
pub struct EP_DT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_DT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `ep_stall` reader - Endpoint Stall"]
pub struct EP_STALL_R(crate::FieldReader<bool, bool>);
impl EP_STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_stall` writer - Endpoint Stall"]
pub struct EP_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_STALL_W<'a> {
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
#[doc = "Field `ep_st_stall` reader - Endpoint Stall Status Stage of Control Transfer"]
pub struct EP_ST_STALL_R(crate::FieldReader<bool, bool>);
impl EP_ST_STALL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_ST_STALL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_ST_STALL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_st_stall` writer - Endpoint Stall Status Stage of Control Transfer"]
pub struct EP_ST_STALL_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_ST_STALL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `ep_st_ack` reader - Endpoint Acknowledge Status Stage of Control Transfer"]
pub struct EP_ST_ACK_R(crate::FieldReader<bool, bool>);
impl EP_ST_ACK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_ST_ACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_ST_ACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_st_ack` writer - Endpoint Acknowledge Status Stage of Control Transfer"]
pub struct EP_ST_ACK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_ST_ACK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Endpoint Direction"]
    #[inline(always)]
    pub fn ep_dir(&self) -> EP_DIR_R {
        EP_DIR_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 3 - Endpoint Double Buffered Enable"]
    #[inline(always)]
    pub fn ep_buf2(&self) -> EP_BUF2_R {
        EP_BUF2_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Endpoint Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ep_int_en(&self) -> EP_INT_EN_R {
        EP_INT_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Endpoint NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep_nak_en(&self) -> EP_NAK_EN_R {
        EP_NAK_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Endpoint Data Toggle Clear"]
    #[inline(always)]
    pub fn ep_dt(&self) -> EP_DT_R {
        EP_DT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Endpoint Stall"]
    #[inline(always)]
    pub fn ep_stall(&self) -> EP_STALL_R {
        EP_STALL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Endpoint Stall Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_stall(&self) -> EP_ST_STALL_R {
        EP_ST_STALL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Endpoint Acknowledge Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_ack(&self) -> EP_ST_ACK_R {
        EP_ST_ACK_R::new(((self.bits >> 10) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Endpoint Direction"]
    #[inline(always)]
    pub fn ep_dir(&mut self) -> EP_DIR_W {
        EP_DIR_W { w: self }
    }
    #[doc = "Bit 3 - Endpoint Double Buffered Enable"]
    #[inline(always)]
    pub fn ep_buf2(&mut self) -> EP_BUF2_W {
        EP_BUF2_W { w: self }
    }
    #[doc = "Bit 4 - Endpoint Transfer Complete Interrupt Enable"]
    #[inline(always)]
    pub fn ep_int_en(&mut self) -> EP_INT_EN_W {
        EP_INT_EN_W { w: self }
    }
    #[doc = "Bit 5 - Endpoint NAK Interrupt Enable"]
    #[inline(always)]
    pub fn ep_nak_en(&mut self) -> EP_NAK_EN_W {
        EP_NAK_EN_W { w: self }
    }
    #[doc = "Bit 6 - Endpoint Data Toggle Clear"]
    #[inline(always)]
    pub fn ep_dt(&mut self) -> EP_DT_W {
        EP_DT_W { w: self }
    }
    #[doc = "Bit 8 - Endpoint Stall"]
    #[inline(always)]
    pub fn ep_stall(&mut self) -> EP_STALL_W {
        EP_STALL_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint Stall Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_stall(&mut self) -> EP_ST_STALL_W {
        EP_ST_STALL_W { w: self }
    }
    #[doc = "Bit 10 - Endpoint Acknowledge Status Stage of Control Transfer"]
    #[inline(always)]
    pub fn ep_st_ack(&mut self) -> EP_ST_ACK_W {
        EP_ST_ACK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Endpoint 5 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep5](index.html) module"]
pub struct EP5_SPEC;
impl crate::RegisterSpec for EP5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep5::R](R) reader structure"]
impl crate::Readable for EP5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep5::W](W) writer structure"]
impl crate::Writable for EP5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP5 to value 0"]
impl crate::Resettable for EP5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
