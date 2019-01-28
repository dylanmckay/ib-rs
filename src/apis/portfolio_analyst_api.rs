/* 
 * Client Portal Web API
 *
 * Production version of the Client Portal Web API
 *
 * OpenAPI spec version: 1.0.0
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::rc::Rc;
use std::borrow::Borrow;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct PortfolioAnalystApiClient<C: hyper::client::Connect> {
    configuration: Rc<configuration::Configuration<C>>,
}

impl<C: hyper::client::Connect> PortfolioAnalystApiClient<C> {
    pub fn new(configuration: Rc<configuration::Configuration<C>>) -> PortfolioAnalystApiClient<C> {
        PortfolioAnalystApiClient {
            configuration: configuration,
        }
    }
}

pub trait PortfolioAnalystApi {
    fn pa_performance_post(&self, body: ::models::Body2) -> Box<Future<Item = ::models::Performance, Error = Error>>;
    fn pa_summary_post(&self, body: ::models::Body3) -> Box<Future<Item = ::models::Summary, Error = Error>>;
}


impl<C: hyper::client::Connect>PortfolioAnalystApi for PortfolioAnalystApiClient<C> {
    fn pa_performance_post(&self, body: ::models::Body2) -> Box<Future<Item = ::models::Performance, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/pa/performance", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::Performance, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

    fn pa_summary_post(&self, body: ::models::Body3) -> Box<Future<Item = ::models::Summary, Error = Error>> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::Post;

        let uri_str = format!("{}/pa/summary", configuration.base_path);

        let uri = uri_str.parse();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut req = hyper::Request::new(method, uri.unwrap());


        let serialized = serde_json::to_string(&body).unwrap();
        req.headers_mut().set(hyper::header::ContentType::json());
        req.headers_mut().set(hyper::header::ContentLength(serialized.len() as u64));
        req.set_body(serialized);

        // send request
        Box::new(
            configuration.client.request(req).and_then(|res| { res.body().concat2() })
            .map_err(|e| Error::from(e))
            .and_then(|body| {
                let parsed: Result<::models::Summary, _> = serde_json::from_slice(&body);
                parsed.map_err(|e| Error::from(e))
            }).map_err(|e| Error::from(e))
        )
    }

}
