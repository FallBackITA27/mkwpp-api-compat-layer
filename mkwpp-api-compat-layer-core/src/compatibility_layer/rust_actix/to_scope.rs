use actix_web::{FromRequest, Handler, Responder, Route, web};
use paste::paste;
use crate::endpoint::{
    Endpoint, Root, Scope,
    cups::{CupsScope, GetCups},
    players::PlayersScope,
};
use super::to_route::ToActixRoute;

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
                pub fn to_actix_scope<F, Args>($([< $route:snake _handler >]: F,),*) -> actix_web::Scope 
                where
                    F: Handler<Args>,
                    Args: FromRequest + 'static,
                    F::Output: Responder + 'static
                {
                        web::scope(Self::PATH)
                        $( .route($route::PATH, $route::to_actix_route([< $route:snake _handler >])))*
                }
            }
        }
    };
    ($scope_name:ident; $($route:ident),*; $($scope:ident),*) => {
        impl $scope_name {
            paste! { 
                pub fn to_actix_scope<F, Args>($([< $route:snake _handler >]: F,),*) -> actix_web::Scope 
                where
                    F: Handler<Args>,
                    Args: FromRequest + 'static,
                    F::Output: Responder + 'static
                {
                        web::scope(Self::PATH)
                        $( .route($route::PATH, $route::to_actix_route([< $route:snake _handler >])))*
                }
            }
        }
    }
}

to_actix_scope_macro!(Root);
to_actix_scope_macro!(CupsScope; GetCups);
