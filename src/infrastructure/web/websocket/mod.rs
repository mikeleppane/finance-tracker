// src/infrastructure/websocket/mod.rs
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        pub mod heartbeat;
        pub mod connection_manager;

        pub use heartbeat::*;
        pub use connection_manager::*;
    }
}
