#[doc = "Register `SHPR3` reader"]
pub struct R(crate::R<SHPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHPR3` writer"]
pub struct W(crate::W<SHPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHPR3_SPEC>;
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
impl From<crate::W<SHPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRI_15` reader - Priority of system handler 15, SysTick"]
pub struct PRI_15_R(crate::FieldReader<u8, u8>);
impl PRI_15_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_15_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_15_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_15` writer - Priority of system handler 15, SysTick"]
pub struct PRI_15_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "Field `PRI_14` reader - Priority of system handler 14, PendSV"]
pub struct PRI_14_R(crate::FieldReader<u8, u8>);
impl PRI_14_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRI_14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRI_14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PRI_14` writer - Priority of system handler 14, PendSV"]
pub struct PRI_14_W<'a> {
    w: &'a mut W,
}
impl<'a> PRI_14_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - Priority of system handler 15, SysTick"]
    #[inline(always)]
    pub fn pri_15(&self) -> PRI_15_R {
        PRI_15_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - Priority of system handler 14, PendSV"]
    #[inline(always)]
    pub fn pri_14(&self) -> PRI_14_R {
        PRI_14_R::new(((self.bits >> 22) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - Priority of system handler 15, SysTick"]
    #[inline(always)]
    pub fn pri_15(&mut self) -> PRI_15_W {
        PRI_15_W { w: self }
    }
    #[doc = "Bits 22:23 - Priority of system handler 14, PendSV"]
    #[inline(always)]
    pub fn pri_14(&mut self) -> PRI_14_W {
        PRI_14_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [shpr3](index.html) module"]
pub struct SHPR3_SPEC;
impl crate::RegisterSpec for SHPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shpr3::R](R) reader structure"]
impl crate::Readable for SHPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shpr3::W](W) writer structure"]
impl crate::Writable for SHPR3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SHPR3 to value 0"]
impl crate::Resettable for SHPR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
