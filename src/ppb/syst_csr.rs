#[doc = "Register `SYST_CSR` reader"]
pub struct R(crate::R<SYST_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYST_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYST_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYST_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYST_CSR` writer"]
pub struct W(crate::W<SYST_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYST_CSR_SPEC>;
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
impl From<crate::W<SYST_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYST_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNTFLAG` reader - Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
pub struct COUNTFLAG_R(crate::FieldReader<bool, bool>);
impl COUNTFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        COUNTFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for COUNTFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSOURCE` reader - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
pub struct CLKSOURCE_R(crate::FieldReader<bool, bool>);
impl CLKSOURCE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CLKSOURCE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLKSOURCE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLKSOURCE` writer - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF.  
 Selects the SysTick timer clock source:  
 0 = External reference clock.  
 1 = Processor clock."]
pub struct CLKSOURCE_W<'a> {
    w: &'a mut W,
}
impl<'a> CLKSOURCE_W<'a> {
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
#[doc = "Field `TICKINT` reader - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
pub struct TICKINT_R(crate::FieldReader<bool, bool>);
impl TICKINT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TICKINT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TICKINT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TICKINT` writer - Enables SysTick exception request:  
 0 = Counting down to zero does not assert the SysTick exception request.  
 1 = Counting down to zero to asserts the SysTick exception request."]
pub struct TICKINT_W<'a> {
    w: &'a mut W,
}
impl<'a> TICKINT_W<'a> {
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
#[doc = "Field `ENABLE` reader - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - Enable SysTick counter:  
 0 = Counter disabled.  
 1 = Counter enabled."]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
impl R {
    #[doc = "Bit 16 - Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
    #[inline(always)]
    pub fn countflag(&self) -> COUNTFLAG_R {
        COUNTFLAG_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 2 - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF. Selects the SysTick timer clock source: 0 = External reference clock. 1 = Processor clock."]
    #[inline(always)]
    pub fn clksource(&self) -> CLKSOURCE_R {
        CLKSOURCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Enables SysTick exception request: 0 = Counting down to zero does not assert the SysTick exception request. 1 = Counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn tickint(&self) -> TICKINT_R {
        TICKINT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Enable SysTick counter: 0 = Counter disabled. 1 = Counter enabled."]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - SysTick clock source. Always reads as one if SYST_CALIB reports NOREF. Selects the SysTick timer clock source: 0 = External reference clock. 1 = Processor clock."]
    #[inline(always)]
    pub fn clksource(&mut self) -> CLKSOURCE_W {
        CLKSOURCE_W { w: self }
    }
    #[doc = "Bit 1 - Enables SysTick exception request: 0 = Counting down to zero does not assert the SysTick exception request. 1 = Counting down to zero to asserts the SysTick exception request."]
    #[inline(always)]
    pub fn tickint(&mut self) -> TICKINT_W {
        TICKINT_W { w: self }
    }
    #[doc = "Bit 0 - Enable SysTick counter: 0 = Counter disabled. 1 = Counter enabled."]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [syst_csr](index.html) module"]
pub struct SYST_CSR_SPEC;
impl crate::RegisterSpec for SYST_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syst_csr::R](R) reader structure"]
impl crate::Readable for SYST_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syst_csr::W](W) writer structure"]
impl crate::Writable for SYST_CSR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SYST_CSR to value 0"]
impl crate::Resettable for SYST_CSR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
