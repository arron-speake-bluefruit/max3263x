#[doc = "Register `INTEN1` reader"]
pub struct R(crate::R<INTEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN1` writer"]
pub struct W(crate::W<INTEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN1_SPEC>;
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
impl From<crate::W<INTEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `sram_addr_wrapped` reader - SRAM Address Wrapped Interrupt Enable/Disable"]
pub struct SRAM_ADDR_WRAPPED_R(crate::FieldReader<bool, bool>);
impl SRAM_ADDR_WRAPPED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SRAM_ADDR_WRAPPED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_ADDR_WRAPPED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `sram_addr_wrapped` writer - SRAM Address Wrapped Interrupt Enable/Disable"]
pub struct SRAM_ADDR_WRAPPED_W<'a> {
    w: &'a mut W,
}
impl<'a> SRAM_ADDR_WRAPPED_W<'a> {
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
#[doc = "Field `invalid_flash_addr` reader - Invalid Flash Address Interrupt Enable/Disable"]
pub struct INVALID_FLASH_ADDR_R(crate::FieldReader<bool, bool>);
impl INVALID_FLASH_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INVALID_FLASH_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVALID_FLASH_ADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `invalid_flash_addr` writer - Invalid Flash Address Interrupt Enable/Disable"]
pub struct INVALID_FLASH_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> INVALID_FLASH_ADDR_W<'a> {
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
#[doc = "Field `flash_read_locked` reader - Flash Read from Locked Area Interrupt Enable/Disable"]
pub struct FLASH_READ_LOCKED_R(crate::FieldReader<bool, bool>);
impl FLASH_READ_LOCKED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLASH_READ_LOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_READ_LOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `flash_read_locked` writer - Flash Read from Locked Area Interrupt Enable/Disable"]
pub struct FLASH_READ_LOCKED_W<'a> {
    w: &'a mut W,
}
impl<'a> FLASH_READ_LOCKED_W<'a> {
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
#[doc = "Field `trim_update_done` reader - Trim Update Complete Interrupt Enable/Disable"]
pub struct TRIM_UPDATE_DONE_R(crate::FieldReader<bool, bool>);
impl TRIM_UPDATE_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TRIM_UPDATE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRIM_UPDATE_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `trim_update_done` writer - Trim Update Complete Interrupt Enable/Disable"]
pub struct TRIM_UPDATE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> TRIM_UPDATE_DONE_W<'a> {
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
#[doc = "Field `flc_state_done` reader - FLC State Machine Reached DONE Interrupt Enable/Disable"]
pub struct FLC_STATE_DONE_R(crate::FieldReader<bool, bool>);
impl FLC_STATE_DONE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLC_STATE_DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLC_STATE_DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `flc_state_done` writer - FLC State Machine Reached DONE Interrupt Enable/Disable"]
pub struct FLC_STATE_DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLC_STATE_DONE_W<'a> {
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
#[doc = "Field `flc_prog_complete` reader - Program (Write or Erase) Op Completed Int Enable/Disable"]
pub struct FLC_PROG_COMPLETE_R(crate::FieldReader<bool, bool>);
impl FLC_PROG_COMPLETE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FLC_PROG_COMPLETE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLC_PROG_COMPLETE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `flc_prog_complete` writer - Program (Write or Erase) Op Completed Int Enable/Disable"]
pub struct FLC_PROG_COMPLETE_W<'a> {
    w: &'a mut W,
}
impl<'a> FLC_PROG_COMPLETE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - SRAM Address Wrapped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn sram_addr_wrapped(&self) -> SRAM_ADDR_WRAPPED_R {
        SRAM_ADDR_WRAPPED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Invalid Flash Address Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn invalid_flash_addr(&self) -> INVALID_FLASH_ADDR_R {
        INVALID_FLASH_ADDR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Flash Read from Locked Area Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn flash_read_locked(&self) -> FLASH_READ_LOCKED_R {
        FLASH_READ_LOCKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Trim Update Complete Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn trim_update_done(&self) -> TRIM_UPDATE_DONE_R {
        TRIM_UPDATE_DONE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLC State Machine Reached DONE Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn flc_state_done(&self) -> FLC_STATE_DONE_R {
        FLC_STATE_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Program (Write or Erase) Op Completed Int Enable/Disable"]
    #[inline(always)]
    pub fn flc_prog_complete(&self) -> FLC_PROG_COMPLETE_R {
        FLC_PROG_COMPLETE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SRAM Address Wrapped Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn sram_addr_wrapped(&mut self) -> SRAM_ADDR_WRAPPED_W {
        SRAM_ADDR_WRAPPED_W { w: self }
    }
    #[doc = "Bit 1 - Invalid Flash Address Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn invalid_flash_addr(&mut self) -> INVALID_FLASH_ADDR_W {
        INVALID_FLASH_ADDR_W { w: self }
    }
    #[doc = "Bit 2 - Flash Read from Locked Area Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn flash_read_locked(&mut self) -> FLASH_READ_LOCKED_W {
        FLASH_READ_LOCKED_W { w: self }
    }
    #[doc = "Bit 3 - Trim Update Complete Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn trim_update_done(&mut self) -> TRIM_UPDATE_DONE_W {
        TRIM_UPDATE_DONE_W { w: self }
    }
    #[doc = "Bit 4 - FLC State Machine Reached DONE Interrupt Enable/Disable"]
    #[inline(always)]
    pub fn flc_state_done(&mut self) -> FLC_STATE_DONE_W {
        FLC_STATE_DONE_W { w: self }
    }
    #[doc = "Bit 5 - Program (Write or Erase) Op Completed Int Enable/Disable"]
    #[inline(always)]
    pub fn flc_prog_complete(&mut self) -> FLC_PROG_COMPLETE_W {
        FLC_PROG_COMPLETE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable/Disable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten1](index.html) module"]
pub struct INTEN1_SPEC;
impl crate::RegisterSpec for INTEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten1::R](R) reader structure"]
impl crate::Readable for INTEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten1::W](W) writer structure"]
impl crate::Writable for INTEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN1 to value 0"]
impl crate::Resettable for INTEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
