#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct __BindgenBitfieldUnit<Storage> {
    storage: Storage,
}
impl<Storage> __BindgenBitfieldUnit<Storage> {
    #[inline]
    pub const fn new(storage: Storage) -> Self {
        Self { storage }
    }
}
impl<Storage> __BindgenBitfieldUnit<Storage>
where
    Storage: AsRef<[u8]> + AsMut<[u8]>,
{
    #[inline]
    pub fn get_bit(&self, index: usize) -> bool {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = self.storage.as_ref()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        byte & mask == mask
    }
    #[inline]
    pub fn set_bit(&mut self, index: usize, val: bool) {
        debug_assert!(index / 8 < self.storage.as_ref().len());
        let byte_index = index / 8;
        let byte = &mut self.storage.as_mut()[byte_index];
        let bit_index = if cfg!(target_endian = "big") {
            7 - (index % 8)
        } else {
            index % 8
        };
        let mask = 1 << bit_index;
        if val {
            *byte |= mask;
        } else {
            *byte &= !mask;
        }
    }
    #[inline]
    pub fn get(&self, bit_offset: usize, bit_width: u8) -> u64 {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        let mut val = 0;
        for i in 0..(bit_width as usize) {
            if self.get_bit(i + bit_offset) {
                let index = if cfg!(target_endian = "big") {
                    bit_width as usize - 1 - i
                } else {
                    i
                };
                val |= 1 << index;
            }
        }
        val
    }
    #[inline]
    pub fn set(&mut self, bit_offset: usize, bit_width: u8, val: u64) {
        debug_assert!(bit_width <= 64);
        debug_assert!(bit_offset / 8 < self.storage.as_ref().len());
        debug_assert!(
            (bit_offset + (bit_width as usize)) / 8 <=
                self.storage.as_ref().len()
        );
        for i in 0..(bit_width as usize) {
            let mask = 1 << i;
            let val_bit_is_set = val & mask == mask;
            let index = if cfg!(target_endian = "big") {
                bit_width as usize - 1 - i
            } else {
                i
            };
            self.set_bit(index + bit_offset, val_bit_is_set);
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct PubPriv {
    pub x: ::std::os::raw::c_int,
    y: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_PubPriv() {
    assert_eq!(
        ::std::mem::size_of::<PubPriv>(),
        8usize,
        concat!("Size of: ", stringify!(PubPriv))
    );
    assert_eq!(
        ::std::mem::align_of::<PubPriv>(),
        4usize,
        concat!("Alignment of ", stringify!(PubPriv))
    );
    fn test_field_x() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<PubPriv>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).x) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(PubPriv),
                "::",
                stringify!(x)
            )
        );
    }
    test_field_x();
    fn test_field_y() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<PubPriv>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).y) as usize - ptr as usize
            },
            4usize,
            concat!(
                "Offset of field: ",
                stringify!(PubPriv),
                "::",
                stringify!(y)
            )
        );
    }
    test_field_y();
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct PrivateBitFields {
    pub _bitfield_align_1: [u8; 0],
    _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_PrivateBitFields() {
    assert_eq!(
        ::std::mem::size_of::<PrivateBitFields>(),
        4usize,
        concat!("Size of: ", stringify!(PrivateBitFields))
    );
    assert_eq!(
        ::std::mem::align_of::<PrivateBitFields>(),
        4usize,
        concat!("Alignment of ", stringify!(PrivateBitFields))
    );
}
impl PrivateBitFields {
    #[inline]
    fn a(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32)
        }
    }
    #[inline]
    fn set_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn b(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32)
        }
    }
    #[inline]
    fn set_b(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn new_bitfield_1(
        a: ::std::os::raw::c_uint,
        b: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let a: u32 = unsafe { ::std::mem::transmute(a) };
            a as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let b: u32 = unsafe { ::std::mem::transmute(b) };
            b as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct PublicBitFields {
    pub _bitfield_align_1: [u8; 0],
    pub _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_PublicBitFields() {
    assert_eq!(
        ::std::mem::size_of::<PublicBitFields>(),
        4usize,
        concat!("Size of: ", stringify!(PublicBitFields))
    );
    assert_eq!(
        ::std::mem::align_of::<PublicBitFields>(),
        4usize,
        concat!("Alignment of ", stringify!(PublicBitFields))
    );
}
impl PublicBitFields {
    #[inline]
    pub fn a(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn b(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_b(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn new_bitfield_1(
        a: ::std::os::raw::c_uint,
        b: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let a: u32 = unsafe { ::std::mem::transmute(a) };
            a as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let b: u32 = unsafe { ::std::mem::transmute(b) };
            b as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[repr(align(4))]
#[derive(Debug, Default, Copy, Clone)]
pub struct MixedBitFields {
    pub _bitfield_align_1: [u8; 0],
    _bitfield_1: __BindgenBitfieldUnit<[u8; 1usize]>,
    pub __bindgen_padding_0: [u8; 3usize],
}
#[test]
fn bindgen_test_layout_MixedBitFields() {
    assert_eq!(
        ::std::mem::size_of::<MixedBitFields>(),
        4usize,
        concat!("Size of: ", stringify!(MixedBitFields))
    );
    assert_eq!(
        ::std::mem::align_of::<MixedBitFields>(),
        4usize,
        concat!("Alignment of ", stringify!(MixedBitFields))
    );
}
impl MixedBitFields {
    #[inline]
    fn a(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(0usize, 4u8) as u32)
        }
    }
    #[inline]
    fn set_a(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(0usize, 4u8, val as u64)
        }
    }
    #[inline]
    pub fn d(&self) -> ::std::os::raw::c_uint {
        unsafe {
            ::std::mem::transmute(self._bitfield_1.get(4usize, 4u8) as u32)
        }
    }
    #[inline]
    pub fn set_d(&mut self, val: ::std::os::raw::c_uint) {
        unsafe {
            let val: u32 = ::std::mem::transmute(val);
            self._bitfield_1.set(4usize, 4u8, val as u64)
        }
    }
    #[inline]
    fn new_bitfield_1(
        a: ::std::os::raw::c_uint,
        d: ::std::os::raw::c_uint,
    ) -> __BindgenBitfieldUnit<[u8; 1usize]> {
        let mut __bindgen_bitfield_unit: __BindgenBitfieldUnit<[u8; 1usize]> =
            Default::default();
        __bindgen_bitfield_unit.set(0usize, 4u8, {
            let a: u32 = unsafe { ::std::mem::transmute(a) };
            a as u64
        });
        __bindgen_bitfield_unit.set(4usize, 4u8, {
            let d: u32 = unsafe { ::std::mem::transmute(d) };
            d as u64
        });
        __bindgen_bitfield_unit
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct Base {
    pub member: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_Base() {
    assert_eq!(
        ::std::mem::size_of::<Base>(),
        4usize,
        concat!("Size of: ", stringify!(Base))
    );
    assert_eq!(
        ::std::mem::align_of::<Base>(),
        4usize,
        concat!("Alignment of ", stringify!(Base))
    );
    fn test_field_member() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<Base>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).member) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(Base),
                "::",
                stringify!(member)
            )
        );
    }
    test_field_member();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InheritsPrivately {
    _base: Base,
}
#[test]
fn bindgen_test_layout_InheritsPrivately() {
    assert_eq!(
        ::std::mem::size_of::<InheritsPrivately>(),
        4usize,
        concat!("Size of: ", stringify!(InheritsPrivately))
    );
    assert_eq!(
        ::std::mem::align_of::<InheritsPrivately>(),
        4usize,
        concat!("Alignment of ", stringify!(InheritsPrivately))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct InheritsPublically {
    pub _base: Base,
}
#[test]
fn bindgen_test_layout_InheritsPublically() {
    assert_eq!(
        ::std::mem::size_of::<InheritsPublically>(),
        4usize,
        concat!("Size of: ", stringify!(InheritsPublically))
    );
    assert_eq!(
        ::std::mem::align_of::<InheritsPublically>(),
        4usize,
        concat!("Alignment of ", stringify!(InheritsPublically))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithAnonStruct {
    __bindgen_anon_1: WithAnonStruct__bindgen_ty_1,
    pub __bindgen_anon_2: WithAnonStruct__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithAnonStruct__bindgen_ty_1 {
    pub a: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WithAnonStruct__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<WithAnonStruct__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(WithAnonStruct__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<WithAnonStruct__bindgen_ty_1>(),
        4usize,
        concat!("Alignment of ", stringify!(WithAnonStruct__bindgen_ty_1))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    WithAnonStruct__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithAnonStruct__bindgen_ty_1),
                "::",
                stringify!(a)
            )
        );
    }
    test_field_a();
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct WithAnonStruct__bindgen_ty_2 {
    pub b: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_WithAnonStruct__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<WithAnonStruct__bindgen_ty_2>(),
        4usize,
        concat!("Size of: ", stringify!(WithAnonStruct__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<WithAnonStruct__bindgen_ty_2>(),
        4usize,
        concat!("Alignment of ", stringify!(WithAnonStruct__bindgen_ty_2))
    );
    fn test_field_b() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    WithAnonStruct__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(WithAnonStruct__bindgen_ty_2),
                "::",
                stringify!(b)
            )
        );
    }
    test_field_b();
}
#[test]
fn bindgen_test_layout_WithAnonStruct() {
    assert_eq!(
        ::std::mem::size_of::<WithAnonStruct>(),
        8usize,
        concat!("Size of: ", stringify!(WithAnonStruct))
    );
    assert_eq!(
        ::std::mem::align_of::<WithAnonStruct>(),
        4usize,
        concat!("Alignment of ", stringify!(WithAnonStruct))
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct WithAnonUnion {
    __bindgen_anon_1: WithAnonUnion__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union WithAnonUnion__bindgen_ty_1 {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_WithAnonUnion__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<WithAnonUnion__bindgen_ty_1>(),
        1usize,
        concat!("Size of: ", stringify!(WithAnonUnion__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<WithAnonUnion__bindgen_ty_1>(),
        1usize,
        concat!("Alignment of ", stringify!(WithAnonUnion__bindgen_ty_1))
    );
}
impl Default for WithAnonUnion__bindgen_ty_1 {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[test]
fn bindgen_test_layout_WithAnonUnion() {
    assert_eq!(
        ::std::mem::size_of::<WithAnonUnion>(),
        1usize,
        concat!("Size of: ", stringify!(WithAnonUnion))
    );
    assert_eq!(
        ::std::mem::align_of::<WithAnonUnion>(),
        1usize,
        concat!("Alignment of ", stringify!(WithAnonUnion))
    );
}
impl Default for WithAnonUnion {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
