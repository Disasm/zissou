#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BRR {
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
}
#[doc = "Values that can be written to the field `BR0`"]
pub enum BR0W {
    #[doc = "No action on the corresponding ODx bit"]
    NOACTION,
    #[doc = "Reset the ODx bit"]
    RESET,
}
impl BR0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BR0W::NOACTION => false,
            BR0W::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BR0W<'a> {
    w: &'a mut W,
}
impl<'a> _BR0W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR0W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR1`"]
pub type BR1W = BR0W;
#[doc = r" Proxy"]
pub struct _BR1W<'a> {
    w: &'a mut W,
}
impl<'a> _BR1W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR1W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 1;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR2`"]
pub type BR2W = BR0W;
#[doc = r" Proxy"]
pub struct _BR2W<'a> {
    w: &'a mut W,
}
impl<'a> _BR2W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR2W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 2;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR3`"]
pub type BR3W = BR0W;
#[doc = r" Proxy"]
pub struct _BR3W<'a> {
    w: &'a mut W,
}
impl<'a> _BR3W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR3W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 3;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR4`"]
pub type BR4W = BR0W;
#[doc = r" Proxy"]
pub struct _BR4W<'a> {
    w: &'a mut W,
}
impl<'a> _BR4W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR4W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR5`"]
pub type BR5W = BR0W;
#[doc = r" Proxy"]
pub struct _BR5W<'a> {
    w: &'a mut W,
}
impl<'a> _BR5W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR5W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 5;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR6`"]
pub type BR6W = BR0W;
#[doc = r" Proxy"]
pub struct _BR6W<'a> {
    w: &'a mut W,
}
impl<'a> _BR6W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR6W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 6;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR7`"]
pub type BR7W = BR0W;
#[doc = r" Proxy"]
pub struct _BR7W<'a> {
    w: &'a mut W,
}
impl<'a> _BR7W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR7W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 7;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR8`"]
pub type BR8W = BR0W;
#[doc = r" Proxy"]
pub struct _BR8W<'a> {
    w: &'a mut W,
}
impl<'a> _BR8W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR8W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR9`"]
pub type BR9W = BR0W;
#[doc = r" Proxy"]
pub struct _BR9W<'a> {
    w: &'a mut W,
}
impl<'a> _BR9W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR9W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR0W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR0W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 9;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR10`"]
pub enum BR10W {
    #[doc = "No action on the corresponding ODx bit"]
    NOACTION,
    #[doc = "Reset the ODx bit"]
    RESET,
}
impl BR10W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline]
    pub fn _bits(&self) -> bool {
        match *self {
            BR10W::NOACTION => false,
            BR10W::RESET => true,
        }
    }
}
#[doc = r" Proxy"]
pub struct _BR10W<'a> {
    w: &'a mut W,
}
impl<'a> _BR10W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR10W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR10W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR10W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 10;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR11`"]
pub type BR11W = BR10W;
#[doc = r" Proxy"]
pub struct _BR11W<'a> {
    w: &'a mut W,
}
impl<'a> _BR11W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR11W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR10W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR10W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 11;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR12`"]
pub type BR12W = BR10W;
#[doc = r" Proxy"]
pub struct _BR12W<'a> {
    w: &'a mut W,
}
impl<'a> _BR12W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR12W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR10W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR10W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR13`"]
pub type BR13W = BR10W;
#[doc = r" Proxy"]
pub struct _BR13W<'a> {
    w: &'a mut W,
}
impl<'a> _BR13W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR13W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR10W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR10W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 13;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR14`"]
pub type BR14W = BR10W;
#[doc = r" Proxy"]
pub struct _BR14W<'a> {
    w: &'a mut W,
}
impl<'a> _BR14W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR14W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR10W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR10W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 14;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = "Values that can be written to the field `BR15`"]
pub type BR15W = BR10W;
#[doc = r" Proxy"]
pub struct _BR15W<'a> {
    w: &'a mut W,
}
impl<'a> _BR15W<'a> {
    #[doc = r" Writes `variant` to the field"]
    #[inline]
    pub fn variant(self, variant: BR15W) -> &'a mut W {
        {
            self.bit(variant._bits())
        }
    }
    #[doc = "No action on the corresponding ODx bit"]
    #[inline]
    pub fn no_action(self) -> &'a mut W {
        self.variant(BR10W::NOACTION)
    }
    #[doc = "Reset the ODx bit"]
    #[inline]
    pub fn reset(self) -> &'a mut W {
        self.variant(BR10W::RESET)
    }
    #[doc = r" Sets the field bit"]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r" Clears the field bit"]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub fn bit(self, value: bool) -> &'a mut W {
        const MASK: bool = true;
        const OFFSET: u8 = 15;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Port Reset bit"]
    #[inline]
    pub fn br0(&mut self) -> _BR0W {
        _BR0W { w: self }
    }
    #[doc = "Bit 1 - Port Reset bit"]
    #[inline]
    pub fn br1(&mut self) -> _BR1W {
        _BR1W { w: self }
    }
    #[doc = "Bit 2 - Port Reset bit"]
    #[inline]
    pub fn br2(&mut self) -> _BR2W {
        _BR2W { w: self }
    }
    #[doc = "Bit 3 - Port Reset bit"]
    #[inline]
    pub fn br3(&mut self) -> _BR3W {
        _BR3W { w: self }
    }
    #[doc = "Bit 4 - Port Reset bit"]
    #[inline]
    pub fn br4(&mut self) -> _BR4W {
        _BR4W { w: self }
    }
    #[doc = "Bit 5 - Port Reset bit"]
    #[inline]
    pub fn br5(&mut self) -> _BR5W {
        _BR5W { w: self }
    }
    #[doc = "Bit 6 - Port Reset bit"]
    #[inline]
    pub fn br6(&mut self) -> _BR6W {
        _BR6W { w: self }
    }
    #[doc = "Bit 7 - Port Reset bit"]
    #[inline]
    pub fn br7(&mut self) -> _BR7W {
        _BR7W { w: self }
    }
    #[doc = "Bit 8 - Port Reset bit"]
    #[inline]
    pub fn br8(&mut self) -> _BR8W {
        _BR8W { w: self }
    }
    #[doc = "Bit 9 - Port Reset bit"]
    #[inline]
    pub fn br9(&mut self) -> _BR9W {
        _BR9W { w: self }
    }
    #[doc = "Bit 10 - Port Reset bit"]
    #[inline]
    pub fn br10(&mut self) -> _BR10W {
        _BR10W { w: self }
    }
    #[doc = "Bit 11 - Port Reset bit"]
    #[inline]
    pub fn br11(&mut self) -> _BR11W {
        _BR11W { w: self }
    }
    #[doc = "Bit 12 - Port Reset bit"]
    #[inline]
    pub fn br12(&mut self) -> _BR12W {
        _BR12W { w: self }
    }
    #[doc = "Bit 13 - Port Reset bit"]
    #[inline]
    pub fn br13(&mut self) -> _BR13W {
        _BR13W { w: self }
    }
    #[doc = "Bit 14 - Port Reset bit"]
    #[inline]
    pub fn br14(&mut self) -> _BR14W {
        _BR14W { w: self }
    }
    #[doc = "Bit 15 - Port Reset bit"]
    #[inline]
    pub fn br15(&mut self) -> _BR15W {
        _BR15W { w: self }
    }
}
