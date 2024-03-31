use std::collections::HashMap;

use leptos::html::Input;
use leptos::*;
use srtemplate::SrTemplate;

fn and(o: Option<HtmlElement<Input>>, f: impl Fn(String) -> bool) -> Option<HtmlElement<Input>> {
    if let Some(ref v) = o {
        if f(v.value()) {
            return o;
        }
    }
    None
}

#[component]
fn App() -> impl IntoView {
    let (ctx, set_ctx) = create_signal(SrTemplate::default());
    let (variables, set_variables) = create_signal(HashMap::from([(
        "var".to_string(),
        "    test    ".to_string(),
    )]));
    let (template_str, set_template) = create_signal(String::from("This is a {{ trim(var) }}"));

    let rendered = create_memo(move |_| format!("{:?}", ctx.get().render(&template_str.get())));

    let input_name_ref = create_node_ref::<Input>();
    let input_value_ref = create_node_ref::<Input>();

    let input_delimiter_start_ref = create_node_ref::<Input>();
    let input_delimiter_close_ref = create_node_ref::<Input>();

    create_effect(move |_| {
        for (k, v) in variables.get().iter() {
            set_ctx.update(move |last| last.add_variable(k.clone(), v));
        }
    });

    view! {
        <div class="lg:w-full lg:h-full flex flex-col justify-center items-center">
            <h1 class="text-bold text-6xl mb-2 text-center">SrTemplate Realtime Render</h1>
            <span class="text-md mb-8 text-gray-500 text-center">Custom Functions are not currently supported for this web</span>

            <section class="flex flex-col lg:flex-row gap-8 mt-8">
                <div class="flex flex-col gap-4">
                    <div class="flex flex-col">
                        <h3 class="text-2xl text-bold">Variables to render</h3>
                        <span class="text-sm text-gray-500">Use the same variable name to replace value</span>
                    </div>
                    <div class="flex flex-col justify-center max-w-150px">
                        <div class="flex flex-row justify-between mb-2">
                            <input
                                class="bg-orange-300/30 text-black focus:outline-none p-3 max-w-[200px]"
                                prop:type="text"
                                prop:placeholder="Variable Name"
                                node_ref=input_name_ref
                            />
                            <input
                                class="bg-orange-300/30 text-black focus:outline-none p-3 max-w-[200px]"
                                prop:type="text"
                                prop:placeholder="Variable Value"
                                node_ref=input_value_ref
                            />
                            <button
                                class="bg-orange-300/50 hover:bg-orange-300/70 px-4 py-3"
                                on:click=move |_| set_variables.update(|last| {
                                    let Some(name) = and(input_name_ref.get(), |v| !v.is_empty()) else { return; };
                                    let Some(value) = and(input_value_ref.get(), |v| !v.is_empty()) else { return; };
                                    last.insert(name.value(), value.value());
                                })
                            >
                                Add
                            </button>
                        </div>
                        <span class="px-4 pt-3 block font-bold">Name: Value</span>
                        <div class="overflow-y-auto w-full h-[100px]">
                            {move || variables.get().iter().map(|(k, v)| view! {
                                <span class="px-4 py-3 block">{k}: {v}</span>
                            }).collect::<Vec<_>>()}
                        </div>
                    </div>
                    <div class="flex flex-row justify-between items-end">
                        <h3 class="text-2xl text-bold">Template to render</h3>
                        <a class="text-sm" prop:href="https://github.com/SergioRibera/srtemplate/wiki/Template-Syntaxis">How syntax works</a>
                    </div>
                    <textarea
                        class="bg-orange-300/30 resize-none w-[500px] h-[250px] focus:outline-none p-4"
                        prop:defaultValue={template_str}
                        on:input={move |e| {
                            set_template.update(|v| *v = event_target_value(&e))
                        }}
                    />
                </div>
                <div class="flex flex-col gap-4 lg:ml-4">
                    <div class="flex flex-col">
                        <h3 class="text-2xl text-bold">Delimiter</h3>
                    </div>
                    <div class="flex flex-row justify-between items-end mb-2">
                        <div class="flex flex-col">
                            <span class="pt-3 block">Start Delimiter</span>
                            <input
                                class="bg-orange-300/30 text-black focus:outline-none p-3 max-w-[200px]"
                                prop:type="text"
                                prop:placeholder="{{"
                                prop:defaultValue="{{"
                                node_ref=input_delimiter_start_ref
                            />
                        </div>
                        <div class="flex flex-col">
                            <span class="px-4 pt-3 block">Start Delimiter</span>
                            <input
                                class="bg-orange-300/30 text-black focus:outline-none p-3 max-w-[200px]"
                                prop:type="text"
                                prop:placeholder="}}"
                                prop:defaultValue="}}"
                                node_ref=input_delimiter_close_ref
                            />
                        </div>
                        <button
                            class="bg-orange-300/50 hover:bg-orange-300/70 h-[48px] px-4 py-3"
                            on:click=move |_| set_ctx.update(|ctx| {
                                let Some(start) = and(input_delimiter_start_ref.get(), |v| !v.is_empty()) else { return; };
                                let Some(close) = and(input_delimiter_close_ref.get(), |v| !v.is_empty()) else { return; };
                                ctx.set_delimiter(start.value(), close.value());
                            })
                        >
                            Change
                        </button>
                    </div>
                    <h3 class="text-2xl text-bold">Render Result</h3>
                    <textarea
                        class="bg-orange-300/30 resize-none w-[500px] h-[365px] focus:outline-none p-4"
                        prop:readonly={true}
                        prop:defaultValue={rendered}
                    />
                </div>
            </section>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
