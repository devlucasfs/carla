use std::ffi::{CStr, CString, c_char, c_void};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaValueTag {
    String,
    Number,
    Bool,
    Map,
    List,
    Nil,
}

#[repr(C)]
pub struct EvaParser {
    pub status: usize,
    pub parser: *mut c_void,
}

#[repr(C)]
pub union EvaValueData {
    pub ptr: *mut c_void,
    pub number: f64,
    pub boolean: bool,
}

#[repr(C)]
pub struct EvaValueFFI {
    pub tag: EvaValueTag,
    pub data: EvaValueData,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EvaMap {
    ptr: *mut c_void,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct EvaList {
    ptr: *mut c_void,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum EvaValue {
    String(String),
    Number(f64),
    Boolean(bool),

    Map(EvaMap),
    List(EvaList),

    #[default]
    Nil,
}

unsafe extern "C" {
    pub fn eva_make_parser(path: *const c_char) -> *mut EvaParser;
    pub fn eva_dump_pointer(value: EvaValueFFI);

    pub fn eva_get_value_from_namespace(
        parser: *mut EvaParser,
        ns: *const c_char,
        name: *const c_char,
    ) -> EvaValueFFI;

    pub fn eva_check_exist_field_in_namespace(
        parser: *mut EvaParser,
        ns: *const c_char,
        name: *const c_char,
    ) -> c_char;

    pub fn eva_get_list_length(list: EvaValueFFI) -> i32;
    pub fn eva_get_list_field(list: EvaValueFFI, index: i32) -> EvaValueFFI;
    pub fn eva_get_map_length(map: EvaValueFFI) -> i32;
    pub fn eva_get_map_field(map: EvaValueFFI, key: *const c_char) -> EvaValueFFI;
    pub fn eva_get_all_keys_from_map(map: EvaValueFFI) -> EvaValueFFI;
    pub fn eva_check_exist_field_in_map(map: EvaValueFFI, field: *const c_char) -> bool;
}

pub struct Eva {
    parser: *mut EvaParser,
}

impl Eva {
    pub fn open(path: &str) -> Self {
        let path = CString::new(path).unwrap();

        let parser = unsafe { eva_make_parser(path.as_ptr()) };

        Self { parser }
    }

    pub(crate) fn convert_ffi(ffi: EvaValueFFI) -> EvaValue {
        unsafe {
            match ffi.tag {
                EvaValueTag::Bool => EvaValue::Boolean(ffi.data.boolean),
                EvaValueTag::Number => EvaValue::Number(ffi.data.number),
                EvaValueTag::List => EvaValue::List(EvaList { ptr: ffi.data.ptr }),
                EvaValueTag::Map => EvaValue::Map(EvaMap { ptr: ffi.data.ptr }),
                EvaValueTag::String => EvaValue::String(
                    CStr::from_ptr(ffi.data.ptr as *mut c_char)
                        .to_str()
                        .ok()
                        .map(|s| s.to_owned())
                        .unwrap(),
                ),
                EvaValueTag::Nil => EvaValue::Nil,
            }
        }
    }

    pub fn get(&self, namespace: impl ToString, name: impl ToString) -> EvaValue {
        let namespace = CString::new(namespace.to_string()).unwrap();
        let name = CString::new(name.to_string()).unwrap();

        unsafe {
            return Eva::convert_ffi(eva_get_value_from_namespace(
                self.parser,
                namespace.as_ptr(),
                name.as_ptr(),
            ));
        }
    }
}

macro_rules! impl_eva_collection {
    (
        $type:ident,
        $tag:ident,
        $len_fn:ident,
        $get_fn:ident,
        $index_type:ty,
        $convert:expr
    ) => {
        impl $type {
            fn make_it(&self) -> EvaValueFFI {
                EvaValueFFI {
                    tag: EvaValueTag::$tag,
                    data: EvaValueData { ptr: self.ptr },
                }
            }

            pub fn len(&self) -> i32 {
                unsafe { $len_fn(self.make_it()) }
            }

            pub fn get<T>(&self, index: T) -> EvaValue
            where
                T: Into<$index_type>,
            {
                unsafe { Eva::convert_ffi($get_fn(self.make_it(), ($convert)(index.into()))) }
            }
        }
    };
}

impl_eva_collection!(
    EvaList,
    List,
    eva_get_list_length,
    eva_get_list_field,
    i32,
    |index| index
);

impl_eva_collection!(
    EvaMap,
    Map,
    eva_get_map_length,
    eva_get_map_field,
    String,
    |field| CString::new(field).unwrap().into_raw()
);
