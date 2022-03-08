#[doc = "Register `CM4_START_CLK_EN1` reader"]
pub struct R(crate::R<CM4_START_CLK_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM4_START_CLK_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM4_START_CLK_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM4_START_CLK_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM4_START_CLK_EN1` writer"]
pub struct W(crate::W<CM4_START_CLK_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM4_START_CLK_EN1_SPEC>;
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
impl From<crate::W<CM4_START_CLK_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM4_START_CLK_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ints` reader - Interrupt Sources 32-63"]
pub struct INTS_R(crate::FieldReader<u32, u32>);
impl INTS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u32) -> Self {
        INTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INTS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ints` writer - Interrupt Sources 32-63"]
pub struct INTS_W<'a> {
    w: &'a mut W,
}
impl<'a> INTS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = value;
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Interrupt Sources 32-63"]
    #[inline(always)]
    pub fn ints(&self) -> INTS_R {
        INTS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Interrupt Sources 32-63"]
    #[inline(always)]
    pub fn ints(&mut self) -> INTS_W {
        INTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CM4 Start Clock on Interrupt Enable 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm4_start_clk_en1](index.html) module"]
pub struct CM4_START_CLK_EN1_SPEC;
impl crate::RegisterSpec for CM4_START_CLK_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm4_start_clk_en1::R](R) reader structure"]
impl crate::Readable for CM4_START_CLK_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm4_start_clk_en1::W](W) writer structure"]
impl crate::Writable for CM4_START_CLK_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CM4_START_CLK_EN1 to value 0"]
impl crate::Resettable for CM4_START_CLK_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
