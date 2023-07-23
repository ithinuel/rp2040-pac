#[doc = "Register `TIMER0` reader"]
pub struct R(crate::R<TIMER0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMER0` writer"]
pub struct W(crate::W<TIMER0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMER0_SPEC>;
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
impl From<crate::W<TIMER0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMER0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Y` reader - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
pub type Y_R = crate::FieldReader<u16, u16>;
#[doc = "Field `Y` writer - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
pub type Y_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER0_SPEC, u16, u16, 16, O>;
#[doc = "Field `X` reader - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
pub type X_R = crate::FieldReader<u16, u16>;
#[doc = "Field `X` writer - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
pub type X_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMER0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn y(&self) -> Y_R {
        Y_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn x(&self) -> X_R {
        X_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    #[must_use]
    pub fn y(&mut self) -> Y_W<0> {
        Y_W::new(self)
    }
    #[doc = "Bits 16:31 - Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    #[must_use]
    pub fn x(&mut self) -> X_W<16> {
        X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Pacing (X/Y) Fractional Timer  
 The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less.  

This register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).  

For information about available fields see [timer0](index.html) module"]
pub struct TIMER0_SPEC;
impl crate::RegisterSpec for TIMER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0::R](R) reader structure"]
impl crate::Readable for TIMER0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timer0::W](W) writer structure"]
impl crate::Writable for TIMER0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER0 to value 0"]
impl crate::Resettable for TIMER0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
