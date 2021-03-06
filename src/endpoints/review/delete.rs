//Anything related to DELETE requests for review app and it's properties goes here.
use super::{ReviewApp, ReviewAppConfig};

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Review App Delete
///
/// Delete an existing review app
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-delete)
/// 
/// # Example:
///
/// ReviewAppDelete takes one required parameter, review_id, and returns the deleted [`ReviewApp`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReviewAppDelete::new("REVIEW_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.ReviewApp.html
pub struct ReviewAppDelete<'a> {
    /// review_id is the unique identifier.
    pub review_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppDelete<'a> {
    pub fn new(review_id: &'a str) -> ReviewAppDelete<'a> {
        ReviewAppDelete { review_id }
    }
}

impl<'a> HerokuEndpoint<ReviewApp> for ReviewAppDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("review-apps/{}", self.review_id)
    }
}

/// Review App Configuration Delete
///
/// Disable review apps for a pipeline
///
/// [See Heroku documentation for more information about this endpoint](https://devcenter.heroku.com/articles/platform-api-reference#review-app-configuration-delete)
/// 
/// # Example:
///
/// ReviewAppConfigDelete takes two required parameters, pipeline_id, and returns the deleted [`ReviewAppConfig`][response].
/// ```rust
/// use heroku_rs::prelude::*;
///#    let api_client = HttpApiClient::create("API_KEY").unwrap();
///
/// let response = api_client.request(&ReviewAppConfigDelete::new("PIPELINE_ID"));
///
///match response {
///     Ok(success) => println!("Success: {:#?}", success),
///     Err(e) => println!("Error: {}", e),
///}
///
/// ```
/// See how to create the Heroku [`api_client`][httpApiClientConfig].
///
/// [httpApiClientConfig]: ../../../framework/struct.HttpApiClient.html
/// [response]: ../struct.ReviewAppConfig.html
pub struct ReviewAppConfigDelete<'a> {
    /// pipeline_id is the unique identifier.
    pub pipeline_id: &'a str,
}

#[cfg(feature = "builder")]
impl<'a> ReviewAppConfigDelete<'a> {
    pub fn new(pipeline_id: &'a str) -> ReviewAppConfigDelete<'a> {
        ReviewAppConfigDelete { pipeline_id }
    }
}

impl<'a> HerokuEndpoint<ReviewAppConfig> for ReviewAppConfigDelete<'a> {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("pipelines/{}/review-app-config", self.pipeline_id)
    }
}
