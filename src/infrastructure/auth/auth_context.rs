use crate::domain::models::user::UserProfile;
use gloo_net::http::Request;
use leptos::prelude::*;

#[derive(Clone)]
pub struct AuthState {
    pub access_token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<UserProfile>>,
    pub is_loading: RwSignal<bool>,
}

impl AuthState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            access_token: RwSignal::new(None),
            user: RwSignal::new(None),
            is_loading: RwSignal::new(false),
        }
    }

    pub fn clear(&self) {
        self.access_token.set(None);
        self.user.set(None);
        self.clear_refresh_token();
    }

    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        self.access_token.get().is_some()
    }

    pub fn set_auth_data(&self, access_token: String, user: UserProfile) {
        self.access_token.set(Some(access_token));
        self.user.set(Some(user));
    }

    pub fn store_refresh_token(&self, refresh_token: &str) {
        if let Some(window) = leptos::web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.set_item("refresh_token", refresh_token);
            }
        }
    }

    #[must_use]
    pub fn get_refresh_token(&self) -> Option<String> {
        leptos::web_sys::window()?
            .local_storage()
            .ok()??
            .get_item("refresh_token")
            .ok()?
    }

    pub fn clear_refresh_token(&self) {
        if let Some(window) = leptos::web_sys::window() {
            if let Ok(Some(storage)) = window.local_storage() {
                let _ = storage.remove_item("refresh_token");
            }
        }
    }

    pub async fn get_valid_access_token(&self) -> Option<String> {
        // Check if current token is still valid
        if let Some(token) = self.access_token.get() {
            // You might want to add token expiry checking here
            return Some(token);
        }

        // Try to refresh the token
        self.refresh_access_token().await
    }

    pub async fn refresh_access_token(&self) -> Option<String> {
        use crate::domain::models::user::{RefreshTokenRequest, RefreshTokenResponse};
        use gloo_net::http::Request;

        let refresh_token = self.get_refresh_token()?;

        self.is_loading.set(true);

        let request = RefreshTokenRequest { refresh_token };

        let result = async {
            let response = Request::post("/api/auth/refresh")
                .json(&request)
                .ok()?
                .send()
                .await
                .ok()?;

            if response.ok() {
                let refresh_response: RefreshTokenResponse = response.json().await.ok()?;

                // Store new tokens
                self.access_token
                    .set(Some(refresh_response.access_token.clone()));
                self.store_refresh_token(&refresh_response.refresh_token);

                Some(refresh_response.access_token)
            } else {
                // Refresh failed, clear everything
                self.clear();
                None
            }
        }
        .await;

        self.is_loading.set(false);
        result
    }

    pub async fn logout(&self) {
        // Optional: Call logout endpoint to invalidate refresh token on server
        if let Some(refresh_token) = self.get_refresh_token() {
            if let Ok(response) = Request::post("/api/auth/logout")
                .json(&serde_json::json!({ "refresh_token": refresh_token }))
            {
                response.json::<serde_json::Value>().await.ok(); // Handle response if needed
                                                                 // Successfully logged out
            } else {
                // Handle error if needed
            }
        }

        self.clear();
    }
}

impl Default for AuthState {
    fn default() -> Self {
        Self::new()
    }
}
