#[doc = "Register `RESYNC` reader"]
pub struct R(crate::R<RESYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RESYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RESYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RESYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RESYNC` writer"]
pub struct W(crate::W<RESYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RESYNC_SPEC>;
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
impl From<crate::W<RESYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RESYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pt0` reader - Resync control for PT0"]
pub struct PT0_R(crate::FieldReader<bool, bool>);
impl PT0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt0` writer - Resync control for PT0"]
pub struct PT0_W<'a> {
    w: &'a mut W,
}
impl<'a> PT0_W<'a> {
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
#[doc = "Field `pt1` reader - Resync control for PT1"]
pub struct PT1_R(crate::FieldReader<bool, bool>);
impl PT1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt1` writer - Resync control for PT1"]
pub struct PT1_W<'a> {
    w: &'a mut W,
}
impl<'a> PT1_W<'a> {
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
#[doc = "Field `pt2` reader - Resync control for PT2"]
pub struct PT2_R(crate::FieldReader<bool, bool>);
impl PT2_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt2` writer - Resync control for PT2"]
pub struct PT2_W<'a> {
    w: &'a mut W,
}
impl<'a> PT2_W<'a> {
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
#[doc = "Field `pt3` reader - Resync control for PT3"]
pub struct PT3_R(crate::FieldReader<bool, bool>);
impl PT3_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt3` writer - Resync control for PT3"]
pub struct PT3_W<'a> {
    w: &'a mut W,
}
impl<'a> PT3_W<'a> {
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
#[doc = "Field `pt4` reader - Resync control for PT4"]
pub struct PT4_R(crate::FieldReader<bool, bool>);
impl PT4_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt4` writer - Resync control for PT4"]
pub struct PT4_W<'a> {
    w: &'a mut W,
}
impl<'a> PT4_W<'a> {
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
#[doc = "Field `pt5` reader - Resync control for PT5"]
pub struct PT5_R(crate::FieldReader<bool, bool>);
impl PT5_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT5_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt5` writer - Resync control for PT5"]
pub struct PT5_W<'a> {
    w: &'a mut W,
}
impl<'a> PT5_W<'a> {
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
#[doc = "Field `pt6` reader - Resync control for PT6"]
pub struct PT6_R(crate::FieldReader<bool, bool>);
impl PT6_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt6` writer - Resync control for PT6"]
pub struct PT6_W<'a> {
    w: &'a mut W,
}
impl<'a> PT6_W<'a> {
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
#[doc = "Field `pt7` reader - Resync control for PT7"]
pub struct PT7_R(crate::FieldReader<bool, bool>);
impl PT7_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt7` writer - Resync control for PT7"]
pub struct PT7_W<'a> {
    w: &'a mut W,
}
impl<'a> PT7_W<'a> {
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
#[doc = "Field `pt8` reader - Resync control for PT8"]
pub struct PT8_R(crate::FieldReader<bool, bool>);
impl PT8_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT8_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT8_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt8` writer - Resync control for PT8"]
pub struct PT8_W<'a> {
    w: &'a mut W,
}
impl<'a> PT8_W<'a> {
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
#[doc = "Field `pt9` reader - Resync control for PT9"]
pub struct PT9_R(crate::FieldReader<bool, bool>);
impl PT9_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT9_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT9_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt9` writer - Resync control for PT9"]
pub struct PT9_W<'a> {
    w: &'a mut W,
}
impl<'a> PT9_W<'a> {
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
#[doc = "Field `pt10` reader - Resync control for PT10"]
pub struct PT10_R(crate::FieldReader<bool, bool>);
impl PT10_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT10_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt10` writer - Resync control for PT10"]
pub struct PT10_W<'a> {
    w: &'a mut W,
}
impl<'a> PT10_W<'a> {
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
#[doc = "Field `pt11` reader - Resync control for PT11"]
pub struct PT11_R(crate::FieldReader<bool, bool>);
impl PT11_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT11_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt11` writer - Resync control for PT11"]
pub struct PT11_W<'a> {
    w: &'a mut W,
}
impl<'a> PT11_W<'a> {
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
#[doc = "Field `pt12` reader - Resync control for PT12"]
pub struct PT12_R(crate::FieldReader<bool, bool>);
impl PT12_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT12_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT12_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt12` writer - Resync control for PT12"]
pub struct PT12_W<'a> {
    w: &'a mut W,
}
impl<'a> PT12_W<'a> {
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
#[doc = "Field `pt13` reader - Resync control for PT13"]
pub struct PT13_R(crate::FieldReader<bool, bool>);
impl PT13_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT13_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT13_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt13` writer - Resync control for PT13"]
pub struct PT13_W<'a> {
    w: &'a mut W,
}
impl<'a> PT13_W<'a> {
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
#[doc = "Field `pt14` reader - Resync control for PT14"]
pub struct PT14_R(crate::FieldReader<bool, bool>);
impl PT14_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT14_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt14` writer - Resync control for PT14"]
pub struct PT14_W<'a> {
    w: &'a mut W,
}
impl<'a> PT14_W<'a> {
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
#[doc = "Field `pt15` reader - Resync control for PT15"]
pub struct PT15_R(crate::FieldReader<bool, bool>);
impl PT15_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PT15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PT15_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pt15` writer - Resync control for PT15"]
pub struct PT15_W<'a> {
    w: &'a mut W,
}
impl<'a> PT15_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    pub fn pt0(&self) -> PT0_R {
        PT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    pub fn pt1(&self) -> PT1_R {
        PT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    pub fn pt2(&self) -> PT2_R {
        PT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    pub fn pt3(&self) -> PT3_R {
        PT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Resync control for PT4"]
    #[inline(always)]
    pub fn pt4(&self) -> PT4_R {
        PT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Resync control for PT5"]
    #[inline(always)]
    pub fn pt5(&self) -> PT5_R {
        PT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Resync control for PT6"]
    #[inline(always)]
    pub fn pt6(&self) -> PT6_R {
        PT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Resync control for PT7"]
    #[inline(always)]
    pub fn pt7(&self) -> PT7_R {
        PT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Resync control for PT8"]
    #[inline(always)]
    pub fn pt8(&self) -> PT8_R {
        PT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Resync control for PT9"]
    #[inline(always)]
    pub fn pt9(&self) -> PT9_R {
        PT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Resync control for PT10"]
    #[inline(always)]
    pub fn pt10(&self) -> PT10_R {
        PT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Resync control for PT11"]
    #[inline(always)]
    pub fn pt11(&self) -> PT11_R {
        PT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Resync control for PT12"]
    #[inline(always)]
    pub fn pt12(&self) -> PT12_R {
        PT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Resync control for PT13"]
    #[inline(always)]
    pub fn pt13(&self) -> PT13_R {
        PT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Resync control for PT14"]
    #[inline(always)]
    pub fn pt14(&self) -> PT14_R {
        PT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Resync control for PT15"]
    #[inline(always)]
    pub fn pt15(&self) -> PT15_R {
        PT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Resync control for PT0"]
    #[inline(always)]
    pub fn pt0(&mut self) -> PT0_W {
        PT0_W { w: self }
    }
    #[doc = "Bit 1 - Resync control for PT1"]
    #[inline(always)]
    pub fn pt1(&mut self) -> PT1_W {
        PT1_W { w: self }
    }
    #[doc = "Bit 2 - Resync control for PT2"]
    #[inline(always)]
    pub fn pt2(&mut self) -> PT2_W {
        PT2_W { w: self }
    }
    #[doc = "Bit 3 - Resync control for PT3"]
    #[inline(always)]
    pub fn pt3(&mut self) -> PT3_W {
        PT3_W { w: self }
    }
    #[doc = "Bit 4 - Resync control for PT4"]
    #[inline(always)]
    pub fn pt4(&mut self) -> PT4_W {
        PT4_W { w: self }
    }
    #[doc = "Bit 5 - Resync control for PT5"]
    #[inline(always)]
    pub fn pt5(&mut self) -> PT5_W {
        PT5_W { w: self }
    }
    #[doc = "Bit 6 - Resync control for PT6"]
    #[inline(always)]
    pub fn pt6(&mut self) -> PT6_W {
        PT6_W { w: self }
    }
    #[doc = "Bit 7 - Resync control for PT7"]
    #[inline(always)]
    pub fn pt7(&mut self) -> PT7_W {
        PT7_W { w: self }
    }
    #[doc = "Bit 8 - Resync control for PT8"]
    #[inline(always)]
    pub fn pt8(&mut self) -> PT8_W {
        PT8_W { w: self }
    }
    #[doc = "Bit 9 - Resync control for PT9"]
    #[inline(always)]
    pub fn pt9(&mut self) -> PT9_W {
        PT9_W { w: self }
    }
    #[doc = "Bit 10 - Resync control for PT10"]
    #[inline(always)]
    pub fn pt10(&mut self) -> PT10_W {
        PT10_W { w: self }
    }
    #[doc = "Bit 11 - Resync control for PT11"]
    #[inline(always)]
    pub fn pt11(&mut self) -> PT11_W {
        PT11_W { w: self }
    }
    #[doc = "Bit 12 - Resync control for PT12"]
    #[inline(always)]
    pub fn pt12(&mut self) -> PT12_W {
        PT12_W { w: self }
    }
    #[doc = "Bit 13 - Resync control for PT13"]
    #[inline(always)]
    pub fn pt13(&mut self) -> PT13_W {
        PT13_W { w: self }
    }
    #[doc = "Bit 14 - Resync control for PT14"]
    #[inline(always)]
    pub fn pt14(&mut self) -> PT14_W {
        PT14_W { w: self }
    }
    #[doc = "Bit 15 - Resync control for PT15"]
    #[inline(always)]
    pub fn pt15(&mut self) -> PT15_W {
        PT15_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Global Resync (All Pulse Trains) Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [resync](index.html) module"]
pub struct RESYNC_SPEC;
impl crate::RegisterSpec for RESYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [resync::R](R) reader structure"]
impl crate::Readable for RESYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [resync::W](W) writer structure"]
impl crate::Writable for RESYNC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RESYNC to value 0"]
impl crate::Resettable for RESYNC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
