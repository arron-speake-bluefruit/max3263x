#[doc = "Register `CLK_CONFIG` reader"]
pub struct R(crate::R<CLK_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_CONFIG` writer"]
pub struct W(crate::W<CLK_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_CONFIG_SPEC>;
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
impl From<crate::W<CLK_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `crypto_enable` reader - Cryptographic (TPU) Relaxation Oscillator Enable"]
pub struct CRYPTO_ENABLE_R(crate::FieldReader<bool, bool>);
impl CRYPTO_ENABLE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        CRYPTO_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crypto_enable` writer - Cryptographic (TPU) Relaxation Oscillator Enable"]
pub struct CRYPTO_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_ENABLE_W<'a> {
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
#[doc = "Field `crypto_stability_count` reader - Crypto Oscillator Stability Select"]
pub struct CRYPTO_STABILITY_COUNT_R(crate::FieldReader<u8, u8>);
impl CRYPTO_STABILITY_COUNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        CRYPTO_STABILITY_COUNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRYPTO_STABILITY_COUNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `crypto_stability_count` writer - Crypto Oscillator Stability Select"]
pub struct CRYPTO_STABILITY_COUNT_W<'a> {
    w: &'a mut W,
}
impl<'a> CRYPTO_STABILITY_COUNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Cryptographic (TPU) Relaxation Oscillator Enable"]
    #[inline(always)]
    pub fn crypto_enable(&self) -> CRYPTO_ENABLE_R {
        CRYPTO_ENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 4:7 - Crypto Oscillator Stability Select"]
    #[inline(always)]
    pub fn crypto_stability_count(&self) -> CRYPTO_STABILITY_COUNT_R {
        CRYPTO_STABILITY_COUNT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cryptographic (TPU) Relaxation Oscillator Enable"]
    #[inline(always)]
    pub fn crypto_enable(&mut self) -> CRYPTO_ENABLE_W {
        CRYPTO_ENABLE_W { w: self }
    }
    #[doc = "Bits 4:7 - Crypto Oscillator Stability Select"]
    #[inline(always)]
    pub fn crypto_stability_count(&mut self) -> CRYPTO_STABILITY_COUNT_W {
        CRYPTO_STABILITY_COUNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System Clock Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_config](index.html) module"]
pub struct CLK_CONFIG_SPEC;
impl crate::RegisterSpec for CLK_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_config::R](R) reader structure"]
impl crate::Readable for CLK_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_config::W](W) writer structure"]
impl crate::Writable for CLK_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_CONFIG to value 0"]
impl crate::Resettable for CLK_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
