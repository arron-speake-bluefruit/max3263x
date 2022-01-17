#[doc = "Register `MSK_FLAGS` reader"]
pub struct R(crate::R<MSK_FLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSK_FLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSK_FLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSK_FLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSK_FLAGS` writer"]
pub struct W(crate::W<MSK_FLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSK_FLAGS_SPEC>;
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
impl From<crate::W<MSK_FLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSK_FLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pwr_sys_reboot` reader - Mask for system reboot detect"]
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
#[doc = "Field `pwr_sys_reboot` writer - Mask for system reboot detect"]
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `pwr_power_fail` reader - Mask for previous power fail detect"]
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
#[doc = "Field `pwr_power_fail` writer - Mask for previous power fail detect"]
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `pwr_boot_fail` reader - Mask for previous boot fail detect"]
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
#[doc = "Field `pwr_boot_fail` writer - Mask for previous boot fail detect"]
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `pwr_flash_discharge` reader - Mask for flash discharge event"]
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
#[doc = "Field `pwr_flash_discharge` writer - Mask for flash discharge event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `pwr_iowakeup` reader - Mask for GPIO wakeup event detect"]
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
#[doc = "Field `pwr_iowakeup` writer - Mask for GPIO wakeup event detect"]
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `pwr_vdd12_rst_bad` reader - Mask for VDD12 rst event"]
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
#[doc = "Field `pwr_vdd12_rst_bad` writer - Mask for VDD12 rst event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `pwr_vdd18_rst_bad` reader - Mask for VDD18 rst event"]
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
#[doc = "Field `pwr_vdd18_rst_bad` writer - Mask for VDD18 rst event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `pwr_vrtc_rst_bad` reader - Mask for VRTC rst event"]
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
#[doc = "Field `pwr_vrtc_rst_bad` writer - Mask for VRTC rst event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `pwr_vddb_rst_bad` reader - Mask for VDDB rst event"]
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
#[doc = "Field `pwr_vddb_rst_bad` writer - Mask for VDDB rst event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `pwr_tvdd12_rst_bad` reader - Mask for TVDD12 rst event"]
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
#[doc = "Field `pwr_tvdd12_rst_bad` writer - Mask for TVDD12 rst event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `pwr_por18z_fail_latch` reader - Mask for POR18 powerfail event"]
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
#[doc = "Field `pwr_por18z_fail_latch` writer - Mask for POR18 powerfail event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `rtc_cmpr0` reader - Mask for RTC compare 0 event"]
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
#[doc = "Field `rtc_cmpr0` writer - Mask for RTC compare 0 event"]
pub struct RTC_CMPR0_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CMPR0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | ((value as u32 & 0x01) << 12);
        self.w
    }
}
#[doc = "Field `rtc_cmpr1` reader - Mask for RTC compare 1 event"]
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
#[doc = "Field `rtc_cmpr1` writer - Mask for RTC compare 1 event"]
pub struct RTC_CMPR1_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_CMPR1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `rtc_prescale_cmp` reader - Mask for RTC prescale compare event"]
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
#[doc = "Field `rtc_prescale_cmp` writer - Mask for RTC prescale compare event"]
pub struct RTC_PRESCALE_CMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_PRESCALE_CMP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `rtc_rollover` reader - Mask for RTC rollover event"]
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
#[doc = "Field `rtc_rollover` writer - Mask for RTC rollover event"]
pub struct RTC_ROLLOVER_W<'a> {
    w: &'a mut W,
}
impl<'a> RTC_ROLLOVER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `pwr_usb_plug_wakeup` reader - Mask for USB plug connect event"]
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
#[doc = "Field `pwr_usb_plug_wakeup` writer - Mask for USB plug connect event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `pwr_usb_remove_wakeup` reader - Mask for USB plug disconnect event"]
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
#[doc = "Field `pwr_usb_remove_wakeup` writer - Mask for USB plug disconnect event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `pwr_tvdd12_bad` reader - Mask for TVDD12 power fail event"]
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
#[doc = "Field `pwr_tvdd12_bad` writer - Mask for TVDD12 power fail event"]
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Mask for system reboot detect"]
    #[inline(always)]
    pub fn pwr_sys_reboot(&self) -> PWR_SYS_REBOOT_R {
        PWR_SYS_REBOOT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Mask for previous power fail detect"]
    #[inline(always)]
    pub fn pwr_power_fail(&self) -> PWR_POWER_FAIL_R {
        PWR_POWER_FAIL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Mask for previous boot fail detect"]
    #[inline(always)]
    pub fn pwr_boot_fail(&self) -> PWR_BOOT_FAIL_R {
        PWR_BOOT_FAIL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Mask for flash discharge event"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&self) -> PWR_FLASH_DISCHARGE_R {
        PWR_FLASH_DISCHARGE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Mask for GPIO wakeup event detect"]
    #[inline(always)]
    pub fn pwr_iowakeup(&self) -> PWR_IOWAKEUP_R {
        PWR_IOWAKEUP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Mask for VDD12 rst event"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&self) -> PWR_VDD12_RST_BAD_R {
        PWR_VDD12_RST_BAD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Mask for VDD18 rst event"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&self) -> PWR_VDD18_RST_BAD_R {
        PWR_VDD18_RST_BAD_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Mask for VRTC rst event"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&self) -> PWR_VRTC_RST_BAD_R {
        PWR_VRTC_RST_BAD_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Mask for VDDB rst event"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&self) -> PWR_VDDB_RST_BAD_R {
        PWR_VDDB_RST_BAD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Mask for TVDD12 rst event"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&self) -> PWR_TVDD12_RST_BAD_R {
        PWR_TVDD12_RST_BAD_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Mask for POR18 powerfail event"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&self) -> PWR_POR18Z_FAIL_LATCH_R {
        PWR_POR18Z_FAIL_LATCH_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Mask for RTC compare 0 event"]
    #[inline(always)]
    pub fn rtc_cmpr0(&self) -> RTC_CMPR0_R {
        RTC_CMPR0_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Mask for RTC compare 1 event"]
    #[inline(always)]
    pub fn rtc_cmpr1(&self) -> RTC_CMPR1_R {
        RTC_CMPR1_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Mask for RTC prescale compare event"]
    #[inline(always)]
    pub fn rtc_prescale_cmp(&self) -> RTC_PRESCALE_CMP_R {
        RTC_PRESCALE_CMP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Mask for RTC rollover event"]
    #[inline(always)]
    pub fn rtc_rollover(&self) -> RTC_ROLLOVER_R {
        RTC_ROLLOVER_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Mask for USB plug connect event"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&self) -> PWR_USB_PLUG_WAKEUP_R {
        PWR_USB_PLUG_WAKEUP_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Mask for USB plug disconnect event"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&self) -> PWR_USB_REMOVE_WAKEUP_R {
        PWR_USB_REMOVE_WAKEUP_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Mask for TVDD12 power fail event"]
    #[inline(always)]
    pub fn pwr_tvdd12_bad(&self) -> PWR_TVDD12_BAD_R {
        PWR_TVDD12_BAD_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Mask for system reboot detect"]
    #[inline(always)]
    pub fn pwr_sys_reboot(&mut self) -> PWR_SYS_REBOOT_W {
        PWR_SYS_REBOOT_W { w: self }
    }
    #[doc = "Bit 2 - Mask for previous power fail detect"]
    #[inline(always)]
    pub fn pwr_power_fail(&mut self) -> PWR_POWER_FAIL_W {
        PWR_POWER_FAIL_W { w: self }
    }
    #[doc = "Bit 3 - Mask for previous boot fail detect"]
    #[inline(always)]
    pub fn pwr_boot_fail(&mut self) -> PWR_BOOT_FAIL_W {
        PWR_BOOT_FAIL_W { w: self }
    }
    #[doc = "Bit 4 - Mask for flash discharge event"]
    #[inline(always)]
    pub fn pwr_flash_discharge(&mut self) -> PWR_FLASH_DISCHARGE_W {
        PWR_FLASH_DISCHARGE_W { w: self }
    }
    #[doc = "Bit 5 - Mask for GPIO wakeup event detect"]
    #[inline(always)]
    pub fn pwr_iowakeup(&mut self) -> PWR_IOWAKEUP_W {
        PWR_IOWAKEUP_W { w: self }
    }
    #[doc = "Bit 6 - Mask for VDD12 rst event"]
    #[inline(always)]
    pub fn pwr_vdd12_rst_bad(&mut self) -> PWR_VDD12_RST_BAD_W {
        PWR_VDD12_RST_BAD_W { w: self }
    }
    #[doc = "Bit 7 - Mask for VDD18 rst event"]
    #[inline(always)]
    pub fn pwr_vdd18_rst_bad(&mut self) -> PWR_VDD18_RST_BAD_W {
        PWR_VDD18_RST_BAD_W { w: self }
    }
    #[doc = "Bit 8 - Mask for VRTC rst event"]
    #[inline(always)]
    pub fn pwr_vrtc_rst_bad(&mut self) -> PWR_VRTC_RST_BAD_W {
        PWR_VRTC_RST_BAD_W { w: self }
    }
    #[doc = "Bit 9 - Mask for VDDB rst event"]
    #[inline(always)]
    pub fn pwr_vddb_rst_bad(&mut self) -> PWR_VDDB_RST_BAD_W {
        PWR_VDDB_RST_BAD_W { w: self }
    }
    #[doc = "Bit 10 - Mask for TVDD12 rst event"]
    #[inline(always)]
    pub fn pwr_tvdd12_rst_bad(&mut self) -> PWR_TVDD12_RST_BAD_W {
        PWR_TVDD12_RST_BAD_W { w: self }
    }
    #[doc = "Bit 11 - Mask for POR18 powerfail event"]
    #[inline(always)]
    pub fn pwr_por18z_fail_latch(&mut self) -> PWR_POR18Z_FAIL_LATCH_W {
        PWR_POR18Z_FAIL_LATCH_W { w: self }
    }
    #[doc = "Bit 12 - Mask for RTC compare 0 event"]
    #[inline(always)]
    pub fn rtc_cmpr0(&mut self) -> RTC_CMPR0_W {
        RTC_CMPR0_W { w: self }
    }
    #[doc = "Bit 13 - Mask for RTC compare 1 event"]
    #[inline(always)]
    pub fn rtc_cmpr1(&mut self) -> RTC_CMPR1_W {
        RTC_CMPR1_W { w: self }
    }
    #[doc = "Bit 14 - Mask for RTC prescale compare event"]
    #[inline(always)]
    pub fn rtc_prescale_cmp(&mut self) -> RTC_PRESCALE_CMP_W {
        RTC_PRESCALE_CMP_W { w: self }
    }
    #[doc = "Bit 15 - Mask for RTC rollover event"]
    #[inline(always)]
    pub fn rtc_rollover(&mut self) -> RTC_ROLLOVER_W {
        RTC_ROLLOVER_W { w: self }
    }
    #[doc = "Bit 16 - Mask for USB plug connect event"]
    #[inline(always)]
    pub fn pwr_usb_plug_wakeup(&mut self) -> PWR_USB_PLUG_WAKEUP_W {
        PWR_USB_PLUG_WAKEUP_W { w: self }
    }
    #[doc = "Bit 17 - Mask for USB plug disconnect event"]
    #[inline(always)]
    pub fn pwr_usb_remove_wakeup(&mut self) -> PWR_USB_REMOVE_WAKEUP_W {
        PWR_USB_REMOVE_WAKEUP_W { w: self }
    }
    #[doc = "Bit 18 - Mask for TVDD12 power fail event"]
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
#[doc = "Power Sequencer Flags Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msk_flags](index.html) module"]
pub struct MSK_FLAGS_SPEC;
impl crate::RegisterSpec for MSK_FLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msk_flags::R](R) reader structure"]
impl crate::Readable for MSK_FLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [msk_flags::W](W) writer structure"]
impl crate::Writable for MSK_FLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MSK_FLAGS to value 0"]
impl crate::Resettable for MSK_FLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
