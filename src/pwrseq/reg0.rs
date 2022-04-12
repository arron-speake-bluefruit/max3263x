#[doc = "Register `REG0` reader"]
pub struct R(crate::R<REG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG0` writer"]
pub struct W(crate::W<REG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG0_SPEC>;
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
impl From<crate::W<REG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_lp1` reader - Shutdown Power Mode Select"]
pub struct PWR_LP1_R(crate::FieldReader<bool, bool>);
impl PWR_LP1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_LP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_LP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_lp1` writer - Shutdown Power Mode Select"]
pub struct PWR_LP1_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_LP1_W<'a> {
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
#[doc = "Field `pwr_first_boot` reader - Wake on First Boot"]
pub struct PWR_FIRST_BOOT_R(crate::FieldReader<bool, bool>);
impl PWR_FIRST_BOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_FIRST_BOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_FIRST_BOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_first_boot` writer - Wake on First Boot"]
pub struct PWR_FIRST_BOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_FIRST_BOOT_W<'a> {
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
#[doc = "Field `pwr_sys_reboot` writer - Firmware System Reboot Request"]
pub struct PWR_SYS_REBOOT_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SYS_REBOOT_W<'a> {
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
#[doc = "Field `pwr_flashen_run` reader - Enable Flash Operation during Run Mode"]
pub struct PWR_FLASHEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_FLASHEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_FLASHEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_FLASHEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_flashen_run` writer - Enable Flash Operation during Run Mode"]
pub struct PWR_FLASHEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_FLASHEN_RUN_W<'a> {
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
#[doc = "Field `pwr_flashen_slp` reader - Enable Flash Operation during Sleep Mode"]
pub struct PWR_FLASHEN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_FLASHEN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_FLASHEN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_FLASHEN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_flashen_slp` writer - Enable Flash Operation during Sleep Mode"]
pub struct PWR_FLASHEN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_FLASHEN_SLP_W<'a> {
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
#[doc = "Field `pwr_retregen_run` reader - Enable Retention Regulator Operation during Run Mode"]
pub struct PWR_RETREGEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_RETREGEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_RETREGEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_RETREGEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_retregen_run` writer - Enable Retention Regulator Operation during Run Mode"]
pub struct PWR_RETREGEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_RETREGEN_RUN_W<'a> {
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
#[doc = "Field `pwr_retregen_slp` reader - Enable Retention Regulator Operation during Sleep Mode"]
pub struct PWR_RETREGEN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_RETREGEN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_RETREGEN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_RETREGEN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_retregen_slp` writer - Enable Retention Regulator Operation during Sleep Mode"]
pub struct PWR_RETREGEN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_RETREGEN_SLP_W<'a> {
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
#[doc = "Field `pwr_roen_run` reader - Enable 96MHz System Relaxation Oscillator in Run Mode"]
pub struct PWR_ROEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_ROEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_ROEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_ROEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_roen_run` writer - Enable 96MHz System Relaxation Oscillator in Run Mode"]
pub struct PWR_ROEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_ROEN_RUN_W<'a> {
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
#[doc = "Field `pwr_roen_slp` reader - Enable 96MHz System Relaxation Oscillator in Sleep Mode"]
pub struct PWR_ROEN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_ROEN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_ROEN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_ROEN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_roen_slp` writer - Enable 96MHz System Relaxation Oscillator in Sleep Mode"]
pub struct PWR_ROEN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_ROEN_SLP_W<'a> {
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
#[doc = "Field `pwr_nren_run` reader - Enable Nano Oscillator in Run Mode"]
pub struct PWR_NREN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_NREN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_NREN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_NREN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_nren_run` writer - Enable Nano Oscillator in Run Mode"]
pub struct PWR_NREN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_NREN_RUN_W<'a> {
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
#[doc = "Field `pwr_nren_slp` reader - Enable Nano Oscillator in Sleep Mode"]
pub struct PWR_NREN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_NREN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_NREN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_NREN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_nren_slp` writer - Enable Nano Oscillator in Sleep Mode"]
pub struct PWR_NREN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_NREN_SLP_W<'a> {
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
#[doc = "Field `pwr_rtcen_run` reader - Enable Real Time Clock Operation during Run Mode"]
pub struct PWR_RTCEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_RTCEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_RTCEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_RTCEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_rtcen_run` writer - Enable Real Time Clock Operation during Run Mode"]
pub struct PWR_RTCEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_RTCEN_RUN_W<'a> {
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
#[doc = "Field `pwr_rtcen_slp` reader - Enable Real Time Clock Operation during Sleep Mode"]
pub struct PWR_RTCEN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_RTCEN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_RTCEN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_RTCEN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_rtcen_slp` writer - Enable Real Time Clock Operation during Sleep Mode"]
pub struct PWR_RTCEN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_RTCEN_SLP_W<'a> {
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
#[doc = "Field `pwr_svm12en_run` reader - Enable VDD12_SW SVM operation during Run Mode"]
pub struct PWR_SVM12EN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_SVM12EN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_SVM12EN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SVM12EN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_svm12en_run` writer - Enable VDD12_SW SVM operation during Run Mode"]
pub struct PWR_SVM12EN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SVM12EN_RUN_W<'a> {
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
#[doc = "Field `pwr_svm18en_run` reader - Enable VDD18_SW SVM operation during Run Mode"]
pub struct PWR_SVM18EN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_SVM18EN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_SVM18EN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SVM18EN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_svm18en_run` writer - Enable VDD18_SW SVM operation during Run Mode"]
pub struct PWR_SVM18EN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SVM18EN_RUN_W<'a> {
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
#[doc = "Field `pwr_svmrtcen_run` reader - Enable VRTC SVM operation during Run Mode"]
pub struct PWR_SVMRTCEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_SVMRTCEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_SVMRTCEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SVMRTCEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_svmrtcen_run` writer - Enable VRTC SVM operation during Run Mode"]
pub struct PWR_SVMRTCEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SVMRTCEN_RUN_W<'a> {
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
#[doc = "Field `pwr_svm_vddb_run` reader - Enable VDDB SVM operation during Run Mode"]
pub struct PWR_SVM_VDDB_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_SVM_VDDB_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_SVM_VDDB_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SVM_VDDB_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_svm_vddb_run` writer - Enable VDDB SVM operation during Run Mode"]
pub struct PWR_SVM_VDDB_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SVM_VDDB_RUN_W<'a> {
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
#[doc = "Field `pwr_svmtvdd12en_run` reader - Enable TVDD12 SVM operation during Run Mode"]
pub struct PWR_SVMTVDD12EN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_SVMTVDD12EN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_SVMTVDD12EN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SVMTVDD12EN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_svmtvdd12en_run` writer - Enable TVDD12 SVM operation during Run Mode"]
pub struct PWR_SVMTVDD12EN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_SVMTVDD12EN_RUN_W<'a> {
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
#[doc = "Field `pwr_vdd12_swen_run` reader - Enable VDD12 switching during Run Mode"]
pub struct PWR_VDD12_SWEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_VDD12_SWEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VDD12_SWEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VDD12_SWEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vdd12_swen_run` writer - Enable VDD12 switching during Run Mode"]
pub struct PWR_VDD12_SWEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VDD12_SWEN_RUN_W<'a> {
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
#[doc = "Field `pwr_vdd12_swen_slp` reader - Enable VDD12 switching during Sleep Mode"]
pub struct PWR_VDD12_SWEN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_VDD12_SWEN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VDD12_SWEN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VDD12_SWEN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vdd12_swen_slp` writer - Enable VDD12 switching during Sleep Mode"]
pub struct PWR_VDD12_SWEN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VDD12_SWEN_SLP_W<'a> {
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
#[doc = "Field `pwr_vdd18_swen_run` reader - Enable VDD18 switching during Run Mode"]
pub struct PWR_VDD18_SWEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_VDD18_SWEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VDD18_SWEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VDD18_SWEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vdd18_swen_run` writer - Enable VDD18 switching during Run Mode"]
pub struct PWR_VDD18_SWEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VDD18_SWEN_RUN_W<'a> {
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
#[doc = "Field `pwr_vdd18_swen_slp` reader - Enable VDD18 switching during Sleep Mode"]
pub struct PWR_VDD18_SWEN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_VDD18_SWEN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VDD18_SWEN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VDD18_SWEN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vdd18_swen_slp` writer - Enable VDD18 switching during Sleep Mode"]
pub struct PWR_VDD18_SWEN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VDD18_SWEN_SLP_W<'a> {
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
#[doc = "Field `pwr_tvdd12_swen_run` reader - Enable TVDD12 switching during Run Mode"]
pub struct PWR_TVDD12_SWEN_RUN_R(crate::FieldReader<bool, bool>);
impl PWR_TVDD12_SWEN_RUN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_TVDD12_SWEN_RUN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TVDD12_SWEN_RUN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_tvdd12_swen_run` writer - Enable TVDD12 switching during Run Mode"]
pub struct PWR_TVDD12_SWEN_RUN_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TVDD12_SWEN_RUN_W<'a> {
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
#[doc = "Field `pwr_tvdd12_swen_slp` reader - Enable TVDD12 switching during Sleep Mode"]
pub struct PWR_TVDD12_SWEN_SLP_R(crate::FieldReader<bool, bool>);
impl PWR_TVDD12_SWEN_SLP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_TVDD12_SWEN_SLP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TVDD12_SWEN_SLP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_tvdd12_swen_slp` writer - Enable TVDD12 switching during Sleep Mode"]
pub struct PWR_TVDD12_SWEN_SLP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TVDD12_SWEN_SLP_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Shutdown Power Mode Select"]
    #[inline(always)]
    pub fn pwr_lp1(&self) -> PWR_LP1_R {
        PWR_LP1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wake on First Boot"]
    #[inline(always)]
    pub fn pwr_first_boot(&self) -> PWR_FIRST_BOOT_R {
        PWR_FIRST_BOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Flash Operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_flashen_run(&self) -> PWR_FLASHEN_RUN_R {
        PWR_FLASHEN_RUN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Flash Operation during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_flashen_slp(&self) -> PWR_FLASHEN_SLP_R {
        PWR_FLASHEN_SLP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Retention Regulator Operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_retregen_run(&self) -> PWR_RETREGEN_RUN_R {
        PWR_RETREGEN_RUN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Retention Regulator Operation during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_retregen_slp(&self) -> PWR_RETREGEN_SLP_R {
        PWR_RETREGEN_SLP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable 96MHz System Relaxation Oscillator in Run Mode"]
    #[inline(always)]
    pub fn pwr_roen_run(&self) -> PWR_ROEN_RUN_R {
        PWR_ROEN_RUN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable 96MHz System Relaxation Oscillator in Sleep Mode"]
    #[inline(always)]
    pub fn pwr_roen_slp(&self) -> PWR_ROEN_SLP_R {
        PWR_ROEN_SLP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Nano Oscillator in Run Mode"]
    #[inline(always)]
    pub fn pwr_nren_run(&self) -> PWR_NREN_RUN_R {
        PWR_NREN_RUN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Nano Oscillator in Sleep Mode"]
    #[inline(always)]
    pub fn pwr_nren_slp(&self) -> PWR_NREN_SLP_R {
        PWR_NREN_SLP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable Real Time Clock Operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_rtcen_run(&self) -> PWR_RTCEN_RUN_R {
        PWR_RTCEN_RUN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable Real Time Clock Operation during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_rtcen_slp(&self) -> PWR_RTCEN_SLP_R {
        PWR_RTCEN_SLP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable VDD12_SW SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svm12en_run(&self) -> PWR_SVM12EN_RUN_R {
        PWR_SVM12EN_RUN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable VDD18_SW SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svm18en_run(&self) -> PWR_SVM18EN_RUN_R {
        PWR_SVM18EN_RUN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable VRTC SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svmrtcen_run(&self) -> PWR_SVMRTCEN_RUN_R {
        PWR_SVMRTCEN_RUN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable VDDB SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svm_vddb_run(&self) -> PWR_SVM_VDDB_RUN_R {
        PWR_SVM_VDDB_RUN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable TVDD12 SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svmtvdd12en_run(&self) -> PWR_SVMTVDD12EN_RUN_R {
        PWR_SVMTVDD12EN_RUN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable VDD12 switching during Run Mode"]
    #[inline(always)]
    pub fn pwr_vdd12_swen_run(&self) -> PWR_VDD12_SWEN_RUN_R {
        PWR_VDD12_SWEN_RUN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable VDD12 switching during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_vdd12_swen_slp(&self) -> PWR_VDD12_SWEN_SLP_R {
        PWR_VDD12_SWEN_SLP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable VDD18 switching during Run Mode"]
    #[inline(always)]
    pub fn pwr_vdd18_swen_run(&self) -> PWR_VDD18_SWEN_RUN_R {
        PWR_VDD18_SWEN_RUN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable VDD18 switching during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_vdd18_swen_slp(&self) -> PWR_VDD18_SWEN_SLP_R {
        PWR_VDD18_SWEN_SLP_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable TVDD12 switching during Run Mode"]
    #[inline(always)]
    pub fn pwr_tvdd12_swen_run(&self) -> PWR_TVDD12_SWEN_RUN_R {
        PWR_TVDD12_SWEN_RUN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Enable TVDD12 switching during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_tvdd12_swen_slp(&self) -> PWR_TVDD12_SWEN_SLP_R {
        PWR_TVDD12_SWEN_SLP_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shutdown Power Mode Select"]
    #[inline(always)]
    pub fn pwr_lp1(&mut self) -> PWR_LP1_W {
        PWR_LP1_W { w: self }
    }
    #[doc = "Bit 1 - Wake on First Boot"]
    #[inline(always)]
    pub fn pwr_first_boot(&mut self) -> PWR_FIRST_BOOT_W {
        PWR_FIRST_BOOT_W { w: self }
    }
    #[doc = "Bit 2 - Firmware System Reboot Request"]
    #[inline(always)]
    pub fn pwr_sys_reboot(&mut self) -> PWR_SYS_REBOOT_W {
        PWR_SYS_REBOOT_W { w: self }
    }
    #[doc = "Bit 3 - Enable Flash Operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_flashen_run(&mut self) -> PWR_FLASHEN_RUN_W {
        PWR_FLASHEN_RUN_W { w: self }
    }
    #[doc = "Bit 4 - Enable Flash Operation during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_flashen_slp(&mut self) -> PWR_FLASHEN_SLP_W {
        PWR_FLASHEN_SLP_W { w: self }
    }
    #[doc = "Bit 5 - Enable Retention Regulator Operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_retregen_run(&mut self) -> PWR_RETREGEN_RUN_W {
        PWR_RETREGEN_RUN_W { w: self }
    }
    #[doc = "Bit 6 - Enable Retention Regulator Operation during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_retregen_slp(&mut self) -> PWR_RETREGEN_SLP_W {
        PWR_RETREGEN_SLP_W { w: self }
    }
    #[doc = "Bit 7 - Enable 96MHz System Relaxation Oscillator in Run Mode"]
    #[inline(always)]
    pub fn pwr_roen_run(&mut self) -> PWR_ROEN_RUN_W {
        PWR_ROEN_RUN_W { w: self }
    }
    #[doc = "Bit 8 - Enable 96MHz System Relaxation Oscillator in Sleep Mode"]
    #[inline(always)]
    pub fn pwr_roen_slp(&mut self) -> PWR_ROEN_SLP_W {
        PWR_ROEN_SLP_W { w: self }
    }
    #[doc = "Bit 9 - Enable Nano Oscillator in Run Mode"]
    #[inline(always)]
    pub fn pwr_nren_run(&mut self) -> PWR_NREN_RUN_W {
        PWR_NREN_RUN_W { w: self }
    }
    #[doc = "Bit 10 - Enable Nano Oscillator in Sleep Mode"]
    #[inline(always)]
    pub fn pwr_nren_slp(&mut self) -> PWR_NREN_SLP_W {
        PWR_NREN_SLP_W { w: self }
    }
    #[doc = "Bit 11 - Enable Real Time Clock Operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_rtcen_run(&mut self) -> PWR_RTCEN_RUN_W {
        PWR_RTCEN_RUN_W { w: self }
    }
    #[doc = "Bit 12 - Enable Real Time Clock Operation during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_rtcen_slp(&mut self) -> PWR_RTCEN_SLP_W {
        PWR_RTCEN_SLP_W { w: self }
    }
    #[doc = "Bit 13 - Enable VDD12_SW SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svm12en_run(&mut self) -> PWR_SVM12EN_RUN_W {
        PWR_SVM12EN_RUN_W { w: self }
    }
    #[doc = "Bit 15 - Enable VDD18_SW SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svm18en_run(&mut self) -> PWR_SVM18EN_RUN_W {
        PWR_SVM18EN_RUN_W { w: self }
    }
    #[doc = "Bit 17 - Enable VRTC SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svmrtcen_run(&mut self) -> PWR_SVMRTCEN_RUN_W {
        PWR_SVMRTCEN_RUN_W { w: self }
    }
    #[doc = "Bit 19 - Enable VDDB SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svm_vddb_run(&mut self) -> PWR_SVM_VDDB_RUN_W {
        PWR_SVM_VDDB_RUN_W { w: self }
    }
    #[doc = "Bit 21 - Enable TVDD12 SVM operation during Run Mode"]
    #[inline(always)]
    pub fn pwr_svmtvdd12en_run(&mut self) -> PWR_SVMTVDD12EN_RUN_W {
        PWR_SVMTVDD12EN_RUN_W { w: self }
    }
    #[doc = "Bit 23 - Enable VDD12 switching during Run Mode"]
    #[inline(always)]
    pub fn pwr_vdd12_swen_run(&mut self) -> PWR_VDD12_SWEN_RUN_W {
        PWR_VDD12_SWEN_RUN_W { w: self }
    }
    #[doc = "Bit 24 - Enable VDD12 switching during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_vdd12_swen_slp(&mut self) -> PWR_VDD12_SWEN_SLP_W {
        PWR_VDD12_SWEN_SLP_W { w: self }
    }
    #[doc = "Bit 25 - Enable VDD18 switching during Run Mode"]
    #[inline(always)]
    pub fn pwr_vdd18_swen_run(&mut self) -> PWR_VDD18_SWEN_RUN_W {
        PWR_VDD18_SWEN_RUN_W { w: self }
    }
    #[doc = "Bit 26 - Enable VDD18 switching during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_vdd18_swen_slp(&mut self) -> PWR_VDD18_SWEN_SLP_W {
        PWR_VDD18_SWEN_SLP_W { w: self }
    }
    #[doc = "Bit 27 - Enable TVDD12 switching during Run Mode"]
    #[inline(always)]
    pub fn pwr_tvdd12_swen_run(&mut self) -> PWR_TVDD12_SWEN_RUN_W {
        PWR_TVDD12_SWEN_RUN_W { w: self }
    }
    #[doc = "Bit 28 - Enable TVDD12 switching during Sleep Mode"]
    #[inline(always)]
    pub fn pwr_tvdd12_swen_slp(&mut self) -> PWR_TVDD12_SWEN_SLP_W {
        PWR_TVDD12_SWEN_SLP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg0](index.html) module"]
pub struct REG0_SPEC;
impl crate::RegisterSpec for REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg0::R](R) reader structure"]
impl crate::Readable for REG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg0::W](W) writer structure"]
impl crate::Writable for REG0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REG0 to value 0"]
impl crate::Resettable for REG0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
