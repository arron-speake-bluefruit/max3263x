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
#[doc = "TX FIFO Almost Empty Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_FIFO_AE_A {
    #[doc = "0: Disable Interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable Interrupt"]
    ENABLED = 1,
}
impl From<TX_FIFO_AE_A> for bool {
    #[inline(always)]
    fn from(variant: TX_FIFO_AE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tx_fifo_ae` reader - TX FIFO Almost Empty Int Enable"]
pub struct TX_FIFO_AE_R(crate::FieldReader<bool, TX_FIFO_AE_A>);
impl TX_FIFO_AE_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_FIFO_AE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_FIFO_AE_A {
        match self.bits {
            false => TX_FIFO_AE_A::DISABLED,
            true => TX_FIFO_AE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TX_FIFO_AE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TX_FIFO_AE_A::ENABLED
    }
}
impl core::ops::Deref for TX_FIFO_AE_R {
    type Target = crate::FieldReader<bool, TX_FIFO_AE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_fifo_ae` writer - TX FIFO Almost Empty Int Enable"]
pub struct TX_FIFO_AE_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_AE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_FIFO_AE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_FIFO_AE_A::DISABLED)
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_FIFO_AE_A::ENABLED)
    }
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
#[doc = "RX FIFO Almost Full Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_FIFO_AF_A {
    #[doc = "0: Disable Interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable Interrupt"]
    ENABLED = 1,
}
impl From<RX_FIFO_AF_A> for bool {
    #[inline(always)]
    fn from(variant: RX_FIFO_AF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rx_fifo_af` reader - RX FIFO Almost Full Int Enable"]
pub struct RX_FIFO_AF_R(crate::FieldReader<bool, RX_FIFO_AF_A>);
impl RX_FIFO_AF_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_FIFO_AF_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_FIFO_AF_A {
        match self.bits {
            false => RX_FIFO_AF_A::DISABLED,
            true => RX_FIFO_AF_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RX_FIFO_AF_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RX_FIFO_AF_A::ENABLED
    }
}
impl core::ops::Deref for RX_FIFO_AF_R {
    type Target = crate::FieldReader<bool, RX_FIFO_AF_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_fifo_af` writer - RX FIFO Almost Full Int Enable"]
pub struct RX_FIFO_AF_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_AF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_FIFO_AF_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RX_FIFO_AF_A::DISABLED)
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RX_FIFO_AF_A::ENABLED)
    }
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
#[doc = "No Data in TX FIFO Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_NO_DATA_A {
    #[doc = "0: Disable Interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable Interrupt"]
    ENABLED = 1,
}
impl From<TX_NO_DATA_A> for bool {
    #[inline(always)]
    fn from(variant: TX_NO_DATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tx_no_data` reader - No Data in TX FIFO Int Enable"]
pub struct TX_NO_DATA_R(crate::FieldReader<bool, TX_NO_DATA_A>);
impl TX_NO_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_NO_DATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_NO_DATA_A {
        match self.bits {
            false => TX_NO_DATA_A::DISABLED,
            true => TX_NO_DATA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TX_NO_DATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TX_NO_DATA_A::ENABLED
    }
}
impl core::ops::Deref for TX_NO_DATA_R {
    type Target = crate::FieldReader<bool, TX_NO_DATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_no_data` writer - No Data in TX FIFO Int Enable"]
pub struct TX_NO_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_NO_DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_NO_DATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_NO_DATA_A::DISABLED)
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_NO_DATA_A::ENABLED)
    }
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
#[doc = "RX FIFO Overflow Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RX_LOST_DATA_A {
    #[doc = "0: Disable Interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable Interrupt"]
    ENABLED = 1,
}
impl From<RX_LOST_DATA_A> for bool {
    #[inline(always)]
    fn from(variant: RX_LOST_DATA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `rx_lost_data` reader - RX FIFO Overflow Int Enable"]
pub struct RX_LOST_DATA_R(crate::FieldReader<bool, RX_LOST_DATA_A>);
impl RX_LOST_DATA_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        RX_LOST_DATA_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RX_LOST_DATA_A {
        match self.bits {
            false => RX_LOST_DATA_A::DISABLED,
            true => RX_LOST_DATA_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == RX_LOST_DATA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == RX_LOST_DATA_A::ENABLED
    }
}
impl core::ops::Deref for RX_LOST_DATA_R {
    type Target = crate::FieldReader<bool, RX_LOST_DATA_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `rx_lost_data` writer - RX FIFO Overflow Int Enable"]
pub struct RX_LOST_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_LOST_DATA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RX_LOST_DATA_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RX_LOST_DATA_A::DISABLED)
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RX_LOST_DATA_A::ENABLED)
    }
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
#[doc = "TX Underflow Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TX_UNDERFLOW_A {
    #[doc = "0: Disable Interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable Interrupt"]
    ENABLED = 1,
}
impl From<TX_UNDERFLOW_A> for bool {
    #[inline(always)]
    fn from(variant: TX_UNDERFLOW_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tx_underflow` reader - TX Underflow Int Enable"]
pub struct TX_UNDERFLOW_R(crate::FieldReader<bool, TX_UNDERFLOW_A>);
impl TX_UNDERFLOW_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        TX_UNDERFLOW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_UNDERFLOW_A {
        match self.bits {
            false => TX_UNDERFLOW_A::DISABLED,
            true => TX_UNDERFLOW_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == TX_UNDERFLOW_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == TX_UNDERFLOW_A::ENABLED
    }
}
impl core::ops::Deref for TX_UNDERFLOW_R {
    type Target = crate::FieldReader<bool, TX_UNDERFLOW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `tx_underflow` writer - TX Underflow Int Enable"]
pub struct TX_UNDERFLOW_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_UNDERFLOW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TX_UNDERFLOW_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_A::DISABLED)
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TX_UNDERFLOW_A::ENABLED)
    }
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
#[doc = "Slave Select Asserted Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_ASSERTED_A {
    #[doc = "0: Disable Interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable Interrupt"]
    ENABLED = 1,
}
impl From<SS_ASSERTED_A> for bool {
    #[inline(always)]
    fn from(variant: SS_ASSERTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ss_asserted` reader - Slave Select Asserted Int Enable"]
pub struct SS_ASSERTED_R(crate::FieldReader<bool, SS_ASSERTED_A>);
impl SS_ASSERTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_ASSERTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_ASSERTED_A {
        match self.bits {
            false => SS_ASSERTED_A::DISABLED,
            true => SS_ASSERTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SS_ASSERTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SS_ASSERTED_A::ENABLED
    }
}
impl core::ops::Deref for SS_ASSERTED_R {
    type Target = crate::FieldReader<bool, SS_ASSERTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_asserted` writer - Slave Select Asserted Int Enable"]
pub struct SS_ASSERTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_ASSERTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_ASSERTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SS_ASSERTED_A::DISABLED)
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SS_ASSERTED_A::ENABLED)
    }
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
#[doc = "Slave Select Deasserted Int Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SS_DEASSERTED_A {
    #[doc = "0: Disable Interrupt"]
    DISABLED = 0,
    #[doc = "1: Enable Interrupt"]
    ENABLED = 1,
}
impl From<SS_DEASSERTED_A> for bool {
    #[inline(always)]
    fn from(variant: SS_DEASSERTED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ss_deasserted` reader - Slave Select Deasserted Int Enable"]
pub struct SS_DEASSERTED_R(crate::FieldReader<bool, SS_DEASSERTED_A>);
impl SS_DEASSERTED_R {
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        SS_DEASSERTED_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SS_DEASSERTED_A {
        match self.bits {
            false => SS_DEASSERTED_A::DISABLED,
            true => SS_DEASSERTED_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        **self == SS_DEASSERTED_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        **self == SS_DEASSERTED_A::ENABLED
    }
}
impl core::ops::Deref for SS_DEASSERTED_R {
    type Target = crate::FieldReader<bool, SS_DEASSERTED_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ss_deasserted` writer - Slave Select Deasserted Int Enable"]
pub struct SS_DEASSERTED_W<'a> {
    w: &'a mut W,
}
impl<'a> SS_DEASSERTED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SS_DEASSERTED_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable Interrupt"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SS_DEASSERTED_A::DISABLED)
    }
    #[doc = "Enable Interrupt"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SS_DEASSERTED_A::ENABLED)
    }
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
impl R {
    #[doc = "Bit 0 - TX FIFO Almost Empty Int Enable"]
    #[inline(always)]
    pub fn tx_fifo_ae(&self) -> TX_FIFO_AE_R {
        TX_FIFO_AE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RX FIFO Almost Full Int Enable"]
    #[inline(always)]
    pub fn rx_fifo_af(&self) -> RX_FIFO_AF_R {
        RX_FIFO_AF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - No Data in TX FIFO Int Enable"]
    #[inline(always)]
    pub fn tx_no_data(&self) -> TX_NO_DATA_R {
        TX_NO_DATA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - RX FIFO Overflow Int Enable"]
    #[inline(always)]
    pub fn rx_lost_data(&self) -> RX_LOST_DATA_R {
        RX_LOST_DATA_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX Underflow Int Enable"]
    #[inline(always)]
    pub fn tx_underflow(&self) -> TX_UNDERFLOW_R {
        TX_UNDERFLOW_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Slave Select Asserted Int Enable"]
    #[inline(always)]
    pub fn ss_asserted(&self) -> SS_ASSERTED_R {
        SS_ASSERTED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Slave Select Deasserted Int Enable"]
    #[inline(always)]
    pub fn ss_deasserted(&self) -> SS_DEASSERTED_R {
        SS_DEASSERTED_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TX FIFO Almost Empty Int Enable"]
    #[inline(always)]
    pub fn tx_fifo_ae(&mut self) -> TX_FIFO_AE_W {
        TX_FIFO_AE_W { w: self }
    }
    #[doc = "Bit 1 - RX FIFO Almost Full Int Enable"]
    #[inline(always)]
    pub fn rx_fifo_af(&mut self) -> RX_FIFO_AF_W {
        RX_FIFO_AF_W { w: self }
    }
    #[doc = "Bit 2 - No Data in TX FIFO Int Enable"]
    #[inline(always)]
    pub fn tx_no_data(&mut self) -> TX_NO_DATA_W {
        TX_NO_DATA_W { w: self }
    }
    #[doc = "Bit 3 - RX FIFO Overflow Int Enable"]
    #[inline(always)]
    pub fn rx_lost_data(&mut self) -> RX_LOST_DATA_W {
        RX_LOST_DATA_W { w: self }
    }
    #[doc = "Bit 4 - TX Underflow Int Enable"]
    #[inline(always)]
    pub fn tx_underflow(&mut self) -> TX_UNDERFLOW_W {
        TX_UNDERFLOW_W { w: self }
    }
    #[doc = "Bit 5 - Slave Select Asserted Int Enable"]
    #[inline(always)]
    pub fn ss_asserted(&mut self) -> SS_ASSERTED_W {
        SS_ASSERTED_W { w: self }
    }
    #[doc = "Bit 6 - Slave Select Deasserted Int Enable"]
    #[inline(always)]
    pub fn ss_deasserted(&mut self) -> SS_DEASSERTED_W {
        SS_DEASSERTED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI Slave Interrupt Enable/Disable Settings\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
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
