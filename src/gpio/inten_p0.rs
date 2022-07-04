#[doc = "Register `INTEN_P0` reader"]
pub struct R(crate::R<INTEN_P0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_P0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_P0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_P0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN_P0` writer"]
pub struct W(crate::W<INTEN_P0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_P0_SPEC>;
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
impl From<crate::W<INTEN_P0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_P0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P0.0 External Interrupt Enable"]
pub type PIN0_R = crate::BitReader<bool>;
#[doc = "Field `pin0` writer - P0.0 External Interrupt Enable"]
pub type PIN0_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 0>;
#[doc = "Field `pin1` reader - P0.1 External Interrupt Enable"]
pub type PIN1_R = crate::BitReader<bool>;
#[doc = "Field `pin1` writer - P0.1 External Interrupt Enable"]
pub type PIN1_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 1>;
#[doc = "Field `pin2` reader - P0.2 External Interrupt Enable"]
pub type PIN2_R = crate::BitReader<bool>;
#[doc = "Field `pin2` writer - P0.2 External Interrupt Enable"]
pub type PIN2_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 2>;
#[doc = "Field `pin3` reader - P0.3 External Interrupt Enable"]
pub type PIN3_R = crate::BitReader<bool>;
#[doc = "Field `pin3` writer - P0.3 External Interrupt Enable"]
pub type PIN3_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 3>;
#[doc = "Field `pin4` reader - P0.4 External Interrupt Enable"]
pub type PIN4_R = crate::BitReader<bool>;
#[doc = "Field `pin4` writer - P0.4 External Interrupt Enable"]
pub type PIN4_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 4>;
#[doc = "Field `pin5` reader - P0.5 External Interrupt Enable"]
pub type PIN5_R = crate::BitReader<bool>;
#[doc = "Field `pin5` writer - P0.5 External Interrupt Enable"]
pub type PIN5_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 5>;
#[doc = "Field `pin6` reader - P0.6 External Interrupt Enable"]
pub type PIN6_R = crate::BitReader<bool>;
#[doc = "Field `pin6` writer - P0.6 External Interrupt Enable"]
pub type PIN6_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 6>;
#[doc = "Field `pin7` reader - P0.7 External Interrupt Enable"]
pub type PIN7_R = crate::BitReader<bool>;
#[doc = "Field `pin7` writer - P0.7 External Interrupt Enable"]
pub type PIN7_W<'a> = crate::BitWriter<'a, u32, INTEN_P0_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - P0.0 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - P0.1 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - P0.2 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - P0.3 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - P0.4 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - P0.5 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - P0.6 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - P0.7 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - P0.0 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin0(&mut self) -> PIN0_W {
        PIN0_W::new(self)
    }
    #[doc = "Bit 1 - P0.1 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin1(&mut self) -> PIN1_W {
        PIN1_W::new(self)
    }
    #[doc = "Bit 2 - P0.2 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin2(&mut self) -> PIN2_W {
        PIN2_W::new(self)
    }
    #[doc = "Bit 3 - P0.3 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin3(&mut self) -> PIN3_W {
        PIN3_W::new(self)
    }
    #[doc = "Bit 4 - P0.4 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin4(&mut self) -> PIN4_W {
        PIN4_W::new(self)
    }
    #[doc = "Bit 5 - P0.5 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin5(&mut self) -> PIN5_W {
        PIN5_W::new(self)
    }
    #[doc = "Bit 6 - P0.6 External Interrupt Enable"]
    #[inline(always)]
    pub fn pin6(&mut self) -> PIN6_W {
        PIN6_W::new(self)
    }
    #[doc = "Bit 7 - P0.7 External Interrupt Enable"]
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
#[doc = "Port P0 Interrupt Enables\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten_p0](index.html) module"]
pub struct INTEN_P0_SPEC;
impl crate::RegisterSpec for INTEN_P0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten_p0::R](R) reader structure"]
impl crate::Readable for INTEN_P0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten_p0::W](W) writer structure"]
impl crate::Writable for INTEN_P0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN_P0 to value 0"]
impl crate::Resettable for INTEN_P0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
