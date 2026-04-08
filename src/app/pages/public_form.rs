use crate::app::controllers::form_controller::{get_form_by_slug, submit_response};
use crate::app::models::form::{
    SubmitAnswerPayload, SubmitResponsePayload
};
use leptos::prelude::*;

#[component]
pub fn PublicFormView() -> impl IntoView {
    let params = leptos_router::hooks::use_params_map();
    let slug = move || params.with(|p| p.get("slug").unwrap_or_default());
    
    let form_res = Resource::new(slug, |s| async move {
        if !s.is_empty() {
            get_form_by_slug(s).await
        } else {
            Err(ServerFnError::new("Missing slug"))
        }
    });

    let pending = RwSignal::new(false);
    let success = RwSignal::new(false);
    let error_msg: RwSignal<Option<String>> = RwSignal::new(None);
    
    // Store answers in a signal that we can update via inputs. Map question_id to string answer.
    // Using a simple flat array of signals or just extracting values on submit. 
    // Since Leptos signals inside lists can be tricky, let's extract values via native DOM on submit.
    let form_ref = NodeRef::<leptos::html::Form>::new();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        use wasm_bindgen::JsCast;
        ev.prevent_default();
        pending.set(true);
        error_msg.set(None);

        let form_data = match form_res.get() {
            Some(Ok(f)) => f,
            _ => {
                pending.set(false);
                return;
            }
        };

        // We'll collect the answers manually since standard FormData might be tricky.
        let mut answers = Vec::new();
        
        let target = ev.target().unwrap();
        let web_sys_form = target.unchecked_into::<web_sys::HtmlFormElement>();
        let form_data_obj = web_sys::FormData::new_with_form(&web_sys_form).unwrap();
        
        for q in form_data.questions.iter() {
            let key = format!("q_{}", q.id);
            if q.question_type == "checkbox" {
                // Checkboxes can have multiple values
                let entries = form_data_obj.get_all(&key);
                let vals: Vec<String> = entries.iter().filter_map(|val: wasm_bindgen::JsValue| val.as_string()).collect();
                if q.is_required && vals.is_empty() {
                    error_msg.set(Some(format!("Please answer the required question: {}", q.label)));
                    pending.set(false);
                    return;
                }
                let answer_str = if vals.is_empty() { None } else { Some(serde_json::to_string(&vals).unwrap()) };
                answers.push(SubmitAnswerPayload {
                    question_id: q.id.clone(),
                    answer_value: answer_str
                });
            } else {
                let val = form_data_obj.get(&key).as_string();
                if q.is_required && (val.is_none() || val.as_ref().unwrap().trim().is_empty()) {
                    error_msg.set(Some(format!("Please answer the required question: {}", q.label)));
                    pending.set(false);
                    return;
                }
                answers.push(SubmitAnswerPayload {
                    question_id: q.id.clone(),
                    answer_value: val
                });
            }
        }

        let payload = SubmitResponsePayload {
            form_id: form_data.id.to_string(),
            answers,
        };

        leptos::task::spawn_local(async move {
            match submit_response(payload).await {
                Ok(_) => {
                    success.set(true);
                    pending.set(false);
                }
                Err(e) => {
                    error_msg.set(Some(e.to_string()));
                    pending.set(false);
                }
            }
        });
    };

    view! {
        <Transition fallback=move || view! { <div class="mt-20 text-navy-300 animate-pulse text-xl">"Loading form..."</div> }>
            {move || {
                match form_res.get() {
                    None => view! { <div></div> }.into_any(),
                    Some(Err(e)) => view! {
                        <div class="mt-20 bg-red-900/50 p-6 rounded-xl border border-red-700 text-red-200 max-w-2xl mx-auto">
                            {format!("Form not found: {}", e)}
                        </div>
                    }.into_any(),
                    Some(Ok(f)) => {
                        let f_info = f.clone();
                        view! {
                            <div class="flex-1 flex flex-col items-center justify-center px-4 py-12 min-h-screen">
                                <Show when=move || success.get() fallback=move || {
                                    let cloned_f = f.clone();
                                    let desc_for_check = cloned_f.description.clone();
                                    let desc_for_render = cloned_f.description.clone();
                                    view! {
                                        <div class="w-full flex flex-col items-center">
                                            // Hero section matching HomePage
                                            <div class="text-center mb-10 max-w-2xl">
                                                <h1 class="text-4xl md:text-5xl font-bold leading-tight mb-4 text-white">
                                                    {cloned_f.title.clone()}
                                                </h1>
                                                <Show when=move || desc_for_check.is_some()>
                                                    <p class="text-navy-400 text-lg">{desc_for_render.clone().unwrap()}</p>
                                                </Show>
                                            </div>

                                            // Form card matching HomePage
                                            <div class="w-full max-w-xl bg-navy-800/60 backdrop-blur-sm rounded-2xl border border-navy-700/50 p-8 shadow-2xl shadow-navy-950/50 text-left">
                                                <form node_ref=form_ref on:submit=on_submit class="space-y-6">
                                                    {cloned_f.questions.into_iter().map(|q| {
                                                        let is_req = q.is_required;
                                                        let key_name = format!("q_{}", q.id);
                                                        view! {
                                                            <div>
                                                                <label class="block text-xs font-bold tracking-widest text-navy-300 uppercase mb-2">
                                                                    {q.label.clone()}
                                                                    {if is_req { view! { <span class="text-red-500 ml-1">"*"</span> }.into_any() } else { view! { <span></span> }.into_any() }}
                                                                </label>
                                                                
                                                                <div class="mt-2 text-navy-100">
                                                                {if q.question_type == "short_text" {
                                                                    view! {
                                                                        <input 
                                                                            type="text" 
                                                                            name=key_name 
                                                                            class="w-full p-4 bg-navy-900/80 text-navy-50 border border-navy-600/50 rounded-xl focus:border-primary-500/50 focus:ring-1 focus:ring-primary-500/30 outline-none transition-all duration-300 placeholder-navy-500"
                                                                            placeholder="Your answer"
                                                                        />
                                                                    }.into_any()
                                                                } else if q.question_type == "long_text" {
                                                                    view! {
                                                                        <textarea 
                                                                            name=key_name 
                                                                            class="w-full p-4 bg-navy-900/80 text-navy-50 border border-navy-600/50 rounded-xl focus:border-primary-500/50 focus:ring-1 focus:ring-primary-500/30 outline-none transition-all duration-300 resize-y min-h-[100px] placeholder-navy-500"
                                                                            placeholder="Your answer"
                                                                        ></textarea>
                                                                    }.into_any()
                                                                } else if q.question_type == "radio" {
                                                                    let opts: Vec<String> = serde_json::from_str(&q.options.unwrap_or_default()).unwrap_or_default();
                                                                    let k = key_name.clone();
                                                                    view! {
                                                                        <div class="space-y-3 mt-2 text-navy-100">
                                                                            {opts.into_iter().map(move |opt: String| {
                                                                                let inner_k = k.clone();
                                                                                view! {
                                                                                    <label class="flex items-center space-x-3 cursor-pointer group">
                                                                                        <input type="radio" name=inner_k value=opt.clone() class="form-radio h-5 w-5 text-primary-500 border-navy-600 bg-navy-900 focus:ring-offset-navy-800" />
                                                                                        <span class="group-hover:text-white transition-colors">{opt}</span>
                                                                                    </label>
                                                                                }
                                                                            }).collect_view()}
                                                                        </div>
                                                                    }.into_any()
                                                                } else if q.question_type == "checkbox" {
                                                                    let opts: Vec<String> = serde_json::from_str(&q.options.unwrap_or_default()).unwrap_or_default();
                                                                    let k = key_name.clone();
                                                                    view! {
                                                                        <div class="space-y-3 mt-2 text-navy-100">
                                                                            {opts.into_iter().map(move |opt: String| {
                                                                                let inner_k = k.clone();
                                                                                view! {
                                                                                    <label class="flex items-center space-x-3 cursor-pointer group">
                                                                                        <input type="checkbox" name=inner_k value=opt.clone() class="form-checkbox h-5 w-5 rounded text-primary-500 border-navy-600 bg-navy-900 focus:ring-offset-navy-800" />
                                                                                        <span class="group-hover:text-white transition-colors">{opt}</span>
                                                                                    </label>
                                                                                }
                                                                            }).collect_view()}
                                                                        </div>
                                                                    }.into_any()
                                                                } else {
                                                                    view! { <div>"Unknown input type"</div> }.into_any()
                                                                }}
                                                            </div>
                                                        </div>
                                                    }
                                                }).collect_view()}

                                                    <Show when=move || error_msg.get().is_some() fallback=|| view! {}>
                                                        <div class="p-3 rounded-lg bg-red-900/40 border border-red-700/50 text-red-300 text-sm">
                                                            {move || error_msg.get().unwrap_or_default()}
                                                        </div>
                                                    </Show>

                                                    <p class="text-xs text-navy-500 flex items-center gap-1.5 pt-2">
                                                        <span>"🔒"</span>
                                                        " Submissions are processed securely."
                                                    </p>

                                                    <div class="pt-2">
                                                        <button
                                                            type="submit"
                                                            disabled=move || pending.get()
                                                            class="w-full py-3.5 px-6 rounded-xl font-bold text-white tracking-wide transition-all duration-300 transform hover:-translate-y-0.5 hover:shadow-lg hover:shadow-primary-500/20 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none bg-primary-500 hover:bg-primary-600"
                                                        >
                                                            {move || if pending.get() { "Submitting..." } else { "Submit Feedback  ▶" }}
                                                        </button>
                                                        <p class="text-xs text-navy-500 mt-4 text-center">
                                                            "Powered by Fitbek"
                                                        </p>
                                                    </div>
                                                </form>
                                            </div>
                                        </div>
                                    }
                                }>
                                    <div class="w-full max-w-xl bg-navy-800/60 backdrop-blur-sm rounded-2xl border border-navy-700/50 p-8 shadow-2xl shadow-navy-950/50 text-center py-16">
                                        <div class="text-5xl mb-4">"🙌"</div>
                                        <h2 class="text-2xl font-bold text-white mb-2">"Thank You!"</h2>
                                        <p class="text-navy-400 mb-6">"Your response has been recorded."</p>
                                        <a href="/" class="text-primary-400 hover:text-primary-300 underline underline-offset-4 transition-colors block">
                                            "Submit another form"
                                        </a>
                                    </div>
                                </Show>
                            </div>
                        }.into_any()
                    }
                }
            }}
        </Transition>
    }
}
