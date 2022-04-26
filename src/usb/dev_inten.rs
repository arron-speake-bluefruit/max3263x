#[doc = "Register `DEV_INTEN` reader"]
pub struct R(crate::R<DEV_INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEV_INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEV_INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEV_INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEV_INTEN` writer"]
pub struct W(crate::W<DEV_INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEV_INTEN_SPEC>;
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
impl From<crate::W<DEV_INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEV_INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `dpact` reader - DPLUS Activity Interrupt Flag"]
pub struct DPACT_R(crate::FieldReader<bool>);
impl DPACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DPACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPACT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dpact` writer - DPLUS Activity Interrupt Flag"]
pub struct DPACT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPACT_W<'a> {
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
#[doc = "Field `rwu_dn` reader - Remote Wakeup Done Interrupt Flag"]
pub struct RWU_DN_R(crate::FieldReader<bool>);
impl RWU_DN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RWU_DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RWU_DN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rwu_dn` writer - Remote Wakeup Done Interrupt Flag"]
pub struct RWU_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> RWU_DN_W<'a> {
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
#[doc = "Field `bact` reader - USB Bus Activity Interrupt Flag"]
pub struct BACT_R(crate::FieldReader<bool>);
impl BACT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BACT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `bact` writer - USB Bus Activity Interrupt Flag"]
pub struct BACT_W<'a> {
    w: &'a mut W,
}
impl<'a> BACT_W<'a> {
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
#[doc = "Field `brst` reader - USB Bus Reset In Progress Interrupt Flag"]
pub struct BRST_R(crate::FieldReader<bool>);
impl BRST_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRST_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `brst` writer - USB Bus Reset In Progress Interrupt Flag"]
pub struct BRST_W<'a> {
    w: &'a mut W,
}
impl<'a> BRST_W<'a> {
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
#[doc = "Field `susp` reader - USB Suspend Interrupt Flag"]
pub struct SUSP_R(crate::FieldReader<bool>);
impl SUSP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SUSP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUSP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `susp` writer - USB Suspend Interrupt Flag"]
pub struct SUSP_W<'a> {
    w: &'a mut W,
}
impl<'a> SUSP_W<'a> {
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
#[doc = "Field `no_vbus` reader - No VBUS Interrupt Flag"]
pub struct NO_VBUS_R(crate::FieldReader<bool>);
impl NO_VBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        NO_VBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NO_VBUS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `no_vbus` writer - No VBUS Interrupt Flag"]
pub struct NO_VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> NO_VBUS_W<'a> {
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
#[doc = "Field `vbus` reader - VBUS Detect Interrupt Flag"]
pub struct VBUS_R(crate::FieldReader<bool>);
impl VBUS_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        VBUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VBUS_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `vbus` writer - VBUS Detect Interrupt Flag"]
pub struct VBUS_W<'a> {
    w: &'a mut W,
}
impl<'a> VBUS_W<'a> {
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
#[doc = "Field `brst_dn` reader - USB Bus Reset Completed Interrupt Flag"]
pub struct BRST_DN_R(crate::FieldReader<bool>);
impl BRST_DN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BRST_DN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BRST_DN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `brst_dn` writer - USB Bus Reset Completed Interrupt Flag"]
pub struct BRST_DN_W<'a> {
    w: &'a mut W,
}
impl<'a> BRST_DN_W<'a> {
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
#[doc = "Field `setup` reader - Setup Packet Interrupt Flag"]
pub struct SETUP_R(crate::FieldReader<bool>);
impl SETUP_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SETUP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETUP_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `setup` writer - Setup Packet Interrupt Flag"]
pub struct SETUP_W<'a> {
    w: &'a mut W,
}
impl<'a> SETUP_W<'a> {
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
#[doc = "Field `ep_in` reader - Endpoint IN Interrupt Flag"]
pub struct EP_IN_R(crate::FieldReader<bool>);
impl EP_IN_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_IN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_IN_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_in` writer - Endpoint IN Interrupt Flag"]
pub struct EP_IN_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_IN_W<'a> {
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
#[doc = "Field `ep_out` reader - Endpoint OUT Interrupt Flag"]
pub struct EP_OUT_R(crate::FieldReader<bool>);
impl EP_OUT_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_OUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_OUT_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_out` writer - Endpoint OUT Interrupt Flag"]
pub struct EP_OUT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_OUT_W<'a> {
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
#[doc = "Field `ep_nak` reader - Endpoint NAK Interrupt Flag"]
pub struct EP_NAK_R(crate::FieldReader<bool>);
impl EP_NAK_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        EP_NAK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_NAK_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ep_nak` writer - Endpoint NAK Interrupt Flag"]
pub struct EP_NAK_W<'a> {
    w: &'a mut W,
}
impl<'a> EP_NAK_W<'a> {
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
#[doc = "Field `dma_err` reader - DMA Error Interrupt Flag"]
pub struct DMA_ERR_R(crate::FieldReader<bool>);
impl DMA_ERR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        DMA_ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMA_ERR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `dma_err` writer - DMA Error Interrupt Flag"]
pub struct DMA_ERR_W<'a> {
    w: &'a mut W,
}
impl<'a> DMA_ERR_W<'a> {
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
#[doc = "Field `buf_ovr` reader - Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR_R(crate::FieldReader<bool>);
impl BUF_OVR_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        BUF_OVR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUF_OVR_R {
    type Target = crate::FieldReader<bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `buf_ovr` writer - Buffer Overflow Interrupt Flag"]
pub struct BUF_OVR_W<'a> {
    w: &'a mut W,
}
impl<'a> BUF_OVR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - DPLUS Activity Interrupt Flag"]
    #[inline(always)]
    pub fn dpact(&self) -> DPACT_R {
        DPACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Remote Wakeup Done Interrupt Flag"]
    #[inline(always)]
    pub fn rwu_dn(&self) -> RWU_DN_R {
        RWU_DN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB Bus Activity Interrupt Flag"]
    #[inline(always)]
    pub fn bact(&self) -> BACT_R {
        BACT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB Bus Reset In Progress Interrupt Flag"]
    #[inline(always)]
    pub fn brst(&self) -> BRST_R {
        BRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - USB Suspend Interrupt Flag"]
    #[inline(always)]
    pub fn susp(&self) -> SUSP_R {
        SUSP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - No VBUS Interrupt Flag"]
    #[inline(always)]
    pub fn no_vbus(&self) -> NO_VBUS_R {
        NO_VBUS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VBUS Detect Interrupt Flag"]
    #[inline(always)]
    pub fn vbus(&self) -> VBUS_R {
        VBUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - USB Bus Reset Completed Interrupt Flag"]
    #[inline(always)]
    pub fn brst_dn(&self) -> BRST_DN_R {
        BRST_DN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Setup Packet Interrupt Flag"]
    #[inline(always)]
    pub fn setup(&self) -> SETUP_R {
        SETUP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Endpoint IN Interrupt Flag"]
    #[inline(always)]
    pub fn ep_in(&self) -> EP_IN_R {
        EP_IN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Endpoint OUT Interrupt Flag"]
    #[inline(always)]
    pub fn ep_out(&self) -> EP_OUT_R {
        EP_OUT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Endpoint NAK Interrupt Flag"]
    #[inline(always)]
    pub fn ep_nak(&self) -> EP_NAK_R {
        EP_NAK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err(&self) -> DMA_ERR_R {
        DMA_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr(&self) -> BUF_OVR_R {
        BUF_OVR_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPLUS Activity Interrupt Flag"]
    #[inline(always)]
    pub fn dpact(&mut self) -> DPACT_W {
        DPACT_W { w: self }
    }
    #[doc = "Bit 1 - Remote Wakeup Done Interrupt Flag"]
    #[inline(always)]
    pub fn rwu_dn(&mut self) -> RWU_DN_W {
        RWU_DN_W { w: self }
    }
    #[doc = "Bit 2 - USB Bus Activity Interrupt Flag"]
    #[inline(always)]
    pub fn bact(&mut self) -> BACT_W {
        BACT_W { w: self }
    }
    #[doc = "Bit 3 - USB Bus Reset In Progress Interrupt Flag"]
    #[inline(always)]
    pub fn brst(&mut self) -> BRST_W {
        BRST_W { w: self }
    }
    #[doc = "Bit 4 - USB Suspend Interrupt Flag"]
    #[inline(always)]
    pub fn susp(&mut self) -> SUSP_W {
        SUSP_W { w: self }
    }
    #[doc = "Bit 5 - No VBUS Interrupt Flag"]
    #[inline(always)]
    pub fn no_vbus(&mut self) -> NO_VBUS_W {
        NO_VBUS_W { w: self }
    }
    #[doc = "Bit 6 - VBUS Detect Interrupt Flag"]
    #[inline(always)]
    pub fn vbus(&mut self) -> VBUS_W {
        VBUS_W { w: self }
    }
    #[doc = "Bit 7 - USB Bus Reset Completed Interrupt Flag"]
    #[inline(always)]
    pub fn brst_dn(&mut self) -> BRST_DN_W {
        BRST_DN_W { w: self }
    }
    #[doc = "Bit 8 - Setup Packet Interrupt Flag"]
    #[inline(always)]
    pub fn setup(&mut self) -> SETUP_W {
        SETUP_W { w: self }
    }
    #[doc = "Bit 9 - Endpoint IN Interrupt Flag"]
    #[inline(always)]
    pub fn ep_in(&mut self) -> EP_IN_W {
        EP_IN_W { w: self }
    }
    #[doc = "Bit 10 - Endpoint OUT Interrupt Flag"]
    #[inline(always)]
    pub fn ep_out(&mut self) -> EP_OUT_W {
        EP_OUT_W { w: self }
    }
    #[doc = "Bit 11 - Endpoint NAK Interrupt Flag"]
    #[inline(always)]
    pub fn ep_nak(&mut self) -> EP_NAK_W {
        EP_NAK_W { w: self }
    }
    #[doc = "Bit 12 - DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn dma_err(&mut self) -> DMA_ERR_W {
        DMA_ERR_W { w: self }
    }
    #[doc = "Bit 13 - Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn buf_ovr(&mut self) -> BUF_OVR_W {
        BUF_OVR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dev_inten](index.html) module"]
pub struct DEV_INTEN_SPEC;
impl crate::RegisterSpec for DEV_INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dev_inten::R](R) reader structure"]
impl crate::Readable for DEV_INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dev_inten::W](W) writer structure"]
impl crate::Writable for DEV_INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DEV_INTEN to value 0"]
impl crate::Resettable for DEV_INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
