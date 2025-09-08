use super::{from_input::InputFromActix, to_route::ToActixRoute};
use crate::{
    endpoint::{
        Endpoint, Root, Scope,
        cups::{CupsScope, GetCups},
        tracks::{GetTracks, TracksScope},
    },
    error::PPResult,
};
use actix_web::web;
use paste::paste;

macro_rules! to_actix_scope_macro {
    ($scope_name:ident) => {
        impl $scope_name {
            pub fn to_actix_scope() -> actix_web::Scope {
                web::scope(Self::PATH)
            }
        }
    };
    ($scope_name:ident; $($route:ident),*) => {
        impl $scope_name {
            paste! {
                pub fn to_actix_scope(
                    $([< $route:snake _handler >]:
                        impl AsyncFn(<$route as Endpoint>::InputStruct)
                            -> PPResult<<$route as Endpoint>::OutputStruct> + 'static
                    ),*
                ) -> actix_web::Scope
                    where
                        $(
                            <$route as Endpoint>::InputStruct: InputFromActix,
                            <$route as Endpoint>::OutputStruct: serde::Serialize
                        ),*
                {
                    web::scope(Self::PATH)
                    $(
                        .route(
                            $route::PATH, $route::to_actix_route([< $route:snake _handler >])
                        )
                    )*
                }
            }
        }
    };
}

to_actix_scope_macro!(Root);
to_actix_scope_macro!(CupsScope; GetCups);
to_actix_scope_macro!(TracksScope; GetTracks);
