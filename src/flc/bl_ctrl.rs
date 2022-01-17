#[doc = "Register `BL_CTRL` reader"]
pub struct R(crate::R<BL_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BL_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BL_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BL_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BL_CTRL` writer"]
pub struct W(crate::W<BL_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BL_CTRL_SPEC>;
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
impl From<crate::W<BL_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BL_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
impl W {
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bootloader Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bl_ctrl](index.html) module"]
pub struct BL_CTRL_SPEC;
impl crate::RegisterSpec for BL_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bl_ctrl::R](R) reader structure"]
impl crate::Readable for BL_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bl_ctrl::W](W) writer structure"]
impl crate::Writable for BL_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BL_CTRL to value 0"]
impl crate::Resettable for BL_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
