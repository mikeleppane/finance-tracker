//use crate::components::login::AuthPage;
//use crate::components::login::AuthPage;
use crate::components::ws_connection_status::ConnectionStatus;
use leptos::prelude::*;
use leptos_meta::{Meta, MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

#[must_use]
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />

                <AutoReload options=options.clone() />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body>
                <App />
            </body>
        </html>
    }
}

#[component]
#[allow(clippy::must_use_candidate)]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/finance-tracker.css" />
        <Title text="Finance Tracker" />
        <Meta name="description" content="Track your finances with ease" />

        <Router>
            <main class="min-h-screen bg-gray-50">
                <Routes fallback=|| "Page not found".into_view()>
                    <Route path=path!("/") view=HomePage />
                    <Route path=path!("/connection") view=ConnectionStatus />
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="container mx-auto px-4 py-8">
            <div class="max-w-4xl mx-auto">
                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-8 mb-8">
                    <h1 class="text-3xl font-bold text-gray-900 mb-4">
                        "Welcome to Test WebSocket Connection"
                    </h1>
                    <p class="text-gray-600 mb-6">
                        "This page allows you to test the WebSocket connection to the server. Click the button below to initiate a connection and see the status updates in real-time."
                    </p>
                    <div class="space-x-4">
                        <a
                            href="/connection"
                            class="inline-flex items-center px-6 py-3 border border-transparent text-base font-medium rounded-md text-white bg-blue-600 hover:bg-blue-700 transition-colors duration-200"
                        >
                            "Test WebSocket Connection"
                        </a>
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn Dashboard() -> impl IntoView {
    view! {
        <div class="min-h-screen bg-gray-50">
            <DashboardHeader />
            <main class="max-w-7xl mx-auto py-6 sm:px-6 lg:px-8">
                <div class="px-4 py-6 sm:px-0">
                    <DashboardContent />
                </div>
            </main>
        </div>
    }
}

#[component]
fn DashboardHeader() -> impl IntoView {
    view! {
        <header class="bg-white shadow">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center py-6">
                    // Logo and title
                    <div class="flex items-center">
                        <div class="h-10 w-10 bg-gradient-to-r from-blue-600 to-indigo-600 rounded-lg flex items-center justify-center mr-4">
                            <svg
                                class="h-6 w-6 text-white"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"
                                />
                            </svg>
                        </div>
                        <div>
                            <h1 class="text-2xl font-bold text-gray-900">"Finance Tracker"</h1>
                            <p class="text-sm text-gray-500">"Dashboard"</p>
                        </div>
                    </div>

                    // User menu
                    <div class="flex items-center space-x-4">
                        <button class="text-gray-500 hover:text-gray-700 transition-colors duration-200">
                            <svg
                                class="h-6 w-6"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M15 17h5l-5 5-5-5h5v-12h5v12z"
                                />
                            </svg>
                        </button>
                        <button class="text-gray-500 hover:text-gray-700 transition-colors duration-200">
                            <svg
                                class="h-6 w-6"
                                fill="none"
                                stroke="currentColor"
                                viewBox="0 0 24 24"
                            >
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
                                />
                                <path
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                    stroke-width="2"
                                    d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
                                />
                            </svg>
                        </button>
                        <div class="relative">
                            <button class="flex items-center space-x-2 text-gray-700 hover:text-gray-900 transition-colors duration-200">
                                <div class="h-8 w-8 bg-gradient-to-r from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
                                    <span class="text-xs font-medium text-white">"JD"</span>
                                </div>
                                <span class="text-sm font-medium hidden sm:block">"John Doe"</span>
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
                                        d="M19 9l-7 7-7-7"
                                    />
                                </svg>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </header>
    }
}

