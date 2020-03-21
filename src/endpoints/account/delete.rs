//Anything related to DELETE requests for account and it's properties goes here.
use super::Account;

use crate::framework::endpoint::{HerokuEndpoint, Method};

/// Account Delete.
/// Delete account. Note that this action cannot be undone.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-delete
pub struct AccountDelete {}

impl HerokuEndpoint<Account> for AccountDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("account")
    }
}

/// User Account Delete.
/// Delete user account. Note that this action cannot be undone.
/// https://devcenter.heroku.com/articles/platform-api-reference#account-delete-by-user
pub struct UserAccountDelete {
    /// identifier can be the account email or id.
    pub account_identifier: String,
}

impl HerokuEndpoint<Account> for UserAccountDelete {
    fn method(&self) -> Method {
        Method::Delete
    }
    fn path(&self) -> String {
        format!("users/{}", self.account_identifier)
    }
}