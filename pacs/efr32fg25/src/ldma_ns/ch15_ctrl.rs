#[doc = "Register `CH15_CTRL` reader"]
pub type R = crate::R<Ch15CtrlSpec>;
#[doc = "Register `CH15_CTRL` writer"]
pub type W = crate::W<Ch15CtrlSpec>;
#[doc = "DMA Structure Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Structtype {
    #[doc = "0: DMA transfer structure type selected."]
    Transfer = 0,
    #[doc = "1: Synchronization structure type selected."]
    Synchronize = 1,
    #[doc = "2: Write immediate value structure type selected."]
    Write = 2,
}
impl From<Structtype> for u8 {
    #[inline(always)]
    fn from(variant: Structtype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Structtype {
    type Ux = u8;
}
impl crate::IsEnum for Structtype {}
#[doc = "Field `STRUCTTYPE` reader - DMA Structure Type"]
pub type StructtypeR = crate::FieldReader<Structtype>;
impl StructtypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Structtype> {
        match self.bits {
            0 => Some(Structtype::Transfer),
            1 => Some(Structtype::Synchronize),
            2 => Some(Structtype::Write),
            _ => None,
        }
    }
    #[doc = "DMA transfer structure type selected."]
    #[inline(always)]
    pub fn is_transfer(&self) -> bool {
        *self == Structtype::Transfer
    }
    #[doc = "Synchronization structure type selected."]
    #[inline(always)]
    pub fn is_synchronize(&self) -> bool {
        *self == Structtype::Synchronize
    }
    #[doc = "Write immediate value structure type selected."]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == Structtype::Write
    }
}
#[doc = "Field `STRUCTTYPE` writer - DMA Structure Type"]
pub type StructtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Structtype>;
impl<'a, REG> StructtypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA transfer structure type selected."]
    #[inline(always)]
    pub fn transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Structtype::Transfer)
    }
    #[doc = "Synchronization structure type selected."]
    #[inline(always)]
    pub fn synchronize(self) -> &'a mut crate::W<REG> {
        self.variant(Structtype::Synchronize)
    }
    #[doc = "Write immediate value structure type selected."]
    #[inline(always)]
    pub fn write(self) -> &'a mut crate::W<REG> {
        self.variant(Structtype::Write)
    }
}
#[doc = "Field `EXTEND` reader - Extend"]
pub type ExtendR = crate::BitReader;
#[doc = "Field `EXTEND` writer - Extend"]
pub type ExtendW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRUCTREQ` reader - Structure DMA Transfer Request"]
pub type StructreqR = crate::BitReader;
#[doc = "Field `XFERCNT` reader - DMA Unit Data Transfer Count"]
pub type XfercntR = crate::FieldReader<u16>;
#[doc = "Field `XFERCNT` writer - DMA Unit Data Transfer Count"]
pub type XfercntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `BYTESWAP` reader - Endian Byte Swap"]
pub type ByteswapR = crate::BitReader;
#[doc = "Field `BYTESWAP` writer - Endian Byte Swap"]
pub type ByteswapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Block Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Blocksize {
    #[doc = "0: 1 unit transfer per arbitration"]
    Unit1 = 0,
    #[doc = "1: 2 unit transfers per arbitration"]
    Unit2 = 1,
    #[doc = "2: 3 unit transfers per arbitration"]
    Unit3 = 2,
    #[doc = "3: 4 unit transfers per arbitration"]
    Unit4 = 3,
    #[doc = "4: 6 unit transfers per arbitration"]
    Unit6 = 4,
    #[doc = "5: 8 unit transfers per arbitration"]
    Unit8 = 5,
    #[doc = "6: 12 unit transfers per arbitration"]
    Unit12 = 6,
    #[doc = "7: 16 unit transfers per arbitration"]
    Unit16 = 7,
    #[doc = "8: 24 unit transfers per arbitration"]
    Unit24 = 8,
    #[doc = "9: 32 unit transfers per arbitration"]
    Unit32 = 9,
    #[doc = "10: 64 unit transfers per arbitration"]
    Unit64 = 10,
    #[doc = "11: 128 unit transfers per arbitration"]
    Unit128 = 11,
    #[doc = "12: 256 unit transfers per arbitration"]
    Unit256 = 12,
    #[doc = "13: 512 unit transfers per arbitration"]
    Unit512 = 13,
    #[doc = "14: 1024 unit transfers per arbitration"]
    Unit1024 = 14,
    #[doc = "15: Transfer all units as specified by the XFRCNT field"]
    All = 15,
}
impl From<Blocksize> for u8 {
    #[inline(always)]
    fn from(variant: Blocksize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Blocksize {
    type Ux = u8;
}
impl crate::IsEnum for Blocksize {}
#[doc = "Field `BLOCKSIZE` reader - Block Transfer Size"]
pub type BlocksizeR = crate::FieldReader<Blocksize>;
impl BlocksizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Blocksize {
        match self.bits {
            0 => Blocksize::Unit1,
            1 => Blocksize::Unit2,
            2 => Blocksize::Unit3,
            3 => Blocksize::Unit4,
            4 => Blocksize::Unit6,
            5 => Blocksize::Unit8,
            6 => Blocksize::Unit12,
            7 => Blocksize::Unit16,
            8 => Blocksize::Unit24,
            9 => Blocksize::Unit32,
            10 => Blocksize::Unit64,
            11 => Blocksize::Unit128,
            12 => Blocksize::Unit256,
            13 => Blocksize::Unit512,
            14 => Blocksize::Unit1024,
            15 => Blocksize::All,
            _ => unreachable!(),
        }
    }
    #[doc = "1 unit transfer per arbitration"]
    #[inline(always)]
    pub fn is_unit1(&self) -> bool {
        *self == Blocksize::Unit1
    }
    #[doc = "2 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit2(&self) -> bool {
        *self == Blocksize::Unit2
    }
    #[doc = "3 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit3(&self) -> bool {
        *self == Blocksize::Unit3
    }
    #[doc = "4 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit4(&self) -> bool {
        *self == Blocksize::Unit4
    }
    #[doc = "6 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit6(&self) -> bool {
        *self == Blocksize::Unit6
    }
    #[doc = "8 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit8(&self) -> bool {
        *self == Blocksize::Unit8
    }
    #[doc = "12 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit12(&self) -> bool {
        *self == Blocksize::Unit12
    }
    #[doc = "16 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit16(&self) -> bool {
        *self == Blocksize::Unit16
    }
    #[doc = "24 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit24(&self) -> bool {
        *self == Blocksize::Unit24
    }
    #[doc = "32 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit32(&self) -> bool {
        *self == Blocksize::Unit32
    }
    #[doc = "64 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit64(&self) -> bool {
        *self == Blocksize::Unit64
    }
    #[doc = "128 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit128(&self) -> bool {
        *self == Blocksize::Unit128
    }
    #[doc = "256 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit256(&self) -> bool {
        *self == Blocksize::Unit256
    }
    #[doc = "512 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit512(&self) -> bool {
        *self == Blocksize::Unit512
    }
    #[doc = "1024 unit transfers per arbitration"]
    #[inline(always)]
    pub fn is_unit1024(&self) -> bool {
        *self == Blocksize::Unit1024
    }
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Blocksize::All
    }
}
#[doc = "Field `BLOCKSIZE` writer - Block Transfer Size"]
pub type BlocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 4, Blocksize, crate::Safe>;
impl<'a, REG> BlocksizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 unit transfer per arbitration"]
    #[inline(always)]
    pub fn unit1(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit1)
    }
    #[doc = "2 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit2(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit2)
    }
    #[doc = "3 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit3(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit3)
    }
    #[doc = "4 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit4(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit4)
    }
    #[doc = "6 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit6(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit6)
    }
    #[doc = "8 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit8(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit8)
    }
    #[doc = "12 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit12(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit12)
    }
    #[doc = "16 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit16(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit16)
    }
    #[doc = "24 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit24(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit24)
    }
    #[doc = "32 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit32(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit32)
    }
    #[doc = "64 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit64(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit64)
    }
    #[doc = "128 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit128(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit128)
    }
    #[doc = "256 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit256(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit256)
    }
    #[doc = "512 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit512(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit512)
    }
    #[doc = "1024 unit transfers per arbitration"]
    #[inline(always)]
    pub fn unit1024(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::Unit1024)
    }
    #[doc = "Transfer all units as specified by the XFRCNT field"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Blocksize::All)
    }
}
#[doc = "Field `DONEIEN` reader - DMA Operation Done Interrupt Flag Set"]
pub type DoneienR = crate::BitReader;
#[doc = "Field `DONEIEN` writer - DMA Operation Done Interrupt Flag Set"]
pub type DoneienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA Request Transfer Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Reqmode {
    #[doc = "0: The LDMA transfers one BLOCKSIZE per transfer request."]
    Block = 0,
    #[doc = "1: One transfer request transfers all units as defined by the XFRCNT field."]
    All = 1,
}
impl From<Reqmode> for bool {
    #[inline(always)]
    fn from(variant: Reqmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REQMODE` reader - DMA Request Transfer Mode Select"]
