#[doc = "Register `FUNC_SEL_P6` reader"]
pub struct R(crate::R<FUNC_SEL_P6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FUNC_SEL_P6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FUNC_SEL_P6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FUNC_SEL_P6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FUNC_SEL_P6` writer"]
pub struct W(crate::W<FUNC_SEL_P6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FUNC_SEL_P6_SPEC>;
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
impl From<crate::W<FUNC_SEL_P6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FUNC_SEL_P6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P6.0 Output Function Select"]
pub type PIN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin0` writer - P6.0 Output Function Select"]
pub type PIN0_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 0>;
#[doc = "Field `pin1` reader - P6.1 Output Function Select"]
pub type PIN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin1` writer - P6.1 Output Function Select"]
pub type PIN1_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 4>;
#[doc = "Field `pin2` reader - P6.2 Output Function Select"]
pub type PIN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin2` writer - P6.2 Output Function Select"]
pub type PIN2_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 8>;
#[doc = "Field `pin3` reader - P6.3 Output Function Select"]
pub type PIN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin3` writer - P6.3 Output Function Select"]
pub type PIN3_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 12>;
#[doc = "Field `pin4` reader - P6.4 Output Function Select"]
pub type PIN4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin4` writer - P6.4 Output Function Select"]
pub type PIN4_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 16>;
#[doc = "Field `pin5` reader - P6.5 Output Function Select"]
pub type PIN5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin5` writer - P6.5 Output Function Select"]
pub type PIN5_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 20>;
#[doc = "Field `pin6` reader - P6.6 Output Function Select"]
pub type PIN6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin6` writer - P6.6 Output Function Select"]
pub type PIN6_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 24>;
#[doc = "Field `pin7` reader - P6.7 Output Function Select"]
pub type PIN7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin7` writer - P6.7 Output Function Select"]
pub type PIN7_W<'a> = crate::FieldWriter<'a, u32, FUNC_SEL_P6_SPEC, u8, u8, 4, 28>;
impl R {
    #[doc = "Bits 0:3 - P6.0 Output Function Select"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - P6.1 Output Function Select"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - P6.2 Output Function Select"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - P6.3 Output Function Select"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - P6.4 Output Function Select"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - P6.5 Output Function Select"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - P6.6 Output Function Select"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - P6.7 Output Function Select"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - P6.0 Output Function Select"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W::new(self)
    }
    #[doc = "Bits 4:7 - P6.1 Output Function Select"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W::new(self)
    }
    #[doc = "Bits 8:11 - P6.2 Output Function Select"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W::new(self)
    }
    #[doc = "Bits 12:15 - P6.3 Output Function Select"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W::new(self)
    }
    #[doc = "Bits 16:19 - P6.4 Output Function Select"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W::new(self)
    }
    #[doc = "Bits 20:23 - P6.5 Output Function Select"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W::new(self)
    }
    #[doc = "Bits 24:27 - P6.6 Output Function Select"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W::new(self)
    }
    #[doc = "Bits 28:31 - P6.7 Output Function Select"]
    #[inline(always)]
    pub fn pin7(&mut self) -> PIN7_W {
        PIN7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port P6 GPIO Function Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [func_sel_p6](index.html) module"]
pub struct FUNC_SEL_P6_SPEC;
impl crate::RegisterSpec for FUNC_SEL_P6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [func_sel_p6::R](R) reader structure"]
impl crate::Readable for FUNC_SEL_P6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [func_sel_p6::W](W) writer structure"]
impl crate::Writable for FUNC_SEL_P6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FUNC_SEL_P6 to value 0"]
impl crate::Resettable for FUNC_SEL_P6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
