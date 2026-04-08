use crate::app::controllers::feedback_controller::get_feedbacks;
use crate::app::controllers::login_page::LoginPage;
use crate::app::pages::admin_forms::{AdminFormBuilder, AdminFormResponses, AdminFormsList};
use crate::app::pages::public_form::PublicFormView;
use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

/// SSR-only HTML shell — provides HydrationScripts and styles.
/// WASM does NOT mount this; it mounts App directly.
#[component]
pub fn Shell(options: leptos::config::LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <Stylesheet id="leptos" href="/pkg/fitbek.css"/>
                <MetaTags/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

/// The reactive application — mounted by WASM on the client.
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Fitbek - Elegant Feedback"/>
        <Router>
            <Routes fallback=|| view! { <NotFound/> }>
                <Route path=path!("") view=HomePage/>
                <Route path=path!("login") view=LoginPage/>
                <Route path=path!("f/:slug") view=PublicFormView/>
                // Admin routes with sidebar layout
                <ParentRoute path=path!("admin") view=AdminLayout>
                    <Route path=path!("") view=AdminDashboard/>
                    <Route path=path!("feedbacks") view=FeedbackListPage/>
                    <Route path=path!("forms") view=AdminFormsList/>
                    <Route path=path!("forms/new") view=AdminFormBuilder/>
                    <Route path=path!("forms/:id/responses") view=AdminFormResponses/>
                </ParentRoute>
            </Routes>
        </Router>
    }
}

// ─── PUBLIC FORM (HOME) ─────────────────────────────────────────────────────

#[component]
fn HomePage() -> impl IntoView {
    use crate::app::controllers::feedback_controller::submit_feedback;

    let content = RwSignal::new(String::new());
    let pending = RwSignal::new(false);
    let success = RwSignal::new(false);
    let error_msg: RwSignal<Option<String>> = RwSignal::new(None);

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        let text = content.get();
        if text.trim().is_empty() {
            error_msg.set(Some("Feedback cannot be empty.".to_string()));
            return;
        }
        pending.set(true);
        success.set(false);
        error_msg.set(None);

        leptos::task::spawn_local(async move {
            match submit_feedback(text).await {
                Ok(_) => {
                    pending.set(false);
                    success.set(true);
                    content.set(String::new());
                }
                Err(e) => {
                    pending.set(false);
                    error_msg.set(Some(format!("{}", e)));
                }
            }
        });
    };

    view! {
        <div class="min-h-screen bg-navy-900 flex flex-col">
            // ── Top bar ──
            <header class="w-full px-8 py-4 flex justify-between items-center border-b border-navy-800/60">
                <a href="/" class="text-xl font-bold tracking-wider text-primary-400">"Fitbek"</a>
                <a href="/admin" class="text-navy-400 hover:text-navy-200 transition-colors text-sm">"Admin"</a>
            </header>

            // ── Main content ──
            <div class="flex-1 flex flex-col items-center justify-center px-4 py-12">
                // Hero
                <div class="text-center mb-10 max-w-2xl">
                    <h1 class="text-4xl md:text-5xl font-bold leading-tight mb-4 text-white">
                        "Your Voice Shapes The Future"
                    </h1>
                    <p class="text-navy-400 text-lg">"Submit your thoughts, critiques, or ideas directly to our development team."</p>
                </div>
                

            </div>

            // ── Footer ──
            <footer class="w-full py-6 flex justify-center">
                <span class="text-xs text-navy-500 uppercase tracking-widest cursor-default">"Privacy Policy"</span>
            </footer>
        </div>
    }
}

// ─── ADMIN LAYOUT (Sidebar + Topbar) ────────────────────────────────────────

