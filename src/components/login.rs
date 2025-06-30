use crate::domain::models::user::{AuthResponse, CreateUserRequest, LoginRequest, UserProfile};
use gloo_net::http::Request;
use leptos::web_sys;
use leptos::{prelude::*, task::spawn_local};
use leptos_router::NavigateOptions;
use leptos_router::hooks::use_navigate;

#[derive(Clone, Default)]
pub struct AuthState {
    pub access_token: RwSignal<Option<String>>,
    pub user: RwSignal<Option<UserProfile>>,
}

impl AuthState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            access_token: RwSignal::new(None),
            user: RwSignal::new(None),
        }
    }

    pub fn clear(&self) {
        self.access_token.set(None);
        self.user.set(None);
    }
    #[must_use]
    pub fn is_authenticated(&self) -> bool {
        self.access_token.get().is_some()
    }
}

#[component]
#[allow(clippy::must_use_candidate)]
#[allow(clippy::too_many_lines)]
pub fn AuthPage() -> impl IntoView {
    let (is_login, set_is_login) = signal(true);
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);

    view! {
        <div class="min-h-screen bg-gradient-to-br from-indigo-100 via-white to-cyan-100 relative overflow-hidden">
            // Background decorative elements
            <div
                class="absolute inset-0 opacity-40"
                style="background-image: url('data:image/svg+xml,%3Csvg width=60 height=60 viewBox=%220 0 60 60%22 xmlns=%22http://www.w3.org/2000/svg%22%3E%3Cg fill=%22none%22 fill-rule=%22evenodd%22%3E%3Cg fill=%22%239C92AC%22 fill-opacity=%220.05%22%3E%3Ccircle cx=%2230%22 cy=%2230%22 r=%222%22/%3E%3C/g%3E%3C/g%3E%3C/svg%3E')"
            ></div>

            // Floating geometric shapes
            <div class="absolute top-20 left-10 w-72 h-72 bg-gradient-to-br from-purple-400 to-pink-400 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob"></div>
            <div class="absolute top-40 right-10 w-72 h-72 bg-gradient-to-br from-yellow-400 to-orange-400 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-2000"></div>
            <div class="absolute -bottom-8 left-20 w-72 h-72 bg-gradient-to-br from-cyan-400 to-blue-400 rounded-full mix-blend-multiply filter blur-xl opacity-20 animate-blob animation-delay-4000"></div>

            <div class="relative flex items-center justify-center min-h-screen p-4">
                <div class="w-full max-w-md">
                    // Enhanced header section
                    <div class="text-center mb-8">
                        <div class="relative mx-auto h-20 w-20 mb-6">
                            <div class="absolute inset-0 bg-gradient-to-r from-violet-600 via-blue-600 to-cyan-600 rounded-2xl rotate-6 opacity-80"></div>
                            <div class="relative bg-gradient-to-r from-violet-600 to-blue-600 rounded-2xl h-full w-full flex items-center justify-center shadow-xl">
                                <svg
                                    class="h-10 w-10 text-white"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2.5"
                                        d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"
                                    />
                                </svg>
                            </div>
                        </div>
                        <h1 class="text-3xl font-bold bg-gradient-to-r from-gray-900 via-gray-800 to-gray-900 bg-clip-text text-transparent mb-3">
                            "Finance Tracker"
                        </h1>
                        <p class="text-gray-600 text-base leading-relaxed">
                            {move || {
                                if is_login.get() {
                                    "Welcome back! Please sign in to continue managing your finances"
                                } else {
                                    "Join thousands of users tracking their financial goals successfully"
                                }
                            }}
                        </p>
                    </div>

                    // Enhanced main card with glassmorphism
                    <div class="backdrop-blur-xl bg-white/90 border border-white/20 shadow-2xl rounded-3xl p-8 relative">
                        // Subtle inner glow
                        <div class="absolute inset-0 rounded-3xl bg-gradient-to-r from-violet-500/5 via-blue-500/5 to-cyan-500/5"></div>

                        <div class="relative">
                            // Enhanced tab switcher
                            <div class="flex bg-gray-50/80 backdrop-blur-sm rounded-2xl p-1.5 mb-8 relative">
                                <div class=move || {
                                    let base = "absolute top-1.5 bottom-1.5 rounded-xl bg-white shadow-lg transition-all duration-300 ease-out";
                                    if is_login.get() {
                                        format!("{base} left-1.5 right-1/2 mr-0.75")
                                    } else {
                                        format!("{base} right-1.5 left-1/2 ml-0.75")
                                    }
                                }></div>

                                <button
                                    type="button"
                                    class=move || {
                                        let base = "relative flex-1 py-3 px-4 text-sm font-semibold rounded-xl transition-all duration-300 z-10";
                                        if is_login.get() {
                                            format!("{base} text-violet-600")
                                        } else {
                                            format!("{base} text-gray-500 hover:text-gray-700")
                                        }
                                    }
                                    on:click=move |_| {
                                        set_is_login.set(true);
                                        set_error.set(None);
                                    }
                                >
                                    <div class="flex items-center justify-center space-x-2">
                                        <svg
                                            class="h-4 w-4"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"
                                            />
                                        </svg>
                                        <span>"Sign In"</span>
                                    </div>
                                </button>
                                <button
                                    type="button"
                                    class=move || {
                                        let base = "relative flex-1 py-3 px-4 text-sm font-semibold rounded-xl transition-all duration-300 z-10";
                                        if is_login.get() {
                                            format!("{base} text-gray-500 hover:text-gray-700")
                                        } else {
                                            format!("{base} text-violet-600")
                                        }
                                    }
                                    on:click=move |_| {
                                        set_is_login.set(false);
                                        set_error.set(None);
                                    }
                                >
                                    <div class="flex items-center justify-center space-x-2">
                                        <svg
                                            class="h-4 w-4"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
                                            />
                                        </svg>
                                        <span>"Sign Up"</span>
                                    </div>
                                </button>
                            </div>

                            // Forms with smooth transitions
                            <div class="relative">
                                <div class=move || {
                                    if is_login.get() {
                                        "opacity-100 translate-x-0 transition-all duration-500 ease-out"
                                    } else {
                                        "opacity-0 translate-x-4 absolute inset-0 pointer-events-none transition-all duration-500 ease-out"
                                    }
                                }>
                                    <LoginForm
                                        loading=loading
                                        set_loading=set_loading
                                        error=error
                                        set_error=set_error
                                    />
                                </div>
                                <div class=move || {
                                    if is_login.get() {
                                        "opacity-0 -translate-x-4 absolute inset-0 pointer-events-none transition-all duration-500 ease-out"
                                    } else {
                                        "opacity-100 translate-x-0 transition-all duration-500 ease-out"
                                    }
                                }>
                                    <RegisterForm
                                        loading=loading
                                        set_loading=set_loading
                                        error=error
                                        set_error=set_error
                                    />
                                </div>
                            </div>
                        </div>
                    </div>

                    // Enhanced footer
                    <div class="text-center mt-8 space-y-4">
                        <div class="flex items-center justify-center space-x-6 text-sm text-gray-500">
                            <div class="flex items-center space-x-2">
                                <div class="w-2 h-2 bg-emerald-500 rounded-full animate-pulse"></div>
                                <span>"Bank-level Security"</span>
                            </div>
                            <div class="flex items-center space-x-2">
                                <div class="w-2 h-2 bg-blue-500 rounded-full animate-pulse animation-delay-1000"></div>
                                <span>"256-bit Encryption"</span>
                            </div>
                        </div>
                        <p class="text-xs text-gray-400">"Trusted by 50,000+ users worldwide"</p>
                    </div>
                </div>
            </div>
        </div>

        // Custom styles for animations
        <style>
            "@keyframes blob {
                0% { transform: translate(0px, 0px) scale(1); }
                33% { transform: translate(30px, -50px) scale(1.1); }
                66% { transform: translate(-20px, 20px) scale(0.9); }
                100% { transform: translate(0px, 0px) scale(1); }
            }
            .animate-blob { animation: blob 7s infinite; }
            .animation-delay-2000 { animation-delay: 2s; }
            .animation-delay-4000 { animation-delay: 4s; }
            .animation-delay-1000 { animation-delay: 1s; }"
        </style>
    }
}

