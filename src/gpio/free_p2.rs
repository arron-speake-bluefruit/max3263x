#[doc = "Register `FREE_P2` reader"]
pub struct R(crate::R<FREE_P2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FREE_P2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FREE_P2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FREE_P2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FREE_P2` writer"]
pub struct W(crate::W<FREE_P2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FREE_P2_SPEC>;
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
impl From<crate::W<FREE_P2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FREE_P2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pin0` reader - P2.0 GPIO Mode Acknowledge"]
pub struct PIN0_R(crate::FieldReader<bool, bool>);
impl PIN0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin1` reader - P2.1 GPIO Mode Acknowledge"]
pub struct PIN1_R(crate::FieldReader<bool, bool>);
impl PIN1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin2` reader - P2.2 GPIO Mode Acknowledge"]
pub struct PIN2_R(crate::FieldReader<bool, bool>);
impl PIN2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin3` reader - P2.3 GPIO Mode Acknowledge"]
pub struct PIN3_R(crate::FieldReader<bool, bool>);
impl PIN3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin4` reader - P2.4 GPIO Mode Acknowledge"]
pub struct PIN4_R(crate::FieldReader<bool, bool>);
impl PIN4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin5` reader - P2.5 GPIO Mode Acknowledge"]
pub struct PIN5_R(crate::FieldReader<bool, bool>);
impl PIN5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin6` reader - P2.6 GPIO Mode Acknowledge"]
pub struct PIN6_R(crate::FieldReader<bool, bool>);
impl PIN6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pin7` reader - P2.7 GPIO Mode Acknowledge"]
pub struct PIN7_R(crate::FieldReader<bool, bool>);
impl PIN7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PIN7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PIN7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - P2.0 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin0(&self) -> PIN0_R {
        PIN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - P2.1 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin1(&self) -> PIN1_R {
        PIN1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - P2.2 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin2(&self) -> PIN2_R {
        PIN2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - P2.3 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin3(&self) -> PIN3_R {
        PIN3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - P2.4 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin4(&self) -> PIN4_R {
        PIN4_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - P2.5 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin5(&self) -> PIN5_R {
        PIN5_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - P2.6 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin6(&self) -> PIN6_R {
        PIN6_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - P2.7 GPIO Mode Acknowledge"]
    #[inline(always)]
    pub fn pin7(&self) -> PIN7_R {
        PIN7_R::new(((self.bits >> 7) & 0x01) != 0)
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
#[doc = "Port P2 Free for GPIO Operation Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [free_p2](index.html) module"]
pub struct FREE_P2_SPEC;
impl crate::RegisterSpec for FREE_P2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [free_p2::R](R) reader structure"]
impl crate::Readable for FREE_P2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [free_p2::W](W) writer structure"]
impl crate::Writable for FREE_P2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FREE_P2 to value 0"]
impl crate::Resettable for FREE_P2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
