use super::Kalshi;
use crate::{kalshi_error::*, LoggedIn, LoggedOut};
use serde::{Deserialize, Serialize};

impl<'a> Kalshi<LoggedOut> {
    /// Asynchronously logs a user into the Kalshi exchange.
    ///
    /// This method sends a POST request to the Kalshi exchange's login endpoint with the user's credentials.
    /// On successful authentication, it updates the current session's token and member ID.
    ///
    /// # Arguments
    /// * `user` - A string slice representing the user's email.
    /// * `password` - A string slice representing the user's password.
    ///
    /// # Returns
    /// - `Ok(Kalshi<LoggedIn>)`: A Kalshi instance with the user logged in.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request or response parsing.
    ///
    /// # Example
    /// ```
    /// let kalshi_instance = kalshi_instance.login("johndoe@example.com", "example_password").await?;
    /// ```
    pub async fn login(self, user: &str, password: &str) -> Result<Kalshi<LoggedIn>, KalshiError> {
        let login_url: &str = &format!("{}/login", self.base_url);

        let login_payload = LoginPayload {
            email: user.to_string(),
            password: password.to_string(),
        };

        let result: LoginResponse = self
            .client
            .post(login_url)
            .json(&login_payload)
            .send()
            .await?
            .json()
            .await?;

        let logged_in = LoggedIn {
            curr_token: format!("Bearer {}", result.token),
            member_id: result.member_id,
        };

        Ok(Kalshi {
            base_url: self.base_url.clone(),
            client: self.client.clone(),
            state: logged_in,
        })
    }
}

impl<'a> Kalshi<LoggedIn> {
    /// Asynchronously logs a user out of the Kalshi exchange.
    ///
    /// Sends a POST request to the Kalshi exchange's logout endpoint. This method
    /// should be called to properly terminate the session initiated by `login`.
    ///
    /// # Returns
    /// - `Ok(Kalshi<LoggedOut>)`: A Kalshi instance with the user logged out.
    /// - `Err(KalshiError)`: Error in case of a failure in the HTTP request.
    ///
    /// # Examples
    /// ```
    /// kalshi_instance.logout().await?;
    /// ```
    pub async fn logout(&self) -> Result<Kalshi<LoggedOut>, KalshiError> {
        let logout_url: &str = &format!("{}/logout", self.base_url);

        self.client
            .post(logout_url)
            .header("Authorization", self.state.curr_token.clone())
            .header("content-type", "application/json".to_string())
            .send()
            .await?;

        Ok(Kalshi {
            base_url: self.base_url.clone(),
            client: self.client.clone(),
            state: LoggedOut,
        })
    }
}

// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    member_id: String,
    token: String,
}
// used in login method
#[derive(Debug, Serialize, Deserialize)]
struct LoginPayload {
    email: String,
    password: String,
}
