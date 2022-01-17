#[doc = "Register `BB` reader"]
pub struct R(crate::R<BB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BB` writer"]
pub struct W(crate::W<BB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BB_SPEC>;
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
impl From<crate::W<BB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `bb_scl_out` reader - Bit Bang SCL Output"]
pub struct BB_SCL_OUT_R(crate::FieldReader<bool, bool>);
impl BB_SCL_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_SCL_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_SCL_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bb_scl_out` writer - Bit Bang SCL Output"]
pub struct BB_SCL_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_SCL_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `bb_sda_out` reader - Bit Bang SDA Output"]
pub struct BB_SDA_OUT_R(crate::FieldReader<bool, bool>);
impl BB_SDA_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_SDA_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_SDA_OUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bb_sda_out` writer - Bit Bang SDA Output"]
pub struct BB_SDA_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> BB_SDA_OUT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `bb_scl_in_val` reader - Bit Bang SCL Input Value"]
pub struct BB_SCL_IN_VAL_R(crate::FieldReader<bool, bool>);
impl BB_SCL_IN_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_SCL_IN_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_SCL_IN_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bb_sda_in_val` reader - Bit Bang SCL Input Value"]
pub struct BB_SDA_IN_VAL_R(crate::FieldReader<bool, bool>);
impl BB_SDA_IN_VAL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BB_SDA_IN_VAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BB_SDA_IN_VAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_cnt` reader - Results FIFO Data Received Count"]
pub struct RX_FIFO_CNT_R(crate::FieldReader<u8, u8>);
impl RX_FIFO_CNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        RX_FIFO_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RX_FIFO_CNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Bit Bang SCL Output"]
    #[inline(always)]
    pub fn bb_scl_out(&self) -> BB_SCL_OUT_R {
        BB_SCL_OUT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Bit Bang SDA Output"]
    #[inline(always)]
    pub fn bb_sda_out(&self) -> BB_SDA_OUT_R {
        BB_SDA_OUT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Bit Bang SCL Input Value"]
    #[inline(always)]
    pub fn bb_scl_in_val(&self) -> BB_SCL_IN_VAL_R {
        BB_SCL_IN_VAL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Bit Bang SCL Input Value"]
    #[inline(always)]
    pub fn bb_sda_in_val(&self) -> BB_SDA_IN_VAL_R {
        BB_SDA_IN_VAL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 16:20 - Results FIFO Data Received Count"]
    #[inline(always)]
    pub fn rx_fifo_cnt(&self) -> RX_FIFO_CNT_R {
        RX_FIFO_CNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Bit Bang SCL Output"]
    #[inline(always)]
    pub fn bb_scl_out(&mut self) -> BB_SCL_OUT_W {
        BB_SCL_OUT_W { w: self }
    }
    #[doc = "Bit 1 - Bit Bang SDA Output"]
    #[inline(always)]
    pub fn bb_sda_out(&mut self) -> BB_SDA_OUT_W {
        BB_SDA_OUT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit-Bang Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bb](index.html) module"]
pub struct BB_SPEC;
impl crate::RegisterSpec for BB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bb::R](R) reader structure"]
impl crate::Readable for BB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bb::W](W) writer structure"]
impl crate::Writable for BB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BB to value 0"]
impl crate::Resettable for BB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
