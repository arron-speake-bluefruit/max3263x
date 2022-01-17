#[doc = "Register `SYS_CLK_CTRL_16_SPIS` reader"]
pub struct R(crate::R<SYS_CLK_CTRL_16_SPIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYS_CLK_CTRL_16_SPIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYS_CLK_CTRL_16_SPIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYS_CLK_CTRL_16_SPIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYS_CLK_CTRL_16_SPIS` writer"]
pub struct W(crate::W<SYS_CLK_CTRL_16_SPIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYS_CLK_CTRL_16_SPIS_SPEC>;
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
impl From<crate::W<SYS_CLK_CTRL_16_SPIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYS_CLK_CTRL_16_SPIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spis_clk_scale` reader - Control Settings for CLK16 - SPI Slave Clock"]
pub struct SPIS_CLK_SCALE_R(crate::FieldReader<u8, u8>);
impl SPIS_CLK_SCALE_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SPIS_CLK_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPIS_CLK_SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `spis_clk_scale` writer - Control Settings for CLK16 - SPI Slave Clock"]
pub struct SPIS_CLK_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> SPIS_CLK_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Control Settings for CLK16 - SPI Slave Clock"]
    #[inline(always)]
    pub fn spis_clk_scale(&self) -> SPIS_CLK_SCALE_R {
        SPIS_CLK_SCALE_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Control Settings for CLK16 - SPI Slave Clock"]
    #[inline(always)]
    pub fn spis_clk_scale(&mut self) -> SPIS_CLK_SCALE_W {
        SPIS_CLK_SCALE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Settings for CLK16 - SPI Slave Clock\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sys_clk_ctrl_16_spis](index.html) module"]
pub struct SYS_CLK_CTRL_16_SPIS_SPEC;
impl crate::RegisterSpec for SYS_CLK_CTRL_16_SPIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sys_clk_ctrl_16_spis::R](R) reader structure"]
impl crate::Readable for SYS_CLK_CTRL_16_SPIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sys_clk_ctrl_16_spis::W](W) writer structure"]
impl crate::Writable for SYS_CLK_CTRL_16_SPIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYS_CLK_CTRL_16_SPIS to value 0"]
impl crate::Resettable for SYS_CLK_CTRL_16_SPIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
