#![allow(
    dead_code,
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals
)]

#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl<T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self {
        __BindgenUnionField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T {
        ::std::mem::transmute(self)
    }
}
impl<T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}
impl<T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __BindgenUnionField<T> {}
impl<T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
impl<T> ::std::hash::Hash for __BindgenUnionField<T> {
    fn hash<H: ::std::hash::Hasher>(&self, _state: &mut H) {}
}
impl<T> ::std::cmp::PartialEq for __BindgenUnionField<T> {
    fn eq(&self, _other: &__BindgenUnionField<T>) -> bool {
        true
    }
}
impl<T> ::std::cmp::Eq for __BindgenUnionField<T> {}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo {
    pub a: __BindgenUnionField<::std::os::raw::c_uint>,
    pub __bindgen_anon_1: __BindgenUnionField<foo__bindgen_ty_1>,
    pub bindgen_union_field: u32,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1 {
    pub __bindgen_anon_1: foo__bindgen_ty_1__bindgen_ty_1,
    pub __bindgen_anon_2: foo__bindgen_ty_1__bindgen_ty_2,
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_1 {
    pub b1: __BindgenUnionField<::std::os::raw::c_ushort>,
    pub b2: __BindgenUnionField<::std::os::raw::c_ushort>,
    pub bindgen_union_field: u16,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_1))
    );
    fn test_field_b1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b1) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(b1)
            )
        );
    }
    test_field_b1();
    fn test_field_b2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_1,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).b2) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_1),
                "::",
                stringify!(b2)
            )
        );
    }
    test_field_b2();
}
impl Clone for foo__bindgen_ty_1__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Hash, PartialEq, Eq)]
pub struct foo__bindgen_ty_1__bindgen_ty_2 {
    pub c1: __BindgenUnionField<::std::os::raw::c_ushort>,
    pub c2: __BindgenUnionField<::std::os::raw::c_ushort>,
    pub bindgen_union_field: u16,
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1__bindgen_ty_2() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        2usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1__bindgen_ty_2>(),
        2usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1__bindgen_ty_2))
    );
    fn test_field_c1() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c1) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(c1)
            )
        );
    }
    test_field_c1();
    fn test_field_c2() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<
                    foo__bindgen_ty_1__bindgen_ty_2,
                >::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).c2) as usize - ptr as usize
            },
            0usize,
            concat!(
                "Offset of field: ",
                stringify!(foo__bindgen_ty_1__bindgen_ty_2),
                "::",
                stringify!(c2)
            )
        );
    }
    test_field_c2();
}
impl Clone for foo__bindgen_ty_1__bindgen_ty_2 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_foo__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<foo__bindgen_ty_1>(),
        4usize,
        concat!("Size of: ", stringify!(foo__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<foo__bindgen_ty_1>(),
        2usize,
        concat!("Alignment of ", stringify!(foo__bindgen_ty_1))
    );
}
impl Clone for foo__bindgen_ty_1 {
    fn clone(&self) -> Self {
        *self
    }
}
#[test]
fn bindgen_test_layout_foo() {
    assert_eq!(
        ::std::mem::size_of::<foo>(),
        4usize,
        concat!("Size of: ", stringify!(foo))
    );
    assert_eq!(
        ::std::mem::align_of::<foo>(),
        4usize,
        concat!("Alignment of ", stringify!(foo))
    );
    fn test_field_a() {
        assert_eq!(
            unsafe {
                let uninit = ::std::mem::MaybeUninit::<foo>::uninit();
                let ptr = uninit.as_ptr();
                ::std::ptr::addr_of!((*ptr).a) as usize - ptr as usize
            },
            0usize,
            concat!("Offset of field: ", stringify!(foo), "::", stringify!(a))
        );
    }
    test_field_a();
}
impl Clone for foo {
    fn clone(&self) -> Self {
        *self
    }
}
