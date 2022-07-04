#[doc = "Register `RST_MODE_P3` reader"]
pub struct R(crate::R<RST_MODE_P3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RST_MODE_P3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RST_MODE_P3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RST_MODE_P3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RST_MODE_P3` writer"]
pub struct W(crate::W<RST_MODE_P3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RST_MODE_P3_SPEC>;
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
impl From<crate::W<RST_MODE_P3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RST_MODE_P3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P3.0 Default Output Drive Mode"]
pub type PIN0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin0` writer - P3.0 Default Output Drive Mode"]
pub type PIN0_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 0>;
#[doc = "Field `pin1` reader - P3.1 Default Output Drive Mode"]
pub type PIN1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin1` writer - P3.1 Default Output Drive Mode"]
pub type PIN1_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 4>;
#[doc = "Field `pin2` reader - P3.2 Default Output Drive Mode"]
pub type PIN2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin2` writer - P3.2 Default Output Drive Mode"]
pub type PIN2_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 8>;
#[doc = "Field `pin3` reader - P3.3 Default Output Drive Mode"]
pub type PIN3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin3` writer - P3.3 Default Output Drive Mode"]
pub type PIN3_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 12>;
#[doc = "Field `pin4` reader - P3.4 Default Output Drive Mode"]
pub type PIN4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin4` writer - P3.4 Default Output Drive Mode"]
pub type PIN4_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 16>;
#[doc = "Field `pin5` reader - P3.5 Default Output Drive Mode"]
pub type PIN5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin5` writer - P3.5 Default Output Drive Mode"]
pub type PIN5_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 20>;
#[doc = "Field `pin6` reader - P3.6 Default Output Drive Mode"]
pub type PIN6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin6` writer - P3.6 Default Output Drive Mode"]
pub type PIN6_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 24>;
#[doc = "Field `pin7` reader - P3.7 Default Output Drive Mode"]
pub type PIN7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `pin7` writer - P3.7 Default Output Drive Mode"]
pub type PIN7_W<'a> = crate::FieldWriter<'a, u32, RST_MODE_P3_SPEC, u8, u8, 3, 28>;
impl R {
    #[doc = "Bits 0:2 - P3.0 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - P3.1 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - P3.2 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - P3.3 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - P3.4 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - P3.5 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - P3.6 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - P3.7 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - P3.0 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W::new(self)
    }
    #[doc = "Bits 4:6 - P3.1 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W::new(self)
    }
    #[doc = "Bits 8:10 - P3.2 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W::new(self)
    }
    #[doc = "Bits 12:14 - P3.3 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W::new(self)
    }
    #[doc = "Bits 16:18 - P3.4 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W::new(self)
    }
    #[doc = "Bits 20:22 - P3.5 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W::new(self)
    }
    #[doc = "Bits 24:26 - P3.6 Default Output Drive Mode"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W::new(self)
    }
    #[doc = "Bits 28:30 - P3.7 Default Output Drive Mode"]
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
#[doc = "Port P3 Default (Power-On Reset) Output Drive Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rst_mode_p3](index.html) module"]
pub struct RST_MODE_P3_SPEC;
impl crate::RegisterSpec for RST_MODE_P3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rst_mode_p3::R](R) reader structure"]
impl crate::Readable for RST_MODE_P3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rst_mode_p3::W](W) writer structure"]
impl crate::Writable for RST_MODE_P3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RST_MODE_P3 to value 0"]
impl crate::Resettable for RST_MODE_P3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
