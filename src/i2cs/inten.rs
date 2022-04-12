#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `byte0` reader - Updated Byte 0"]
pub struct BYTE0_R(crate::FieldReader<bool, bool>);
impl BYTE0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte0` writer - Updated Byte 0"]
pub struct BYTE0_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE0_W<'a> {
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
        self.w.bits = (self.w.bits & !1) | (value as u32 & 1);
        self.w
    }
}
#[doc = "Field `byte1` reader - Updated Byte 1"]
pub struct BYTE1_R(crate::FieldReader<bool, bool>);
impl BYTE1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte1` writer - Updated Byte 1"]
pub struct BYTE1_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE1_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 1)) | ((value as u32 & 1) << 1);
        self.w
    }
}
#[doc = "Field `byte2` reader - Updated Byte 2"]
pub struct BYTE2_R(crate::FieldReader<bool, bool>);
impl BYTE2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte2` writer - Updated Byte 2"]
pub struct BYTE2_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 2)) | ((value as u32 & 1) << 2);
        self.w
    }
}
#[doc = "Field `byte3` reader - Updated Byte 3"]
pub struct BYTE3_R(crate::FieldReader<bool, bool>);
impl BYTE3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte3` writer - Updated Byte 3"]
pub struct BYTE3_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE3_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 3)) | ((value as u32 & 1) << 3);
        self.w
    }
}
#[doc = "Field `byte4` reader - Updated Byte 4"]
pub struct BYTE4_R(crate::FieldReader<bool, bool>);
impl BYTE4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte4` writer - Updated Byte 4"]
pub struct BYTE4_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE4_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 4)) | ((value as u32 & 1) << 4);
        self.w
    }
}
#[doc = "Field `byte5` reader - Updated Byte 5"]
pub struct BYTE5_R(crate::FieldReader<bool, bool>);
impl BYTE5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte5` writer - Updated Byte 5"]
pub struct BYTE5_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE5_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 5)) | ((value as u32 & 1) << 5);
        self.w
    }
}
#[doc = "Field `byte6` reader - Updated Byte 6"]
pub struct BYTE6_R(crate::FieldReader<bool, bool>);
impl BYTE6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte6` writer - Updated Byte 6"]
pub struct BYTE6_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE6_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 6)) | ((value as u32 & 1) << 6);
        self.w
    }
}
#[doc = "Field `byte7` reader - Updated Byte 7"]
pub struct BYTE7_R(crate::FieldReader<bool, bool>);
impl BYTE7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte7` writer - Updated Byte 7"]
pub struct BYTE7_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE7_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 7)) | ((value as u32 & 1) << 7);
        self.w
    }
}
#[doc = "Field `byte8` reader - Updated Byte 8"]
pub struct BYTE8_R(crate::FieldReader<bool, bool>);
impl BYTE8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte8` writer - Updated Byte 8"]
pub struct BYTE8_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE8_W<'a> {
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
#[doc = "Field `byte9` reader - Updated Byte 9"]
pub struct BYTE9_R(crate::FieldReader<bool, bool>);
impl BYTE9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte9` writer - Updated Byte 9"]
pub struct BYTE9_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE9_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 9)) | ((value as u32 & 1) << 9);
        self.w
    }
}
#[doc = "Field `byte10` reader - Updated Byte 10"]
pub struct BYTE10_R(crate::FieldReader<bool, bool>);
impl BYTE10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte10` writer - Updated Byte 10"]
pub struct BYTE10_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE10_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 10)) | ((value as u32 & 1) << 10);
        self.w
    }
}
#[doc = "Field `byte11` reader - Updated Byte 11"]
pub struct BYTE11_R(crate::FieldReader<bool, bool>);
impl BYTE11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte11` writer - Updated Byte 11"]
pub struct BYTE11_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE11_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 11)) | ((value as u32 & 1) << 11);
        self.w
    }
}
#[doc = "Field `byte12` reader - Updated Byte 12"]
pub struct BYTE12_R(crate::FieldReader<bool, bool>);
impl BYTE12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte12` writer - Updated Byte 12"]
pub struct BYTE12_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE12_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 12)) | ((value as u32 & 1) << 12);
        self.w
    }
}
#[doc = "Field `byte13` reader - Updated Byte 13"]
pub struct BYTE13_R(crate::FieldReader<bool, bool>);
impl BYTE13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte13` writer - Updated Byte 13"]
pub struct BYTE13_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE13_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 13)) | ((value as u32 & 1) << 13);
        self.w
    }
}
#[doc = "Field `byte14` reader - Updated Byte 14"]
pub struct BYTE14_R(crate::FieldReader<bool, bool>);
impl BYTE14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte14` writer - Updated Byte 14"]
pub struct BYTE14_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE14_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 14)) | ((value as u32 & 1) << 14);
        self.w
    }
}
#[doc = "Field `byte15` reader - Updated Byte 15"]
pub struct BYTE15_R(crate::FieldReader<bool, bool>);
impl BYTE15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte15` writer - Updated Byte 15"]
pub struct BYTE15_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE15_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 15)) | ((value as u32 & 1) << 15);
        self.w
    }
}
#[doc = "Field `byte16` reader - Updated Byte 16"]
pub struct BYTE16_R(crate::FieldReader<bool, bool>);
impl BYTE16_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE16_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE16_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte16` writer - Updated Byte 16"]
pub struct BYTE16_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE16_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 16)) | ((value as u32 & 1) << 16);
        self.w
    }
}
#[doc = "Field `byte17` reader - Updated Byte 17"]
pub struct BYTE17_R(crate::FieldReader<bool, bool>);
impl BYTE17_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE17_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte17` writer - Updated Byte 17"]
pub struct BYTE17_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE17_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 17)) | ((value as u32 & 1) << 17);
        self.w
    }
}
#[doc = "Field `byte18` reader - Updated Byte 18"]
pub struct BYTE18_R(crate::FieldReader<bool, bool>);
impl BYTE18_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE18_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE18_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte18` writer - Updated Byte 18"]
pub struct BYTE18_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE18_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 18)) | ((value as u32 & 1) << 18);
        self.w
    }
}
#[doc = "Field `byte19` reader - Updated Byte 19"]
pub struct BYTE19_R(crate::FieldReader<bool, bool>);
impl BYTE19_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE19_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE19_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte19` writer - Updated Byte 19"]
pub struct BYTE19_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE19_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 19)) | ((value as u32 & 1) << 19);
        self.w
    }
}
#[doc = "Field `byte20` reader - Updated Byte 20"]
pub struct BYTE20_R(crate::FieldReader<bool, bool>);
impl BYTE20_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE20_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte20` writer - Updated Byte 20"]
pub struct BYTE20_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE20_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 20)) | ((value as u32 & 1) << 20);
        self.w
    }
}
#[doc = "Field `byte21` reader - Updated Byte 21"]
pub struct BYTE21_R(crate::FieldReader<bool, bool>);
impl BYTE21_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE21_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte21` writer - Updated Byte 21"]
pub struct BYTE21_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE21_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 21)) | ((value as u32 & 1) << 21);
        self.w
    }
}
#[doc = "Field `byte22` reader - Updated Byte 22"]
pub struct BYTE22_R(crate::FieldReader<bool, bool>);
impl BYTE22_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE22_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE22_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte22` writer - Updated Byte 22"]
pub struct BYTE22_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE22_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 22)) | ((value as u32 & 1) << 22);
        self.w
    }
}
#[doc = "Field `byte23` reader - Updated Byte 23"]
pub struct BYTE23_R(crate::FieldReader<bool, bool>);
impl BYTE23_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE23_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE23_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte23` writer - Updated Byte 23"]
pub struct BYTE23_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE23_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 23)) | ((value as u32 & 1) << 23);
        self.w
    }
}
#[doc = "Field `byte24` reader - Updated Byte 24"]
pub struct BYTE24_R(crate::FieldReader<bool, bool>);
impl BYTE24_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE24_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE24_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte24` writer - Updated Byte 24"]
pub struct BYTE24_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE24_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 24)) | ((value as u32 & 1) << 24);
        self.w
    }
}
#[doc = "Field `byte25` reader - Updated Byte 25"]
pub struct BYTE25_R(crate::FieldReader<bool, bool>);
impl BYTE25_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE25_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte25` writer - Updated Byte 25"]
pub struct BYTE25_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE25_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 25)) | ((value as u32 & 1) << 25);
        self.w
    }
}
#[doc = "Field `byte26` reader - Updated Byte 26"]
pub struct BYTE26_R(crate::FieldReader<bool, bool>);
impl BYTE26_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE26_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE26_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte26` writer - Updated Byte 26"]
pub struct BYTE26_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE26_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 26)) | ((value as u32 & 1) << 26);
        self.w
    }
}
#[doc = "Field `byte27` reader - Updated Byte 27"]
pub struct BYTE27_R(crate::FieldReader<bool, bool>);
impl BYTE27_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE27_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE27_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte27` writer - Updated Byte 27"]
pub struct BYTE27_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE27_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 27)) | ((value as u32 & 1) << 27);
        self.w
    }
}
#[doc = "Field `byte28` reader - Updated Byte 28"]
pub struct BYTE28_R(crate::FieldReader<bool, bool>);
impl BYTE28_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE28_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte28` writer - Updated Byte 28"]
pub struct BYTE28_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE28_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 28)) | ((value as u32 & 1) << 28);
        self.w
    }
}
#[doc = "Field `byte29` reader - Updated Byte 29"]
pub struct BYTE29_R(crate::FieldReader<bool, bool>);
impl BYTE29_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE29_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE29_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte29` writer - Updated Byte 29"]
pub struct BYTE29_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE29_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 29)) | ((value as u32 & 1) << 29);
        self.w
    }
}
#[doc = "Field `byte30` reader - Updated Byte 30"]
pub struct BYTE30_R(crate::FieldReader<bool, bool>);
impl BYTE30_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE30_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE30_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte30` writer - Updated Byte 30"]
pub struct BYTE30_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE30_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 30)) | ((value as u32 & 1) << 30);
        self.w
    }
}
#[doc = "Field `byte31` reader - Updated Byte 31"]
pub struct BYTE31_R(crate::FieldReader<bool, bool>);
impl BYTE31_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BYTE31_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BYTE31_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `byte31` writer - Updated Byte 31"]
pub struct BYTE31_W<'a> {
    w: &'a mut W,
}
impl<'a> BYTE31_W<'a> {
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
        self.w.bits = (self.w.bits & !(1 << 31)) | ((value as u32 & 1) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Updated Byte 0"]
    #[inline(always)]
    pub fn byte0(&self) -> BYTE0_R {
        BYTE0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Updated Byte 1"]
    #[inline(always)]
    pub fn byte1(&self) -> BYTE1_R {
        BYTE1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Updated Byte 2"]
    #[inline(always)]
    pub fn byte2(&self) -> BYTE2_R {
        BYTE2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Updated Byte 3"]
    #[inline(always)]
    pub fn byte3(&self) -> BYTE3_R {
        BYTE3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Updated Byte 4"]
    #[inline(always)]
    pub fn byte4(&self) -> BYTE4_R {
        BYTE4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Updated Byte 5"]
    #[inline(always)]
    pub fn byte5(&self) -> BYTE5_R {
        BYTE5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Updated Byte 6"]
    #[inline(always)]
    pub fn byte6(&self) -> BYTE6_R {
        BYTE6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Updated Byte 7"]
    #[inline(always)]
    pub fn byte7(&self) -> BYTE7_R {
        BYTE7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Updated Byte 8"]
    #[inline(always)]
    pub fn byte8(&self) -> BYTE8_R {
        BYTE8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Updated Byte 9"]
    #[inline(always)]
    pub fn byte9(&self) -> BYTE9_R {
        BYTE9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Updated Byte 10"]
    #[inline(always)]
    pub fn byte10(&self) -> BYTE10_R {
        BYTE10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Updated Byte 11"]
    #[inline(always)]
    pub fn byte11(&self) -> BYTE11_R {
        BYTE11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Updated Byte 12"]
    #[inline(always)]
    pub fn byte12(&self) -> BYTE12_R {
        BYTE12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Updated Byte 13"]
    #[inline(always)]
    pub fn byte13(&self) -> BYTE13_R {
        BYTE13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Updated Byte 14"]
    #[inline(always)]
    pub fn byte14(&self) -> BYTE14_R {
        BYTE14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Updated Byte 15"]
    #[inline(always)]
    pub fn byte15(&self) -> BYTE15_R {
        BYTE15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Updated Byte 16"]
    #[inline(always)]
    pub fn byte16(&self) -> BYTE16_R {
        BYTE16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Updated Byte 17"]
    #[inline(always)]
    pub fn byte17(&self) -> BYTE17_R {
        BYTE17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Updated Byte 18"]
    #[inline(always)]
    pub fn byte18(&self) -> BYTE18_R {
        BYTE18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Updated Byte 19"]
    #[inline(always)]
    pub fn byte19(&self) -> BYTE19_R {
        BYTE19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Updated Byte 20"]
    #[inline(always)]
    pub fn byte20(&self) -> BYTE20_R {
        BYTE20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Updated Byte 21"]
    #[inline(always)]
    pub fn byte21(&self) -> BYTE21_R {
        BYTE21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Updated Byte 22"]
    #[inline(always)]
    pub fn byte22(&self) -> BYTE22_R {
        BYTE22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Updated Byte 23"]
    #[inline(always)]
    pub fn byte23(&self) -> BYTE23_R {
        BYTE23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Updated Byte 24"]
    #[inline(always)]
    pub fn byte24(&self) -> BYTE24_R {
        BYTE24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Updated Byte 25"]
    #[inline(always)]
    pub fn byte25(&self) -> BYTE25_R {
        BYTE25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Updated Byte 26"]
    #[inline(always)]
    pub fn byte26(&self) -> BYTE26_R {
        BYTE26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Updated Byte 27"]
    #[inline(always)]
    pub fn byte27(&self) -> BYTE27_R {
        BYTE27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Updated Byte 28"]
    #[inline(always)]
    pub fn byte28(&self) -> BYTE28_R {
        BYTE28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Updated Byte 29"]
    #[inline(always)]
    pub fn byte29(&self) -> BYTE29_R {
        BYTE29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Updated Byte 30"]
    #[inline(always)]
    pub fn byte30(&self) -> BYTE30_R {
        BYTE30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Updated Byte 31"]
    #[inline(always)]
    pub fn byte31(&self) -> BYTE31_R {
        BYTE31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Updated Byte 0"]
    #[inline(always)]
    pub fn byte0(&mut self) -> BYTE0_W {
        BYTE0_W { w: self }
    }
    #[doc = "Bit 1 - Updated Byte 1"]
    #[inline(always)]
    pub fn byte1(&mut self) -> BYTE1_W {
        BYTE1_W { w: self }
    }
    #[doc = "Bit 2 - Updated Byte 2"]
    #[inline(always)]
    pub fn byte2(&mut self) -> BYTE2_W {
        BYTE2_W { w: self }
    }
    #[doc = "Bit 3 - Updated Byte 3"]
    #[inline(always)]
    pub fn byte3(&mut self) -> BYTE3_W {
        BYTE3_W { w: self }
    }
    #[doc = "Bit 4 - Updated Byte 4"]
    #[inline(always)]
    pub fn byte4(&mut self) -> BYTE4_W {
        BYTE4_W { w: self }
    }
    #[doc = "Bit 5 - Updated Byte 5"]
    #[inline(always)]
    pub fn byte5(&mut self) -> BYTE5_W {
        BYTE5_W { w: self }
    }
    #[doc = "Bit 6 - Updated Byte 6"]
    #[inline(always)]
    pub fn byte6(&mut self) -> BYTE6_W {
        BYTE6_W { w: self }
    }
    #[doc = "Bit 7 - Updated Byte 7"]
    #[inline(always)]
    pub fn byte7(&mut self) -> BYTE7_W {
        BYTE7_W { w: self }
    }
    #[doc = "Bit 8 - Updated Byte 8"]
    #[inline(always)]
    pub fn byte8(&mut self) -> BYTE8_W {
        BYTE8_W { w: self }
    }
    #[doc = "Bit 9 - Updated Byte 9"]
    #[inline(always)]
    pub fn byte9(&mut self) -> BYTE9_W {
        BYTE9_W { w: self }
    }
    #[doc = "Bit 10 - Updated Byte 10"]
    #[inline(always)]
    pub fn byte10(&mut self) -> BYTE10_W {
        BYTE10_W { w: self }
    }
    #[doc = "Bit 11 - Updated Byte 11"]
    #[inline(always)]
    pub fn byte11(&mut self) -> BYTE11_W {
        BYTE11_W { w: self }
    }
    #[doc = "Bit 12 - Updated Byte 12"]
    #[inline(always)]
    pub fn byte12(&mut self) -> BYTE12_W {
        BYTE12_W { w: self }
    }
    #[doc = "Bit 13 - Updated Byte 13"]
    #[inline(always)]
    pub fn byte13(&mut self) -> BYTE13_W {
        BYTE13_W { w: self }
    }
    #[doc = "Bit 14 - Updated Byte 14"]
    #[inline(always)]
    pub fn byte14(&mut self) -> BYTE14_W {
        BYTE14_W { w: self }
    }
    #[doc = "Bit 15 - Updated Byte 15"]
    #[inline(always)]
    pub fn byte15(&mut self) -> BYTE15_W {
        BYTE15_W { w: self }
    }
    #[doc = "Bit 16 - Updated Byte 16"]
    #[inline(always)]
    pub fn byte16(&mut self) -> BYTE16_W {
        BYTE16_W { w: self }
    }
    #[doc = "Bit 17 - Updated Byte 17"]
    #[inline(always)]
    pub fn byte17(&mut self) -> BYTE17_W {
        BYTE17_W { w: self }
    }
    #[doc = "Bit 18 - Updated Byte 18"]
    #[inline(always)]
    pub fn byte18(&mut self) -> BYTE18_W {
        BYTE18_W { w: self }
    }
    #[doc = "Bit 19 - Updated Byte 19"]
    #[inline(always)]
    pub fn byte19(&mut self) -> BYTE19_W {
        BYTE19_W { w: self }
    }
    #[doc = "Bit 20 - Updated Byte 20"]
    #[inline(always)]
    pub fn byte20(&mut self) -> BYTE20_W {
        BYTE20_W { w: self }
    }
    #[doc = "Bit 21 - Updated Byte 21"]
    #[inline(always)]
    pub fn byte21(&mut self) -> BYTE21_W {
        BYTE21_W { w: self }
    }
    #[doc = "Bit 22 - Updated Byte 22"]
    #[inline(always)]
    pub fn byte22(&mut self) -> BYTE22_W {
        BYTE22_W { w: self }
    }
    #[doc = "Bit 23 - Updated Byte 23"]
    #[inline(always)]
    pub fn byte23(&mut self) -> BYTE23_W {
        BYTE23_W { w: self }
    }
    #[doc = "Bit 24 - Updated Byte 24"]
    #[inline(always)]
    pub fn byte24(&mut self) -> BYTE24_W {
        BYTE24_W { w: self }
    }
    #[doc = "Bit 25 - Updated Byte 25"]
    #[inline(always)]
    pub fn byte25(&mut self) -> BYTE25_W {
        BYTE25_W { w: self }
    }
    #[doc = "Bit 26 - Updated Byte 26"]
    #[inline(always)]
    pub fn byte26(&mut self) -> BYTE26_W {
        BYTE26_W { w: self }
    }
    #[doc = "Bit 27 - Updated Byte 27"]
    #[inline(always)]
    pub fn byte27(&mut self) -> BYTE27_W {
        BYTE27_W { w: self }
    }
    #[doc = "Bit 28 - Updated Byte 28"]
    #[inline(always)]
    pub fn byte28(&mut self) -> BYTE28_W {
        BYTE28_W { w: self }
    }
    #[doc = "Bit 29 - Updated Byte 29"]
    #[inline(always)]
    pub fn byte29(&mut self) -> BYTE29_W {
        BYTE29_W { w: self }
    }
    #[doc = "Bit 30 - Updated Byte 30"]
    #[inline(always)]
    pub fn byte30(&mut self) -> BYTE30_W {
        BYTE30_W { w: self }
    }
    #[doc = "Bit 31 - Updated Byte 31"]
    #[inline(always)]
    pub fn byte31(&mut self) -> BYTE31_W {
        BYTE31_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I2CS Interrupt Enable/Disable Controls\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
