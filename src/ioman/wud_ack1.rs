#[doc = "Register `WUD_ACK1` reader"]
pub struct R(crate::R<WUD_ACK1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WUD_ACK1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WUD_ACK1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WUD_ACK1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WUD_ACK1` writer"]
pub struct W(crate::W<WUD_ACK1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WUD_ACK1_SPEC>;
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
impl From<crate::W<WUD_ACK1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WUD_ACK1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `wud_ack_p4` reader - WUD Mode Acknowledge: P4\\[7:0\\]"]
pub struct WUD_ACK_P4_R(crate::FieldReader<u8, u8>);
impl WUD_ACK_P4_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WUD_ACK_P4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUD_ACK_P4_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wud_ack_p5` reader - WUD Mode Acknowledge: P5\\[7:0\\]"]
pub struct WUD_ACK_P5_R(crate::FieldReader<u8, u8>);
impl WUD_ACK_P5_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        WUD_ACK_P5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUD_ACK_P5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `wud_ack_p6` reader - WUD Mode Acknowledge: P6\\[7:0\\]"]
pub struct WUD_ACK_P6_R(crate::FieldReader<bool, bool>);
impl WUD_ACK_P6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        WUD_ACK_P6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WUD_ACK_P6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - WUD Mode Acknowledge: P4\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_ack_p4(&self) -> WUD_ACK_P4_R {
        WUD_ACK_P4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - WUD Mode Acknowledge: P5\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_ack_p5(&self) -> WUD_ACK_P5_R {
        WUD_ACK_P5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - WUD Mode Acknowledge: P6\\[7:0\\]"]
    #[inline(always)]
    pub fn wud_ack_p6(&self) -> WUD_ACK_P6_R {
        WUD_ACK_P6_R::new(((self.bits >> 16) & 1) != 0)
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
#[doc = "Wakeup Detect Mode Acknowledge Register 1 (P4/P5/P6)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wud_ack1](index.html) module"]
pub struct WUD_ACK1_SPEC;
impl crate::RegisterSpec for WUD_ACK1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wud_ack1::R](R) reader structure"]
impl crate::Readable for WUD_ACK1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wud_ack1::W](W) writer structure"]
impl crate::Writable for WUD_ACK1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WUD_ACK1 to value 0"]
impl crate::Resettable for WUD_ACK1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