#[component]
fn LoginForm(
    loading: ReadSignal<bool>,
    set_loading: WriteSignal<bool>,
    error: ReadSignal<Option<String>>,
    set_error: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (show_password, set_show_password) = signal(false);
    let (remember_me, set_remember_me) = signal(false);
    let navigate = use_navigate();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        set_loading.set(true);
        set_error.set(None);

        let email_val = email.get();
        let password_val = password.get();
        let navigate = navigate.clone();

        spawn_local(async move {
            let result = async {
                let request = LoginRequest {
                    email: email_val,
                    password: password_val,
                };
                if let Ok(response) = Request::post("/api/auth/login")
                    .json(&request)
                    .map_err(|_| "Failed to serialize login data".to_string())?
                    .send()
                    .await
                {
                    if response.ok() {
                        if let Ok(auth_response) = response.json::<AuthResponse>().await {
                            if let Some(window) = web_sys::window() {
                                if let Ok(Some(storage)) = window.local_storage() {
                                    let _ = storage
                                        .set_item("refresh_token", &auth_response.refresh_token);
                                }
                            }
                            navigate("/dashboard", NavigateOptions::default());
                            Ok(())
                        } else {
                            set_error.set(Some("Failed to parse response".to_string()));
                            Err("Failed to parse response".to_string())
                        }
                    } else {
                        set_error.set(Some("Invalid email or password".to_string()));
                        Err("Invalid email or password".to_string())
                    }
                } else {
                    set_error.set(Some(
                        "Network error occurred. Please try again.".to_string(),
                    ));
                    Err("Network error occurred. Please try again.".to_string())
                }
            }
            .await;

            if let Err(err_msg) = result {
                set_error.set(Some(err_msg));
            }

            set_loading.set(false);
        });
    };

    view! {
        <form class="space-y-6" on:submit=on_submit>
            // Enhanced error message
            <Show when=move || error.get().is_some()>
                <div class="rounded-2xl bg-gradient-to-r from-red-50 to-pink-50 border border-red-200/50 p-4 animate-in slide-in-from-top duration-300">
                    <div class="flex items-center">
                        <div class="flex-shrink-0">
                            <div class="w-8 h-8 bg-red-100 rounded-full flex items-center justify-center">
                                <svg
                                    class="h-4 w-4 text-red-600"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 15.5c-.77.833.192 2.5 1.732 2.5z"
                                    />
                                </svg>
                            </div>
                        </div>
                        <div class="ml-3">
                            <p class="text-sm font-medium text-red-800">
                                {move || error.get().unwrap_or_default()}
                            </p>
                        </div>
                    </div>
                </div>
            </Show>

            // Enhanced email field
            <div class="space-y-2">
                <label for="email" class="block text-sm font-semibold text-gray-800">
                    "Email Address"
                </label>
                <div class="relative group">
                    <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none group-focus-within:text-violet-600 transition-colors duration-200">
                        <svg
                            class="h-5 w-5 text-gray-400 group-focus-within:text-violet-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
                            />
                        </svg>
                    </div>
                    <input
                        id="email"
                        name="email"
                        type="email"
                        required
                        class="block w-full pl-12 pr-4 py-4 bg-gray-50/50 border border-gray-200 rounded-2xl placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 focus:bg-white transition-all duration-300 text-gray-900"
                        placeholder="Enter your email address"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                    />
                </div>
            </div>

            // Enhanced password field
            <div class="space-y-2">
                <label for="password" class="block text-sm font-semibold text-gray-800">
                    "Password"
                </label>
                <div class="relative group">
                    <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none group-focus-within:text-violet-600 transition-colors duration-200">
                        <svg
                            class="h-5 w-5 text-gray-400 group-focus-within:text-violet-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                            />
                        </svg>
                    </div>
                    <input
                        id="password"
                        name="password"
                        type=move || if show_password.get() { "text" } else { "password" }
                        required
                        class="block w-full pl-12 pr-14 py-4 bg-gray-50/50 border border-gray-200 rounded-2xl placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 focus:bg-white transition-all duration-300 text-gray-900"
                        placeholder="Enter your password"
                        prop:value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                    />
                    <div class="absolute inset-y-0 right-0 pr-4 flex items-center">
                        <button
                            type="button"
                            class="text-gray-400 hover:text-violet-600 transition-colors duration-200 p-1 rounded-lg hover:bg-violet-50"
                            on:click=move |_| set_show_password.update(|show| *show = !*show)
                        >
                            {move || {
                                if show_password.get() {
                                    view! {
                                        <svg
                                            class="h-5 w-5"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"
                                            />
                                        </svg>
                                    }
                                } else {
                                    view! {
                                        <svg
                                            class="h-5 w-5"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                            />
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                                            />
                                        </svg>
                                    }
                                }
                            }}
                        </button>
                    </div>
                </div>
            </div>

            // Enhanced options row
            <div class="flex items-center justify-between">
                <div class="flex items-center">
                    <input
                        id="remember-me"
                        name="remember-me"
                        type="checkbox"
                        class="h-4 w-4 text-violet-600 focus:ring-violet-500 border-gray-300 rounded transition-colors duration-200"
                        prop:checked=remember_me
                        on:change=move |ev| set_remember_me.set(event_target_checked(&ev))
                    />
                    <label for="remember-me" class="ml-3 block text-sm text-gray-700 font-medium">
                        "Remember me for 30 days"
                    </label>
                </div>
                <div class="text-sm">
                    <a
                        href="#"
                        class="font-semibold text-violet-600 hover:text-violet-700 transition-colors duration-200 hover:underline"
                    >
                        "Forgot password?"
                    </a>
                </div>
            </div>

            // Enhanced submit button
            <div class="pt-2">
                <button
                    type="submit"
                    disabled=loading
                    class="group relative w-full flex justify-center py-4 px-6 border border-transparent text-sm font-semibold rounded-2xl text-white bg-gradient-to-r from-violet-600 via-purple-600 to-blue-600 hover:from-violet-700 hover:via-purple-700 hover:to-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-[1.02] active:scale-[0.98] overflow-hidden"
                >
                    // Button background shimmer effect
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -skew-x-12 -translate-x-full group-hover:translate-x-full transition-transform duration-1000"></div>

                    {move || {
                        if loading.get() {
                            view! {
                                <div class="flex items-center relative z-10">
                                    <div class="animate-spin -ml-1 mr-3 h-5 w-5 text-white">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24">
                                            <circle
                                                class="opacity-25"
                                                cx="12"
                                                cy="12"
                                                r="10"
                                                stroke="currentColor"
                                                stroke-width="4"
                                            ></circle>
                                            <path
                                                class="opacity-75"
                                                fill="currentColor"
                                                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                            ></path>
                                        </svg>
                                    </div>
                                    "Signing you in..."
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class="flex items-center relative z-10">
                                    <svg
                                        class="w-5 h-5 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M11 16l-4-4m0 0l4-4m-4 4h14m-5 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h7a3 3 0 013 3v1"
                                        />
                                    </svg>
                                    <span>"Sign in to your account"</span>
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </button>
            </div>

            // Social login options (placeholder)
            <div class="relative">
                <div class="absolute inset-0 flex items-center">
                    <div class="w-full border-t border-gray-300"></div>
                </div>
                <div class="relative flex justify-center text-sm">
                    <span class="px-4 bg-white text-gray-500 font-medium">"Or continue with"</span>
                </div>
            </div>

            <div class="grid grid-cols-2 gap-3">
                <button
                    type="button"
                    class="w-full inline-flex justify-center py-3 px-4 border border-gray-300 rounded-xl shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 hover:border-gray-400 transition-all duration-200"
                >
                    <svg class="w-5 h-5" viewBox="0 0 24 24">
                        <path
                            fill="currentColor"
                            d="M22.56 12.25c0-.78-.07-1.53-.2-2.25H12v4.26h5.92c-.26 1.37-1.04 2.53-2.21 3.31v2.77h3.57c2.08-1.92 3.28-4.74 3.28-8.09z"
                        />
                        <path
                            fill="currentColor"
                            d="M12 23c2.97 0 5.46-.98 7.28-2.66l-3.57-2.77c-.98.66-2.23 1.06-3.71 1.06-2.86 0-5.29-1.93-6.16-4.53H2.18v2.84C3.99 20.53 7.7 23 12 23z"
                        />
                        <path
                            fill="currentColor"
                            d="M5.84 14.09c-.22-.66-.35-1.36-.35-2.09s.13-1.43.35-2.09V7.07H2.18C1.43 8.55 1 10.22 1 12s.43 3.45 1.18 4.93l2.85-2.22.81-.62z"
                        />
                        <path
                            fill="currentColor"
                            d="M12 5.38c1.62 0 3.06.56 4.21 1.64l3.15-3.15C17.45 2.09 14.97 1 12 1 7.7 1 3.99 3.47 2.18 7.07l3.66 2.84c.87-2.6 3.3-4.53 6.16-4.53z"
                        />
                    </svg>
                    <span class="ml-2">"Google"</span>
                </button>
                <button
                    type="button"
                    class="w-full inline-flex justify-center py-3 px-4 border border-gray-300 rounded-xl shadow-sm bg-white text-sm font-medium text-gray-500 hover:bg-gray-50 hover:border-gray-400 transition-all duration-200"
                >
                    <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
                        <path d="M24 12.073c0-6.627-5.373-12-12-12s-12 5.373-12 12c0 5.99 4.388 10.954 10.125 11.854v-8.385H7.078v-3.47h3.047V9.43c0-3.007 1.792-4.669 4.533-4.669 1.312 0 2.686.235 2.686.235v2.953H15.83c-1.491 0-1.956.925-1.956 1.874v2.25h3.328l-.532 3.47h-2.796v8.385C19.612 23.027 24 18.062 24 12.073z" />
                    </svg>
                    <span class="ml-2">"Facebook"</span>
                </button>
            </div>
        </form>
    }
}

// Similar enhancements for RegisterForm...
#[component]
fn RegisterForm(
    loading: ReadSignal<bool>,
    set_loading: WriteSignal<bool>,
    error: ReadSignal<Option<String>>,
    set_error: WriteSignal<Option<String>>,
) -> impl IntoView {
    let (email, set_email) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (first_name, set_first_name) = signal(String::new());
    let (last_name, set_last_name) = signal(String::new());
    let (show_password, set_show_password) = signal(false);
    let (accept_terms, set_accept_terms) = signal(false);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        if !accept_terms.get() {
            set_error.set(Some("Please accept the terms and conditions".to_string()));
            return;
        }

        set_loading.set(true);
        set_error.set(None);

        let email_val = email.get();
        let password_val = password.get();
        let first_name_val = first_name.get();
        let last_name_val = last_name.get();

        spawn_local(async move {
            let request = CreateUserRequest {
                email: email_val,
                password: password_val,
                first_name: first_name_val,
                last_name: last_name_val,
            };

            let request_result = Request::post("/api/auth/register").json(&request);
            match request_result {
                Ok(request_builder) => {
                    match request_builder.send().await {
                        Ok(response) => {
                            if response.ok() {
                                match response.json::<AuthResponse>().await {
                                    Ok(auth_response) => {
                                        if let Some(window) = web_sys::window() {
                                            if let Ok(Some(storage)) = window.local_storage() {
                                                let _ = storage.set_item(
                                                    "refresh_token",
                                                    &auth_response.refresh_token,
                                                );
                                            }
                                            // Refresh the page after successful registration
                                            let _ = window.location().reload();
                                        }
                                    }
                                    Err(_) => {
                                        set_error.set(Some("Failed to parse response".to_string()));
                                    }
                                }
                            } else if response.status() == 409 {
                                set_error.set(Some(
                                    "An account with this email already exists".to_string(),
                                ));
                            } else {
                                set_error.set(Some(
                                    "Registration failed. Please try again.".to_string(),
                                ));
                            }
                        }
                        Err(_) => set_error.set(Some(
                            "Network error occurred. Please check your connection.".to_string(),
                        )),
                    }
                }
                Err(_) => set_error.set(Some("Failed to serialize request data".to_string())),
            }
            set_loading.set(false);
        });
    };

    view! {
        <form class="space-y-6" on:submit=on_submit>
            // Enhanced error message (same as login)
            <Show when=move || error.get().is_some()>
                <div class="rounded-2xl bg-gradient-to-r from-red-50 to-pink-50 border border-red-200/50 p-4 animate-in slide-in-from-top duration-300">
                    <div class="flex items-center">
                        <div class="flex-shrink-0">
                            <div class="w-8 h-8 bg-red-100 rounded-full flex items-center justify-center">
                                <svg
                                    class="h-4 w-4 text-red-600"
                                    fill="none"
                                    stroke="currentColor"
                                    viewBox="0 0 24 24"
                                >
                                    <path
                                        stroke-linecap="round"
                                        stroke-linejoin="round"
                                        stroke-width="2"
                                        d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L4.082 15.5c-.77.833.192 2.5 1.732 2.5z"
                                    />
                                </svg>
                            </div>
                        </div>
                        <div class="ml-3">
                            <p class="text-sm font-medium text-red-800">
                                {move || error.get().unwrap_or_default()}
                            </p>
                        </div>
                    </div>
                </div>
            </Show>

            // Enhanced name fields
            <div class="grid grid-cols-2 gap-4">
                <div class="space-y-2">
                    <label for="first_name" class="block text-sm font-semibold text-gray-800">
                        "First Name"
                    </label>
                    <div class="relative group">
                        <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none group-focus-within:text-violet-600 transition-colors duration-200">
                            <svg
                                class="h-5 w-5 text-gray-400 group-focus-within:text-violet-500"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                />
                            </svg>
                        </div>
                        <input
                            id="first_name"
                            name="first_name"
                            type="text"
                            required
                            class="block w-full pl-12 pr-4 py-4 bg-gray-50/50 border border-gray-200 rounded-2xl placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 focus:bg-white transition-all duration-300 text-gray-900"
                            placeholder="First name"
                            prop:value=first_name
                            on:input=move |ev| set_first_name.set(event_target_value(&ev))
                        />
                    </div>
                </div>
                <div class="space-y-2">
                    <label for="last_name" class="block text-sm font-semibold text-gray-800">
                        "Last Name"
                    </label>
                    <div class="relative group">
                        <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none group-focus-within:text-violet-600 transition-colors duration-200">
                            <svg
                                class="h-5 w-5 text-gray-400 group-focus-within:text-violet-500"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                                />
                            </svg>
                        </div>
                        <input
                            id="last_name"
                            name="last_name"
                            type="text"
                            required
                            class="block w-full pl-12 pr-4 py-4 bg-gray-50/50 border border-gray-200 rounded-2xl placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 focus:bg-white transition-all duration-300 text-gray-900"
                            placeholder="Last name"
                            prop:value=last_name
                            on:input=move |ev| set_last_name.set(event_target_value(&ev))
                        />
                    </div>
                </div>
            </div>

            // Enhanced email field (similar to login)
            <div class="space-y-2">
                <label for="email" class="block text-sm font-semibold text-gray-800">
                    "Email Address"
                </label>
                <div class="relative group">
                    <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none group-focus-within:text-violet-600 transition-colors duration-200">
                        <svg
                            class="h-5 w-5 text-gray-400 group-focus-within:text-violet-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
                            />
                        </svg>
                    </div>
                    <input
                        id="email"
                        name="email"
                        type="email"
                        required
                        class="block w-full pl-12 pr-4 py-4 bg-gray-50/50 border border-gray-200 rounded-2xl placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 focus:bg-white transition-all duration-300 text-gray-900"
                        placeholder="Enter your email address"
                        prop:value=email
                        on:input=move |ev| set_email.set(event_target_value(&ev))
                    />
                </div>
            </div>

            // Enhanced password field (similar to login)
            <div class="space-y-2">
                <label for="password" class="block text-sm font-semibold text-gray-800">
                    "Password"
                </label>
                <div class="relative group">
                    <div class="absolute inset-y-0 left-0 pl-4 flex items-center pointer-events-none group-focus-within:text-violet-600 transition-colors duration-200">
                        <svg
                            class="h-5 w-5 text-gray-400 group-focus-within:text-violet-500"
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                            />
                        </svg>
                    </div>
                    <input
                        id="password"
                        name="password"
                        type=move || if show_password.get() { "text" } else { "password" }
                        required
                        class="block w-full pl-12 pr-14 py-4 bg-gray-50/50 border border-gray-200 rounded-2xl placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-violet-500/50 focus:border-violet-500 focus:bg-white transition-all duration-300 text-gray-900"
                        placeholder="Create a strong password"
                        prop:value=password
                        on:input=move |ev| set_password.set(event_target_value(&ev))
                    />
                    <div class="absolute inset-y-0 right-0 pr-4 flex items-center">
                        <button
                            type="button"
                            class="text-gray-400 hover:text-violet-600 transition-colors duration-200 p-1 rounded-lg hover:bg-violet-50"
                            on:click=move |_| set_show_password.update(|show| *show = !*show)
                        >
                            {move || {
                                if show_password.get() {
                                    view! {
                                        <svg
                                            class="h-5 w-5"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.878 9.878L3 3m6.878 6.878L21 21"
                                            />
                                        </svg>
                                    }
                                } else {
                                    view! {
                                        <svg
                                            class="h-5 w-5"
                                            fill="none"
                                            stroke="currentColor"
                                            viewBox="0 0 24 24"
                                        >
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                            />
                                            <path
                                                stroke-linecap="round"
                                                stroke-linejoin="round"
                                                stroke-width="2"
                                                d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"
                                            />
                                        </svg>
                                    }
                                }
                            }}
                        </button>
                    </div>
                </div>
                <div class="flex items-center mt-2">
                    <div class="flex-1 bg-gray-200 rounded-full h-1.5">
                        <div class=move || {
                            let length = password.get().len();
                            let (width, color) = if length < 6 {
                                ("25%", "bg-red-500")
                            } else if length < 8 {
                                ("50%", "bg-yellow-500")
                            } else if length < 12 {
                                ("75%", "bg-blue-500")
                            } else {
                                ("100%", "bg-green-500")
                            };
                            format!(
                                "h-1.5 rounded-full transition-all duration-300 {color} w-[{width}]",
                            )
                        }></div>
                    </div>
                    <span class="ml-3 text-xs text-gray-500">"Strong password"</span>
                </div>
            </div>

            // Enhanced terms checkbox
            <div class="flex items-start">
                <div class="flex items-center h-5">
                    <input
                        id="accept-terms"
                        name="accept-terms"
                        type="checkbox"
                        class="h-4 w-4 text-violet-600 focus:ring-violet-500 border-gray-300 rounded transition-colors duration-200"
                        prop:checked=accept_terms
                        on:change=move |ev| set_accept_terms.set(event_target_checked(&ev))
                    />
                </div>
                <div class="ml-3">
                    <label for="accept-terms" class="text-sm text-gray-700 leading-relaxed">
                        "I agree to the "
                        <a
                            href="#"
                            class="text-violet-600 hover:text-violet-700 font-semibold hover:underline transition-colors duration-200"
                        >
                            "Terms of Service"
                        </a>
                        " and "
                        <a
                            href="#"
                            class="text-violet-600 hover:text-violet-700 font-semibold hover:underline transition-colors duration-200"
                        >
                            "Privacy Policy"
                        </a>
                    </label>
                </div>
            </div>

            // Enhanced submit button (similar to login but different text)
            <div class="pt-2">
                <button
                    type="submit"
                    disabled=move || loading.get() || !accept_terms.get()
                    class="group relative w-full flex justify-center py-4 px-6 border border-transparent text-sm font-semibold rounded-2xl text-white bg-gradient-to-r from-violet-600 via-purple-600 to-blue-600 hover:from-violet-700 hover:via-purple-700 hover:to-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-violet-500 disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-300 shadow-lg hover:shadow-xl transform hover:scale-[1.02] active:scale-[0.98] overflow-hidden"
                >
                    <div class="absolute inset-0 bg-gradient-to-r from-transparent via-white/20 to-transparent -skew-x-12 -translate-x-full group-hover:translate-x-full transition-transform duration-1000"></div>

                    {move || {
                        if loading.get() {
                            view! {
                                <div class="flex items-center relative z-10">
                                    <div class="animate-spin -ml-1 mr-3 h-5 w-5 text-white">
                                        <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24">
                                            <circle
                                                class="opacity-25"
                                                cx="12"
                                                cy="12"
                                                r="10"
                                                stroke="currentColor"
                                                stroke-width="4"
                                            ></circle>
                                            <path
                                                class="opacity-75"
                                                fill="currentColor"
                                                d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"
                                            ></path>
                                        </svg>
                                    </div>
                                    "Creating your account..."
                                </div>
                            }
                                .into_any()
                        } else {
                            view! {
                                <div class="flex items-center relative z-10">
                                    <svg
                                        class="w-5 h-5 mr-2"
                                        fill="none"
                                        stroke="currentColor"
                                        viewBox="0 0 24 24"
                                    >
                                        <path
                                            stroke-linecap="round"
                                            stroke-linejoin="round"
                                            stroke-width="2"
                                            d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"
                                        />
                                    </svg>
                                    <span>"Create your account"</span>
                                </div>
                            }
                                .into_any()
                        }
                    }}
                </button>
            </div>
        </form>
    }
}