pub type ReqmodeR = crate::BitReader<Reqmode>;
impl ReqmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Reqmode {
        match self.bits {
            false => Reqmode::Block,
            true => Reqmode::All,
        }
    }
    #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
    #[inline(always)]
    pub fn is_block(&self) -> bool {
        *self == Reqmode::Block
    }
    #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Reqmode::All
    }
}
#[doc = "Field `REQMODE` writer - DMA Request Transfer Mode Select"]
pub type ReqmodeW<'a, REG> = crate::BitWriter<'a, REG, Reqmode>;
impl<'a, REG> ReqmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The LDMA transfers one BLOCKSIZE per transfer request."]
    #[inline(always)]
    pub fn block(self) -> &'a mut crate::W<REG> {
        self.variant(Reqmode::Block)
    }
    #[doc = "One transfer request transfers all units as defined by the XFRCNT field."]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Reqmode::All)
    }
}
#[doc = "Field `DECLOOPCNT` reader - Decrement Loop Count"]
pub type DecloopcntR = crate::BitReader;
#[doc = "Field `DECLOOPCNT` writer - Decrement Loop Count"]
pub type DecloopcntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IGNORESREQ` reader - Ignore Sreq"]
pub type IgnoresreqR = crate::BitReader;
#[doc = "Field `IGNORESREQ` writer - Ignore Sreq"]
pub type IgnoresreqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Source Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Srcinc {
    #[doc = "0: Increment source address by one unit data size after each read"]
    One = 0,
    #[doc = "1: Increment source address by two unit data sizes after each read"]
    Two = 1,
    #[doc = "2: Increment source address by four unit data sizes after each read"]
    Four = 2,
    #[doc = "3: Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    None = 3,
}
impl From<Srcinc> for u8 {
    #[inline(always)]
    fn from(variant: Srcinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Srcinc {
    type Ux = u8;
}
impl crate::IsEnum for Srcinc {}
#[doc = "Field `SRCINC` reader - Source Address Increment Size"]
pub type SrcincR = crate::FieldReader<Srcinc>;
impl SrcincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcinc {
        match self.bits {
            0 => Srcinc::One,
            1 => Srcinc::Two,
            2 => Srcinc::Four,
            3 => Srcinc::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Increment source address by one unit data size after each read"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Srcinc::One
    }
    #[doc = "Increment source address by two unit data sizes after each read"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Srcinc::Two
    }
    #[doc = "Increment source address by four unit data sizes after each read"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Srcinc::Four
    }
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Srcinc::None
    }
}
#[doc = "Field `SRCINC` writer - Source Address Increment Size"]
pub type SrcincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Srcinc, crate::Safe>;
impl<'a, REG> SrcincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment source address by one unit data size after each read"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::One)
    }
    #[doc = "Increment source address by two unit data sizes after each read"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::Two)
    }
    #[doc = "Increment source address by four unit data sizes after each read"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::Four)
    }
    #[doc = "Do not increment the source address. In this mode reads are made from a fixed source address, for example reading FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Srcinc::None)
    }
}
#[doc = "Unit Data Transfer Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Size {
    #[doc = "0: Each unit transfer is a byte"]
    Byte = 0,
    #[doc = "1: Each unit transfer is a half-word"]
    Halfword = 1,
    #[doc = "2: Each unit transfer is a word"]
    Word = 2,
}
impl From<Size> for u8 {
    #[inline(always)]
    fn from(variant: Size) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Size {
    type Ux = u8;
}
impl crate::IsEnum for Size {}
#[doc = "Field `SIZE` reader - Unit Data Transfer Size"]
pub type SizeR = crate::FieldReader<Size>;
impl SizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Size> {
        match self.bits {
            0 => Some(Size::Byte),
            1 => Some(Size::Halfword),
            2 => Some(Size::Word),
            _ => None,
        }
    }
    #[doc = "Each unit transfer is a byte"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == Size::Byte
    }
    #[doc = "Each unit transfer is a half-word"]
    #[inline(always)]
    pub fn is_halfword(&self) -> bool {
        *self == Size::Halfword
    }
    #[doc = "Each unit transfer is a word"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == Size::Word
    }
}
#[doc = "Field `SIZE` writer - Unit Data Transfer Size"]
pub type SizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Size>;
impl<'a, REG> SizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Each unit transfer is a byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Byte)
    }
    #[doc = "Each unit transfer is a half-word"]
    #[inline(always)]
    pub fn halfword(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Halfword)
    }
    #[doc = "Each unit transfer is a word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut crate::W<REG> {
        self.variant(Size::Word)
    }
}
#[doc = "Destination Address Increment Size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dstinc {
    #[doc = "0: Increment destination address by one unit data size after each write"]
    One = 0,
    #[doc = "1: Increment destination address by two unit data sizes after each write"]
    Two = 1,
    #[doc = "2: Increment destination address by four unit data sizes after each write"]
    Four = 2,
    #[doc = "3: Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    None = 3,
}
impl From<Dstinc> for u8 {
    #[inline(always)]
    fn from(variant: Dstinc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dstinc {
    type Ux = u8;
}
impl crate::IsEnum for Dstinc {}
#[doc = "Field `DSTINC` reader - Destination Address Increment Size"]
pub type DstincR = crate::FieldReader<Dstinc>;
impl DstincR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstinc {
        match self.bits {
            0 => Dstinc::One,
            1 => Dstinc::Two,
            2 => Dstinc::Four,
            3 => Dstinc::None,
            _ => unreachable!(),
        }
    }
    #[doc = "Increment destination address by one unit data size after each write"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Dstinc::One
    }
    #[doc = "Increment destination address by two unit data sizes after each write"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Dstinc::Two
    }
    #[doc = "Increment destination address by four unit data sizes after each write"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        *self == Dstinc::Four
    }
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dstinc::None
    }
}
#[doc = "Field `DSTINC` writer - Destination Address Increment Size"]
pub type DstincW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dstinc, crate::Safe>;
impl<'a, REG> DstincW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Increment destination address by one unit data size after each write"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::One)
    }
    #[doc = "Increment destination address by two unit data sizes after each write"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::Two)
    }
    #[doc = "Increment destination address by four unit data sizes after each write"]
    #[inline(always)]
    pub fn four(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::Four)
    }
    #[doc = "Do not increment the destination address. Writes are made to a fixed destination address, for example writing to a FIFO."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dstinc::None)
    }
}
#[doc = "Source Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srcmode {
    #[doc = "0: The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
    Absolute = 0,
    #[doc = "1: The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
    Relative = 1,
}
impl From<Srcmode> for bool {
    #[inline(always)]
    fn from(variant: Srcmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRCMODE` reader - Source Addressing Mode"]
