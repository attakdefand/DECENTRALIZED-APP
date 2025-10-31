//! Toast notification component
//!
//! Provides accessible toast notifications for user feedback with:
//! - Success, error, warning, and info variants
//! - Auto-dismiss with configurable duration
//! - Accessible ARIA attributes
//! - Keyboard dismissal support

use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use gloo_timers::future::TimeoutFuture;

/// Toast notification type
#[derive(Clone, PartialEq)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

/// Toast notification data
#[derive(Clone, PartialEq)]
pub struct ToastData {
    pub id: String,
    pub message: String,
    pub toast_type: ToastType,
    pub duration: u32, // milliseconds
}

/// Toast props
#[derive(Properties, PartialEq)]
pub struct ToastProps {
    pub toast: ToastData,
    pub on_dismiss: Callback<String>,
}

/// Single toast notification component
#[function_component(Toast)]
pub fn toast(props: &ToastProps) -> Html {
    let visible = use_state(|| true);
    
    // Auto-dismiss after duration
    {
        let visible = visible.clone();
        let on_dismiss = props.on_dismiss.clone();
        let toast_id = props.toast.id.clone();
        let duration = props.toast.duration;
        
        use_effect_with((), move |_| {
            if duration > 0 {
                spawn_local(async move {
                    TimeoutFuture::new(duration).await;
                    visible.set(false);
                    on_dismiss.emit(toast_id);
                });
            }
            || ()
        });
    }
    
    if !*visible {
        return html! {};
    }
    
    let (bg_color, icon_color, icon_path, role) = match props.toast.toast_type {
        ToastType::Success => (
            "bg-green-50",
            "text-green-400",
            "M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z",
            "status"
        ),
        ToastType::Error => (
            "bg-red-50",
            "text-red-400",
            "M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z",
            "alert"
        ),
        ToastType::Warning => (
            "bg-yellow-50",
            "text-yellow-400",
            "M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z",
            "alert"
        ),
        ToastType::Info => (
            "bg-blue-50",
            "text-blue-400",
            "M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z",
            "status"
        ),
    };
    
    let on_dismiss_click = {
        let visible = visible.clone();
        let on_dismiss = props.on_dismiss.clone();
        let toast_id = props.toast.id.clone();
        
        Callback::from(move |_| {
            visible.set(false);
            on_dismiss.emit(toast_id.clone());
        })
    };
    
    let on_keydown = {
        let on_dismiss_click = on_dismiss_click.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            // Dismiss on Escape key
            if e.key() == "Escape" {
                on_dismiss_click.emit(());
            }
        })
    };
    
    html! {
        <div
            class={format!("rounded-md p-4 {} mb-4 shadow-lg", bg_color)}
            role={role}
            aria-live="polite"
            aria-atomic="true"
            tabindex="0"
            onkeydown={on_keydown}
        >
            <div class="flex">
                <div class="flex-shrink-0">
                    <svg class={format!("h-5 w-5 {}", icon_color)} xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                        <path fill-rule="evenodd" d={icon_path} clip-rule="evenodd" />
                    </svg>
                </div>
                <div class="ml-3 flex-1">
                    <p class="text-sm font-medium text-gray-900">
                        {&props.toast.message}
                    </p>
                </div>
                <div class="ml-auto pl-3">
                    <div class="-mx-1.5 -my-1.5">
                        <button
                            type="button"
                            class="inline-flex rounded-md p-1.5 text-gray-400 hover:bg-gray-100 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500"
                            onclick={on_dismiss_click}
                            aria-label="Dismiss notification"
                        >
                            <span class="sr-only">{"Dismiss"}</span>
                            <svg class="h-5 w-5" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" aria-hidden="true">
                                <path fill-rule="evenodd" d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z" clip-rule="evenodd" />
                            </svg>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}

/// Toast container props
#[derive(Properties, PartialEq)]
pub struct ToastContainerProps {
    pub toasts: Vec<ToastData>,
    pub on_dismiss: Callback<String>,
}

/// Toast container component
#[function_component(ToastContainer)]
pub fn toast_container(props: &ToastContainerProps) -> Html {
    html! {
        <div
            class="fixed top-4 right-4 z-50 w-96 max-w-full"
            aria-label="Notifications"
            role="region"
        >
            {for props.toasts.iter().map(|toast| {
                html! {
                    <Toast
                        key={toast.id.clone()}
                        toast={toast.clone()}
                        on_dismiss={props.on_dismiss.clone()}
                    />
                }
            })}
        </div>
    }
}

/// Toast service for managing notifications
pub struct ToastService {
    toasts: Vec<ToastData>,
    next_id: usize,
}

impl ToastService {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            next_id: 0,
        }
    }
    
    /// Add a success toast
    pub fn success(&mut self, message: impl Into<String>) -> String {
        self.add_toast(message.into(), ToastType::Success, 3000)
    }
    
    /// Add an error toast
    pub fn error(&mut self, message: impl Into<String>) -> String {
        self.add_toast(message.into(), ToastType::Error, 5000)
    }
    
    /// Add a warning toast
    pub fn warning(&mut self, message: impl Into<String>) -> String {
        self.add_toast(message.into(), ToastType::Warning, 4000)
    }
    
    /// Add an info toast
    pub fn info(&mut self, message: impl Into<String>) -> String {
        self.add_toast(message.into(), ToastType::Info, 3000)
    }
    
    /// Add a toast with custom duration
    pub fn add_toast(&mut self, message: String, toast_type: ToastType, duration: u32) -> String {
        let id = format!("toast-{}", self.next_id);
        self.next_id += 1;
        
        self.toasts.push(ToastData {
            id: id.clone(),
            message,
            toast_type,
            duration,
        });
        
        id
    }
    
    /// Remove a toast by ID
    pub fn remove_toast(&mut self, id: &str) {
        self.toasts.retain(|t| t.id != id);
    }
    
    /// Get all toasts
    pub fn get_toasts(&self) -> &[ToastData] {
        &self.toasts
    }
    
    /// Clear all toasts
    pub fn clear(&mut self) {
        self.toasts.clear();
    }
}

impl Default for ToastService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    
    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn test_toast_service_creation() {
        let service = ToastService::new();
        assert_eq!(service.get_toasts().len(), 0);
    }
    
    #[wasm_bindgen_test]
    fn test_toast_service_add_success() {
        let mut service = ToastService::new();
        let id = service.success("Operation successful");
        
        assert_eq!(service.get_toasts().len(), 1);
        assert_eq!(service.get_toasts()[0].id, id);
        assert_eq!(service.get_toasts()[0].message, "Operation successful");
    }
    
    #[wasm_bindgen_test]
    fn test_toast_service_remove() {
        let mut service = ToastService::new();
        let id = service.error("Error occurred");
        
        assert_eq!(service.get_toasts().len(), 1);
        
        service.remove_toast(&id);
        assert_eq!(service.get_toasts().len(), 0);
    }
    
    #[wasm_bindgen_test]
    fn test_toast_service_multiple_toasts() {
        let mut service = ToastService::new();
        
        service.success("Success 1");
        service.error("Error 1");
        service.warning("Warning 1");
        service.info("Info 1");
        
        assert_eq!(service.get_toasts().len(), 4);
    }
    
    #[wasm_bindgen_test]
    fn test_toast_service_clear() {
        let mut service = ToastService::new();
        
        service.success("Success");
        service.error("Error");
        
        assert_eq!(service.get_toasts().len(), 2);
        
        service.clear();
        assert_eq!(service.get_toasts().len(), 0);
    }
}
