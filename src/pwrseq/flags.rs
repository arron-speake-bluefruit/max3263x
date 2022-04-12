#[doc = "Register `FLAGS` reader"]
pub struct R(crate::R<FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLAGS` writer"]
pub struct W(crate::W<FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLAGS_SPEC>;
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
impl From<crate::W<FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_first_boot` reader - Initial Boot event detected flag"]
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
#[doc = "Field `pwr_sys_reboot` reader - Firmware Reset event detected flag"]
pub struct PWR_SYS_REBOOT_R(crate::FieldReader<bool, bool>);
impl PWR_SYS_REBOOT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_SYS_REBOOT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_SYS_REBOOT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_power_fail` reader - Power Fail event detected flag"]
pub struct PWR_POWER_FAIL_R(crate::FieldReader<bool, bool>);
impl PWR_POWER_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_POWER_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_POWER_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_power_fail` writer - Power Fail event detected flag"]
pub struct PWR_POWER_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_POWER_FAIL_W<'a> {
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
#[doc = "Field `pwr_boot_fail` reader - Boot Fail event detected flag"]
pub struct PWR_BOOT_FAIL_R(crate::FieldReader<bool, bool>);
impl PWR_BOOT_FAIL_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_BOOT_FAIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_BOOT_FAIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_boot_fail` writer - Boot Fail event detected flag"]
pub struct PWR_BOOT_FAIL_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_BOOT_FAIL_W<'a> {
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
#[doc = "Field `pwr_flash_discharge` reader - Flash Discharged During Powerfail event detected flag"]
pub struct PWR_FLASH_DISCHARGE_R(crate::FieldReader<bool, bool>);
impl PWR_FLASH_DISCHARGE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_FLASH_DISCHARGE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_FLASH_DISCHARGE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_flash_discharge` writer - Flash Discharged During Powerfail event detected flag"]
pub struct PWR_FLASH_DISCHARGE_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_FLASH_DISCHARGE_W<'a> {
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
#[doc = "Field `pwr_iowakeup` reader - GPIO Wakeup event detected flag"]
pub struct PWR_IOWAKEUP_R(crate::FieldReader<bool, bool>);
impl PWR_IOWAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_IOWAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_IOWAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_iowakeup` writer - GPIO Wakeup event detected flag"]
pub struct PWR_IOWAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_IOWAKEUP_W<'a> {
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
#[doc = "Field `pwr_vdd12_rst_bad` reader - VDD12_SW Comparator Tripped event detected flag"]
pub struct PWR_VDD12_RST_BAD_R(crate::FieldReader<bool, bool>);
impl PWR_VDD12_RST_BAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VDD12_RST_BAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VDD12_RST_BAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vdd12_rst_bad` writer - VDD12_SW Comparator Tripped event detected flag"]
pub struct PWR_VDD12_RST_BAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VDD12_RST_BAD_W<'a> {
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
#[doc = "Field `pwr_vdd18_rst_bad` reader - VDD18_SW Comparator Tripped event detected flag"]
pub struct PWR_VDD18_RST_BAD_R(crate::FieldReader<bool, bool>);
impl PWR_VDD18_RST_BAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VDD18_RST_BAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VDD18_RST_BAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vdd18_rst_bad` writer - VDD18_SW Comparator Tripped event detected flag"]
pub struct PWR_VDD18_RST_BAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VDD18_RST_BAD_W<'a> {
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
#[doc = "Field `pwr_vrtc_rst_bad` reader - VRTC Comparator Tripped event detected flag"]
pub struct PWR_VRTC_RST_BAD_R(crate::FieldReader<bool, bool>);
impl PWR_VRTC_RST_BAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VRTC_RST_BAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VRTC_RST_BAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vrtc_rst_bad` writer - VRTC Comparator Tripped event detected flag"]
pub struct PWR_VRTC_RST_BAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VRTC_RST_BAD_W<'a> {
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
#[doc = "Field `pwr_vddb_rst_bad` reader - VDDB Comparator Tripped event detected flag"]
pub struct PWR_VDDB_RST_BAD_R(crate::FieldReader<bool, bool>);
impl PWR_VDDB_RST_BAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_VDDB_RST_BAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_VDDB_RST_BAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_vddb_rst_bad` writer - VDDB Comparator Tripped event detected flag"]
pub struct PWR_VDDB_RST_BAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_VDDB_RST_BAD_W<'a> {
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
#[doc = "Field `pwr_tvdd12_rst_bad` reader - TVDD12 Comparator Tripped event detected flag"]
pub struct PWR_TVDD12_RST_BAD_R(crate::FieldReader<bool, bool>);
impl PWR_TVDD12_RST_BAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_TVDD12_RST_BAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TVDD12_RST_BAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_tvdd12_rst_bad` writer - TVDD12 Comparator Tripped event detected flag"]
pub struct PWR_TVDD12_RST_BAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TVDD12_RST_BAD_W<'a> {
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
#[doc = "Field `pwr_por18z_fail_latch` reader - POR18 and POR18_bg have been tripped"]
pub struct PWR_POR18Z_FAIL_LATCH_R(crate::FieldReader<bool, bool>);
impl PWR_POR18Z_FAIL_LATCH_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_POR18Z_FAIL_LATCH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_POR18Z_FAIL_LATCH_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_por18z_fail_latch` writer - POR18 and POR18_bg have been tripped"]
pub struct PWR_POR18Z_FAIL_LATCH_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_POR18Z_FAIL_LATCH_W<'a> {
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
#[doc = "Field `rtc_cmpr0` reader - RTC Comparator 0 Match event detected flag"]
pub struct RTC_CMPR0_R(crate::FieldReader<bool, bool>);
impl RTC_CMPR0_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CMPR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CMPR0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_cmpr1` reader - RTC Comparator 1 Match event detected flag"]
pub struct RTC_CMPR1_R(crate::FieldReader<bool, bool>);
impl RTC_CMPR1_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_CMPR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_CMPR1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_prescale_cmp` reader - RTC Prescale Comparator Match event detected flag"]
pub struct RTC_PRESCALE_CMP_R(crate::FieldReader<bool, bool>);
impl RTC_PRESCALE_CMP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_PRESCALE_CMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_PRESCALE_CMP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rtc_rollover` reader - RTC Rollover event detected flag"]
pub struct RTC_ROLLOVER_R(crate::FieldReader<bool, bool>);
impl RTC_ROLLOVER_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RTC_ROLLOVER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTC_ROLLOVER_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_usb_plug_wakeup` reader - USB Power Connect Wakeup event detected flag"]
pub struct PWR_USB_PLUG_WAKEUP_R(crate::FieldReader<bool, bool>);
impl PWR_USB_PLUG_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_USB_PLUG_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_USB_PLUG_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_usb_plug_wakeup` writer - USB Power Connect Wakeup event detected flag"]
pub struct PWR_USB_PLUG_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_USB_PLUG_WAKEUP_W<'a> {
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
#[doc = "Field `pwr_usb_remove_wakeup` reader - USB Power Remove Wakeup event detected flag"]
pub struct PWR_USB_REMOVE_WAKEUP_R(crate::FieldReader<bool, bool>);
impl PWR_USB_REMOVE_WAKEUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_USB_REMOVE_WAKEUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_USB_REMOVE_WAKEUP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_usb_remove_wakeup` writer - USB Power Remove Wakeup event detected flag"]
pub struct PWR_USB_REMOVE_WAKEUP_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_USB_REMOVE_WAKEUP_W<'a> {
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
#[doc = "Field `pwr_tvdd12_bad` reader - Retention Regulator POR Tripped event detected flag"]
pub struct PWR_TVDD12_BAD_R(crate::FieldReader<bool, bool>);
impl PWR_TVDD12_BAD_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        PWR_TVDD12_BAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PWR_TVDD12_BAD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `pwr_tvdd12_bad` writer - Retention Regulator POR Tripped event detected flag"]
pub struct PWR_TVDD12_BAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PWR_TVDD12_BAD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Initial Boot event detected flag"]
    #[inline(always)]
    pub fn pwr_first_boot(&self) -> PWR_FIRST_BOOT_R {
        PWR_FIRST_BOOT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Firmware Reset event detected flag"]
    #[inline(always)]
    pub fn pwr_sys_reboot(&self) -> PWR_SYS_REBOOT_R {
        PWR_SYS_REBOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_power_fail(&self) -> PWR_POWER_FAIL_R {
        PWR_POWER_FAIL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Boot Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_boot_fail(&self) -> PWR_BOOT_FAIL_R {
        PWR_BOOT_FAIL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Discharged During Powerfail event detected flag"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&self) -> PWR_FLASH_DISCHARGE_R {
        PWR_FLASH_DISCHARGE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_iowakeup(&self) -> PWR_IOWAKEUP_R {
        PWR_IOWAKEUP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VDD12_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&self) -> PWR_VDD12_RST_BAD_R {
        PWR_VDD12_RST_BAD_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VDD18_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&self) -> PWR_VDD18_RST_BAD_R {
        PWR_VDD18_RST_BAD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VRTC Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&self) -> PWR_VRTC_RST_BAD_R {
        PWR_VRTC_RST_BAD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - VDDB Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&self) -> PWR_VDDB_RST_BAD_R {
        PWR_VDDB_RST_BAD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TVDD12 Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&self) -> PWR_TVDD12_RST_BAD_R {
        PWR_TVDD12_RST_BAD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - POR18 and POR18_bg have been tripped"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&self) -> PWR_POR18Z_FAIL_LATCH_R {
        PWR_POR18Z_FAIL_LATCH_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - RTC Comparator 0 Match event detected flag"]
    #[inline(always)]
    pub fn rtc_cmpr0(&self) -> RTC_CMPR0_R {
        RTC_CMPR0_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RTC Comparator 1 Match event detected flag"]
    #[inline(always)]
    pub fn rtc_cmpr1(&self) -> RTC_CMPR1_R {
        RTC_CMPR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RTC Prescale Comparator Match event detected flag"]
    #[inline(always)]
    pub fn rtc_prescale_cmp(&self) -> RTC_PRESCALE_CMP_R {
        RTC_PRESCALE_CMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - RTC Rollover event detected flag"]
    #[inline(always)]
    pub fn rtc_rollover(&self) -> RTC_ROLLOVER_R {
        RTC_ROLLOVER_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - USB Power Connect Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&self) -> PWR_USB_PLUG_WAKEUP_R {
        PWR_USB_PLUG_WAKEUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USB Power Remove Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&self) -> PWR_USB_REMOVE_WAKEUP_R {
        PWR_USB_REMOVE_WAKEUP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Retention Regulator POR Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_bad(&self) -> PWR_TVDD12_BAD_R {
        PWR_TVDD12_BAD_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Power Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_power_fail(&mut self) -> PWR_POWER_FAIL_W {
        PWR_POWER_FAIL_W { w: self }
    }
    #[doc = "Bit 3 - Boot Fail event detected flag"]
    #[inline(always)]
    pub fn pwr_boot_fail(&mut self) -> PWR_BOOT_FAIL_W {
        PWR_BOOT_FAIL_W { w: self }
    }
    #[doc = "Bit 4 - Flash Discharged During Powerfail event detected flag"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&mut self) -> PWR_FLASH_DISCHARGE_W {
        PWR_FLASH_DISCHARGE_W { w: self }
    }
    #[doc = "Bit 5 - GPIO Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_iowakeup(&mut self) -> PWR_IOWAKEUP_W {
        PWR_IOWAKEUP_W { w: self }
    }
    #[doc = "Bit 6 - VDD12_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&mut self) -> PWR_VDD12_RST_BAD_W {
        PWR_VDD12_RST_BAD_W { w: self }
    }
    #[doc = "Bit 7 - VDD18_SW Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&mut self) -> PWR_VDD18_RST_BAD_W {
        PWR_VDD18_RST_BAD_W { w: self }
    }
    #[doc = "Bit 8 - VRTC Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&mut self) -> PWR_VRTC_RST_BAD_W {
        PWR_VRTC_RST_BAD_W { w: self }
    }
    #[doc = "Bit 9 - VDDB Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&mut self) -> PWR_VDDB_RST_BAD_W {
        PWR_VDDB_RST_BAD_W { w: self }
    }
    #[doc = "Bit 10 - TVDD12 Comparator Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&mut self) -> PWR_TVDD12_RST_BAD_W {
        PWR_TVDD12_RST_BAD_W { w: self }
    }
    #[doc = "Bit 11 - POR18 and POR18_bg have been tripped"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&mut self) -> PWR_POR18Z_FAIL_LATCH_W {
        PWR_POR18Z_FAIL_LATCH_W { w: self }
    }
    #[doc = "Bit 16 - USB Power Connect Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&mut self) -> PWR_USB_PLUG_WAKEUP_W {
        PWR_USB_PLUG_WAKEUP_W { w: self }
    }
    #[doc = "Bit 17 - USB Power Remove Wakeup event detected flag"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&mut self) -> PWR_USB_REMOVE_WAKEUP_W {
        PWR_USB_REMOVE_WAKEUP_W { w: self }
    }
    #[doc = "Bit 18 - Retention Regulator POR Tripped event detected flag"]
    #[inline(always)]
    pub fn pwr_tvdd12_bad(&mut self) -> PWR_TVDD12_BAD_W {
        PWR_TVDD12_BAD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Sequencer Flags\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flags](index.html) module"]
pub struct FLAGS_SPEC;
impl crate::RegisterSpec for FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flags::R](R) reader structure"]
impl crate::Readable for FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flags::W](W) writer structure"]
impl crate::Writable for FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLAGS to value 0"]
impl crate::Resettable for FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
