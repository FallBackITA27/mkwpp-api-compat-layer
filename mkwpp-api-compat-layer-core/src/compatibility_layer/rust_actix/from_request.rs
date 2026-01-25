use std::convert::Infallible;

use actix_web::FromRequest;

use crate::{
    common_types::NoData,
    endpoint::{Endpoint, cups::GetCups, scores::charts::GetCharts},
    error::FinalErrorResponse,
};

impl FromRequest for NoData {
    type Error = Infallible;
    type Future = core::future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        _req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        core::future::ready(Ok(Self))
    }
}

impl FromRequest for <GetCharts as Endpoint>::OutputStruct {
    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let data = req.query_string().split(&['?', '&']).fold(
            (None, None, None, None, None, None),
            |mut acc, next| {
                let mut split = next.split('=');
                match split.next() {
                    Some("id") => acc.0 = split.next().map(FromStr::<i32>::from_str).flatten(),
                    Some("cat") => acc.1 = split.next(),
                    Some("lap") => acc.2 = split.next(),
                    Some("dat") => acc.3 = split.next(),
                    Some("reg") => acc.4 = split.next(),
                    Some("lim") => acc.5 = split.next(),
                    _ => (),
                };
                acc
            },
        );

        Ok(Self {
            id: 9,
            category: Category::Normal,
            is_lap: false,
            max_date: UtcTimestamp::from(10),
            region_id: 0,
            limit: Limit::from(10),
        })
    }
}
