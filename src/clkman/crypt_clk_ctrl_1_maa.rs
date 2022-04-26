#[doc = "Register `CRYPT_CLK_CTRL_1_MAA` reader"]
pub struct R(crate::R<CRYPT_CLK_CTRL_1_MAA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CRYPT_CLK_CTRL_1_MAA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CRYPT_CLK_CTRL_1_MAA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CRYPT_CLK_CTRL_1_MAA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CRYPT_CLK_CTRL_1_MAA` writer"]
pub struct W(crate::W<CRYPT_CLK_CTRL_1_MAA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CRYPT_CLK_CTRL_1_MAA_SPEC>;
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
impl From<crate::W<CRYPT_CLK_CTRL_1_MAA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CRYPT_CLK_CTRL_1_MAA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `maa_clk_scale` reader - Control Settings for Crypto Clock 1 - MAA"]
pub struct MAA_CLK_SCALE_R(crate::FieldReader<u8>);
impl MAA_CLK_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MAA_CLK_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MAA_CLK_SCALE_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `maa_clk_scale` writer - Control Settings for Crypto Clock 1 - MAA"]
pub struct MAA_CLK_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> MAA_CLK_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Control Settings for Crypto Clock 1 - MAA"]
    #[inline(always)]
    pub fn maa_clk_scale(&self) -> MAA_CLK_SCALE_R {
        MAA_CLK_SCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control Settings for Crypto Clock 1 - MAA"]
    #[inline(always)]
    pub fn maa_clk_scale(&mut self) -> MAA_CLK_SCALE_W {
        MAA_CLK_SCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Settings for Crypto Clock 1 - MAA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [crypt_clk_ctrl_1_maa](index.html) module"]
pub struct CRYPT_CLK_CTRL_1_MAA_SPEC;
impl crate::RegisterSpec for CRYPT_CLK_CTRL_1_MAA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [crypt_clk_ctrl_1_maa::R](R) reader structure"]
impl crate::Readable for CRYPT_CLK_CTRL_1_MAA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [crypt_clk_ctrl_1_maa::W](W) writer structure"]
impl crate::Writable for CRYPT_CLK_CTRL_1_MAA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CRYPT_CLK_CTRL_1_MAA to value 0"]
impl crate::Resettable for CRYPT_CLK_CTRL_1_MAA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