#[component]
fn AdminLayout() -> impl IntoView {
    let logout_action = ServerAction::<crate::app::controllers::auth_controller::Logout>::new();
    let pathname = leptos_router::hooks::use_location().pathname;

    view! {
        <div class="min-h-screen bg-navy-900 flex flex-col w-full">
            // ── Admin top bar ──
            <header class="w-full h-14 px-6 flex items-center justify-between border-b border-navy-700/50 bg-navy-900/95 backdrop-blur-sm shrink-0">
                <div class="flex items-center gap-8">
                    <a href="/" class="text-lg font-bold tracking-wider bg-gradient-to-r from-primary-400 to-tertiary-400 bg-clip-text text-transparent">"Fitbek"</a>
                    <nav class="hidden md:flex items-center gap-1 text-sm">
                        <a href="/admin" class="px-3 py-1.5 rounded-md text-navy-300 hover:text-white hover:bg-navy-800 transition-all">"Submissions"</a>
                        <a href="/admin/feedbacks" class="px-3 py-1.5 rounded-md text-navy-300 hover:text-white hover:bg-navy-800 transition-all">"Analytics"</a>
                        <a href="/admin/forms" class="px-3 py-1.5 rounded-md text-navy-300 hover:text-white hover:bg-navy-800 transition-all">"Settings"</a>
                    </nav>
                </div>
                <div class="flex items-center gap-3">
                    <div class="w-8 h-8 rounded-full bg-gradient-to-br from-primary-500 to-tertiary-400 flex items-center justify-center text-white text-xs font-bold">"A"</div>
                </div>
            </header>

            <div class="flex flex-1 overflow-hidden">
                // ── Sidebar ──
                <aside class="w-56 shrink-0 border-r border-navy-700/50 bg-navy-900 flex flex-col justify-between py-6 px-4 hidden md:flex">
                    <div>
                        <div class="mb-8 px-2">
                            <h2 class="text-sm font-bold text-white tracking-wide">"Admin Console"</h2>
                            <p class="text-xs text-navy-500 uppercase tracking-widest mt-0.5">"Precision Management"</p>
                        </div>
                        <nav class="space-y-1">
                            <a
                                href="/admin"
                                class=move || {
                                    let p = pathname.get();
                                    if p == "/admin" { "sidebar-link active" } else { "sidebar-link" }
                                }
                            >
                                <span class="sidebar-icon">"📊"</span>
                                "Dashboard"
                            </a>
                            <a
                                href="/admin/feedbacks"
                                class=move || {
                                    let p = pathname.get();
                                    if p == "/admin/feedbacks" { "sidebar-link active" } else { "sidebar-link" }
                                }
                            >
                                <span class="sidebar-icon">"💬"</span>
                                "Feedback Loop"
                            </a>
                            <a
                                href="/admin/forms"
                                class=move || {
                                    let p = pathname.get();
                                    if p.starts_with("/admin/forms") { "sidebar-link active" } else { "sidebar-link" }
                                }
                            >
                                <span class="sidebar-icon">"📝"</span>
                                "Form Manager"
                            </a>
                        </nav>
                    </div>
                    <div class="space-y-2 px-2">
                        <ActionForm action=logout_action>
                            <button
                                type="submit"
                                class="flex items-center gap-2 text-sm text-navy-400 hover:text-red-400 transition-colors w-full py-2"
                            >
                                <span>"🚪"</span>
                                "Logout"
                            </button>
                        </ActionForm>
                    </div>
                </aside>

                // ── Main content area ──
                <main class="flex-1 overflow-y-auto p-6 md:p-8">
                    <Outlet/>
                </main>
            </div>
        </div>
    }
}

// ─── ADMIN DASHBOARD ────────────────────────────────────────────────────────

