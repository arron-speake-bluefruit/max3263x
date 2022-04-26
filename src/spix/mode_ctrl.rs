#[doc = "Register `MODE_CTRL` reader"]
pub struct R(crate::R<MODE_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE_CTRL` writer"]
pub struct W(crate::W<MODE_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_CTRL_SPEC>;
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
impl From<crate::W<MODE_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `mode_clocks` reader - Mode Clocks"]
pub struct MODE_CLOCKS_R(crate::FieldReader<u8>);
impl MODE_CLOCKS_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        MODE_CLOCKS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MODE_CLOCKS_R {
    type Target = crate::FieldReader<u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `mode_clocks` writer - Mode Clocks"]
pub struct MODE_CLOCKS_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE_CLOCKS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `no_cmd_mode` reader - No Command Mode"]
pub struct NO_CMD_MODE_R(crate::FieldReader<bool>);
impl NO_CMD_MODE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NO_CMD_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NO_CMD_MODE_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `no_cmd_mode` writer - No Command Mode"]
pub struct NO_CMD_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_CMD_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 8)) | ((value as u32 & 1) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Mode Clocks"]
    #[inline(always)]
    pub fn mode_clocks(&self) -> MODE_CLOCKS_R {
        MODE_CLOCKS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - No Command Mode"]
    #[inline(always)]
    pub fn no_cmd_mode(&self) -> NO_CMD_MODE_R {
        NO_CMD_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Mode Clocks"]
    #[inline(always)]
    pub fn mode_clocks(&mut self) -> MODE_CLOCKS_W {
        MODE_CLOCKS_W { w: self }
    }
    #[doc = "Bit 8 - No Command Mode"]
    #[inline(always)]
    pub fn no_cmd_mode(&mut self) -> NO_CMD_MODE_W {
        NO_CMD_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPIX Mode Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode_ctrl](index.html) module"]
pub struct MODE_CTRL_SPEC;
impl crate::RegisterSpec for MODE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode_ctrl::R](R) reader structure"]
impl crate::Readable for MODE_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode_ctrl::W](W) writer structure"]
impl crate::Writable for MODE_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MODE_CTRL to value 0"]
impl crate::Resettable for MODE_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
