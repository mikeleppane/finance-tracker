// src/components/ws_connection_status.rs
use cfg_if::cfg_if;
use leptos::prelude::*;

cfg_if! {
    if #[cfg(feature = "hydrate")] {
        use leptos_use::{UseWebSocketReturn, core::ConnectionReadyState, use_websocket};
        use crate::components::ws_connection_status::codee::string::FromToStringCodec;
    }
}

#[component]
#[allow(clippy::must_use_candidate)]
pub fn ConnectionStatus() -> impl IntoView {
    cfg_if! {
        if #[cfg(feature = "hydrate")] {
            // Client-side implementation
            view! {
                <ClientSideConnectionStatus />
            }
        }
    }
}

#[component]
#[cfg(feature = "hydrate")]
fn ClientSideConnectionStatus() -> impl IntoView {
    use serde::{Deserialize, Serialize};

    // Define the WebSocket message structure to match your server
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "type", content = "data")]
    enum WebSocketMessage {
        Heartbeat {
            timestamp: u64,
            server_time: String,
        },
        ConnectionStatus {
            connected_users: usize,
            uptime_seconds: u64,
        },
        Error {
            message: String,
        },
        Pong,
    }

    // Safe WebSocket URL setup
    let ws_url = Memo::new(move |_| {
        if let Some(window) = web_sys::window() {
            if let (Ok(protocol), Ok(host)) =
                (window.location().protocol(), window.location().host())
            {
                let ws_protocol = if protocol == "https:" { "wss" } else { "ws" };
                format!("{ws_protocol}://{host}/ws")
            } else {
                leptos::logging::error!("Failed to get window location");
                "ws://localhost:3000/ws".to_string()
            }
        } else {
            leptos::logging::error!("Window not available");
            "ws://localhost:3000/ws".to_string()
        }
    });

    // WebSocket connection
    let UseWebSocketReturn {
        ready_state,
        message,
        send,
        open,
        close,
        ..
    } = use_websocket::<String, String, FromToStringCodec>(&ws_url.get());

    // State for server data
    let last_heartbeat = RwSignal::new(None::<String>);
    let last_heartbeat_time = RwSignal::new(None::<chrono::DateTime<chrono::Utc>>);
    let connected_users = RwSignal::new(0usize);
    let server_uptime = RwSignal::new(0u64);
    let error_message = RwSignal::new(None::<String>);
    let connection_established_at = RwSignal::new(None::<chrono::DateTime<chrono::Utc>>);

    // Handle incoming messages from server
    Effect::new(move |_| {
        if let Some(msg) = message.get() {
            leptos::logging::log!("Received WebSocket message: {}", msg);

            match serde_json::from_str::<WebSocketMessage>(&msg) {
                Ok(parsed_message) => {
                    match parsed_message {
                        WebSocketMessage::Heartbeat {
                            server_time,
                            timestamp,
                        } => {
                            // Update heartbeat info
                            last_heartbeat.set(Some(server_time.clone()));
                            last_heartbeat_time.set(Some(chrono::Utc::now()));
                            error_message.set(None); // Clear any errors on successful heartbeat

                            leptos::logging::log!(
                                "Heartbeat received at {} (server time: {})",
                                timestamp,
                                server_time
                            );
                        }
                        WebSocketMessage::ConnectionStatus {
                            connected_users: users,
                            uptime_seconds: uptime,
                        } => {
                            // Update connection statistics
                            connected_users.set(users);
                            server_uptime.set(uptime);

                            leptos::logging::log!(
                                "Connection status updated: {} users, {}s uptime",
                                users,
                                uptime
                            );
                        }
                        WebSocketMessage::Error { message: err_msg } => {
                            error_message.set(Some(err_msg));
                            leptos::logging::error!(
                                "Server error: {}",
                                error_message.get().unwrap_or_default()
                            );
                        }
                        WebSocketMessage::Pong => {
                            leptos::logging::log!("Received pong from server");
                        }
                    }
                }
                Err(e) => {
                    leptos::logging::warn!("Failed to parse WebSocket message: {}", e);
                    leptos::logging::log!("Raw message was: {}", msg);
                }
            }
        }
    });

    // Track connection state changes
    Effect::new(move |_| {
        let state = ready_state.get();
        match state {
            ConnectionReadyState::Open => {
                connection_established_at.set(Some(chrono::Utc::now()));
                leptos::logging::log!("WebSocket connection established");
            }
            ConnectionReadyState::Closed => {
                connection_established_at.set(None);
                leptos::logging::log!("WebSocket connection closed");
            }
            ConnectionReadyState::Connecting => {
                leptos::logging::log!("WebSocket connecting...");
            }
            ConnectionReadyState::Closing => {
                leptos::logging::log!("WebSocket closing...");
            }
        }
    });

    // Auto-connect on component mount
    Effect::new({
        let open = open.clone();
        move |_| {
            leptos::logging::log!("Auto-connecting to WebSocket...");
            open();
        }
    });

    // Optional: Send periodic pings to keep connection alive
    // (Your server sends heartbeats, but client pings can help detect connection issues faster)
    Effect::new({
        let send = send.clone();
        move |_| {
            let send = send.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let mut interval_counter = 0;
                let interval = gloo_timers::callback::Interval::new(60_000, move || {
                    // Every 60 seconds
                    if ready_state.get() == ConnectionReadyState::Open {
                        interval_counter += 1;
                        let pong_message = WebSocketMessage::Pong;
                        if let Ok(json) = serde_json::to_string(&pong_message) {
                            send(&json);
                            leptos::logging::log!("Sent ping #{} to server", interval_counter);
                        }
                    }
                });
                interval.forget();
            });
        }
    });

    // Helper functions
    let status = move || ready_state.get().to_string();
    let connected = move || ready_state.get() == ConnectionReadyState::Open;
    let connecting = move || ready_state.get() == ConnectionReadyState::Connecting;

    let format_uptime = move |seconds: u64| -> String {
        let hours = seconds / 3600;
        let minutes = (seconds % 3600) / 60;
        let secs = seconds % 60;

        if hours > 0 {
            format!("{hours}h {minutes}m {secs}s")
        } else if minutes > 0 {
            format!("{minutes}m {secs}s")
        } else {
            format!("{secs}s")
        }
    };

    // Calculate time since last heartbeat
    let heartbeat_status = move || match last_heartbeat_time.get() {
        Some(last_time) => {
            let now = chrono::Utc::now();
            let duration = now.signed_duration_since(last_time);
            let seconds = duration.num_seconds();

            if seconds < 35 {
                ("🟢 Active", "text-green-600")
            } else if seconds < 60 {
                ("🟡 Delayed", "text-yellow-600")
            } else {
                ("🔴 Stale", "text-red-600")
            }
        }
        None => ("⚪ Waiting", "text-gray-600"),
    };

    view! {
        <div class="bg-white rounded-lg shadow-sm border border-gray-200 p-4">
            <div class="flex items-center justify-between mb-4">
                <h3 class="text-lg font-medium text-gray-900">"Server Connection"</h3>
                <div class="flex space-x-2">
                    <button
                        class="px-3 py-1 text-sm text-blue-600 hover:text-blue-800 font-medium disabled:opacity-50 disabled:cursor-not-allowed border border-blue-200 rounded hover:bg-blue-50"
                        on:click=move |_| {
                            leptos::logging::log!("Manual reconnect requested");
                            open();
                        }
                        disabled=connected
                    >
                        "Connect"
                    </button>
                    <button
                        class="px-3 py-1 text-sm text-red-600 hover:text-red-800 font-medium disabled:opacity-50 disabled:cursor-not-allowed border border-red-200 rounded hover:bg-red-50"
                        on:click=move |_| {
                            leptos::logging::log!("Manual disconnect requested");
                            close();
                        }
                        disabled=move || !connected()
                    >
                        "Disconnect"
                    </button>
                    <button
                        class="px-3 py-1 text-sm text-green-600 hover:text-green-800 font-medium disabled:opacity-50 disabled:cursor-not-allowed border border-green-200 rounded hover:bg-green-50"
                        on:click=move |_| {
                            if connected() {
                                let pong_message = WebSocketMessage::Pong;
                                if let Ok(json) = serde_json::to_string(&pong_message) {
                                    send(&json);
                                    leptos::logging::log!("Manual ping sent");
                                }
                            }
                        }
                        disabled=move || !connected()
                    >
                        "Ping Now"
                    </button>
                </div>
            </div>

            <div class="space-y-3">
                // Connection Status Indicator
                <div class="flex items-center space-x-3">
                    <div class=move || {
                        let state = ready_state.get();
                        format!(
                            "w-3 h-3 rounded-full {}",
                            match state {
                                ConnectionReadyState::Open => "bg-green-500",
                                ConnectionReadyState::Connecting => "bg-yellow-400 animate-pulse",
                                ConnectionReadyState::Closed => "bg-gray-400",
                                ConnectionReadyState::Closing => "bg-orange-400",
                            },
                        )
                    }></div>
                    <span class=move || {
                        let state = ready_state.get();
                        format!(
                            "text-sm font-medium {}",
                            match state {
                                ConnectionReadyState::Open => "text-green-600",
                                ConnectionReadyState::Connecting => "text-yellow-600",
                                ConnectionReadyState::Closed => "text-gray-600",
                                ConnectionReadyState::Closing => "text-orange-600",
                            },
                        )
                    }>{status}</span>
                    <span class="text-xs text-gray-400 font-mono">{move || ws_url.get()}</span>
                </div>

                // Heartbeat Status
                <div class="flex justify-between">
                    <span class="text-gray-600 text-sm">"Heartbeat Status:"</span>
                    <span class=move || {
                        let (_, class) = heartbeat_status();
                        format!("text-sm font-medium {class}")
                    }>{move || heartbeat_status().0}</span>
                </div>

                // Connection Statistics
                <div class="text-sm space-y-2">
                    <div class="flex justify-between">
                        <span class="text-gray-600">"Connected Users:"</span>
                        <span class="font-medium text-gray-900">
                            {move || connected_users.get()}
                        </span>
                    </div>

                    <div class="flex justify-between">
                        <span class="text-gray-600">"Server Uptime:"</span>
                        <span class="font-medium text-gray-900">
                            {move || format_uptime(server_uptime.get())}
                        </span>
                    </div>

                    <div class="flex justify-between">
                        <span class="text-gray-600">"Last Heartbeat:"</span>
                        <span class="font-medium text-gray-900">
                            {move || {
                                last_heartbeat
                                    .get()
                                    .and_then(|time| {
                                        chrono::DateTime::parse_from_rfc3339(&time)
                                            .ok()
                                            .map(|dt| dt.format("%H:%M:%S").to_string())
                                    })
                                    .unwrap_or_else(|| "Never".to_string())
                            }}
                        </span>
                    </div>

                    // Show connection duration
                    <Show when=move || connection_established_at.get().is_some()>
                        <div class="flex justify-between">
                            <span class="text-gray-600">"Connected For:"</span>
                            <span class="font-medium text-gray-900">
                                {move || {
                                    connection_established_at
                                        .get()
                                        .map_or_else(
                                            || "Unknown".to_string(),
                                            |start_time| {
                                                let duration = chrono::Utc::now()
                                                    .signed_duration_since(start_time);
                                                #[allow(clippy::cast_sign_loss)]
                                                let seconds = duration.num_seconds().max(0) as u64;
                                                format_uptime(seconds)
                                            },
                                        )
                                }}
                            </span>
                        </div>
                    </Show>
                </div>

                // Error Display
                <Show when=move || error_message.get().is_some()>
                    <div class="mt-3 p-3 bg-red-50 border border-red-200 rounded-md">
                        <div class="text-sm text-red-700">
                            {move || error_message.get().unwrap_or_default()}
                        </div>
                    </div>
                </Show>

                // Debug Information
                <Show when=move || connected() && message.get().is_some()>
                    <div class="pt-2 border-t border-gray-100">
                        <details class="text-xs">
                            <summary class="text-gray-600 cursor-pointer hover:text-gray-800">
                                "Debug: Last Message"
                            </summary>
                            <pre class="mt-2 p-2 bg-gray-50 rounded text-xs overflow-x-auto max-h-32 overflow-y-auto">
                                {move || message.get().unwrap_or_default()}
                            </pre>
                        </details>
                    </div>
                </Show>

                // Live Connection Indicator
                <Show when=connected>
                    <div class="flex items-center space-x-2 pt-2 border-t border-gray-100">
                        <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
                        <span class="text-xs text-green-600 font-medium">
                            "Live Connection Active"
                        </span>
                    </div>
                </Show>

                // Connecting Indicator
                <Show when=connecting>
                    <div class="flex items-center space-x-2 pt-2 border-t border-gray-100">
                        <div class="w-2 h-2 bg-yellow-400 rounded-full animate-spin"></div>
                        <span class="text-xs text-yellow-600 font-medium">
                            "Connecting to server..."
                        </span>
                    </div>
                </Show>
            </div>
        </div>
    }
}
