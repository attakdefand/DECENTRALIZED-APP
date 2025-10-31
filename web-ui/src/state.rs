//! Global application state management
//!
//! This module provides centralized state management using Yew's context API.

use std::rc::Rc;
use yew::prelude::*;
use serde::{Deserialize, Serialize};

use crate::services::auth::AuthToken;

/// User authentication state
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum AuthState {
    /// User is not authenticated
    Unauthenticated,
    /// User is authenticated with token
    Authenticated(AuthToken),
    /// Authentication is in progress
    Authenticating,
}

/// Global application state
#[derive(Clone, PartialEq)]
pub struct AppState {
    /// Current authentication state
    pub auth: AuthState,
    /// Loading state indicator
    pub loading: bool,
    /// Error message if any
    pub error: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            auth: AuthState::Unauthenticated,
            loading: false,
            error: None,
        }
    }
}

/// Actions that can be dispatched to modify state
pub enum AppAction {
    /// Set authentication state
    SetAuth(AuthState),
    /// Set loading state
    SetLoading(bool),
    /// Set error message
    SetError(Option<String>),
    /// Clear error
    ClearError,
}

impl Reducible for AppState {
    type Action = AppAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut state = (*self).clone();
        
        match action {
            AppAction::SetAuth(auth) => {
                state.auth = auth;
            }
            AppAction::SetLoading(loading) => {
                state.loading = loading;
            }
            AppAction::SetError(error) => {
                state.error = error;
            }
            AppAction::ClearError => {
                state.error = None;
            }
        }
        
        Rc::new(state)
    }
}

/// Context type for the app state
pub type AppStateContext = UseReducerHandle<AppState>;
