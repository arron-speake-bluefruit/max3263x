#[doc = "Register `MD_CTRL` reader"]
pub struct R(crate::R<MD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MD_CTRL` writer"]
pub struct W(crate::W<MD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MD_CTRL_SPEC>;
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
impl From<crate::W<MD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `slave_addr` reader - Slave Address"]
pub struct SLAVE_ADDR_R(crate::FieldReader<u8, u8>);
impl SLAVE_ADDR_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slave_addr` writer - Slave Address"]
pub struct SLAVE_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `slave_addr_msk` reader - Slave Address Mask"]
pub struct SLAVE_ADDR_MSK_R(crate::FieldReader<u8, u8>);
impl SLAVE_ADDR_MSK_R {
    #[inline(always)]
    pub(crate) fn new(bits: u8) -> Self {
        SLAVE_ADDR_MSK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SLAVE_ADDR_MSK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `slave_addr_msk` writer - Slave Address Mask"]
pub struct SLAVE_ADDR_MSK_W<'a> {
    w: &'a mut W,
}
impl<'a> SLAVE_ADDR_MSK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `md_mstr` reader - Multidrop Master"]
pub struct MD_MSTR_R(crate::FieldReader<bool, bool>);
impl MD_MSTR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        MD_MSTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MD_MSTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `md_mstr` writer - Multidrop Master"]
pub struct MD_MSTR_W<'a> {
    w: &'a mut W,
}
impl<'a> MD_MSTR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `tx_addr_mark` reader - RX Address Mark"]
pub struct TX_ADDR_MARK_R(crate::FieldReader<bool, bool>);
impl TX_ADDR_MARK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_ADDR_MARK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TX_ADDR_MARK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_addr_mark` writer - RX Address Mark"]
pub struct TX_ADDR_MARK_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_ADDR_MARK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - Slave Address"]
    #[inline(always)]
    pub fn slave_addr(&self) -> SLAVE_ADDR_R {
        SLAVE_ADDR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Slave Address Mask"]
    #[inline(always)]
    pub fn slave_addr_msk(&self) -> SLAVE_ADDR_MSK_R {
        SLAVE_ADDR_MSK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Multidrop Master"]
    #[inline(always)]
    pub fn md_mstr(&self) -> MD_MSTR_R {
        MD_MSTR_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - RX Address Mark"]
    #[inline(always)]
    pub fn tx_addr_mark(&self) -> TX_ADDR_MARK_R {
        TX_ADDR_MARK_R::new(((self.bits >> 17) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Slave Address"]
    #[inline(always)]
    pub fn slave_addr(&mut self) -> SLAVE_ADDR_W {
        SLAVE_ADDR_W { w: self }
    }
    #[doc = "Bits 8:15 - Slave Address Mask"]
    #[inline(always)]
    pub fn slave_addr_msk(&mut self) -> SLAVE_ADDR_MSK_W {
        SLAVE_ADDR_MSK_W { w: self }
    }
    #[doc = "Bit 16 - Multidrop Master"]
    #[inline(always)]
    pub fn md_mstr(&mut self) -> MD_MSTR_W {
        MD_MSTR_W { w: self }
    }
    #[doc = "Bit 17 - RX Address Mark"]
    #[inline(always)]
    pub fn tx_addr_mark(&mut self) -> TX_ADDR_MARK_W {
        TX_ADDR_MARK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "UART Multidrop Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [md_ctrl](index.html) module"]
pub struct MD_CTRL_SPEC;
impl crate::RegisterSpec for MD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [md_ctrl::R](R) reader structure"]
impl crate::Readable for MD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [md_ctrl::W](W) writer structure"]
impl crate::Writable for MD_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MD_CTRL to value 0"]
impl crate::Resettable for MD_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
