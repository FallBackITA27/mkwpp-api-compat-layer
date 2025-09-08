#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
pub mod common_data_traits {
    use crate::common_types::NoData;
    /// Marker Type
    pub trait DataTraits: GetId + GetSessionToken {}
    impl<T> DataTraits for T
    where
        T: GetId + GetSessionToken,
    {}
    pub trait HasId {
        fn get_id(&self) -> i32;
    }
    pub trait GetId {
        const HAS_ID: bool = false;
        fn get_id(&self) -> i32 {
            Default::default()
        }
    }
    impl<T> GetId for T
    where
        T: HasId,
    {
        const HAS_ID: bool = true;
        fn get_id(&self) -> i32 {
            HasId::get_id(self)
        }
    }
    pub trait HasSessionToken {
        fn get_token(&self) -> &str;
    }
    pub trait GetSessionToken {
        const HAS_SESSION_TOKEN: bool = false;
        fn get_token(&self) -> &str {
            Default::default()
        }
    }
    impl<T> GetSessionToken for T
    where
        T: HasSessionToken,
    {
        const HAS_SESSION_TOKEN: bool = true;
        fn get_token(&self) -> &str {
            HasSessionToken::get_token(self)
        }
    }
    impl GetId for NoData {}
    impl GetSessionToken for NoData {}
}
pub mod common_types {
    pub enum Category {
        Normal,
        Shortcut,
        Unrestricted,
    }
    #[wasm_bindgen()]
    pub struct NoData;
    #[automatically_derived]
    impl wasm_bindgen::__rt::marker::SupportsConstructor for NoData {}
    #[automatically_derived]
    impl wasm_bindgen::__rt::marker::SupportsInstanceProperty for NoData {}
    #[automatically_derived]
    impl wasm_bindgen::__rt::marker::SupportsStaticProperty for NoData {}
    #[automatically_derived]
    impl wasm_bindgen::describe::WasmDescribe for NoData {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(RUST_STRUCT);
            inform(6u32);
            inform(78u32);
            inform(111u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::IntoWasmAbi for NoData {
        type Abi = u32;
        fn into_abi(self) -> u32 {
            use wasm_bindgen::__rt::alloc::rc::Rc;
            use wasm_bindgen::__rt::WasmRefCell;
            Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::FromWasmAbi for NoData {
        type Abi = u32;
        unsafe fn from_abi(js: u32) -> Self {
            use wasm_bindgen::__rt::alloc::rc::Rc;
            use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
            use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
            let ptr = js as *mut WasmRefCell<NoData>;
            assert_not_null(ptr);
            let rc = Rc::from_raw(ptr);
            match Rc::try_unwrap(rc) {
                Ok(cell) => cell.into_inner(),
                Err(_) => {
                    wasm_bindgen::throw_str(
                        "attempted to take ownership of Rust value while it was borrowed",
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::core::convert::From<NoData> for wasm_bindgen::JsValue {
        fn from(value: NoData) -> Self {
            let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
            unsafe fn __wbg_nodata_new(_: u32) -> u32 {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "cannot convert to JsValue outside of the Wasm target",
                        ),
                    );
                }
            }
            unsafe {
                <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                    __wbg_nodata_new(ptr),
                )
            }
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::RefFromWasmAbi for NoData {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::RcRef<NoData>;
        unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
            use wasm_bindgen::__rt::alloc::rc::Rc;
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<NoData>;
            wasm_bindgen::__rt::assert_not_null(js);
            Rc::increment_strong_count(js);
            let rc = Rc::from_raw(js);
            wasm_bindgen::__rt::RcRef::new(rc)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::RefMutFromWasmAbi for NoData {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::RcRefMut<NoData>;
        unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
            use wasm_bindgen::__rt::alloc::rc::Rc;
            let js = js as *mut wasm_bindgen::__rt::WasmRefCell<NoData>;
            wasm_bindgen::__rt::assert_not_null(js);
            Rc::increment_strong_count(js);
            let rc = Rc::from_raw(js);
            wasm_bindgen::__rt::RcRefMut::new(rc)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::LongRefFromWasmAbi for NoData {
        type Abi = u32;
        type Anchor = wasm_bindgen::__rt::RcRef<NoData>;
        unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
            <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionIntoWasmAbi for NoData {
        #[inline]
        fn none() -> Self::Abi {
            0
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionFromWasmAbi for NoData {
        #[inline]
        fn is_none(abi: &Self::Abi) -> bool {
            *abi == 0
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::TryFromJsValue for NoData {
        type Error = wasm_bindgen::JsValue;
        fn try_from_js_value(
            value: wasm_bindgen::JsValue,
        ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
            let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
            unsafe fn __wbg_nodata_unwrap(_: u32) -> u32 {
                {
                    ::core::panicking::panic_fmt(
                        format_args!(
                            "cannot convert from JsValue outside of the Wasm target",
                        ),
                    );
                }
            }
            let ptr = unsafe { __wbg_nodata_unwrap(idx) };
            if ptr == 0 {
                wasm_bindgen::__rt::core::result::Result::Err(value)
            } else {
                #[allow(clippy::mem_forget)]
                wasm_bindgen::__rt::core::mem::forget(value);
                unsafe {
                    wasm_bindgen::__rt::core::result::Result::Ok(
                        <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::describe::WasmDescribeVector for NoData {
        fn describe_vector() {
            use wasm_bindgen::describe::*;
            inform(VECTOR);
            inform(NAMED_EXTERNREF);
            inform(6u32);
            inform(78u32);
            inform(111u32);
            inform(68u32);
            inform(97u32);
            inform(116u32);
            inform(97u32);
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::VectorIntoWasmAbi for NoData {
        type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
        fn vector_into_abi(
            vector: wasm_bindgen::__rt::alloc::boxed::Box<[NoData]>,
        ) -> Self::Abi {
            wasm_bindgen::convert::js_value_vector_into_abi(vector)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::VectorFromWasmAbi for NoData {
        type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::FromWasmAbi>::Abi;
        unsafe fn vector_from_abi(
            js: Self::Abi,
        ) -> wasm_bindgen::__rt::alloc::boxed::Box<[NoData]> {
            wasm_bindgen::convert::js_value_vector_from_abi(js)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::VectorIntoJsValue for NoData {
        fn vector_into_jsvalue(
            vector: wasm_bindgen::__rt::alloc::boxed::Box<[NoData]>,
        ) -> wasm_bindgen::JsValue {
            wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
        }
    }
}
pub mod compatibility_layer {
    pub mod typescript_wasm {
        use std::collections::HashMap;
        use futures::TryFutureExt;
        use js_sys::Array;
        use wasm_bindgen::{JsValue, convert::{FromWasmAbi, IntoWasmAbi, WasmAbi}};
        use wasm_bindgen_futures::JsFuture;
        use web_sys::RequestInit;
        use crate::{
            common_data_traits::{GetId, GetSessionToken},
            common_types::NoData, endpoint::{cups::GetCups, Endpoint},
            error::{ErrorCodes, FinalErrorResponse},
            new_final_error, request_method::RequestMethod,
        };
        impl GetCups {
            pub async fn fetch(
                &self,
                inputs: <Self as Endpoint>::InputStruct,
            ) -> Result<<Self as Endpoint>::OutputStruct, JsValue> {
                #[automatically_derived]
                const _: () = {
                    pub unsafe extern "C" fn __wasm_bindgen_generated_GetCups_fetch(
                        me: u32,
                        arg1_1: <<<GetCups as Endpoint>::InputStruct as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim1,
                        arg1_2: <<<GetCups as Endpoint>::InputStruct as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim2,
                        arg1_3: <<<GetCups as Endpoint>::InputStruct as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim3,
                        arg1_4: <<<GetCups as Endpoint>::InputStruct as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::Prim4,
                    ) -> wasm_bindgen::convert::WasmRet<
                        <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::Abi,
                    > {
                        const _: () = {};
                        let _ret = wasm_bindgen_futures::future_to_promise(async move {
                                {
                                    let me = unsafe {
                                        <GetCups as wasm_bindgen::convert::LongRefFromWasmAbi>::long_ref_from_abi(
                                            me,
                                        )
                                    };
                                    let me = <<GetCups as wasm_bindgen::convert::LongRefFromWasmAbi>::Anchor as wasm_bindgen::__rt::core::borrow::Borrow<
                                        GetCups,
                                    >>::borrow(&me);
                                    let arg1 = unsafe {
                                        <<GetCups as Endpoint>::InputStruct as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                                            <<<GetCups as Endpoint>::InputStruct as wasm_bindgen::convert::FromWasmAbi>::Abi as wasm_bindgen::convert::WasmAbi>::join(
                                                arg1_1,
                                                arg1_2,
                                                arg1_3,
                                                arg1_4,
                                            ),
                                        )
                                    };
                                    let _ret = me.fetch(arg1);
                                    <Result<
                                        <GetCups as Endpoint>::OutputStruct,
                                        JsValue,
                                    > as wasm_bindgen::__rt::IntoJsResult>::into_js_result(
                                        _ret.await,
                                    )
                                }
                            })
                            .into();
                        <wasm_bindgen::JsValue as wasm_bindgen::convert::ReturnWasmAbi>::return_abi(
                                _ret,
                            )
                            .into()
                    }
                };
                let request = RequestInit::new();
                request.set_method(Self::REQUEST_METHOD.to_str());
                request.set_mode(web_sys::RequestMode::Cors);
                let headers = web_sys::Headers::new().unwrap();
                if <Self as Endpoint>::InputStruct::HAS_ID {
                    headers.append("Bearer-Token", inputs.get_token());
                }
                if Self::REQUEST_METHOD != RequestMethod::Get {
                    headers.append("Content-Type", "application/json");
                }
                request.set_headers(&headers);
                JsFuture::from(
                        web_sys::window()
                            .expect("Couldn't access window")
                            .fetch_with_str_and_init(
                                Self::construct_full_path().as_str(),
                                &request,
                            ),
                    )
                    .map_ok(|ok_val| {
                        serde_wasm_bindgen::from_value::<
                            <Self as Endpoint>::OutputStruct,
                        >(ok_val)
                            .map_err(|e| {
                                serde_wasm_bindgen::to_value(
                                        &ErrorCodes::into_final_error(
                                            ErrorCodes::SerializingDataToJSON,
                                            Some(e),
                                            "mkwpp-api-compat-layer-core/src/compatibility_layer/typescript_wasm/mod.rs",
                                            46u32,
                                        ),
                                    )
                                    .expect("This should never be able to fail")
                            })
                    })
                    .await
                    .flatten()
            }
        }
    }
}
pub mod endpoint {
    use crate::{
        common_data_traits::DataTraits, request_method::RequestMethod,
        required_permission::RequiredPermission,
    };
    pub mod cups {
        use std::ffi::FromBytesUntilNulError;
        use crate::{
            common_data_traits::{GetId, HasId},
            common_types::NoData, endpoint::{Endpoint, RequiredPermission, Root, Scope},
            request_method::RequestMethod,
        };
        pub struct CupsScope;
        impl Scope for CupsScope {
            const PATH: &'static str = "/cups";
            type OuterScope = Root;
        }
        #[wasm_bindgen()]
        pub struct GetCups;
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsConstructor for GetCups {}
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsInstanceProperty for GetCups {}
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsStaticProperty for GetCups {}
        #[automatically_derived]
        impl wasm_bindgen::describe::WasmDescribe for GetCups {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(RUST_STRUCT);
                inform(7u32);
                inform(71u32);
                inform(101u32);
                inform(116u32);
                inform(67u32);
                inform(117u32);
                inform(112u32);
                inform(115u32);
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::IntoWasmAbi for GetCups {
            type Abi = u32;
            fn into_abi(self) -> u32 {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                use wasm_bindgen::__rt::WasmRefCell;
                Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::FromWasmAbi for GetCups {
            type Abi = u32;
            unsafe fn from_abi(js: u32) -> Self {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
                use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
                let ptr = js as *mut WasmRefCell<GetCups>;
                assert_not_null(ptr);
                let rc = Rc::from_raw(ptr);
                match Rc::try_unwrap(rc) {
                    Ok(cell) => cell.into_inner(),
                    Err(_) => {
                        wasm_bindgen::throw_str(
                            "attempted to take ownership of Rust value while it was borrowed",
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::__rt::core::convert::From<GetCups> for wasm_bindgen::JsValue {
            fn from(value: GetCups) -> Self {
                let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
                unsafe fn __wbg_getcups_new(_: u32) -> u32 {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "cannot convert to JsValue outside of the Wasm target",
                            ),
                        );
                    }
                }
                unsafe {
                    <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        __wbg_getcups_new(ptr),
                    )
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::RefFromWasmAbi for GetCups {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRef<GetCups>;
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<GetCups>;
                wasm_bindgen::__rt::assert_not_null(js);
                Rc::increment_strong_count(js);
                let rc = Rc::from_raw(js);
                wasm_bindgen::__rt::RcRef::new(rc)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::RefMutFromWasmAbi for GetCups {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRefMut<GetCups>;
            unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<GetCups>;
                wasm_bindgen::__rt::assert_not_null(js);
                Rc::increment_strong_count(js);
                let rc = Rc::from_raw(js);
                wasm_bindgen::__rt::RcRefMut::new(rc)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::LongRefFromWasmAbi for GetCups {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRef<GetCups>;
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::OptionIntoWasmAbi for GetCups {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::OptionFromWasmAbi for GetCups {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::TryFromJsValue for GetCups {
            type Error = wasm_bindgen::JsValue;
            fn try_from_js_value(
                value: wasm_bindgen::JsValue,
            ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
                let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
                unsafe fn __wbg_getcups_unwrap(_: u32) -> u32 {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "cannot convert from JsValue outside of the Wasm target",
                            ),
                        );
                    }
                }
                let ptr = unsafe { __wbg_getcups_unwrap(idx) };
                if ptr == 0 {
                    wasm_bindgen::__rt::core::result::Result::Err(value)
                } else {
                    #[allow(clippy::mem_forget)]
                    wasm_bindgen::__rt::core::mem::forget(value);
                    unsafe {
                        wasm_bindgen::__rt::core::result::Result::Ok(
                            <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::describe::WasmDescribeVector for GetCups {
            fn describe_vector() {
                use wasm_bindgen::describe::*;
                inform(VECTOR);
                inform(NAMED_EXTERNREF);
                inform(7u32);
                inform(71u32);
                inform(101u32);
                inform(116u32);
                inform(67u32);
                inform(117u32);
                inform(112u32);
                inform(115u32);
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::VectorIntoWasmAbi for GetCups {
            type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
                [wasm_bindgen::JsValue],
            > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
            fn vector_into_abi(
                vector: wasm_bindgen::__rt::alloc::boxed::Box<[GetCups]>,
            ) -> Self::Abi {
                wasm_bindgen::convert::js_value_vector_into_abi(vector)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::VectorFromWasmAbi for GetCups {
            type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
                [wasm_bindgen::JsValue],
            > as wasm_bindgen::convert::FromWasmAbi>::Abi;
            unsafe fn vector_from_abi(
                js: Self::Abi,
            ) -> wasm_bindgen::__rt::alloc::boxed::Box<[GetCups]> {
                wasm_bindgen::convert::js_value_vector_from_abi(js)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::__rt::VectorIntoJsValue for GetCups {
            fn vector_into_jsvalue(
                vector: wasm_bindgen::__rt::alloc::boxed::Box<[GetCups]>,
            ) -> wasm_bindgen::JsValue {
                wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for GetCups {
            #[inline]
            fn default() -> GetCups {
                GetCups {}
            }
        }
        impl Endpoint for GetCups {
            const PATH: &'static str = "/get";
            const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
            const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;
            type InputStruct = NoData;
            type OutputStruct = Vec<GetCupsOutput>;
            type ScopeStruct = CupsScope;
        }
        #[wasm_bindgen()]
        pub struct GetCupsOutput {
            pub id: i32,
            pub code: &'static str,
            pub track_ids: CupSlots,
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl _serde::Deserialize<'static> for GetCupsOutput {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'static>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "id" => _serde::__private::Ok(__Field::__field0),
                                "code" => _serde::__private::Ok(__Field::__field1),
                                "track_ids" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"id" => _serde::__private::Ok(__Field::__field0),
                                b"code" => _serde::__private::Ok(__Field::__field1),
                                b"track_ids" => _serde::__private::Ok(__Field::__field2),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor {
                        marker: _serde::__private::PhantomData<GetCupsOutput>,
                        lifetime: _serde::__private::PhantomData<&'static ()>,
                    }
                    #[automatically_derived]
                    impl _serde::de::Visitor<'static> for __Visitor {
                        type Value = GetCupsOutput;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct GetCupsOutput",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'static>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                i32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct GetCupsOutput with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                &'static str,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct GetCupsOutput with 3 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                CupSlots,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct GetCupsOutput with 3 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(GetCupsOutput {
                                id: __field0,
                                code: __field1,
                                track_ids: __field2,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'static>,
                        {
                            let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<&'static str> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<CupSlots> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field("code"),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<
                                                &'static str,
                                            >(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "track_ids",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<CupSlots>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("id")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("code")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("track_ids")?
                                }
                            };
                            _serde::__private::Ok(GetCupsOutput {
                                id: __field0,
                                code: __field1,
                                track_ids: __field2,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &["id", "code", "track_ids"];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "GetCupsOutput",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<GetCupsOutput>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsConstructor for GetCupsOutput {}
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsInstanceProperty for GetCupsOutput {}
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsStaticProperty for GetCupsOutput {}
        #[automatically_derived]
        impl wasm_bindgen::describe::WasmDescribe for GetCupsOutput {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(RUST_STRUCT);
                inform(13u32);
                inform(71u32);
                inform(101u32);
                inform(116u32);
                inform(67u32);
                inform(117u32);
                inform(112u32);
                inform(115u32);
                inform(79u32);
                inform(117u32);
                inform(116u32);
                inform(112u32);
                inform(117u32);
                inform(116u32);
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::IntoWasmAbi for GetCupsOutput {
            type Abi = u32;
            fn into_abi(self) -> u32 {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                use wasm_bindgen::__rt::WasmRefCell;
                Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::FromWasmAbi for GetCupsOutput {
            type Abi = u32;
            unsafe fn from_abi(js: u32) -> Self {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
                use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
                let ptr = js as *mut WasmRefCell<GetCupsOutput>;
                assert_not_null(ptr);
                let rc = Rc::from_raw(ptr);
                match Rc::try_unwrap(rc) {
                    Ok(cell) => cell.into_inner(),
                    Err(_) => {
                        wasm_bindgen::throw_str(
                            "attempted to take ownership of Rust value while it was borrowed",
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::__rt::core::convert::From<GetCupsOutput>
        for wasm_bindgen::JsValue {
            fn from(value: GetCupsOutput) -> Self {
                let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
                unsafe fn __wbg_getcupsoutput_new(_: u32) -> u32 {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "cannot convert to JsValue outside of the Wasm target",
                            ),
                        );
                    }
                }
                unsafe {
                    <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        __wbg_getcupsoutput_new(ptr),
                    )
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::RefFromWasmAbi for GetCupsOutput {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRef<GetCupsOutput>;
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<GetCupsOutput>;
                wasm_bindgen::__rt::assert_not_null(js);
                Rc::increment_strong_count(js);
                let rc = Rc::from_raw(js);
                wasm_bindgen::__rt::RcRef::new(rc)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::RefMutFromWasmAbi for GetCupsOutput {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRefMut<GetCupsOutput>;
            unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<GetCupsOutput>;
                wasm_bindgen::__rt::assert_not_null(js);
                Rc::increment_strong_count(js);
                let rc = Rc::from_raw(js);
                wasm_bindgen::__rt::RcRefMut::new(rc)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::LongRefFromWasmAbi for GetCupsOutput {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRef<GetCupsOutput>;
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::OptionIntoWasmAbi for GetCupsOutput {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::OptionFromWasmAbi for GetCupsOutput {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::TryFromJsValue for GetCupsOutput {
            type Error = wasm_bindgen::JsValue;
            fn try_from_js_value(
                value: wasm_bindgen::JsValue,
            ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
                let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
                unsafe fn __wbg_getcupsoutput_unwrap(_: u32) -> u32 {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "cannot convert from JsValue outside of the Wasm target",
                            ),
                        );
                    }
                }
                let ptr = unsafe { __wbg_getcupsoutput_unwrap(idx) };
                if ptr == 0 {
                    wasm_bindgen::__rt::core::result::Result::Err(value)
                } else {
                    #[allow(clippy::mem_forget)]
                    wasm_bindgen::__rt::core::mem::forget(value);
                    unsafe {
                        wasm_bindgen::__rt::core::result::Result::Ok(
                            <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::describe::WasmDescribeVector for GetCupsOutput {
            fn describe_vector() {
                use wasm_bindgen::describe::*;
                inform(VECTOR);
                inform(NAMED_EXTERNREF);
                inform(13u32);
                inform(71u32);
                inform(101u32);
                inform(116u32);
                inform(67u32);
                inform(117u32);
                inform(112u32);
                inform(115u32);
                inform(79u32);
                inform(117u32);
                inform(116u32);
                inform(112u32);
                inform(117u32);
                inform(116u32);
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::VectorIntoWasmAbi for GetCupsOutput {
            type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
                [wasm_bindgen::JsValue],
            > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
            fn vector_into_abi(
                vector: wasm_bindgen::__rt::alloc::boxed::Box<[GetCupsOutput]>,
            ) -> Self::Abi {
                wasm_bindgen::convert::js_value_vector_into_abi(vector)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::VectorFromWasmAbi for GetCupsOutput {
            type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
                [wasm_bindgen::JsValue],
            > as wasm_bindgen::convert::FromWasmAbi>::Abi;
            unsafe fn vector_from_abi(
                js: Self::Abi,
            ) -> wasm_bindgen::__rt::alloc::boxed::Box<[GetCupsOutput]> {
                wasm_bindgen::convert::js_value_vector_from_abi(js)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::__rt::VectorIntoJsValue for GetCupsOutput {
            fn vector_into_jsvalue(
                vector: wasm_bindgen::__rt::alloc::boxed::Box<[GetCupsOutput]>,
            ) -> wasm_bindgen::JsValue {
                wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
            }
        }
        #[automatically_derived]
        const _: () = {
            #[doc(hidden)]
            pub unsafe extern "C" fn __wbg_get_getcupsoutput_id(
                js: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <i32 as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            > {
                use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
                use wasm_bindgen::convert::IntoWasmAbi;
                fn assert_copy<T: Copy>() {}
                assert_copy::<i32>();
                let js = js as *mut WasmRefCell<GetCupsOutput>;
                assert_not_null(js);
                let val = (*js).borrow().id;
                <i32 as IntoWasmAbi>::into_abi(val).into()
            }
        };
        #[automatically_derived]
        const _: () = {
            #[doc(hidden)]
            pub unsafe extern "C" fn __wbg_get_getcupsoutput_code(
                js: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <&'static str as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            > {
                use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
                use wasm_bindgen::convert::IntoWasmAbi;
                fn assert_copy<T: Copy>() {}
                assert_copy::<&'static str>();
                let js = js as *mut WasmRefCell<GetCupsOutput>;
                assert_not_null(js);
                let val = (*js).borrow().code;
                <&'static str as IntoWasmAbi>::into_abi(val).into()
            }
        };
        #[automatically_derived]
        const _: () = {
            #[doc(hidden)]
            pub unsafe extern "C" fn __wbg_get_getcupsoutput_track_ids(
                js: u32,
            ) -> wasm_bindgen::convert::WasmRet<
                <CupSlots as wasm_bindgen::convert::IntoWasmAbi>::Abi,
            > {
                use wasm_bindgen::__rt::{WasmRefCell, assert_not_null};
                use wasm_bindgen::convert::IntoWasmAbi;
                fn assert_copy<T: Copy>() {}
                assert_copy::<CupSlots>();
                let js = js as *mut WasmRefCell<GetCupsOutput>;
                assert_not_null(js);
                let val = (*js).borrow().track_ids;
                <CupSlots as IntoWasmAbi>::into_abi(val).into()
            }
        };
        #[wasm_bindgen()]
        pub struct CupSlots {
            track_id_slot_1: i32,
            track_id_slot_2: i32,
            track_id_slot_3: i32,
            track_id_slot_4: i32,
        }
        #[doc(hidden)]
        #[allow(
            non_upper_case_globals,
            unused_attributes,
            unused_qualifications,
            clippy::absolute_paths,
        )]
        const _: () = {
            #[allow(unused_extern_crates, clippy::useless_attribute)]
            extern crate serde as _serde;
            #[automatically_derived]
            impl<'de> _serde::Deserialize<'de> for CupSlots {
                fn deserialize<__D>(
                    __deserializer: __D,
                ) -> _serde::__private::Result<Self, __D::Error>
                where
                    __D: _serde::Deserializer<'de>,
                {
                    #[allow(non_camel_case_types)]
                    #[doc(hidden)]
                    enum __Field {
                        __field0,
                        __field1,
                        __field2,
                        __field3,
                        __ignore,
                    }
                    #[doc(hidden)]
                    struct __FieldVisitor;
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                        type Value = __Field;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "field identifier",
                            )
                        }
                        fn visit_u64<__E>(
                            self,
                            __value: u64,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                0u64 => _serde::__private::Ok(__Field::__field0),
                                1u64 => _serde::__private::Ok(__Field::__field1),
                                2u64 => _serde::__private::Ok(__Field::__field2),
                                3u64 => _serde::__private::Ok(__Field::__field3),
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<__E>(
                            self,
                            __value: &str,
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                "track_id_slot_1" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                "track_id_slot_2" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                "track_id_slot_3" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                "track_id_slot_4" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<__E>(
                            self,
                            __value: &[u8],
                        ) -> _serde::__private::Result<Self::Value, __E>
                        where
                            __E: _serde::de::Error,
                        {
                            match __value {
                                b"track_id_slot_1" => {
                                    _serde::__private::Ok(__Field::__field0)
                                }
                                b"track_id_slot_2" => {
                                    _serde::__private::Ok(__Field::__field1)
                                }
                                b"track_id_slot_3" => {
                                    _serde::__private::Ok(__Field::__field2)
                                }
                                b"track_id_slot_4" => {
                                    _serde::__private::Ok(__Field::__field3)
                                }
                                _ => _serde::__private::Ok(__Field::__ignore),
                            }
                        }
                    }
                    #[automatically_derived]
                    impl<'de> _serde::Deserialize<'de> for __Field {
                        #[inline]
                        fn deserialize<__D>(
                            __deserializer: __D,
                        ) -> _serde::__private::Result<Self, __D::Error>
                        where
                            __D: _serde::Deserializer<'de>,
                        {
                            _serde::Deserializer::deserialize_identifier(
                                __deserializer,
                                __FieldVisitor,
                            )
                        }
                    }
                    #[doc(hidden)]
                    struct __Visitor<'de> {
                        marker: _serde::__private::PhantomData<CupSlots>,
                        lifetime: _serde::__private::PhantomData<&'de ()>,
                    }
                    #[automatically_derived]
                    impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                        type Value = CupSlots;
                        fn expecting(
                            &self,
                            __formatter: &mut _serde::__private::Formatter,
                        ) -> _serde::__private::fmt::Result {
                            _serde::__private::Formatter::write_str(
                                __formatter,
                                "struct CupSlots",
                            )
                        }
                        #[inline]
                        fn visit_seq<__A>(
                            self,
                            mut __seq: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::SeqAccess<'de>,
                        {
                            let __field0 = match _serde::de::SeqAccess::next_element::<
                                i32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            0usize,
                                            &"struct CupSlots with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field1 = match _serde::de::SeqAccess::next_element::<
                                i32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            1usize,
                                            &"struct CupSlots with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field2 = match _serde::de::SeqAccess::next_element::<
                                i32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            2usize,
                                            &"struct CupSlots with 4 elements",
                                        ),
                                    );
                                }
                            };
                            let __field3 = match _serde::de::SeqAccess::next_element::<
                                i32,
                            >(&mut __seq)? {
                                _serde::__private::Some(__value) => __value,
                                _serde::__private::None => {
                                    return _serde::__private::Err(
                                        _serde::de::Error::invalid_length(
                                            3usize,
                                            &"struct CupSlots with 4 elements",
                                        ),
                                    );
                                }
                            };
                            _serde::__private::Ok(CupSlots {
                                track_id_slot_1: __field0,
                                track_id_slot_2: __field1,
                                track_id_slot_3: __field2,
                                track_id_slot_4: __field3,
                            })
                        }
                        #[inline]
                        fn visit_map<__A>(
                            self,
                            mut __map: __A,
                        ) -> _serde::__private::Result<Self::Value, __A::Error>
                        where
                            __A: _serde::de::MapAccess<'de>,
                        {
                            let mut __field0: _serde::__private::Option<i32> = _serde::__private::None;
                            let mut __field1: _serde::__private::Option<i32> = _serde::__private::None;
                            let mut __field2: _serde::__private::Option<i32> = _serde::__private::None;
                            let mut __field3: _serde::__private::Option<i32> = _serde::__private::None;
                            while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                                __Field,
                            >(&mut __map)? {
                                match __key {
                                    __Field::__field0 => {
                                        if _serde::__private::Option::is_some(&__field0) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "track_id_slot_1",
                                                ),
                                            );
                                        }
                                        __field0 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field1 => {
                                        if _serde::__private::Option::is_some(&__field1) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "track_id_slot_2",
                                                ),
                                            );
                                        }
                                        __field1 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field2 => {
                                        if _serde::__private::Option::is_some(&__field2) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "track_id_slot_3",
                                                ),
                                            );
                                        }
                                        __field2 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                        );
                                    }
                                    __Field::__field3 => {
                                        if _serde::__private::Option::is_some(&__field3) {
                                            return _serde::__private::Err(
                                                <__A::Error as _serde::de::Error>::duplicate_field(
                                                    "track_id_slot_4",
                                                ),
                                            );
                                        }
                                        __field3 = _serde::__private::Some(
                                            _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                        );
                                    }
                                    _ => {
                                        let _ = _serde::de::MapAccess::next_value::<
                                            _serde::de::IgnoredAny,
                                        >(&mut __map)?;
                                    }
                                }
                            }
                            let __field0 = match __field0 {
                                _serde::__private::Some(__field0) => __field0,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("track_id_slot_1")?
                                }
                            };
                            let __field1 = match __field1 {
                                _serde::__private::Some(__field1) => __field1,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("track_id_slot_2")?
                                }
                            };
                            let __field2 = match __field2 {
                                _serde::__private::Some(__field2) => __field2,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("track_id_slot_3")?
                                }
                            };
                            let __field3 = match __field3 {
                                _serde::__private::Some(__field3) => __field3,
                                _serde::__private::None => {
                                    _serde::__private::de::missing_field("track_id_slot_4")?
                                }
                            };
                            _serde::__private::Ok(CupSlots {
                                track_id_slot_1: __field0,
                                track_id_slot_2: __field1,
                                track_id_slot_3: __field2,
                                track_id_slot_4: __field3,
                            })
                        }
                    }
                    #[doc(hidden)]
                    const FIELDS: &'static [&'static str] = &[
                        "track_id_slot_1",
                        "track_id_slot_2",
                        "track_id_slot_3",
                        "track_id_slot_4",
                    ];
                    _serde::Deserializer::deserialize_struct(
                        __deserializer,
                        "CupSlots",
                        FIELDS,
                        __Visitor {
                            marker: _serde::__private::PhantomData::<CupSlots>,
                            lifetime: _serde::__private::PhantomData,
                        },
                    )
                }
            }
        };
        #[automatically_derived]
        impl ::core::clone::Clone for CupSlots {
            #[inline]
            fn clone(&self) -> CupSlots {
                let _: ::core::clone::AssertParamIsClone<i32>;
                *self
            }
        }
        #[automatically_derived]
        impl ::core::marker::Copy for CupSlots {}
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsConstructor for CupSlots {}
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsInstanceProperty for CupSlots {}
        #[automatically_derived]
        impl wasm_bindgen::__rt::marker::SupportsStaticProperty for CupSlots {}
        #[automatically_derived]
        impl wasm_bindgen::describe::WasmDescribe for CupSlots {
            fn describe() {
                use wasm_bindgen::describe::*;
                inform(RUST_STRUCT);
                inform(8u32);
                inform(67u32);
                inform(117u32);
                inform(112u32);
                inform(83u32);
                inform(108u32);
                inform(111u32);
                inform(116u32);
                inform(115u32);
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::IntoWasmAbi for CupSlots {
            type Abi = u32;
            fn into_abi(self) -> u32 {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                use wasm_bindgen::__rt::WasmRefCell;
                Rc::into_raw(Rc::new(WasmRefCell::new(self))) as u32
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::FromWasmAbi for CupSlots {
            type Abi = u32;
            unsafe fn from_abi(js: u32) -> Self {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                use wasm_bindgen::__rt::core::result::Result::{Ok, Err};
                use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
                let ptr = js as *mut WasmRefCell<CupSlots>;
                assert_not_null(ptr);
                let rc = Rc::from_raw(ptr);
                match Rc::try_unwrap(rc) {
                    Ok(cell) => cell.into_inner(),
                    Err(_) => {
                        wasm_bindgen::throw_str(
                            "attempted to take ownership of Rust value while it was borrowed",
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::__rt::core::convert::From<CupSlots>
        for wasm_bindgen::JsValue {
            fn from(value: CupSlots) -> Self {
                let ptr = wasm_bindgen::convert::IntoWasmAbi::into_abi(value);
                unsafe fn __wbg_cupslots_new(_: u32) -> u32 {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "cannot convert to JsValue outside of the Wasm target",
                            ),
                        );
                    }
                }
                unsafe {
                    <wasm_bindgen::JsValue as wasm_bindgen::convert::FromWasmAbi>::from_abi(
                        __wbg_cupslots_new(ptr),
                    )
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::RefFromWasmAbi for CupSlots {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRef<CupSlots>;
            unsafe fn ref_from_abi(js: Self::Abi) -> Self::Anchor {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<CupSlots>;
                wasm_bindgen::__rt::assert_not_null(js);
                Rc::increment_strong_count(js);
                let rc = Rc::from_raw(js);
                wasm_bindgen::__rt::RcRef::new(rc)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::RefMutFromWasmAbi for CupSlots {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRefMut<CupSlots>;
            unsafe fn ref_mut_from_abi(js: Self::Abi) -> Self::Anchor {
                use wasm_bindgen::__rt::alloc::rc::Rc;
                let js = js as *mut wasm_bindgen::__rt::WasmRefCell<CupSlots>;
                wasm_bindgen::__rt::assert_not_null(js);
                Rc::increment_strong_count(js);
                let rc = Rc::from_raw(js);
                wasm_bindgen::__rt::RcRefMut::new(rc)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::LongRefFromWasmAbi for CupSlots {
            type Abi = u32;
            type Anchor = wasm_bindgen::__rt::RcRef<CupSlots>;
            unsafe fn long_ref_from_abi(js: Self::Abi) -> Self::Anchor {
                <Self as wasm_bindgen::convert::RefFromWasmAbi>::ref_from_abi(js)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::OptionIntoWasmAbi for CupSlots {
            #[inline]
            fn none() -> Self::Abi {
                0
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::OptionFromWasmAbi for CupSlots {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool {
                *abi == 0
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::TryFromJsValue for CupSlots {
            type Error = wasm_bindgen::JsValue;
            fn try_from_js_value(
                value: wasm_bindgen::JsValue,
            ) -> wasm_bindgen::__rt::core::result::Result<Self, Self::Error> {
                let idx = wasm_bindgen::convert::IntoWasmAbi::into_abi(&value);
                unsafe fn __wbg_cupslots_unwrap(_: u32) -> u32 {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!(
                                "cannot convert from JsValue outside of the Wasm target",
                            ),
                        );
                    }
                }
                let ptr = unsafe { __wbg_cupslots_unwrap(idx) };
                if ptr == 0 {
                    wasm_bindgen::__rt::core::result::Result::Err(value)
                } else {
                    #[allow(clippy::mem_forget)]
                    wasm_bindgen::__rt::core::mem::forget(value);
                    unsafe {
                        wasm_bindgen::__rt::core::result::Result::Ok(
                            <Self as wasm_bindgen::convert::FromWasmAbi>::from_abi(ptr),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::describe::WasmDescribeVector for CupSlots {
            fn describe_vector() {
                use wasm_bindgen::describe::*;
                inform(VECTOR);
                inform(NAMED_EXTERNREF);
                inform(8u32);
                inform(67u32);
                inform(117u32);
                inform(112u32);
                inform(83u32);
                inform(108u32);
                inform(111u32);
                inform(116u32);
                inform(115u32);
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::VectorIntoWasmAbi for CupSlots {
            type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
                [wasm_bindgen::JsValue],
            > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
            fn vector_into_abi(
                vector: wasm_bindgen::__rt::alloc::boxed::Box<[CupSlots]>,
            ) -> Self::Abi {
                wasm_bindgen::convert::js_value_vector_into_abi(vector)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::convert::VectorFromWasmAbi for CupSlots {
            type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
                [wasm_bindgen::JsValue],
            > as wasm_bindgen::convert::FromWasmAbi>::Abi;
            unsafe fn vector_from_abi(
                js: Self::Abi,
            ) -> wasm_bindgen::__rt::alloc::boxed::Box<[CupSlots]> {
                wasm_bindgen::convert::js_value_vector_from_abi(js)
            }
        }
        #[automatically_derived]
        impl wasm_bindgen::__rt::VectorIntoJsValue for CupSlots {
            fn vector_into_jsvalue(
                vector: wasm_bindgen::__rt::alloc::boxed::Box<[CupSlots]>,
            ) -> wasm_bindgen::JsValue {
                wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
            }
        }
        impl From<[i32; 4]> for CupSlots {
            fn from(value: [i32; 4]) -> Self {
                let [track_id_slot_1, track_id_slot_2, track_id_slot_3, track_id_slot_4,
                ] = value;
                Self {
                    track_id_slot_1,
                    track_id_slot_2,
                    track_id_slot_3,
                    track_id_slot_4,
                }
            }
        }
        impl HasId for GetCupsOutput {
            fn get_id(&self) -> i32 {
                self.id
            }
        }
    }
    pub mod players {
        use crate::endpoint::{Root, Scope};
        pub struct PlayersScope;
        impl Scope for PlayersScope {
            const PATH: &'static str = "/players";
            type OuterScope = Root;
        }
    }
    pub mod tracks {
        use crate::{
            common_data_traits::{GetId, HasId},
            common_types::{Category, NoData},
            endpoint::{Endpoint, RequiredPermission, Root, Scope},
            request_method::RequestMethod,
        };
        pub struct TracksScope;
        impl Scope for TracksScope {
            const PATH: &'static str = "/tracks";
            type OuterScope = Root;
        }
        pub struct GetTracks;
        #[automatically_derived]
        impl ::core::default::Default for GetTracks {
            #[inline]
            fn default() -> GetTracks {
                GetTracks {}
            }
        }
        impl Endpoint for GetTracks {
            const PATH: &'static str = "/get";
            const REQUEST_METHOD: RequestMethod = RequestMethod::Get;
            const REQUIRED_PERMISSION: RequiredPermission = RequiredPermission::None;
            type InputStruct = NoData;
            type OutputStruct = Vec<GetTracksOutput>;
            type ScopeStruct = TracksScope;
        }
        pub struct GetTracksOutput {
            pub id: i32,
            pub abbr: String,
            pub cup_id: i32,
            pub categories: Vec<Category>,
        }
        impl HasId for GetTracksOutput {
            fn get_id(&self) -> i32 {
                self.id
            }
        }
    }
    pub trait Endpoint: Default {
        const PATH: &'static str;
        const REQUEST_METHOD: RequestMethod;
        const REQUIRED_PERMISSION: RequiredPermission;
        type InputStruct: DataTraits;
        type OutputStruct;
        type ScopeStruct: Scope;
        fn construct_full_path() -> String {
            Self::ScopeStruct::construct_full_path() + Self::PATH
        }
    }
    pub trait Scope {
        const PATH: &'static str;
        type OuterScope: Scope;
        fn construct_full_path() -> String {
            Self::OuterScope::construct_full_path() + Self::PATH
        }
    }
    pub struct Root;
    impl Scope for Root {
        const PATH: &'static str = "/v1";
        type OuterScope = Root;
    }
}
pub mod error {
    use crate::status_code::StatusCode;
    pub type PPResult<T> = Result<T, FinalErrorResponse>;
    pub struct FinalErrorResponse {
        pub status_code: StatusCode,
        pub errors: Vec<FinalError>,
    }
    impl FinalErrorResponse {
        #[inline]
        pub fn new(
            status_code: StatusCode,
            errors: Vec<FinalError>,
        ) -> FinalErrorResponse {
            FinalErrorResponse {
                status_code,
                errors,
            }
        }
        #[inline]
        pub fn new_from_final_error(
            status_code: StatusCode,
            error: FinalError,
        ) -> FinalErrorResponse {
            FinalErrorResponse {
                status_code,
                errors: <[_]>::into_vec(::alloc::boxed::box_new([error])),
            }
        }
        #[inline]
        pub fn push(&mut self, error: FinalError) {
            self.errors.push(error);
        }
    }
    pub struct FinalError {
        pub error_code: u64,
        pub field: Option<&'static str>,
        pub error_text: &'static str,
        pub library_error: Option<String>,
        pub backend_file: &'static str,
        pub backend_line: u32,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for FinalError {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "FinalError",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "error_code",
                    &self.error_code,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "field",
                    &self.field,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "error_text",
                    &self.error_text,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "library_error",
                    &self.library_error,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "backend_file",
                    &self.backend_file,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "backend_line",
                    &self.backend_line,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    impl FinalError {
        fn new(
            error_code: u64,
            field: Option<&'static str>,
            error_text: &'static str,
            library_error: Option<impl ToString>,
            backend_file: &'static str,
            backend_line: u32,
        ) -> Self {
            Self {
                error_code,
                field,
                error_text,
                backend_file,
                backend_line,
                library_error: library_error.map(|r| r.to_string()),
            }
        }
        #[inline]
        pub fn into_response(self, status_code: StatusCode) -> FinalErrorResponse {
            FinalErrorResponse::new_from_final_error(status_code, self)
        }
    }
    pub enum ErrorCodes {
        NoConnectionFromPGPool,
        SerializingDataToJSON,
        ClosingConnectionFromPGPool,
        GettingFromDatabase,
        DecodingDatabaseRows,
        UserIdToPlayerId,
        GenerateTimesheet,
        GenerateMatchup,
        UsernameTooShort,
        UsernameTooLong,
        PasswordTooLong,
        PasswordTooShort,
        PasswordMustHaveSpecial,
        PasswordMustHaveLowercase,
        PasswordMustHaveUppercase,
        PasswordMustHaveNumber,
        EmailTooLong,
        EmailInvalid,
        UserIDDoesntExist,
        InvalidSessionToken,
        UserHasNoAssociatedPlayer,
        CreatePGTransaction,
        CommitPGTransaction,
        InsufficientPermissions,
        GeneratingToken,
        MismatchedIds,
        NothingChanged,
        InvalidInput,
        TechnicallyUnreachableCode,
        CreatingEmailClient,
        SendingEmail,
        UserNotVerified,
        UserOnCooldown,
        NoAssociatedPlayer,
        InvalidChadsoftID,
        NoDataToSerialize,
        CannotReadFile,
        RollBackPGTransaction,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ErrorCodes {
        #[inline]
        fn clone(&self) -> ErrorCodes {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for ErrorCodes {}
    impl From<ErrorCodes> for u64 {
        fn from(val: ErrorCodes) -> Self {
            match val {
                ErrorCodes::NoConnectionFromPGPool => 0,
                ErrorCodes::SerializingDataToJSON => 1,
                ErrorCodes::ClosingConnectionFromPGPool => 2,
                ErrorCodes::GettingFromDatabase => 3,
                ErrorCodes::DecodingDatabaseRows => 4,
                ErrorCodes::UserIdToPlayerId => 5,
                ErrorCodes::GenerateTimesheet => 6,
                ErrorCodes::GenerateMatchup => 7,
                ErrorCodes::UsernameTooShort => 8,
                ErrorCodes::UsernameTooLong => 9,
                ErrorCodes::PasswordTooLong => 10,
                ErrorCodes::PasswordTooShort => 11,
                ErrorCodes::PasswordMustHaveSpecial => 12,
                ErrorCodes::PasswordMustHaveLowercase => 13,
                ErrorCodes::PasswordMustHaveUppercase => 14,
                ErrorCodes::PasswordMustHaveNumber => 15,
                ErrorCodes::EmailTooLong => 16,
                ErrorCodes::EmailInvalid => 17,
                ErrorCodes::UserIDDoesntExist => 18,
                ErrorCodes::InvalidSessionToken => 19,
                ErrorCodes::UserHasNoAssociatedPlayer => 20,
                ErrorCodes::CreatePGTransaction => 21,
                ErrorCodes::CommitPGTransaction => 22,
                ErrorCodes::InsufficientPermissions => 23,
                ErrorCodes::GeneratingToken => 24,
                ErrorCodes::MismatchedIds => 25,
                ErrorCodes::NothingChanged => 26,
                ErrorCodes::InvalidInput => 27,
                ErrorCodes::TechnicallyUnreachableCode => 28,
                ErrorCodes::CreatingEmailClient => 29,
                ErrorCodes::SendingEmail => 30,
                ErrorCodes::UserNotVerified => 31,
                ErrorCodes::UserOnCooldown => 32,
                ErrorCodes::NoAssociatedPlayer => 33,
                ErrorCodes::InvalidChadsoftID => 34,
                ErrorCodes::NoDataToSerialize => 35,
                ErrorCodes::CannotReadFile => 36,
                ErrorCodes::RollBackPGTransaction => 37,
            }
        }
    }
    impl ErrorCodes {
        pub fn into_final_error(
            self,
            library_error: Option<impl ToString>,
            backend_file: &'static str,
            backend_line: u32,
        ) -> FinalError {
            let (field, error_text) = match self {
                Self::NoConnectionFromPGPool => {
                    (None, "Couldn't get connection from data pool")
                }
                Self::SerializingDataToJSON => (None, "Error serializing data to JSON"),
                Self::ClosingConnectionFromPGPool => {
                    (None, "Error closing Database connection")
                }
                Self::GettingFromDatabase => (None, "Couldn't get rows from database"),
                Self::DecodingDatabaseRows => (None, "Error decoding database rows"),
                Self::UserIdToPlayerId => (None, "Error converting User ID to Player ID"),
                Self::GenerateTimesheet => (None, "Error generating timesheet"),
                Self::GenerateMatchup => (None, "Error generating matchup"),
                Self::UsernameTooShort => (Some("username"), "Username too short"),
                Self::UsernameTooLong => (Some("username"), "Username too long"),
                Self::PasswordTooLong => (Some("password"), "Password too long"),
                Self::PasswordTooShort => (Some("password"), "Password too short"),
                Self::PasswordMustHaveSpecial => {
                    (Some("password"), "Password must have a special character")
                }
                Self::PasswordMustHaveLowercase => {
                    (Some("password"), "Password must have a lowercase character")
                }
                Self::PasswordMustHaveUppercase => {
                    (Some("password"), "Password must have a uppercase character")
                }
                Self::PasswordMustHaveNumber => {
                    (Some("password"), "Password must have a number")
                }
                Self::EmailTooLong => (Some("email"), "Email too long"),
                Self::EmailInvalid => (Some("email"), "Email invalid"),
                Self::UserIDDoesntExist => (None, "Error getting user ID"),
                Self::InvalidSessionToken => (None, "Invalid session token"),
                Self::UserHasNoAssociatedPlayer => {
                    (None, "User has no associated player profile")
                }
                Self::CreatePGTransaction => {
                    (None, "Error creating postgres transaction")
                }
                Self::CommitPGTransaction => {
                    (None, "Error committing postgres transaction")
                }
                Self::InsufficientPermissions => (None, "Insufficient permissions"),
                Self::GeneratingToken => (None, "Error generating token"),
                Self::MismatchedIds => (None, "Mismatched IDs"),
                Self::NothingChanged => (None, "Nothing to update"),
                Self::InvalidInput => (None, "Input is invalid"),
                Self::TechnicallyUnreachableCode => {
                    (None, "Technically unreachable code has been reached")
                }
                Self::CreatingEmailClient => {
                    (None, "There was an error creating the email client")
                }
                Self::SendingEmail => (None, "There was an error sending the email"),
                Self::UserNotVerified => (None, "User is not verified"),
                Self::UserOnCooldown => (None, "User is on cooldown"),
                Self::NoAssociatedPlayer => (None, "There is no associated player"),
                Self::InvalidChadsoftID => (None, "Chadsoft ID is not valid"),
                Self::NoDataToSerialize => (None, "Whoever coded this is a moron"),
                Self::CannotReadFile => (None, "Could not read file"),
                Self::RollBackPGTransaction => {
                    (None, "Error rolling back postgres transaction")
                }
            };
            FinalError::new(
                self.into(),
                field,
                error_text,
                library_error,
                backend_file,
                backend_line,
            )
        }
    }
}
pub mod request_method {
    pub enum RequestMethod {
        Get,
        Head,
        Post,
        Put,
        Delete,
        Connect,
        Options,
        Trace,
        Patch,
    }
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for RequestMethod {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for RequestMethod {
        #[inline]
        fn eq(&self, other: &RequestMethod) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
        }
    }
    impl RequestMethod {
        pub const fn to_str(&self) -> &'static str {
            match self {
                Self::Get => "GET",
                Self::Head => "HEAD",
                Self::Post => "POST",
                Self::Put => "PUT",
                Self::Delete => "DELETE",
                Self::Connect => "CONNECT",
                Self::Options => "OPTIONS",
                Self::Trace => "TRACE",
                Self::Patch => "PATCH",
            }
        }
    }
}
pub mod required_permission {
    pub enum RequiredPermission {
        None,
        LoggedIn,
        Admin,
    }
}
pub mod status_code {
    pub enum StatusCode {
        Continue,
        SwitchingProtocols,
        Processing,
        EarlyHints,
        OK,
        Created,
        Accepted,
        NonAuthoritativeInformation,
        NoContent,
        ResetContent,
        PartialContent,
        MultiStatus,
        AlreadyReported,
        IMUsed,
        MultipleChoices,
        MovedPermanently,
        Found,
        SeeOther,
        NotModified,
        UseProxy,
        TemporaryRedirect,
        PermanentRedirect,
        BadRequest,
        Unauthorized,
        PaymentRequired,
        Forbidden,
        NotFound,
        MethodNotAllowed,
        NotAcceptable,
        ProxyAuthenticationRequired,
        RequestTimeout,
        Conflict,
        Gone,
        LengthRequired,
        PreconditionFailed,
        ContentTooLarge,
        URITooLong,
        UnsupportedMediaType,
        RangeNotSatisfiable,
        ExpectationFailed,
        MisdirectedRequest,
        UnprocessableContent,
        Locked,
        FailedDependency,
        TooEarly,
        UpgradeRequired,
        PreconditionRequired,
        TooManyRequests,
        RequestHeaderFieldsTooLarge,
        UnavailableForLegalReasons,
        InternalServerError,
        NotImplemented,
        BadGateway,
        ServiceUnavailable,
        GatewayTimeout,
        HTTPVersionNotSupported,
        VariantAlsoNegotiates,
        InsufficientStorage,
        LoopDetected,
        NetworkAuthenticationRequired,
    }
    #[doc(hidden)]
    #[allow(
        non_upper_case_globals,
        unused_attributes,
        unused_qualifications,
        clippy::absolute_paths,
    )]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for StatusCode {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __field9,
                    __field10,
                    __field11,
                    __field12,
                    __field13,
                    __field14,
                    __field15,
                    __field16,
                    __field17,
                    __field18,
                    __field19,
                    __field20,
                    __field21,
                    __field22,
                    __field23,
                    __field24,
                    __field25,
                    __field26,
                    __field27,
                    __field28,
                    __field29,
                    __field30,
                    __field31,
                    __field32,
                    __field33,
                    __field34,
                    __field35,
                    __field36,
                    __field37,
                    __field38,
                    __field39,
                    __field40,
                    __field41,
                    __field42,
                    __field43,
                    __field44,
                    __field45,
                    __field46,
                    __field47,
                    __field48,
                    __field49,
                    __field50,
                    __field51,
                    __field52,
                    __field53,
                    __field54,
                    __field55,
                    __field56,
                    __field57,
                    __field58,
                    __field59,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "variant identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            9u64 => _serde::__private::Ok(__Field::__field9),
                            10u64 => _serde::__private::Ok(__Field::__field10),
                            11u64 => _serde::__private::Ok(__Field::__field11),
                            12u64 => _serde::__private::Ok(__Field::__field12),
                            13u64 => _serde::__private::Ok(__Field::__field13),
                            14u64 => _serde::__private::Ok(__Field::__field14),
                            15u64 => _serde::__private::Ok(__Field::__field15),
                            16u64 => _serde::__private::Ok(__Field::__field16),
                            17u64 => _serde::__private::Ok(__Field::__field17),
                            18u64 => _serde::__private::Ok(__Field::__field18),
                            19u64 => _serde::__private::Ok(__Field::__field19),
                            20u64 => _serde::__private::Ok(__Field::__field20),
                            21u64 => _serde::__private::Ok(__Field::__field21),
                            22u64 => _serde::__private::Ok(__Field::__field22),
                            23u64 => _serde::__private::Ok(__Field::__field23),
                            24u64 => _serde::__private::Ok(__Field::__field24),
                            25u64 => _serde::__private::Ok(__Field::__field25),
                            26u64 => _serde::__private::Ok(__Field::__field26),
                            27u64 => _serde::__private::Ok(__Field::__field27),
                            28u64 => _serde::__private::Ok(__Field::__field28),
                            29u64 => _serde::__private::Ok(__Field::__field29),
                            30u64 => _serde::__private::Ok(__Field::__field30),
                            31u64 => _serde::__private::Ok(__Field::__field31),
                            32u64 => _serde::__private::Ok(__Field::__field32),
                            33u64 => _serde::__private::Ok(__Field::__field33),
                            34u64 => _serde::__private::Ok(__Field::__field34),
                            35u64 => _serde::__private::Ok(__Field::__field35),
                            36u64 => _serde::__private::Ok(__Field::__field36),
                            37u64 => _serde::__private::Ok(__Field::__field37),
                            38u64 => _serde::__private::Ok(__Field::__field38),
                            39u64 => _serde::__private::Ok(__Field::__field39),
                            40u64 => _serde::__private::Ok(__Field::__field40),
                            41u64 => _serde::__private::Ok(__Field::__field41),
                            42u64 => _serde::__private::Ok(__Field::__field42),
                            43u64 => _serde::__private::Ok(__Field::__field43),
                            44u64 => _serde::__private::Ok(__Field::__field44),
                            45u64 => _serde::__private::Ok(__Field::__field45),
                            46u64 => _serde::__private::Ok(__Field::__field46),
                            47u64 => _serde::__private::Ok(__Field::__field47),
                            48u64 => _serde::__private::Ok(__Field::__field48),
                            49u64 => _serde::__private::Ok(__Field::__field49),
                            50u64 => _serde::__private::Ok(__Field::__field50),
                            51u64 => _serde::__private::Ok(__Field::__field51),
                            52u64 => _serde::__private::Ok(__Field::__field52),
                            53u64 => _serde::__private::Ok(__Field::__field53),
                            54u64 => _serde::__private::Ok(__Field::__field54),
                            55u64 => _serde::__private::Ok(__Field::__field55),
                            56u64 => _serde::__private::Ok(__Field::__field56),
                            57u64 => _serde::__private::Ok(__Field::__field57),
                            58u64 => _serde::__private::Ok(__Field::__field58),
                            59u64 => _serde::__private::Ok(__Field::__field59),
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::invalid_value(
                                        _serde::de::Unexpected::Unsigned(__value),
                                        &"variant index 0 <= i < 60",
                                    ),
                                )
                            }
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "Continue" => _serde::__private::Ok(__Field::__field0),
                            "SwitchingProtocols" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            "Processing" => _serde::__private::Ok(__Field::__field2),
                            "EarlyHints" => _serde::__private::Ok(__Field::__field3),
                            "OK" => _serde::__private::Ok(__Field::__field4),
                            "Created" => _serde::__private::Ok(__Field::__field5),
                            "Accepted" => _serde::__private::Ok(__Field::__field6),
                            "NonAuthoritativeInformation" => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            "NoContent" => _serde::__private::Ok(__Field::__field8),
                            "ResetContent" => _serde::__private::Ok(__Field::__field9),
                            "PartialContent" => _serde::__private::Ok(__Field::__field10),
                            "MultiStatus" => _serde::__private::Ok(__Field::__field11),
                            "AlreadyReported" => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            "IMUsed" => _serde::__private::Ok(__Field::__field13),
                            "MultipleChoices" => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            "MovedPermanently" => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            "Found" => _serde::__private::Ok(__Field::__field16),
                            "SeeOther" => _serde::__private::Ok(__Field::__field17),
                            "NotModified" => _serde::__private::Ok(__Field::__field18),
                            "UseProxy" => _serde::__private::Ok(__Field::__field19),
                            "TemporaryRedirect" => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            "PermanentRedirect" => {
                                _serde::__private::Ok(__Field::__field21)
                            }
                            "BadRequest" => _serde::__private::Ok(__Field::__field22),
                            "Unauthorized" => _serde::__private::Ok(__Field::__field23),
                            "PaymentRequired" => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            "Forbidden" => _serde::__private::Ok(__Field::__field25),
                            "NotFound" => _serde::__private::Ok(__Field::__field26),
                            "MethodNotAllowed" => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            "NotAcceptable" => _serde::__private::Ok(__Field::__field28),
                            "ProxyAuthenticationRequired" => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            "RequestTimeout" => _serde::__private::Ok(__Field::__field30),
                            "Conflict" => _serde::__private::Ok(__Field::__field31),
                            "Gone" => _serde::__private::Ok(__Field::__field32),
                            "LengthRequired" => _serde::__private::Ok(__Field::__field33),
                            "PreconditionFailed" => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            "ContentTooLarge" => {
                                _serde::__private::Ok(__Field::__field35)
                            }
                            "URITooLong" => _serde::__private::Ok(__Field::__field36),
                            "UnsupportedMediaType" => {
                                _serde::__private::Ok(__Field::__field37)
                            }
                            "RangeNotSatisfiable" => {
                                _serde::__private::Ok(__Field::__field38)
                            }
                            "ExpectationFailed" => {
                                _serde::__private::Ok(__Field::__field39)
                            }
                            "MisdirectedRequest" => {
                                _serde::__private::Ok(__Field::__field40)
                            }
                            "UnprocessableContent" => {
                                _serde::__private::Ok(__Field::__field41)
                            }
                            "Locked" => _serde::__private::Ok(__Field::__field42),
                            "FailedDependency" => {
                                _serde::__private::Ok(__Field::__field43)
                            }
                            "TooEarly" => _serde::__private::Ok(__Field::__field44),
                            "UpgradeRequired" => {
                                _serde::__private::Ok(__Field::__field45)
                            }
                            "PreconditionRequired" => {
                                _serde::__private::Ok(__Field::__field46)
                            }
                            "TooManyRequests" => {
                                _serde::__private::Ok(__Field::__field47)
                            }
                            "RequestHeaderFieldsTooLarge" => {
                                _serde::__private::Ok(__Field::__field48)
                            }
                            "UnavailableForLegalReasons" => {
                                _serde::__private::Ok(__Field::__field49)
                            }
                            "InternalServerError" => {
                                _serde::__private::Ok(__Field::__field50)
                            }
                            "NotImplemented" => _serde::__private::Ok(__Field::__field51),
                            "BadGateway" => _serde::__private::Ok(__Field::__field52),
                            "ServiceUnavailable" => {
                                _serde::__private::Ok(__Field::__field53)
                            }
                            "GatewayTimeout" => _serde::__private::Ok(__Field::__field54),
                            "HTTPVersionNotSupported" => {
                                _serde::__private::Ok(__Field::__field55)
                            }
                            "VariantAlsoNegotiates" => {
                                _serde::__private::Ok(__Field::__field56)
                            }
                            "InsufficientStorage" => {
                                _serde::__private::Ok(__Field::__field57)
                            }
                            "LoopDetected" => _serde::__private::Ok(__Field::__field58),
                            "NetworkAuthenticationRequired" => {
                                _serde::__private::Ok(__Field::__field59)
                            }
                            _ => {
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"Continue" => _serde::__private::Ok(__Field::__field0),
                            b"SwitchingProtocols" => {
                                _serde::__private::Ok(__Field::__field1)
                            }
                            b"Processing" => _serde::__private::Ok(__Field::__field2),
                            b"EarlyHints" => _serde::__private::Ok(__Field::__field3),
                            b"OK" => _serde::__private::Ok(__Field::__field4),
                            b"Created" => _serde::__private::Ok(__Field::__field5),
                            b"Accepted" => _serde::__private::Ok(__Field::__field6),
                            b"NonAuthoritativeInformation" => {
                                _serde::__private::Ok(__Field::__field7)
                            }
                            b"NoContent" => _serde::__private::Ok(__Field::__field8),
                            b"ResetContent" => _serde::__private::Ok(__Field::__field9),
                            b"PartialContent" => {
                                _serde::__private::Ok(__Field::__field10)
                            }
                            b"MultiStatus" => _serde::__private::Ok(__Field::__field11),
                            b"AlreadyReported" => {
                                _serde::__private::Ok(__Field::__field12)
                            }
                            b"IMUsed" => _serde::__private::Ok(__Field::__field13),
                            b"MultipleChoices" => {
                                _serde::__private::Ok(__Field::__field14)
                            }
                            b"MovedPermanently" => {
                                _serde::__private::Ok(__Field::__field15)
                            }
                            b"Found" => _serde::__private::Ok(__Field::__field16),
                            b"SeeOther" => _serde::__private::Ok(__Field::__field17),
                            b"NotModified" => _serde::__private::Ok(__Field::__field18),
                            b"UseProxy" => _serde::__private::Ok(__Field::__field19),
                            b"TemporaryRedirect" => {
                                _serde::__private::Ok(__Field::__field20)
                            }
                            b"PermanentRedirect" => {
                                _serde::__private::Ok(__Field::__field21)
                            }
                            b"BadRequest" => _serde::__private::Ok(__Field::__field22),
                            b"Unauthorized" => _serde::__private::Ok(__Field::__field23),
                            b"PaymentRequired" => {
                                _serde::__private::Ok(__Field::__field24)
                            }
                            b"Forbidden" => _serde::__private::Ok(__Field::__field25),
                            b"NotFound" => _serde::__private::Ok(__Field::__field26),
                            b"MethodNotAllowed" => {
                                _serde::__private::Ok(__Field::__field27)
                            }
                            b"NotAcceptable" => _serde::__private::Ok(__Field::__field28),
                            b"ProxyAuthenticationRequired" => {
                                _serde::__private::Ok(__Field::__field29)
                            }
                            b"RequestTimeout" => {
                                _serde::__private::Ok(__Field::__field30)
                            }
                            b"Conflict" => _serde::__private::Ok(__Field::__field31),
                            b"Gone" => _serde::__private::Ok(__Field::__field32),
                            b"LengthRequired" => {
                                _serde::__private::Ok(__Field::__field33)
                            }
                            b"PreconditionFailed" => {
                                _serde::__private::Ok(__Field::__field34)
                            }
                            b"ContentTooLarge" => {
                                _serde::__private::Ok(__Field::__field35)
                            }
                            b"URITooLong" => _serde::__private::Ok(__Field::__field36),
                            b"UnsupportedMediaType" => {
                                _serde::__private::Ok(__Field::__field37)
                            }
                            b"RangeNotSatisfiable" => {
                                _serde::__private::Ok(__Field::__field38)
                            }
                            b"ExpectationFailed" => {
                                _serde::__private::Ok(__Field::__field39)
                            }
                            b"MisdirectedRequest" => {
                                _serde::__private::Ok(__Field::__field40)
                            }
                            b"UnprocessableContent" => {
                                _serde::__private::Ok(__Field::__field41)
                            }
                            b"Locked" => _serde::__private::Ok(__Field::__field42),
                            b"FailedDependency" => {
                                _serde::__private::Ok(__Field::__field43)
                            }
                            b"TooEarly" => _serde::__private::Ok(__Field::__field44),
                            b"UpgradeRequired" => {
                                _serde::__private::Ok(__Field::__field45)
                            }
                            b"PreconditionRequired" => {
                                _serde::__private::Ok(__Field::__field46)
                            }
                            b"TooManyRequests" => {
                                _serde::__private::Ok(__Field::__field47)
                            }
                            b"RequestHeaderFieldsTooLarge" => {
                                _serde::__private::Ok(__Field::__field48)
                            }
                            b"UnavailableForLegalReasons" => {
                                _serde::__private::Ok(__Field::__field49)
                            }
                            b"InternalServerError" => {
                                _serde::__private::Ok(__Field::__field50)
                            }
                            b"NotImplemented" => {
                                _serde::__private::Ok(__Field::__field51)
                            }
                            b"BadGateway" => _serde::__private::Ok(__Field::__field52),
                            b"ServiceUnavailable" => {
                                _serde::__private::Ok(__Field::__field53)
                            }
                            b"GatewayTimeout" => {
                                _serde::__private::Ok(__Field::__field54)
                            }
                            b"HTTPVersionNotSupported" => {
                                _serde::__private::Ok(__Field::__field55)
                            }
                            b"VariantAlsoNegotiates" => {
                                _serde::__private::Ok(__Field::__field56)
                            }
                            b"InsufficientStorage" => {
                                _serde::__private::Ok(__Field::__field57)
                            }
                            b"LoopDetected" => _serde::__private::Ok(__Field::__field58),
                            b"NetworkAuthenticationRequired" => {
                                _serde::__private::Ok(__Field::__field59)
                            }
                            _ => {
                                let __value = &_serde::__private::from_utf8_lossy(__value);
                                _serde::__private::Err(
                                    _serde::de::Error::unknown_variant(__value, VARIANTS),
                                )
                            }
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<StatusCode>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = StatusCode;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "enum StatusCode",
                        )
                    }
                    fn visit_enum<__A>(
                        self,
                        __data: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::EnumAccess<'de>,
                    {
                        match _serde::de::EnumAccess::variant(__data)? {
                            (__Field::__field0, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Continue)
                            }
                            (__Field::__field1, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::SwitchingProtocols)
                            }
                            (__Field::__field2, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Processing)
                            }
                            (__Field::__field3, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::EarlyHints)
                            }
                            (__Field::__field4, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::OK)
                            }
                            (__Field::__field5, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Created)
                            }
                            (__Field::__field6, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Accepted)
                            }
                            (__Field::__field7, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    StatusCode::NonAuthoritativeInformation,
                                )
                            }
                            (__Field::__field8, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::NoContent)
                            }
                            (__Field::__field9, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::ResetContent)
                            }
                            (__Field::__field10, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::PartialContent)
                            }
                            (__Field::__field11, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::MultiStatus)
                            }
                            (__Field::__field12, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::AlreadyReported)
                            }
                            (__Field::__field13, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::IMUsed)
                            }
                            (__Field::__field14, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::MultipleChoices)
                            }
                            (__Field::__field15, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::MovedPermanently)
                            }
                            (__Field::__field16, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Found)
                            }
                            (__Field::__field17, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::SeeOther)
                            }
                            (__Field::__field18, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::NotModified)
                            }
                            (__Field::__field19, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::UseProxy)
                            }
                            (__Field::__field20, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::TemporaryRedirect)
                            }
                            (__Field::__field21, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::PermanentRedirect)
                            }
                            (__Field::__field22, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::BadRequest)
                            }
                            (__Field::__field23, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Unauthorized)
                            }
                            (__Field::__field24, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::PaymentRequired)
                            }
                            (__Field::__field25, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Forbidden)
                            }
                            (__Field::__field26, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::NotFound)
                            }
                            (__Field::__field27, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::MethodNotAllowed)
                            }
                            (__Field::__field28, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::NotAcceptable)
                            }
                            (__Field::__field29, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    StatusCode::ProxyAuthenticationRequired,
                                )
                            }
                            (__Field::__field30, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::RequestTimeout)
                            }
                            (__Field::__field31, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Conflict)
                            }
                            (__Field::__field32, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Gone)
                            }
                            (__Field::__field33, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::LengthRequired)
                            }
                            (__Field::__field34, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::PreconditionFailed)
                            }
                            (__Field::__field35, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::ContentTooLarge)
                            }
                            (__Field::__field36, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::URITooLong)
                            }
                            (__Field::__field37, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::UnsupportedMediaType)
                            }
                            (__Field::__field38, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::RangeNotSatisfiable)
                            }
                            (__Field::__field39, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::ExpectationFailed)
                            }
                            (__Field::__field40, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::MisdirectedRequest)
                            }
                            (__Field::__field41, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::UnprocessableContent)
                            }
                            (__Field::__field42, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::Locked)
                            }
                            (__Field::__field43, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::FailedDependency)
                            }
                            (__Field::__field44, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::TooEarly)
                            }
                            (__Field::__field45, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::UpgradeRequired)
                            }
                            (__Field::__field46, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::PreconditionRequired)
                            }
                            (__Field::__field47, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::TooManyRequests)
                            }
                            (__Field::__field48, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    StatusCode::RequestHeaderFieldsTooLarge,
                                )
                            }
                            (__Field::__field49, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    StatusCode::UnavailableForLegalReasons,
                                )
                            }
                            (__Field::__field50, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::InternalServerError)
                            }
                            (__Field::__field51, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::NotImplemented)
                            }
                            (__Field::__field52, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::BadGateway)
                            }
                            (__Field::__field53, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::ServiceUnavailable)
                            }
                            (__Field::__field54, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::GatewayTimeout)
                            }
                            (__Field::__field55, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::HTTPVersionNotSupported)
                            }
                            (__Field::__field56, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::VariantAlsoNegotiates)
                            }
                            (__Field::__field57, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::InsufficientStorage)
                            }
                            (__Field::__field58, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(StatusCode::LoopDetected)
                            }
                            (__Field::__field59, __variant) => {
                                _serde::de::VariantAccess::unit_variant(__variant)?;
                                _serde::__private::Ok(
                                    StatusCode::NetworkAuthenticationRequired,
                                )
                            }
                        }
                    }
                }
                #[doc(hidden)]
                const VARIANTS: &'static [&'static str] = &[
                    "Continue",
                    "SwitchingProtocols",
                    "Processing",
                    "EarlyHints",
                    "OK",
                    "Created",
                    "Accepted",
                    "NonAuthoritativeInformation",
                    "NoContent",
                    "ResetContent",
                    "PartialContent",
                    "MultiStatus",
                    "AlreadyReported",
                    "IMUsed",
                    "MultipleChoices",
                    "MovedPermanently",
                    "Found",
                    "SeeOther",
                    "NotModified",
                    "UseProxy",
                    "TemporaryRedirect",
                    "PermanentRedirect",
                    "BadRequest",
                    "Unauthorized",
                    "PaymentRequired",
                    "Forbidden",
                    "NotFound",
                    "MethodNotAllowed",
                    "NotAcceptable",
                    "ProxyAuthenticationRequired",
                    "RequestTimeout",
                    "Conflict",
                    "Gone",
                    "LengthRequired",
                    "PreconditionFailed",
                    "ContentTooLarge",
                    "URITooLong",
                    "UnsupportedMediaType",
                    "RangeNotSatisfiable",
                    "ExpectationFailed",
                    "MisdirectedRequest",
                    "UnprocessableContent",
                    "Locked",
                    "FailedDependency",
                    "TooEarly",
                    "UpgradeRequired",
                    "PreconditionRequired",
                    "TooManyRequests",
                    "RequestHeaderFieldsTooLarge",
                    "UnavailableForLegalReasons",
                    "InternalServerError",
                    "NotImplemented",
                    "BadGateway",
                    "ServiceUnavailable",
                    "GatewayTimeout",
                    "HTTPVersionNotSupported",
                    "VariantAlsoNegotiates",
                    "InsufficientStorage",
                    "LoopDetected",
                    "NetworkAuthenticationRequired",
                ];
                _serde::Deserializer::deserialize_enum(
                    __deserializer,
                    "StatusCode",
                    VARIANTS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<StatusCode>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    #[automatically_derived]
    impl ::core::clone::Clone for StatusCode {
        #[inline]
        fn clone(&self) -> StatusCode {
            *self
        }
    }
    #[automatically_derived]
    impl ::core::marker::Copy for StatusCode {}
    #[automatically_derived]
    impl wasm_bindgen::convert::IntoWasmAbi for StatusCode {
        type Abi = u32;
        #[inline]
        fn into_abi(self) -> u32 {
            self as u32
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::FromWasmAbi for StatusCode {
        type Abi = u32;
        #[inline]
        unsafe fn from_abi(js: u32) -> Self {
            if js == StatusCode::Continue as u32 {
                StatusCode::Continue
            } else if js == StatusCode::SwitchingProtocols as u32 {
                StatusCode::SwitchingProtocols
            } else if js == StatusCode::Processing as u32 {
                StatusCode::Processing
            } else if js == StatusCode::EarlyHints as u32 {
                StatusCode::EarlyHints
            } else if js == StatusCode::OK as u32 {
                StatusCode::OK
            } else if js == StatusCode::Created as u32 {
                StatusCode::Created
            } else if js == StatusCode::Accepted as u32 {
                StatusCode::Accepted
            } else if js == StatusCode::NonAuthoritativeInformation as u32 {
                StatusCode::NonAuthoritativeInformation
            } else if js == StatusCode::NoContent as u32 {
                StatusCode::NoContent
            } else if js == StatusCode::ResetContent as u32 {
                StatusCode::ResetContent
            } else if js == StatusCode::PartialContent as u32 {
                StatusCode::PartialContent
            } else if js == StatusCode::MultiStatus as u32 {
                StatusCode::MultiStatus
            } else if js == StatusCode::AlreadyReported as u32 {
                StatusCode::AlreadyReported
            } else if js == StatusCode::IMUsed as u32 {
                StatusCode::IMUsed
            } else if js == StatusCode::MultipleChoices as u32 {
                StatusCode::MultipleChoices
            } else if js == StatusCode::MovedPermanently as u32 {
                StatusCode::MovedPermanently
            } else if js == StatusCode::Found as u32 {
                StatusCode::Found
            } else if js == StatusCode::SeeOther as u32 {
                StatusCode::SeeOther
            } else if js == StatusCode::NotModified as u32 {
                StatusCode::NotModified
            } else if js == StatusCode::UseProxy as u32 {
                StatusCode::UseProxy
            } else if js == StatusCode::TemporaryRedirect as u32 {
                StatusCode::TemporaryRedirect
            } else if js == StatusCode::PermanentRedirect as u32 {
                StatusCode::PermanentRedirect
            } else if js == StatusCode::BadRequest as u32 {
                StatusCode::BadRequest
            } else if js == StatusCode::Unauthorized as u32 {
                StatusCode::Unauthorized
            } else if js == StatusCode::PaymentRequired as u32 {
                StatusCode::PaymentRequired
            } else if js == StatusCode::Forbidden as u32 {
                StatusCode::Forbidden
            } else if js == StatusCode::NotFound as u32 {
                StatusCode::NotFound
            } else if js == StatusCode::MethodNotAllowed as u32 {
                StatusCode::MethodNotAllowed
            } else if js == StatusCode::NotAcceptable as u32 {
                StatusCode::NotAcceptable
            } else if js == StatusCode::ProxyAuthenticationRequired as u32 {
                StatusCode::ProxyAuthenticationRequired
            } else if js == StatusCode::RequestTimeout as u32 {
                StatusCode::RequestTimeout
            } else if js == StatusCode::Conflict as u32 {
                StatusCode::Conflict
            } else if js == StatusCode::Gone as u32 {
                StatusCode::Gone
            } else if js == StatusCode::LengthRequired as u32 {
                StatusCode::LengthRequired
            } else if js == StatusCode::PreconditionFailed as u32 {
                StatusCode::PreconditionFailed
            } else if js == StatusCode::ContentTooLarge as u32 {
                StatusCode::ContentTooLarge
            } else if js == StatusCode::URITooLong as u32 {
                StatusCode::URITooLong
            } else if js == StatusCode::UnsupportedMediaType as u32 {
                StatusCode::UnsupportedMediaType
            } else if js == StatusCode::RangeNotSatisfiable as u32 {
                StatusCode::RangeNotSatisfiable
            } else if js == StatusCode::ExpectationFailed as u32 {
                StatusCode::ExpectationFailed
            } else if js == StatusCode::MisdirectedRequest as u32 {
                StatusCode::MisdirectedRequest
            } else if js == StatusCode::UnprocessableContent as u32 {
                StatusCode::UnprocessableContent
            } else if js == StatusCode::Locked as u32 {
                StatusCode::Locked
            } else if js == StatusCode::FailedDependency as u32 {
                StatusCode::FailedDependency
            } else if js == StatusCode::TooEarly as u32 {
                StatusCode::TooEarly
            } else if js == StatusCode::UpgradeRequired as u32 {
                StatusCode::UpgradeRequired
            } else if js == StatusCode::PreconditionRequired as u32 {
                StatusCode::PreconditionRequired
            } else if js == StatusCode::TooManyRequests as u32 {
                StatusCode::TooManyRequests
            } else if js == StatusCode::RequestHeaderFieldsTooLarge as u32 {
                StatusCode::RequestHeaderFieldsTooLarge
            } else if js == StatusCode::UnavailableForLegalReasons as u32 {
                StatusCode::UnavailableForLegalReasons
            } else if js == StatusCode::InternalServerError as u32 {
                StatusCode::InternalServerError
            } else if js == StatusCode::NotImplemented as u32 {
                StatusCode::NotImplemented
            } else if js == StatusCode::BadGateway as u32 {
                StatusCode::BadGateway
            } else if js == StatusCode::ServiceUnavailable as u32 {
                StatusCode::ServiceUnavailable
            } else if js == StatusCode::GatewayTimeout as u32 {
                StatusCode::GatewayTimeout
            } else if js == StatusCode::HTTPVersionNotSupported as u32 {
                StatusCode::HTTPVersionNotSupported
            } else if js == StatusCode::VariantAlsoNegotiates as u32 {
                StatusCode::VariantAlsoNegotiates
            } else if js == StatusCode::InsufficientStorage as u32 {
                StatusCode::InsufficientStorage
            } else if js == StatusCode::LoopDetected as u32 {
                StatusCode::LoopDetected
            } else if js == StatusCode::NetworkAuthenticationRequired as u32 {
                StatusCode::NetworkAuthenticationRequired
            } else {
                wasm_bindgen::throw_str("invalid enum value passed")
            }
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionFromWasmAbi for StatusCode {
        #[inline]
        fn is_none(val: &Self::Abi) -> bool {
            *val == 60u32 as u32
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::OptionIntoWasmAbi for StatusCode {
        #[inline]
        fn none() -> Self::Abi {
            60u32 as u32
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::describe::WasmDescribe for StatusCode {
        fn describe() {
            use wasm_bindgen::describe::*;
            inform(ENUM);
            inform(10u32);
            inform(83u32);
            inform(116u32);
            inform(97u32);
            inform(116u32);
            inform(117u32);
            inform(115u32);
            inform(67u32);
            inform(111u32);
            inform(100u32);
            inform(101u32);
            inform(60u32);
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::core::convert::From<StatusCode> for wasm_bindgen::JsValue {
        fn from(value: StatusCode) -> Self {
            wasm_bindgen::JsValue::from_f64((value as u32).into())
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::TryFromJsValue for StatusCode {
        type Error = wasm_bindgen::JsValue;
        fn try_from_js_value(
            value: wasm_bindgen::JsValue,
        ) -> wasm_bindgen::__rt::core::result::Result<
            Self,
            <StatusCode as wasm_bindgen::convert::TryFromJsValue>::Error,
        > {
            use wasm_bindgen::__rt::core::convert::TryFrom;
            let js = f64::try_from(&value)? as u32;
            wasm_bindgen::__rt::core::result::Result::Ok(
                if js == StatusCode::Continue as u32 {
                    StatusCode::Continue
                } else if js == StatusCode::SwitchingProtocols as u32 {
                    StatusCode::SwitchingProtocols
                } else if js == StatusCode::Processing as u32 {
                    StatusCode::Processing
                } else if js == StatusCode::EarlyHints as u32 {
                    StatusCode::EarlyHints
                } else if js == StatusCode::OK as u32 {
                    StatusCode::OK
                } else if js == StatusCode::Created as u32 {
                    StatusCode::Created
                } else if js == StatusCode::Accepted as u32 {
                    StatusCode::Accepted
                } else if js == StatusCode::NonAuthoritativeInformation as u32 {
                    StatusCode::NonAuthoritativeInformation
                } else if js == StatusCode::NoContent as u32 {
                    StatusCode::NoContent
                } else if js == StatusCode::ResetContent as u32 {
                    StatusCode::ResetContent
                } else if js == StatusCode::PartialContent as u32 {
                    StatusCode::PartialContent
                } else if js == StatusCode::MultiStatus as u32 {
                    StatusCode::MultiStatus
                } else if js == StatusCode::AlreadyReported as u32 {
                    StatusCode::AlreadyReported
                } else if js == StatusCode::IMUsed as u32 {
                    StatusCode::IMUsed
                } else if js == StatusCode::MultipleChoices as u32 {
                    StatusCode::MultipleChoices
                } else if js == StatusCode::MovedPermanently as u32 {
                    StatusCode::MovedPermanently
                } else if js == StatusCode::Found as u32 {
                    StatusCode::Found
                } else if js == StatusCode::SeeOther as u32 {
                    StatusCode::SeeOther
                } else if js == StatusCode::NotModified as u32 {
                    StatusCode::NotModified
                } else if js == StatusCode::UseProxy as u32 {
                    StatusCode::UseProxy
                } else if js == StatusCode::TemporaryRedirect as u32 {
                    StatusCode::TemporaryRedirect
                } else if js == StatusCode::PermanentRedirect as u32 {
                    StatusCode::PermanentRedirect
                } else if js == StatusCode::BadRequest as u32 {
                    StatusCode::BadRequest
                } else if js == StatusCode::Unauthorized as u32 {
                    StatusCode::Unauthorized
                } else if js == StatusCode::PaymentRequired as u32 {
                    StatusCode::PaymentRequired
                } else if js == StatusCode::Forbidden as u32 {
                    StatusCode::Forbidden
                } else if js == StatusCode::NotFound as u32 {
                    StatusCode::NotFound
                } else if js == StatusCode::MethodNotAllowed as u32 {
                    StatusCode::MethodNotAllowed
                } else if js == StatusCode::NotAcceptable as u32 {
                    StatusCode::NotAcceptable
                } else if js == StatusCode::ProxyAuthenticationRequired as u32 {
                    StatusCode::ProxyAuthenticationRequired
                } else if js == StatusCode::RequestTimeout as u32 {
                    StatusCode::RequestTimeout
                } else if js == StatusCode::Conflict as u32 {
                    StatusCode::Conflict
                } else if js == StatusCode::Gone as u32 {
                    StatusCode::Gone
                } else if js == StatusCode::LengthRequired as u32 {
                    StatusCode::LengthRequired
                } else if js == StatusCode::PreconditionFailed as u32 {
                    StatusCode::PreconditionFailed
                } else if js == StatusCode::ContentTooLarge as u32 {
                    StatusCode::ContentTooLarge
                } else if js == StatusCode::URITooLong as u32 {
                    StatusCode::URITooLong
                } else if js == StatusCode::UnsupportedMediaType as u32 {
                    StatusCode::UnsupportedMediaType
                } else if js == StatusCode::RangeNotSatisfiable as u32 {
                    StatusCode::RangeNotSatisfiable
                } else if js == StatusCode::ExpectationFailed as u32 {
                    StatusCode::ExpectationFailed
                } else if js == StatusCode::MisdirectedRequest as u32 {
                    StatusCode::MisdirectedRequest
                } else if js == StatusCode::UnprocessableContent as u32 {
                    StatusCode::UnprocessableContent
                } else if js == StatusCode::Locked as u32 {
                    StatusCode::Locked
                } else if js == StatusCode::FailedDependency as u32 {
                    StatusCode::FailedDependency
                } else if js == StatusCode::TooEarly as u32 {
                    StatusCode::TooEarly
                } else if js == StatusCode::UpgradeRequired as u32 {
                    StatusCode::UpgradeRequired
                } else if js == StatusCode::PreconditionRequired as u32 {
                    StatusCode::PreconditionRequired
                } else if js == StatusCode::TooManyRequests as u32 {
                    StatusCode::TooManyRequests
                } else if js == StatusCode::RequestHeaderFieldsTooLarge as u32 {
                    StatusCode::RequestHeaderFieldsTooLarge
                } else if js == StatusCode::UnavailableForLegalReasons as u32 {
                    StatusCode::UnavailableForLegalReasons
                } else if js == StatusCode::InternalServerError as u32 {
                    StatusCode::InternalServerError
                } else if js == StatusCode::NotImplemented as u32 {
                    StatusCode::NotImplemented
                } else if js == StatusCode::BadGateway as u32 {
                    StatusCode::BadGateway
                } else if js == StatusCode::ServiceUnavailable as u32 {
                    StatusCode::ServiceUnavailable
                } else if js == StatusCode::GatewayTimeout as u32 {
                    StatusCode::GatewayTimeout
                } else if js == StatusCode::HTTPVersionNotSupported as u32 {
                    StatusCode::HTTPVersionNotSupported
                } else if js == StatusCode::VariantAlsoNegotiates as u32 {
                    StatusCode::VariantAlsoNegotiates
                } else if js == StatusCode::InsufficientStorage as u32 {
                    StatusCode::InsufficientStorage
                } else if js == StatusCode::LoopDetected as u32 {
                    StatusCode::LoopDetected
                } else if js == StatusCode::NetworkAuthenticationRequired as u32 {
                    StatusCode::NetworkAuthenticationRequired
                } else {
                    return wasm_bindgen::__rt::core::result::Result::Err(value)
                },
            )
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::describe::WasmDescribeVector for StatusCode {
        fn describe_vector() {
            use wasm_bindgen::describe::*;
            inform(VECTOR);
            <wasm_bindgen::JsValue as wasm_bindgen::describe::WasmDescribe>::describe();
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::VectorIntoWasmAbi for StatusCode {
        type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::IntoWasmAbi>::Abi;
        fn vector_into_abi(
            vector: wasm_bindgen::__rt::alloc::boxed::Box<[StatusCode]>,
        ) -> Self::Abi {
            wasm_bindgen::convert::js_value_vector_into_abi(vector)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::convert::VectorFromWasmAbi for StatusCode {
        type Abi = <wasm_bindgen::__rt::alloc::boxed::Box<
            [wasm_bindgen::JsValue],
        > as wasm_bindgen::convert::FromWasmAbi>::Abi;
        unsafe fn vector_from_abi(
            js: Self::Abi,
        ) -> wasm_bindgen::__rt::alloc::boxed::Box<[StatusCode]> {
            wasm_bindgen::convert::js_value_vector_from_abi(js)
        }
    }
    #[automatically_derived]
    impl wasm_bindgen::__rt::VectorIntoJsValue for StatusCode {
        fn vector_into_jsvalue(
            vector: wasm_bindgen::__rt::alloc::boxed::Box<[StatusCode]>,
        ) -> wasm_bindgen::JsValue {
            wasm_bindgen::__rt::js_value_vector_into_jsvalue(vector)
        }
    }
    impl StatusCode {
        pub const fn to_number(&self) -> u16 {
            match self {
                Self::Continue => 100,
                Self::SwitchingProtocols => 101,
                Self::Processing => 102,
                Self::EarlyHints => 103,
                Self::OK => 200,
                Self::Created => 201,
                Self::Accepted => 202,
                Self::NonAuthoritativeInformation => 203,
                Self::NoContent => 204,
                Self::ResetContent => 205,
                Self::PartialContent => 206,
                Self::MultiStatus => 207,
                Self::AlreadyReported => 208,
                Self::IMUsed => 226,
                Self::MultipleChoices => 300,
                Self::MovedPermanently => 301,
                Self::Found => 302,
                Self::SeeOther => 303,
                Self::NotModified => 304,
                Self::UseProxy => 305,
                Self::TemporaryRedirect => 307,
                Self::PermanentRedirect => 308,
                Self::BadRequest => 400,
                Self::Unauthorized => 401,
                Self::PaymentRequired => 402,
                Self::Forbidden => 403,
                Self::NotFound => 404,
                Self::MethodNotAllowed => 405,
                Self::NotAcceptable => 406,
                Self::ProxyAuthenticationRequired => 407,
                Self::RequestTimeout => 408,
                Self::Conflict => 409,
                Self::Gone => 410,
                Self::LengthRequired => 411,
                Self::PreconditionFailed => 412,
                Self::ContentTooLarge => 413,
                Self::URITooLong => 414,
                Self::UnsupportedMediaType => 415,
                Self::RangeNotSatisfiable => 416,
                Self::ExpectationFailed => 417,
                Self::MisdirectedRequest => 421,
                Self::UnprocessableContent => 422,
                Self::Locked => 423,
                Self::FailedDependency => 424,
                Self::TooEarly => 425,
                Self::UpgradeRequired => 426,
                Self::PreconditionRequired => 428,
                Self::TooManyRequests => 429,
                Self::RequestHeaderFieldsTooLarge => 431,
                Self::UnavailableForLegalReasons => 451,
                Self::InternalServerError => 500,
                Self::NotImplemented => 501,
                Self::BadGateway => 502,
                Self::ServiceUnavailable => 503,
                Self::GatewayTimeout => 504,
                Self::HTTPVersionNotSupported => 505,
                Self::VariantAlsoNegotiates => 506,
                Self::InsufficientStorage => 507,
                Self::LoopDetected => 508,
                Self::NetworkAuthenticationRequired => 511,
            }
        }
    }
}
