#[doc = "Register `CC` reader"]
pub type R = crate::R<CC_SPEC>;
#[doc = "Register `CC` writer"]
pub type W = crate::W<CC_SPEC>;
#[doc = "Field `A` reader - "]
pub type A_R = crate::FieldReader<u16>;
#[doc = "Field `A` writer - "]
pub type A_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `B` reader - "]
pub type B_R = crate::FieldReader<u16>;
#[doc = "Field `B` writer - "]
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn a(&self) -> A_R {
        A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CC")
            .field("b", &self.b())
            .field("a", &self.a())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn a(&mut self) -> A_W<CC_SPEC> {
        A_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W<CC_SPEC> {
        B_W::new(self, 16)
    }
}
#[doc = "Counter compare values  

You can [`read`](crate::Reg::read) this register and get [`cc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC_SPEC;
impl crate::RegisterSpec for CC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc::R`](R) reader structure"]
impl crate::Readable for CC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc::W`](W) writer structure"]
impl crate::Writable for CC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC to value 0"]
impl crate::Resettable for CC_SPEC {}
