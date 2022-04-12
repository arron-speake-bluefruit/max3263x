#[doc = "Register `PERFORM` reader"]
pub struct R(crate::R<PERFORM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERFORM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERFORM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERFORM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERFORM` writer"]
pub struct W(crate::W<PERFORM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERFORM_SPEC>;
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
impl From<crate::W<PERFORM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERFORM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `delay_se_en` reader - Delay SE Enable (Deprecated)"]
pub struct DELAY_SE_EN_R(crate::FieldReader<bool, bool>);
impl DELAY_SE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DELAY_SE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DELAY_SE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `delay_se_en` writer - Delay SE Enable (Deprecated)"]
pub struct DELAY_SE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DELAY_SE_EN_W<'a> {
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
#[doc = "Field `fast_read_mode_en` reader - Fast Read Mode Enable (Deprecated)"]
pub struct FAST_READ_MODE_EN_R(crate::FieldReader<bool, bool>);
impl FAST_READ_MODE_EN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        FAST_READ_MODE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FAST_READ_MODE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `fast_read_mode_en` writer - Fast Read Mode Enable (Deprecated)"]
pub struct FAST_READ_MODE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> FAST_READ_MODE_EN_W<'a> {
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
#[doc = "Field `en_prevent_fail` reader - Prevent Fail Flag Set on FLC Busy"]
pub struct EN_PREVENT_FAIL_R(crate::FieldReader<bool, bool>);
impl EN_PREVENT_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_PREVENT_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_PREVENT_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `en_prevent_fail` writer - Prevent Fail Flag Set on FLC Busy"]
pub struct EN_PREVENT_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_PREVENT_FAIL_W<'a> {
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
#[doc = "Field `en_back2back_rds` reader - Enable Back To Back Reads"]
pub struct EN_BACK2BACK_RDS_R(crate::FieldReader<bool, bool>);
impl EN_BACK2BACK_RDS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_BACK2BACK_RDS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_BACK2BACK_RDS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `en_back2back_rds` writer - Enable Back To Back Reads"]
pub struct EN_BACK2BACK_RDS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_BACK2BACK_RDS_W<'a> {
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
#[doc = "Field `en_back2back_wrs` reader - Enable Back To Back Writes"]
pub struct EN_BACK2BACK_WRS_R(crate::FieldReader<bool, bool>);
impl EN_BACK2BACK_WRS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_BACK2BACK_WRS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_BACK2BACK_WRS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `en_back2back_wrs` writer - Enable Back To Back Writes"]
pub struct EN_BACK2BACK_WRS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_BACK2BACK_WRS_W<'a> {
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
#[doc = "Field `en_merge_grab_gnt` reader - Enable Merge Grab GNT"]
pub struct EN_MERGE_GRAB_GNT_R(crate::FieldReader<bool, bool>);
impl EN_MERGE_GRAB_GNT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EN_MERGE_GRAB_GNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_MERGE_GRAB_GNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `en_merge_grab_gnt` writer - Enable Merge Grab GNT"]
pub struct EN_MERGE_GRAB_GNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_MERGE_GRAB_GNT_W<'a> {
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
#[doc = "Field `auto_tacc` reader - Auto TACC"]
pub struct AUTO_TACC_R(crate::FieldReader<bool, bool>);
impl AUTO_TACC_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_TACC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_TACC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `auto_tacc` writer - Auto TACC"]
pub struct AUTO_TACC_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_TACC_W<'a> {
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
#[doc = "Field `auto_clkdiv` reader - Auto CLKDIV"]
pub struct AUTO_CLKDIV_R(crate::FieldReader<bool, bool>);
impl AUTO_CLKDIV_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        AUTO_CLKDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUTO_CLKDIV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `auto_clkdiv` writer - Auto CLKDIV"]
pub struct AUTO_CLKDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> AUTO_CLKDIV_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Delay SE Enable (Deprecated)"]
    #[inline(always)]
    pub fn delay_se_en(&self) -> DELAY_SE_EN_R {
        DELAY_SE_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Fast Read Mode Enable (Deprecated)"]
    #[inline(always)]
    pub fn fast_read_mode_en(&self) -> FAST_READ_MODE_EN_R {
        FAST_READ_MODE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Prevent Fail Flag Set on FLC Busy"]
    #[inline(always)]
    pub fn en_prevent_fail(&self) -> EN_PREVENT_FAIL_R {
        EN_PREVENT_FAIL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable Back To Back Reads"]
    #[inline(always)]
    pub fn en_back2back_rds(&self) -> EN_BACK2BACK_RDS_R {
        EN_BACK2BACK_RDS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable Back To Back Writes"]
    #[inline(always)]
    pub fn en_back2back_wrs(&self) -> EN_BACK2BACK_WRS_R {
        EN_BACK2BACK_WRS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable Merge Grab GNT"]
    #[inline(always)]
    pub fn en_merge_grab_gnt(&self) -> EN_MERGE_GRAB_GNT_R {
        EN_MERGE_GRAB_GNT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Auto TACC"]
    #[inline(always)]
    pub fn auto_tacc(&self) -> AUTO_TACC_R {
        AUTO_TACC_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Auto CLKDIV"]
    #[inline(always)]
    pub fn auto_clkdiv(&self) -> AUTO_CLKDIV_R {
        AUTO_CLKDIV_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Delay SE Enable (Deprecated)"]
    #[inline(always)]
    pub fn delay_se_en(&mut self) -> DELAY_SE_EN_W {
        DELAY_SE_EN_W { w: self }
    }
    #[doc = "Bit 8 - Fast Read Mode Enable (Deprecated)"]
    #[inline(always)]
    pub fn fast_read_mode_en(&mut self) -> FAST_READ_MODE_EN_W {
        FAST_READ_MODE_EN_W { w: self }
    }
    #[doc = "Bit 12 - Prevent Fail Flag Set on FLC Busy"]
    #[inline(always)]
    pub fn en_prevent_fail(&mut self) -> EN_PREVENT_FAIL_W {
        EN_PREVENT_FAIL_W { w: self }
    }
    #[doc = "Bit 16 - Enable Back To Back Reads"]
    #[inline(always)]
    pub fn en_back2back_rds(&mut self) -> EN_BACK2BACK_RDS_W {
        EN_BACK2BACK_RDS_W { w: self }
    }
    #[doc = "Bit 20 - Enable Back To Back Writes"]
    #[inline(always)]
    pub fn en_back2back_wrs(&mut self) -> EN_BACK2BACK_WRS_W {
        EN_BACK2BACK_WRS_W { w: self }
    }
    #[doc = "Bit 24 - Enable Merge Grab GNT"]
    #[inline(always)]
    pub fn en_merge_grab_gnt(&mut self) -> EN_MERGE_GRAB_GNT_W {
        EN_MERGE_GRAB_GNT_W { w: self }
    }
    #[doc = "Bit 28 - Auto TACC"]
    #[inline(always)]
    pub fn auto_tacc(&mut self) -> AUTO_TACC_W {
        AUTO_TACC_W { w: self }
    }
    #[doc = "Bit 29 - Auto CLKDIV"]
    #[inline(always)]
    pub fn auto_clkdiv(&mut self) -> AUTO_CLKDIV_W {
        AUTO_CLKDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Performance Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perform](index.html) module"]
pub struct PERFORM_SPEC;
impl crate::RegisterSpec for PERFORM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perform::R](R) reader structure"]
impl crate::Readable for PERFORM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perform::W](W) writer structure"]
impl crate::Writable for PERFORM_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERFORM to value 0"]
impl crate::Resettable for PERFORM_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