#[component]
fn DashboardContent() -> impl IntoView {
    view! {
        <div class="space-y-6">
            // Welcome section
            <div class="bg-gradient-to-r from-blue-600 to-indigo-600 rounded-2xl p-6 text-white">
                <div class="flex items-center justify-between">
                    <div>
                        <h2 class="text-2xl font-bold mb-2">"Welcome back, John!"</h2>
                        <p class="text-blue-100">"Here's your financial overview for today"</p>
                    </div>
                    <div class="hidden md:block">
                        <svg
                            class="h-16 w-16 text-blue-200"
                            fill="currentColor"
                            viewBox="0 0 20 20"
                        >
                            <path
                                fill-rule="evenodd"
                                d="M4 4a2 2 0 00-2 2v4a2 2 0 002 2V6h10a2 2 0 00-2-2H4zm2 6a2 2 0 012-2h8a2 2 0 012 2v4a2 2 0 01-2 2H8a2 2 0 01-2-2v-4zm6 4a2 2 0 100-4 2 2 0 000 4z"
                                clip-rule="evenodd"
                            />
                        </svg>
                    </div>
                </div>
            </div>

            // Stats grid
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                <StatCard
                    title="Total Balance"
                    value="$12,426.50"
                    change="+2.5%"
                    change_type="positive"
                    icon=view! {
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1"
                            />
                        </svg>
                    }
                />
                <StatCard
                    title="Monthly Income"
                    value="$4,850.00"
                    change="+12.3%"
                    change_type="positive"
                    icon=view! {
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M7 11l5-5m0 0l5 5m-5-5v12"
                            />
                        </svg>
                    }
                />
                <StatCard
                    title="Monthly Expenses"
                    value="$2,340.75"
                    change="-5.2%"
                    change_type="negative"
                    icon=view! {
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M17 13l-5 5m0 0l-5-5m5 5V6"
                            />
                        </svg>
                    }
                />
                <StatCard
                    title="Savings Rate"
                    value="28.4%"
                    change="+3.1%"
                    change_type="positive"
                    icon=view! {
                        <svg class="h-6 w-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                            />
                        </svg>
                    }
                />
            </div>

            // Recent transactions and quick actions
            <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
                // Recent transactions
                <div class="lg:col-span-2 bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                    <div class="flex items-center justify-between mb-6">
                        <h3 class="text-lg font-semibold text-gray-900">"Recent Transactions"</h3>
                        <button class="text-blue-600 hover:text-blue-700 text-sm font-medium">
                            "View all"
                        </button>
                    </div>
                    <div class="space-y-4">
                        <TransactionItem
                            name="Grocery Store"
                            category="Food"
                            amount="-$89.50"
                            date="Today"
                            is_expense=true
                        />
                        <TransactionItem
                            name="Salary Deposit"
                            category="Income"
                            amount="+$2,500.00"
                            date="Yesterday"
                            is_expense=false
                        />
                        <TransactionItem
                            name="Netflix"
                            category="Entertainment"
                            amount="-$15.99"
                            date="2 days ago"
                            is_expense=true
                        />
                        <TransactionItem
                            name="Coffee Shop"
                            category="Food"
                            amount="-$4.50"
                            date="3 days ago"
                            is_expense=true
                        />
                    </div>
                </div>

                // Quick actions
                <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
                    <h3 class="text-lg font-semibold text-gray-900 mb-6">"Quick Actions"</h3>
                    <div class="space-y-3">
                        <QuickActionButton
                            title="Add Transaction"
                            subtitle="Record income or expense"
                            icon=view! {
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
                                        d="M12 4v16m8-8H4"
                                    />
                                </svg>
                            }
                        />
                        <QuickActionButton
                            title="View Budget"
                            subtitle="Check your spending limits"
                            icon=view! {
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
                                        d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z"
                                    />
                                </svg>
                            }
                        />
                        <QuickActionButton
                            title="Generate Report"
                            subtitle="Monthly financial summary"
                            icon=view! {
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
                                        d="M9 17v-2m3 2v-4m3 4v-6m2 10H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                                    />
                                </svg>
                            }
                        />
                        <QuickActionButton
                            title="Set Goals"
                            subtitle="Financial targets & milestones"
                            icon=view! {
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
                                        d="M13 7h8m0 0v8m0-8l-8 8-4-4-6 6"
                                    />
                                </svg>
                            }
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
fn StatCard(
    title: &'static str,
    value: &'static str,
    change: &'static str,
    change_type: &'static str,
    icon: impl IntoView,
) -> impl IntoView {
    let change_color = if change_type == "positive" {
        "text-green-600"
    } else {
        "text-red-600"
    };

    view! {
        <div class="bg-white rounded-xl shadow-sm border border-gray-200 p-6">
            <div class="flex items-center justify-between">
                <div class="flex items-center space-x-3">
                    <div class="p-2 bg-blue-100 rounded-lg text-blue-600">{icon}</div>
                    <div>
                        <p class="text-sm font-medium text-gray-600">{title}</p>
                        <p class="text-2xl font-bold text-gray-900">{value}</p>
                    </div>
                </div>
            </div>
            <div class="mt-4 flex items-center">
                <span class=format!("text-sm font-medium {}", change_color)>{change}</span>
                <span class="text-sm text-gray-500 ml-1">"from last month"</span>
            </div>
        </div>
    }
}

#[component]
fn TransactionItem(
    name: &'static str,
    category: &'static str,
    amount: &'static str,
    date: &'static str,
    is_expense: bool,
) -> impl IntoView {
    let amount_color = if is_expense {
        "text-red-600"
    } else {
        "text-green-600"
    };

    view! {
        <div class="flex items-center justify-between py-3 border-b border-gray-100 last:border-b-0">
            <div class="flex items-center space-x-3">
                <div class="h-10 w-10 bg-gray-100 rounded-lg flex items-center justify-center">
                    <span class="text-sm font-medium text-gray-600">{&name[0..1]}</span>
                </div>
                <div>
                    <p class="font-medium text-gray-900">{name}</p>
                    <p class="text-sm text-gray-500">{category}</p>
                </div>
            </div>
            <div class="text-right">
                <p class=format!("font-semibold {}", amount_color)>{amount}</p>
                <p class="text-sm text-gray-500">{date}</p>
            </div>
        </div>
    }
}

#[component]
fn QuickActionButton(
    title: &'static str,
    subtitle: &'static str,
    icon: impl IntoView,
) -> impl IntoView {
    view! {
        <button class="w-full flex items-center space-x-3 p-3 rounded-lg hover:bg-gray-50 transition-colors duration-200 text-left">
            <div class="p-2 bg-blue-100 rounded-lg text-blue-600">{icon}</div>
            <div>
                <p class="font-medium text-gray-900">{title}</p>
                <p class="text-sm text-gray-500">{subtitle}</p>
            </div>
        </button>
    }
}