#[component]
fn AdminDashboard() -> impl IntoView {
    let feedbacks = Resource::new(|| (), |_| async move { get_feedbacks().await });

    view! {
        <div class="max-w-6xl mx-auto text-left">
            // Header
            <div class="flex justify-between items-start mb-8">
                <div>
                    <h1 class="text-3xl font-bold text-white mb-1">"Command Overview"</h1>
                    <p class="text-navy-400">"System signals and real-time feedback processing."</p>
                </div>
                <div class="bg-navy-800 border border-navy-700 rounded-lg px-4 py-2 text-sm text-navy-300 flex items-center gap-2">
                    <span>"📅"</span>
                    "Last 30 Days"
                </div>
            </div>

            <Transition fallback=move || view! { <div class="text-navy-400 animate-pulse">"Loading..."</div> }>
                {move || {
                    match feedbacks.get() {
                        None => view! { <div></div> }.into_any(),
                        Some(Err(e)) => {
                            if e.to_string().contains("Tidak terautentikasi") {
                                let navigate = leptos_router::hooks::use_navigate();
                                let loc = leptos_router::hooks::use_location().pathname.get_untracked();
                                navigate(&format!("/login?returnUrl={}", loc), Default::default());
                                view! { <div>"Redirecting to login..."</div> }.into_any()
                            } else {
                                view! {
                                    <div class="bg-red-900/40 p-4 rounded-xl border border-red-700/50 text-red-300">
                                        {format!("Error: {}", e)}
                                    </div>
                                }.into_any()
                            }
                        },
                        Some(Ok(items)) => {
                            let total = items.len();
                            let latest: Vec<_> = items.iter().take(3).cloned().collect();

                            view! {
                                <div>
                                    // ── Stats cards ──
                                    <div class="grid grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
                                        <div class="stat-card">
                                            <div class="flex justify-between items-start">
                                                <div>
                                                    <p class="text-xs font-bold text-navy-400 uppercase tracking-widest mb-1">"Total Feedback"</p>
                                                    <p class="text-3xl font-bold text-white">{total}</p>
                                                </div>
                                                <span class="text-2xl opacity-30">"📨"</span>
                                            </div>
                                            <p class="text-xs text-green-400 mt-2 flex items-center gap-1">"↗ Active"</p>
                                        </div>
                                        <div class="stat-card">
                                            <p class="text-xs font-bold text-navy-400 uppercase tracking-widest mb-1">"Unread Signals"</p>
                                            <p class="text-3xl font-bold text-white">{total.min(99)}</p>
                                            <p class="text-xs text-amber-400 mt-2">"● Requires Action"</p>
                                        </div>
                                        <div class="stat-card">
                                            <p class="text-xs font-bold text-navy-400 uppercase tracking-widest mb-1">"Avg Rating"</p>
                                            <p class="text-3xl font-bold text-white">"4.82"</p>
                                            <p class="text-xs text-navy-400 mt-2">"★★★★★"</p>
                                        </div>
                                        <div class="stat-card">
                                            <p class="text-xs font-bold text-navy-400 uppercase tracking-widest mb-1">"Avg Response"</p>
                                            <p class="text-3xl font-bold text-white">"1.2h"</p>
                                            <p class="text-xs text-primary-400 mt-2">"⚡ 98th Percentile"</p>
                                        </div>
                                    </div>

                                    // ── Chart + Latest signals ──
                                    <div class="grid grid-cols-1 lg:grid-cols-5 gap-6">
                                        // Bar chart
                                        <div class="lg:col-span-3 bg-navy-800/60 rounded-2xl border border-navy-700/50 p-6">
                                            <div class="flex justify-between items-center mb-6">
                                                <h3 class="text-lg font-bold text-white">"Feedback Volume"</h3>
                                                <div class="flex gap-1">
                                                    <span class="px-3 py-1 rounded-full text-xs font-bold bg-primary-500/20 text-primary-400 border border-primary-500/30">"Daily"</span>
                                                    <span class="px-3 py-1 rounded-full text-xs text-navy-400 hover:text-navy-300 cursor-pointer">"Weekly"</span>
                                                </div>
                                            </div>
                                            <div class="flex items-end justify-between gap-3 h-48 px-2">
                                                <div class="chart-bar" style="height: 45%"><span class="chart-label">"Mon"</span></div>
                                                <div class="chart-bar" style="height: 70%"><span class="chart-label">"Tue"</span></div>
                                                <div class="chart-bar" style="height: 55%"><span class="chart-label">"Wed"</span></div>
                                                <div class="chart-bar" style="height: 85%"><span class="chart-label">"Thu"</span></div>
                                                <div class="chart-bar" style="height: 60%"><span class="chart-label">"Fri"</span></div>
                                                <div class="chart-bar" style="height: 95%"><span class="chart-label">"Sat"</span></div>
                                                <div class="chart-bar" style="height: 40%"><span class="chart-label">"Sun"</span></div>
                                            </div>
                                        </div>

                                        // Latest signals
                                        <div class="lg:col-span-2 bg-navy-800/60 rounded-2xl border border-navy-700/50 p-6">
                                            <div class="flex justify-between items-center mb-4">
                                                <h3 class="text-lg font-bold text-white">"Latest Signals"</h3>
                                                <span class="text-navy-500 cursor-pointer hover:text-navy-300">"•••"</span>
                                            </div>
                                            <div class="space-y-3">
                                                {latest.into_iter().map(|item| {
                                                    let truncated = if item.content.len() > 40 {
                                                        format!("{}...", &item.content[..40])
                                                    } else {
                                                        item.content.clone()
                                                    };
                                                    view! {
                                                        <div class="bg-navy-900/50 rounded-xl p-4 border border-navy-700/30 hover:border-navy-600/50 transition-colors">
                                                            <div class="flex justify-between items-start mb-2">
                                                                <span class="text-xs font-bold px-2 py-0.5 rounded-md bg-primary-500/15 text-primary-400 border border-primary-500/20 uppercase">"Feedback"</span>
                                                                <span class="text-xs text-navy-500">{item.created_at.format("%H:%M").to_string()}</span>
                                                            </div>
                                                            <p class="text-sm text-navy-200 font-medium">{truncated}</p>
                                                        </div>
                                                    }
                                                }).collect_view()}
                                            </div>
                                            <a href="/admin/feedbacks" class="block text-center text-sm text-navy-400 hover:text-primary-400 mt-4 transition-colors">"View All Activity"</a>
                                        </div>
                                    </div>
                                </div>
                            }.into_any()
                        }
                    }
                }}
            </Transition>
        </div>
    }
}

