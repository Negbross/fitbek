use crate::app::controllers::form_controller::{create_form, get_form_responses, get_forms, DeleteForm, UpdateForm, get_form_by_slug};
use crate::app::models::form::{
    CreateFormPayload, CreateFormQuestionPayload
};
use leptos::prelude::*;

#[component]
pub fn AdminFormsList() -> impl IntoView {
    let forms_res = Resource::new(|| (), |_| async move { get_forms().await });

    view! {
        <div class="max-w-5xl w-full p-6 mx-auto mt-6">
            <header class="flex justify-between items-center mb-8 pb-4 border-b border-navy-700">
                <h1 class="text-4xl font-bold text-white">"Form Manager"</h1>
                <div class="flex items-center gap-4">
                    <a
                        href="/admin/forms/new"
                        class="bg-blue-600 hover:bg-blue-500 text-white px-6 py-2 rounded-lg transition-colors border border-blue-500 shadow-lg"
                    >
                        "+ Create Form"
                    </a>
                    <a
                        href="/admin"
                        class="bg-navy-700 hover:bg-navy-600 text-navy-100 px-4 py-2 rounded-lg transition-colors border border-navy-600"
                    >
                        "Back to Dashboard"
                    </a>
                </div>
            </header>

            <Transition fallback=move || view! { <div class="text-navy-300 animate-pulse">"Loading forms..."</div> }>
                {move || {
                    let items = forms_res.get();
                    match items {
                        None => view! { <div></div> }.into_any(),
                        Some(Err(e)) => {
                            if e.to_string().contains("Tidak terautentikasi") {
                                let navigate = leptos_router::hooks::use_navigate();
                                let loc = leptos_router::hooks::use_location().pathname.get_untracked();
                                navigate(&format!("/login?returnUrl={}", loc), Default::default());
                                view! { <div>"Redirecting to login..."</div> }.into_any()
                            } else {
                                view! {
                                    <div class="bg-red-900/50 p-6 rounded-xl border border-red-700 text-red-200">
                                        {format!("Error: {}", e)}
                                    </div>
                                }.into_any()
                            }
                        },
                        Some(Ok(items)) => {
                            if items.is_empty() {
                                view! {
                                    <div class="bg-navy-800 p-12 rounded-2xl border border-navy-700 text-navy-300 text-center">
                                        "No forms created yet."
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 text-left">
                                        {items.into_iter().map(|item| {
                                            let item_title = item.title;
                                            let item_response_count = item.response_count;
                                            let item_url_slug = item.url_slug;
                                            let item_id_delete = item.id.clone();
                                            let item_id_responses = item.id.clone();
                                            let slug_link = format!("/f/{}", item_url_slug);
                                            let slug_link_text = slug_link.clone();
                                            let edit_link = format!("/admin/forms/{}/edit", item_url_slug);
                                            let response_link = format!("/admin/forms/{}/responses", item_id_responses);
                                            
                                            view! {
                                                <div class="bg-navy-800 p-6 rounded-2xl shadow-lg border border-navy-700 hover:border-blue-500 transition-colors duration-300 flex flex-col justify-between">
                                                    <div>
                                                        <h2 class="text-2xl font-bold text-white mb-2">{item_title}</h2>
                                                        <div class="text-navy-300 text-sm mb-4">
                                                            "Responses: " <span class="text-blue-400 font-bold">{item_response_count}</span>
                                                        </div>
                                                        <div class="bg-navy-900 py-2 px-3 rounded-lg text-sm text-navy-200 border border-navy-700 flex items-center justify-between">
                                                            <span>"Share: " <a href=slug_link target="_blank" class="text-blue-400 hover:underline">{slug_link_text}</a></span>
                                                        </div>
                                                    </div>
                                                    <div class="mt-6 flex justify-end gap-2">
                                                        <a
                                                            href=edit_link
                                                            class="bg-blue-900 hover:bg-blue-800 text-blue-200 px-3 py-2 rounded-lg transition-colors border border-blue-700 text-sm"
                                                        >
                                                            "Edit Info"
                                                        </a>
                                                        <ActionForm action={ServerAction::<DeleteForm>::new()}>
                                                            <input type="hidden" name="form_id" value=item_id_delete/>
                                                            <button
                                                                type="submit"
                                                                class="bg-red-900 hover:bg-red-800 text-red-200 px-3 py-2 rounded-lg transition-colors border border-red-700 text-sm"
                                                                on:click=move |ev| {
                                                                    if !window().confirm_with_message("Are you sure you want to delete this form and all its responses?").unwrap_or(false) {
                                                                        ev.prevent_default();
                                                                    }
                                                                }
                                                            >
                                                                "Delete"
                                                            </button>
                                                        </ActionForm>
                                                        <a
                                                            href=response_link
                                                            class="bg-navy-700 hover:bg-navy-600 text-white px-4 py-2 rounded-lg transition-colors border border-navy-600 ml-2 text-sm"
                                                        >
                                                            "View Responses"
                                                        </a>
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
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

#[component]
pub fn AdminFormBuilder() -> impl IntoView {
    let title = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());

    #[derive(Clone)]
    struct LocalQuestion {
        id: usize,
        qtype: RwSignal<String>,
        label: RwSignal<String>,
        options: RwSignal<String>,
        is_required: RwSignal<bool>,
    }

    let questions = RwSignal::new(Vec::<LocalQuestion>::new());
    let next_id = RwSignal::new(0);
    
    let pending = RwSignal::new(false);
    let success_slug: RwSignal<Option<String>> = RwSignal::new(None);
    let error_msg: RwSignal<Option<String>> = RwSignal::new(None);

    let add_question = move |_| {
        let id = next_id.get_untracked();
        next_id.set(id + 1);
        
        questions.update(|qs| {
            qs.push(LocalQuestion {
                id,
                qtype: RwSignal::new("short_text".to_string()),
                label: RwSignal::new("".to_string()),
                options: RwSignal::new("Option 1, Option 2".to_string()),
                is_required: RwSignal::new(true),
            });
        });
    };

    let remove_question = move |id_to_remove: usize| {
        questions.update(|qs| {
            qs.retain(|q| q.id != id_to_remove);
        });
    };

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();
        pending.set(true);
        error_msg.set(None);

        let t = title.get_untracked();
        let d = description.get_untracked();
        if t.trim().is_empty() {
            error_msg.set(Some("Title is required.".to_string()));
            pending.set(false);
            return;
        }

        let qs = questions.get_untracked();
        let mut built_qs = Vec::new();
        for (i, q) in qs.iter().enumerate() {
            let label = q.label.get_untracked();
            if label.trim().is_empty() {
                error_msg.set(Some(format!("Question #{} is missing a label.", i + 1)));
                pending.set(false);
                return;
            }
            
            let qtype = q.qtype.get_untracked();
            let mut opts = None;
            if qtype == "radio" || qtype == "checkbox" {
                let o = q.options.get_untracked();
                let list: Vec<String> = o.split(',').map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
                opts = Some(serde_json::to_string(&list).unwrap_or_default());
            }

            built_qs.push(CreateFormQuestionPayload {
                question_type: qtype,
                label,
                options: opts,
                is_required: q.is_required.get_untracked(),
                order_index: i as i32,
            });
        }

        let payload = CreateFormPayload {
            title: t,
            description: if d.is_empty() { None } else { Some(d) },
            questions: built_qs,
        };

        leptos::task::spawn_local(async move {
            match create_form(payload).await {
                Ok(slug) => {
                    success_slug.set(Some(slug));
                    pending.set(false);
                }
                Err(e) => {
                    if e.to_string().contains("Tidak terautentikasi") {
                        let navigate = leptos_router::hooks::use_navigate();
                        let loc = leptos_router::hooks::use_location().pathname.get_untracked();
                        navigate(&format!("/login?returnUrl={}", loc), Default::default());
                    } else {
                        error_msg.set(Some(e.to_string()));
                    }
                    pending.set(false);
                }
            }
        });
    };

    view! {
        <div class="max-w-4xl w-full p-6 mx-auto mt-6 text-left">
            <header class="flex justify-between items-center mb-6 border-b border-navy-700 pb-4">
                <h1 class="text-3xl font-bold text-white">"Create New Form"</h1>
                <a href="/admin/forms" class="text-navy-300 hover:text-white underline">"Cancel"</a>
            </header>

            <Show when=move || success_slug.get().is_some() fallback=move || view! {
                <form on:submit=on_submit class="space-y-8">
                    <div class="bg-navy-800 p-8 rounded-2xl border-t-8 border-t-blue-500 shadow-xl">
                        <input
                            type="text"
                            placeholder="Form Title"
                            class="w-full bg-transparent text-4xl font-bold text-white mb-4 border-b border-navy-600 focus:border-blue-500 outline-none pb-2 transition-colors"
                            prop:value=move || title.get()
                            on:input=move |ev| title.set(event_target_value(&ev))
                        />
                        <textarea
                            placeholder="Form Description"
                            class="w-full bg-transparent text-navy-200 border-b border-navy-600 focus:border-blue-500 outline-none pb-2 transition-colors resize-none"
                            prop:value=move || description.get()
                            on:input=move |ev| description.set(event_target_value(&ev))
                        ></textarea>
                    </div>

                    <div class="space-y-6">
                        <For
                            each=move || questions.get()
                            key=|q| q.id
                            children=move |q| {
                                let id = q.id;
                                let qtype = q.qtype;
                                let label = q.label;
                                let options = q.options;
                                let is_required = q.is_required;
                                
                                view! {
                                    <div class="bg-navy-800 p-6 rounded-2xl shadow-md border border-navy-700 hover:border-navy-500 transition-colors relative">
                                        <button 
                                            type="button" 
                                            class="absolute top-4 right-4 text-red-400 hover:text-red-300"
                                            on:click=move |_| remove_question(id)
                                        >
                                            "✕ Remove"
                                        </button>
                                        
                                        <div class="flex flex-col md:flex-row gap-4 mb-4">
                                            <input
                                                type="text"
                                                placeholder="Question"
                                                class="flex-1 bg-navy-900 p-3 rounded-lg text-white border border-navy-600 focus:border-blue-500 outline-none"
                                                prop:value=move || label.get()
                                                on:input=move |ev| label.set(event_target_value(&ev))
                                            />
                                            <select
                                                class="bg-navy-900 p-3 rounded-lg text-white border border-navy-600 focus:border-blue-500 outline-none cursor-pointer"
                                                prop:value=move || qtype.get()
                                                on:change=move |ev| qtype.set(event_target_value(&ev))
                                            >
                                                <option value="short_text">"Short Text"</option>
                                                <option value="long_text">"Long Text"</option>
                                                <option value="radio">"Multiple Choice"</option>
                                                <option value="checkbox">"Checkboxes"</option>
                                            </select>
                                        </div>

                                        {move || {
                                            let t = qtype.get();
                                            if t == "radio" || t == "checkbox" {
                                                view! {
                                                    <div class="mb-4">
                                                        <label class="block text-navy-300 text-sm mb-2">"Options (comma separated)"</label>
                                                        <input
                                                            type="text"
                                                            class="w-full bg-navy-900 p-3 rounded-lg text-navy-100 border border-navy-600 focus:border-blue-500 outline-none"
                                                            placeholder="Option 1, Option 2, Option 3"
                                                            prop:value=move || options.get()
                                                            on:input=move |ev| options.set(event_target_value(&ev))
                                                        />
                                                    </div>
                                                }.into_any()
                                            } else {
                                                view! { <div></div> }.into_any()
                                            }
                                        }}

                                        <div class="flex justify-end border-t border-navy-700 mt-4 pt-4">
                                            <label class="flex items-center space-x-2 text-navy-200 cursor-pointer">
                                                <input
                                                    type="checkbox"
                                                    class="form-checkbox h-5 w-5 text-blue-500 rounded border-navy-600 bg-navy-900"
                                                    prop:checked=move || is_required.get()
                                                    on:change=move |ev| is_required.set(event_target_checked(&ev))
                                                />
                                                <span>"Required"</span>
                                            </label>
                                        </div>
                                    </div>
                                }
                            }
                        />
                    </div>

                    <div class="flex justify-center border-t border-b border-navy-700 py-6 my-6 bg-navy-800 rounded-2xl shadow border-dashed">
                        <button
                            type="button"
                            on:click=add_question
                            class="flex items-center space-x-2 text-blue-400 font-bold hover:text-blue-300 transition-colors bg-navy-900 px-6 py-3 rounded-full shadow-inner"
                        >
                            <span class="text-xl">"+"</span>
                            <span>"Add Question"</span>
                        </button>
                    </div>

                    <Show when=move || error_msg.get().is_some() fallback=|| view! {}>
                        <div class="p-4 rounded-xl bg-red-900/50 border border-red-700 text-red-200">
                            {move || error_msg.get().unwrap_or_default()}
                        </div>
                    </Show>

                    <div class="flex justify-end gap-4 mt-8 pb-10">
                        <button
                            type="submit"
                            disabled=move || pending.get()
                            class="bg-blue-600 hover:bg-blue-500 disabled:bg-blue-900 text-white font-bold py-3 px-8 rounded-xl transition duration-300 transform hover:-translate-y-1 shadow-lg"
                        >
                            {move || if pending.get() { "Saving..." } else { "Save Form" }}
                        </button>
                    </div>
                </form>
            }>
                <div class="bg-navy-800 p-10 rounded-2xl border-t-8 border-t-green-500 shadow-2xl text-center mt-10">
                    <h2 class="text-4xl font-bold text-white mb-6">"Form Created Successsfully!"</h2>
                    <p class="text-navy-300 mb-8 text-lg">"Your form is live and ready to receive feedback."</p>
                    <div class="bg-navy-900 p-6 rounded-xl border border-navy-700 inline-block mb-8">
                        <p class="text-sm tracking-wide text-navy-400 mb-2 uppercase">"Shareable Link"</p>
                        <a href=format!("/f/{}", success_slug.get().unwrap()) target="_blank" class="text-2xl font-mono text-blue-400 hover:underline">
                            {format!("/f/{}", success_slug.get().unwrap())}
                        </a>
                    </div>
                    <div>
                        <a href="/admin/forms" class="bg-navy-700 hover:bg-navy-600 text-white font-bold py-3 px-8 rounded-xl transition-colors inline-block">
                            "Back to Forms"
                        </a>
                    </div>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn AdminFormResponses() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let form_id = move || params.with(|p| p.get("id").unwrap_or_default());
    
    let responses_res = Resource::new(form_id, |id| async move {
        if !id.is_empty() {
            get_form_responses(id).await
        } else {
            Ok(vec![])
        }
    });

    view! {
        <div class="max-w-6xl w-full p-6 mx-auto mt-6">
            <header class="flex justify-between items-center mb-8 pb-4 border-b border-navy-700">
                <h1 class="text-3xl font-bold text-white">"Form Responses"</h1>
                <a
                    href="/admin/forms"
                    class="bg-navy-700 hover:bg-navy-600 text-navy-100 px-4 py-2 rounded-lg transition-colors border border-navy-600"
                >
                    "Back to Forms"
                </a>
            </header>

            <Transition fallback=move || view! { <div class="text-navy-300 animate-pulse">"Loading responses..."</div> }>
                {move || {
                    match responses_res.get() {
                        None => view! { <div></div> }.into_any(),
                        Some(Err(e)) => {
                            if e.to_string().contains("Tidak terautentikasi") {
                                let navigate = leptos_router::hooks::use_navigate();
                                let loc = leptos_router::hooks::use_location().pathname.get_untracked();
                                navigate(&format!("/login?returnUrl={}", loc), Default::default());
                                view! { <div>"Redirecting to login..."</div> }.into_any()
                            } else {
                                view! {
                                    <div class="bg-red-900/50 p-6 rounded-xl border border-red-700 text-red-200">
                                        {format!("Error: {}", e)}
                                    </div>
                                }.into_any()
                            }
                        },
                        Some(Ok(items)) => {
                            if items.is_empty() {
                                view! {
                                    <div class="bg-navy-800 p-12 rounded-2xl border border-navy-700 text-navy-300 text-center text-lg">
                                        "No responses yet."
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <div class="space-y-6 text-left">
                                        {items.into_iter().map(|item| {
                                            view! {
                                                <div class="bg-navy-800 p-6 rounded-2xl shadow-lg border border-navy-700">
                                                    <div class="mb-4 pb-2 border-b border-navy-700 flex justify-between">
                                                        <span class="text-sm font-mono text-navy-400">"Response #" {item.id}</span>
                                                        <span class="text-sm italic text-navy-300">{item.submitted_at.unwrap_or_default()}</span>
                                                    </div>
                                                    <div class="space-y-4">
                                                        {item.answers.into_iter().map(|ans| {
                                                            view! {
                                                                <div class="bg-navy-900/50 p-4 rounded-xl">
                                                                    <div class="text-xs text-navy-400 uppercase tracking-widest mb-1">{ans.question_label.unwrap_or(ans.question_id.clone())}</div>
                                                                    <div class="text-navy-50 font-medium whitespace-pre-wrap">{ans.answer_value.unwrap_or_else(|| "-".to_string())}</div>
                                                                </div>
                                                            }
                                                        }).collect_view()}
                                                    </div>
                                                </div>
                                            }
                                        }).collect_view()}
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

#[component]
pub fn AdminFormEdit() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let slug = move || params.with(|p| p.get("id").unwrap_or_default());
    
    let form_res = Resource::new(slug, |s| async move {
        if !s.is_empty() {
            get_form_by_slug(s).await
        } else {
            Err(ServerFnError::new("Missing form slug"))
        }
    });

    let update_action = ServerAction::<UpdateForm>::new();
    let pending = update_action.pending();
    let value = update_action.value();

    view! {
        <div class="max-w-4xl w-full p-6 mx-auto mt-6 text-left">
            <header class="flex justify-between items-center mb-6 border-b border-navy-700 pb-4">
                <h1 class="text-3xl font-bold text-white">"Edit Form Info"</h1>
                <a href="/admin/forms" class="text-navy-300 hover:text-white underline">"Cancel"</a>
            </header>

            <Show when=move || value.get().is_some() fallback=move || view! {}>
                {move || {
                    let v = value.get().unwrap();
                    match v {
                        Ok(_) => view! {
                            <div class="p-4 rounded-xl bg-green-900/50 border border-green-700 text-green-200 mb-6">
                                "Form updated successfully! "
                                <a href="/admin/forms" class="underline">"Back to forms"</a>
                            </div>
                        }.into_any(),
                        Err(e) => view! {
                            <div class="p-4 rounded-xl bg-red-900/50 border border-red-700 text-red-200 mb-6">
                                {format!("Error: {}", e)}
                            </div>
                        }.into_any()
                    }
                }}
            </Show>

            <Transition fallback=move || view! { <div class="text-navy-300 animate-pulse">"Loading form..."</div> }>
                {move || match form_res.get() {
                    None => view! { <div></div> }.into_any(),
                    Some(Err(e)) => view! {
                        <div class="bg-red-900/50 p-6 rounded-xl border border-red-700 text-red-200">
                            {format!("Error: {}", e)}
                        </div>
                    }.into_any(),
                    Some(Ok(f)) => {
                        let f_id = f.id.to_string();
                        let f_title = f.title.clone();
                        let f_desc = f.description.clone().unwrap_or_default();
                        
                        view! {
                            <ActionForm action=update_action>
                                <div class="space-y-8">
                                    <div class="bg-navy-800 p-8 rounded-2xl border-t-8 border-t-blue-500 shadow-xl">
                                        <input type="hidden" name="payload[id]" value=f_id/>
                                        <div class="mb-4">
                                            <label class="block text-navy-300 mb-2 font-bold">"Title"</label>
                                            <input
                                                type="text"
                                                name="payload[title]"
                                                class="w-full bg-navy-900 p-3 rounded-lg text-white border border-navy-600 focus:border-blue-500 outline-none"
                                                value=f_title
                                            />
                                        </div>
                                        <div class="mb-4">
                                            <label class="block text-navy-300 mb-2 font-bold">"Description"</label>
                                            <textarea
                                                name="payload[description]"
                                                class="w-full bg-navy-900 p-3 rounded-lg text-white border border-navy-600 focus:border-blue-500 outline-none h-32 resize-none"
                                            >{f_desc}</textarea>
                                        </div>
                                    </div>
                                    
                                    <div class="flex justify-end gap-4 mt-8 pb-10">
                                        <button
                                            type="submit"
                                            disabled=move || pending.get()
                                            class="bg-blue-600 hover:bg-blue-500 disabled:bg-blue-900 text-white font-bold py-3 px-8 rounded-xl transition duration-300 transform hover:-translate-y-1 shadow-lg"
                                        >
                                            {move || if pending.get() { "Saving..." } else { "Save Changes" }}
                                        </button>
                                    </div>
                                </div>
                            </ActionForm>
                        }.into_any()
                    }
                }}
            </Transition>
        </div>
    }
}
