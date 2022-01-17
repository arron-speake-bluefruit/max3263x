#[doc = "Register `SECURITY` reader"]
pub struct R(crate::R<SECURITY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SECURITY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SECURITY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SECURITY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SECURITY` writer"]
pub struct W(crate::W<SECURITY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SECURITY_SPEC>;
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
impl From<crate::W<SECURITY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SECURITY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `debug_disable` reader - Debug Lockout"]
pub struct DEBUG_DISABLE_R(crate::FieldReader<u8, u8>);
impl DEBUG_DISABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DEBUG_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEBUG_DISABLE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `debug_disable` writer - Debug Lockout"]
pub struct DEBUG_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> DEBUG_DISABLE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `mass_erase_lock` reader - Mass Erase Lockout"]
pub struct MASS_ERASE_LOCK_R(crate::FieldReader<u8, u8>);
impl MASS_ERASE_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MASS_ERASE_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASS_ERASE_LOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mass_erase_lock` writer - Mass Erase Lockout"]
pub struct MASS_ERASE_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASS_ERASE_LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | ((value as u32 & 0x0f) << 8);
        self.w
    }
}
#[doc = "Field `disable_ahb_wr` reader - Disable AHB Flash Write Operations"]
pub struct DISABLE_AHB_WR_R(crate::FieldReader<u8, u8>);
impl DISABLE_AHB_WR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        DISABLE_AHB_WR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DISABLE_AHB_WR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `disable_ahb_wr` writer - Disable AHB Flash Write Operations"]
pub struct DISABLE_AHB_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> DISABLE_AHB_WR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | ((value as u32 & 0x0f) << 16);
        self.w
    }
}
#[doc = "Field `flc_settings_lock` reader - FLC Settings Lock"]
pub struct FLC_SETTINGS_LOCK_R(crate::FieldReader<u8, u8>);
impl FLC_SETTINGS_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        FLC_SETTINGS_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLC_SETTINGS_LOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `flc_settings_lock` writer - FLC Settings Lock"]
pub struct FLC_SETTINGS_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> FLC_SETTINGS_LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `security_lock` reader - Security Lock"]
pub struct SECURITY_LOCK_R(crate::FieldReader<u8, u8>);
impl SECURITY_LOCK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SECURITY_LOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SECURITY_LOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `security_lock` writer - Security Lock"]
pub struct SECURITY_LOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> SECURITY_LOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 28)) | ((value as u32 & 0x0f) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Debug Lockout"]
    #[inline(always)]
    pub fn debug_disable(&self) -> DEBUG_DISABLE_R {
        DEBUG_DISABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Mass Erase Lockout"]
    #[inline(always)]
    pub fn mass_erase_lock(&self) -> MASS_ERASE_LOCK_R {
        MASS_ERASE_LOCK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Disable AHB Flash Write Operations"]
    #[inline(always)]
    pub fn disable_ahb_wr(&self) -> DISABLE_AHB_WR_R {
        DISABLE_AHB_WR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - FLC Settings Lock"]
    #[inline(always)]
    pub fn flc_settings_lock(&self) -> FLC_SETTINGS_LOCK_R {
        FLC_SETTINGS_LOCK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Security Lock"]
    #[inline(always)]
    pub fn security_lock(&self) -> SECURITY_LOCK_R {
        SECURITY_LOCK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Debug Lockout"]
    #[inline(always)]
    pub fn debug_disable(&mut self) -> DEBUG_DISABLE_W {
        DEBUG_DISABLE_W { w: self }
    }
    #[doc = "Bits 8:11 - Mass Erase Lockout"]
    #[inline(always)]
    pub fn mass_erase_lock(&mut self) -> MASS_ERASE_LOCK_W {
        MASS_ERASE_LOCK_W { w: self }
    }
    #[doc = "Bits 16:19 - Disable AHB Flash Write Operations"]
    #[inline(always)]
    pub fn disable_ahb_wr(&mut self) -> DISABLE_AHB_WR_W {
        DISABLE_AHB_WR_W { w: self }
    }
    #[doc = "Bits 24:27 - FLC Settings Lock"]
    #[inline(always)]
    pub fn flc_settings_lock(&mut self) -> FLC_SETTINGS_LOCK_W {
        FLC_SETTINGS_LOCK_W { w: self }
    }
    #[doc = "Bits 28:31 - Security Lock"]
    #[inline(always)]
    pub fn security_lock(&mut self) -> SECURITY_LOCK_W {
        SECURITY_LOCK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Controller Security Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [security](index.html) module"]
pub struct SECURITY_SPEC;
impl crate::RegisterSpec for SECURITY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [security::R](R) reader structure"]
impl crate::Readable for SECURITY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [security::W](W) writer structure"]
impl crate::Writable for SECURITY_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SECURITY to value 0"]
impl crate::Resettable for SECURITY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