pub type SrcmodeR = crate::BitReader<Srcmode>;
impl SrcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srcmode {
        match self.bits {
            false => Srcmode::Absolute,
            true => Srcmode::Relative,
        }
    }
    #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the absolute address of the source data."]
    #[inline(always)]
    pub fn is_absolute(&self) -> bool {
        *self == Srcmode::Absolute
    }
    #[doc = "The SRCADDR field of LDMA_CHx_SRC contains the relative offset of the source data."]
    #[inline(always)]
    pub fn is_relative(&self) -> bool {
        *self == Srcmode::Relative
    }
}
#[doc = "Destination Addressing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dstmode {
    #[doc = "0: The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
    Absolute = 0,
    #[doc = "1: The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
    Relative = 1,
}
impl From<Dstmode> for bool {
    #[inline(always)]
    fn from(variant: Dstmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DSTMODE` reader - Destination Addressing Mode"]
pub type DstmodeR = crate::BitReader<Dstmode>;
impl DstmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dstmode {
        match self.bits {
            false => Dstmode::Absolute,
            true => Dstmode::Relative,
        }
    }
    #[doc = "The DSTADDR field of LDMA_CHx_DST contains the absolute address of the destination data."]
    #[inline(always)]
    pub fn is_absolute(&self) -> bool {
        *self == Dstmode::Absolute
    }
    #[doc = "The DSTADDR field of LDMA_CHx_DST contains the relative offset of the destination data."]
    #[inline(always)]
    pub fn is_relative(&self) -> bool {
        *self == Dstmode::Relative
    }
}
impl R {
    #[doc = "Bits 0:1 - DMA Structure Type"]
    #[inline(always)]
    pub fn structtype(&self) -> StructtypeR {
        StructtypeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Extend"]
    #[inline(always)]
    pub fn extend(&self) -> ExtendR {
        ExtendR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Structure DMA Transfer Request"]
    #[inline(always)]
    pub fn structreq(&self) -> StructreqR {
        StructreqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    pub fn xfercnt(&self) -> XfercntR {
        XfercntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    pub fn byteswap(&self) -> ByteswapR {
        ByteswapR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    pub fn blocksize(&self) -> BlocksizeR {
        BlocksizeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set"]
    #[inline(always)]
    pub fn doneien(&self) -> DoneienR {
        DoneienR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    pub fn reqmode(&self) -> ReqmodeR {
        ReqmodeR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    pub fn decloopcnt(&self) -> DecloopcntR {
        DecloopcntR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    pub fn ignoresreq(&self) -> IgnoresreqR {
        IgnoresreqR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    pub fn srcinc(&self) -> SrcincR {
        SrcincR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    pub fn size(&self) -> SizeR {
        SizeR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    pub fn dstinc(&self) -> DstincR {
        DstincR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - Source Addressing Mode"]
    #[inline(always)]
    pub fn srcmode(&self) -> SrcmodeR {
        SrcmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Destination Addressing Mode"]
    #[inline(always)]
    pub fn dstmode(&self) -> DstmodeR {
        DstmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - DMA Structure Type"]
    #[inline(always)]
    pub fn structtype(&mut self) -> StructtypeW<'_, Ch15CtrlSpec> {
        StructtypeW::new(self, 0)
    }
    #[doc = "Bit 2 - Extend"]
    #[inline(always)]
    pub fn extend(&mut self) -> ExtendW<'_, Ch15CtrlSpec> {
        ExtendW::new(self, 2)
    }
    #[doc = "Bits 4:14 - DMA Unit Data Transfer Count"]
    #[inline(always)]
    pub fn xfercnt(&mut self) -> XfercntW<'_, Ch15CtrlSpec> {
        XfercntW::new(self, 4)
    }
    #[doc = "Bit 15 - Endian Byte Swap"]
    #[inline(always)]
    pub fn byteswap(&mut self) -> ByteswapW<'_, Ch15CtrlSpec> {
        ByteswapW::new(self, 15)
    }
    #[doc = "Bits 16:19 - Block Transfer Size"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BlocksizeW<'_, Ch15CtrlSpec> {
        BlocksizeW::new(self, 16)
    }
    #[doc = "Bit 20 - DMA Operation Done Interrupt Flag Set"]
    #[inline(always)]
    pub fn doneien(&mut self) -> DoneienW<'_, Ch15CtrlSpec> {
        DoneienW::new(self, 20)
    }
    #[doc = "Bit 21 - DMA Request Transfer Mode Select"]
    #[inline(always)]
    pub fn reqmode(&mut self) -> ReqmodeW<'_, Ch15CtrlSpec> {
        ReqmodeW::new(self, 21)
    }
    #[doc = "Bit 22 - Decrement Loop Count"]
    #[inline(always)]
    pub fn decloopcnt(&mut self) -> DecloopcntW<'_, Ch15CtrlSpec> {
        DecloopcntW::new(self, 22)
    }
    #[doc = "Bit 23 - Ignore Sreq"]
    #[inline(always)]
    pub fn ignoresreq(&mut self) -> IgnoresreqW<'_, Ch15CtrlSpec> {
        IgnoresreqW::new(self, 23)
    }
    #[doc = "Bits 24:25 - Source Address Increment Size"]
    #[inline(always)]
    pub fn srcinc(&mut self) -> SrcincW<'_, Ch15CtrlSpec> {
        SrcincW::new(self, 24)
    }
    #[doc = "Bits 26:27 - Unit Data Transfer Size"]
    #[inline(always)]
    pub fn size(&mut self) -> SizeW<'_, Ch15CtrlSpec> {
        SizeW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Destination Address Increment Size"]
    #[inline(always)]
    pub fn dstinc(&mut self) -> DstincW<'_, Ch15CtrlSpec> {
        DstincW::new(self, 28)
    }
}
#[doc = "Channel Descriptor Control Word Register (Writes will only take effect when EN=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`ch15_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch15_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch15CtrlSpec;
impl crate::RegisterSpec for Ch15CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch15_ctrl::R`](R) reader structure"]
impl crate::Readable for Ch15CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ch15_ctrl::W`](W) writer structure"]
impl crate::Writable for Ch15CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH15_CTRL to value 0"]
impl crate::Resettable for Ch15CtrlSpec {}
