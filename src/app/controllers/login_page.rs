use leptos::prelude::*;

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_action = ServerAction::<crate::app::controllers::auth_controller::Login>::new();

    let pending = login_action.pending();
    let value = login_action.value();

    // Automatically redirect on successful login
    Effect::new(move |_| {
        if let Some(Ok(_)) = value.get() {
            let navigate = leptos_router::hooks::use_navigate();
            let query = leptos_router::hooks::use_query_map().get_untracked();
            let return_url = query.get("returnUrl").unwrap_or_else(|| "/admin".to_string());
            navigate(&return_url, Default::default());
        }
    });

    view! {
        <div class="min-h-screen bg-navy-900 flex items-center justify-center px-4">
            <div class="max-w-md w-full p-8 bg-navy-800/60 rounded-2xl shadow-2xl border border-navy-700/50 text-left backdrop-blur-sm">
                <div class="text-center mb-8">
                    <a href="/" class="text-xl font-bold tracking-wider bg-gradient-to-r from-primary-400 to-tertiary-400 bg-clip-text text-transparent">"Fitbek"</a>
                    <h1 class="text-3xl font-bold mt-4 text-white">"Admin Login"</h1>
                    <p class="text-navy-400 text-sm mt-1">"Sign in to access the admin console"</p>
                </div>

                <ActionForm action=login_action>
                    <div class="flex flex-col">
                        <label for="username" class="text-xs font-bold tracking-widest text-navy-300 uppercase mb-2">"Username"</label>
                        <input
                            type="text"
                            id="username"
                            name="username"
                            class="p-3.5 bg-navy-900/80 text-navy-50 border border-navy-600/50 rounded-xl focus:border-primary-500/50 focus:ring-1 focus:ring-primary-500/30 outline-none transition-all duration-300 mb-5"
                            required
                        />

                        <label for="password" class="text-xs font-bold tracking-widest text-navy-300 uppercase mb-2">"Password"</label>
                        <input
                            type="password"
                            id="password"
                            name="password"
                            class="p-3.5 bg-navy-900/80 text-navy-50 border border-navy-600/50 rounded-xl focus:border-primary-500/50 focus:ring-1 focus:ring-primary-500/30 outline-none transition-all duration-300 mb-6"
                            required
                        />

                        <div class="flex items-center gap-3">
                            <a
                                href="/"
                                class="bg-navy-700/60 hover:bg-navy-600/60 text-navy-200 text-center font-bold py-3 px-6 rounded-xl transition-all duration-300 w-full border border-navy-600/50"
                            >
                                "Back"
                            </a>
                            <button
                                type="submit"
                                disabled=move || pending.get()
                                class="w-full py-3 px-6 rounded-xl font-bold text-white tracking-wide transition-all duration-300 transform hover:-translate-y-0.5 hover:shadow-lg hover:shadow-primary-500/20 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none"
                                style="background: linear-gradient(135deg, #3F51B5 0%, #5C6BC0 50%, #9FA8DA 100%)"
                            >
                                {move || if pending.get() { "Logging in..." } else { "Login" }}
                            </button>
                        </div>
                    </div>
                </ActionForm>

                <Show when=move || value.get().is_some() fallback=|| view!{}>
                    {move || {
                        if let Some(Err(e)) = value.get() {
                            view! {
                                <div class="mt-6 p-4 rounded-xl bg-red-900/40 border border-red-700/50 text-red-300 text-sm">
                                    {e.to_string()}
                                </div>
                            }.into_any()
                        } else {
                            view! { <div></div> }.into_any()
                        }
                    }}
                </Show>
            </div>
        </div>
    }
}