// ─── FEEDBACK LIST PAGE ─────────────────────────────────────────────────────

#[component]
fn FeedbackListPage() -> impl IntoView {
    let feedbacks = Resource::new(|| (), |_| async move { get_feedbacks().await });

    view! {
        <div class="max-w-6xl mx-auto text-left">
            // Header
            <div class="mb-8">
                <p class="text-xs font-bold text-primary-400 uppercase tracking-widest mb-2">"Overview"</p>
                <h1 class="text-3xl font-bold text-white mb-1">"Feedback Loop"</h1>
                <p class="text-navy-400">"Analyze high-fidelity user submissions and architectural insights generated across the platform."</p>
            </div>

            <Transition fallback=move || view! { <div class="text-navy-400 animate-pulse">"Loading feedback..."</div> }>
                {move || {
                    match feedbacks.get() {
                        None => view! { <div></div> }.into_any(),
                        Some(Err(e)) => {
                            if e.to_string().contains("Tidak terautentikasi") {
                                let navigate = leptos_router::hooks::use_navigate();
                                let loc = leptos_router::hooks::use_location().pathname.get_untracked();
                                navigate(&format!("/login?returnUrl={}", loc), Default::default());
                                view! { <div>"Redirecting to login..."</div> }.into_any()
                            } else {
                                view! {
                                    <div class="bg-red-900/40 p-4 rounded-xl border border-red-700/50 text-red-300">
                                        {format!("Error: {}", e)}
                                    </div>
                                }.into_any()
                            }
                        },
                        Some(Ok(items)) => {
                            if items.is_empty() {
                                view! {
                                    <div class="bg-navy-800/60 p-12 rounded-2xl border border-navy-700/50 text-navy-400 text-center text-lg">
                                        "No feedback submitted yet."
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-5">
                                        {items.into_iter().enumerate().map(|(i, item)| {
                                            let initials = format!("{}", (b'A' + (i as u8 % 26)) as char);
                                            let colors = ["from-primary-500 to-primary-300", "from-tertiary-400 to-tertiary-300", "from-secondary-500 to-secondary-300", "from-primary-400 to-tertiary-400", "from-secondary-400 to-primary-300"];
                                            let color = colors[i % colors.len()];
                                            let time_str = item.created_at.format("%b %d, %H:%M").to_string();
                                            let truncated = if item.content.len() > 120 {
                                                format!("{}...", &item.content[..120])
                                            } else {
                                                item.content.clone()
                                            };

                                            view! {
                                                <div class="bg-navy-800/60 rounded-2xl border border-navy-700/40 p-5 hover:border-navy-600/60 transition-all duration-300 group hover:shadow-lg hover:shadow-navy-950/30 flex flex-col">
                                                    // Header with avatar + time
                                                    <div class="flex items-center justify-between mb-3">
                                                        <div class="flex items-center gap-3">
                                                            <div class=format!("w-9 h-9 rounded-full bg-gradient-to-br {} flex items-center justify-center text-white text-xs font-bold shadow-md", color)>
                                                                {initials}
                                                            </div>
                                                            <div>
                                                                <p class="text-sm font-semibold text-white">"Anonymous"</p>
                                                                <p class="text-xs text-navy-500">"User"</p>
                                                            </div>
                                                        </div>
                                                        <span class="text-xs text-navy-500 bg-navy-900/60 px-2 py-1 rounded-md">{time_str}</span>
                                                    </div>
                                                    // Content
                                                    <p class="text-sm text-navy-200 leading-relaxed flex-1 whitespace-pre-wrap">
                                                        {truncated}
                                                    </p>
                                                    // Footer tag
                                                    <div class="mt-4 pt-3 border-t border-navy-700/30">
                                                        <span class="text-xs font-bold px-2.5 py-1 rounded-md bg-primary-500/15 text-primary-400 border border-primary-500/20">"FEEDBACK"</span>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}

                                        // New entry card
                                        <a href="/" class="bg-navy-800/30 rounded-2xl border border-dashed border-navy-700/40 p-5 flex flex-col items-center justify-center text-center hover:border-primary-500/40 hover:bg-navy-800/50 transition-all duration-300 min-h-[200px] cursor-pointer group">
                                            <div class="w-12 h-12 rounded-full bg-navy-700/50 flex items-center justify-center text-2xl text-navy-400 group-hover:text-primary-400 group-hover:bg-primary-500/10 transition-all mb-3">
                                                "+"
                                            </div>
                                            <p class="text-sm font-semibold text-navy-400 group-hover:text-navy-300">"New Feedback Entry"</p>
                                            <p class="text-xs text-navy-500 mt-1">"Manually document external insight"</p>
                                        </a>
                                    </div>
                                }.into_any()
                            }
                        }
                    }
                }}
            </Transition>
        </div>
    }
}

// ─── 404 PAGE ───────────────────────────────────────────────────────────────

#[component]
fn NotFound() -> impl IntoView {
    #[cfg(feature = "ssr")]
    {
        use leptos_axum::ResponseOptions;
        let resp = expect_context::<ResponseOptions>();
        resp.set_status(http::StatusCode::NOT_FOUND);
    }
    view! {
        <div class="min-h-screen bg-navy-900 flex items-center justify-center">
            <div class="p-10 bg-navy-800/60 rounded-3xl shadow-xl border border-navy-700/50 max-w-md w-full text-center">
                <h1 class="text-6xl font-black mb-4 text-white">"404"</h1>
                <p class="text-navy-400 text-lg mb-8">"Lost in the dark void of space."</p>
                <a href="/" class="inline-block py-3 px-8 rounded-xl font-bold text-white transition-all duration-300 transform hover:-translate-y-1 hover:shadow-lg" style="background: linear-gradient(135deg, #3F51B5 0%, #5C6BC0 50%, #9FA8DA 100%)">
                    "Return to base"
                </a>
            </div>
        </div>
    }
}
