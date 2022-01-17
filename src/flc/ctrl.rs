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
#[doc = "Field `write` reader - Start Flash Write Operation"]
pub struct WRITE_R(crate::FieldReader<bool, bool>);
impl WRITE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `write` writer - Start Flash Write Operation"]
pub struct WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> WRITE_W<'a> {
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
#[doc = "Field `mass_erase` reader - Start Flash Mass Erase Operation"]
pub struct MASS_ERASE_R(crate::FieldReader<bool, bool>);
impl MASS_ERASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MASS_ERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASS_ERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mass_erase` writer - Start Flash Mass Erase Operation"]
pub struct MASS_ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> MASS_ERASE_W<'a> {
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
#[doc = "Field `page_erase` reader - Start Flash Page Erase Operation"]
pub struct PAGE_ERASE_R(crate::FieldReader<bool, bool>);
impl PAGE_ERASE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PAGE_ERASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PAGE_ERASE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `page_erase` writer - Start Flash Page Erase Operation"]
pub struct PAGE_ERASE_W<'a> {
    w: &'a mut W,
}
impl<'a> PAGE_ERASE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `erase_code` reader - Flash Erase Code"]
pub struct ERASE_CODE_R(crate::FieldReader<u8, u8>);
impl ERASE_CODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        ERASE_CODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERASE_CODE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `erase_code` writer - Flash Erase Code"]
pub struct ERASE_CODE_W<'a> {
    w: &'a mut W,
}
impl<'a> ERASE_CODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `info_block_unlock` reader - Flash Info Block Locked"]
pub struct INFO_BLOCK_UNLOCK_R(crate::FieldReader<bool, bool>);
impl INFO_BLOCK_UNLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFO_BLOCK_UNLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFO_BLOCK_UNLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `write_enable` reader - Flash Writes Enabled"]
pub struct WRITE_ENABLE_R(crate::FieldReader<bool, bool>);
impl WRITE_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WRITE_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRITE_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pending` reader - Flash Controller Status"]
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
#[doc = "Field `info_block_valid` reader - Info Block Valid Status"]
pub struct INFO_BLOCK_VALID_R(crate::FieldReader<bool, bool>);
impl INFO_BLOCK_VALID_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        INFO_BLOCK_VALID_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INFO_BLOCK_VALID_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `auto_incre_mode` reader - Address Auto-Increment Mode"]
pub struct AUTO_INCRE_MODE_R(crate::FieldReader<bool, bool>);
impl AUTO_INCRE_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_INCRE_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_INCRE_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `auto_incre_mode` writer - Address Auto-Increment Mode"]
pub struct AUTO_INCRE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_INCRE_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "Field `flsh_unlock` reader - Flash Write/Erase Enable"]
pub struct FLSH_UNLOCK_R(crate::FieldReader<u8, u8>);
impl FLSH_UNLOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLSH_UNLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLSH_UNLOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `flsh_unlock` writer - Flash Write/Erase Enable"]
pub struct FLSH_UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLSH_UNLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Start Flash Write Operation"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Start Flash Mass Erase Operation"]
    #[inline(always)]
    pub fn mass_erase(&self) -> MASS_ERASE_R {
        MASS_ERASE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Start Flash Page Erase Operation"]
    #[inline(always)]
    pub fn page_erase(&self) -> PAGE_ERASE_R {
        PAGE_ERASE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - Flash Erase Code"]
    #[inline(always)]
    pub fn erase_code(&self) -> ERASE_CODE_R {
        ERASE_CODE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Flash Info Block Locked"]
    #[inline(always)]
    pub fn info_block_unlock(&self) -> INFO_BLOCK_UNLOCK_R {
        INFO_BLOCK_UNLOCK_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Flash Writes Enabled"]
    #[inline(always)]
    pub fn write_enable(&self) -> WRITE_ENABLE_R {
        WRITE_ENABLE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Flash Controller Status"]
    #[inline(always)]
    pub fn pending(&self) -> PENDING_R {
        PENDING_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Info Block Valid Status"]
    #[inline(always)]
    pub fn info_block_valid(&self) -> INFO_BLOCK_VALID_R {
        INFO_BLOCK_VALID_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Address Auto-Increment Mode"]
    #[inline(always)]
    pub fn auto_incre_mode(&self) -> AUTO_INCRE_MODE_R {
        AUTO_INCRE_MODE_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:31 - Flash Write/Erase Enable"]
    #[inline(always)]
    pub fn flsh_unlock(&self) -> FLSH_UNLOCK_R {
        FLSH_UNLOCK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Flash Write Operation"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W { w: self }
    }
    #[doc = "Bit 1 - Start Flash Mass Erase Operation"]
    #[inline(always)]
    pub fn mass_erase(&mut self) -> MASS_ERASE_W {
        MASS_ERASE_W { w: self }
    }
    #[doc = "Bit 2 - Start Flash Page Erase Operation"]
    #[inline(always)]
    pub fn page_erase(&mut self) -> PAGE_ERASE_W {
        PAGE_ERASE_W { w: self }
    }
    #[doc = "Bits 8:15 - Flash Erase Code"]
    #[inline(always)]
    pub fn erase_code(&mut self) -> ERASE_CODE_W {
        ERASE_CODE_W { w: self }
    }
    #[doc = "Bit 27 - Address Auto-Increment Mode"]
    #[inline(always)]
    pub fn auto_incre_mode(&mut self) -> AUTO_INCRE_MODE_W {
        AUTO_INCRE_MODE_W { w: self }
    }
    #[doc = "Bits 28:31 - Flash Write/Erase Enable"]
    #[inline(always)]
    pub fn flsh_unlock(&mut self) -> FLSH_UNLOCK_W {
        FLSH_UNLOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
