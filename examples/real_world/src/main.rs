use std::collections::HashMap;

use leptos::html::Input;
use leptos::*;
use srtemplate::{Error, SrTemplate};

fn and(o: Option<HtmlElement<Input>>, f: impl Fn(String) -> bool) -> Option<HtmlElement<Input>> {
    if let Some(ref v) = o {
        if f(v.value()) {
            return o;
        }
    }
    None
}

fn render_time(ctx: SrTemplate, s: &str) -> (Option<Error>, Option<String>, String) {
    let start = instant::Instant::now();
    let render = ctx.render(s);
    let duration = start.elapsed();
    let seconds = duration.as_secs();
    let milliseconds = duration.subsec_millis();
    let nanoseconds = duration.subsec_nanos();

    (
        render.clone().err(),
        render.ok(),
        format!(
            "{}chars {seconds}s {milliseconds}ms {nanoseconds}ns",
            s.chars().count()
        ),
    )
}

#[component]
fn App() -> impl IntoView {
    let (ctx, set_ctx) = create_signal(SrTemplate::default());
    let (variables, set_variables) = create_signal(HashMap::from([(
        "var".to_string(),
        "    test    ".to_string(),
    )]));
    let (template_str, set_template) = create_signal(String::from("This is a {{ trim(var) }}"));
    let (time_str, set_time) = create_signal(String::from("0s 0ms 0ns"));

    let has = create_memo(move |_| {
        let (err, render, time) = render_time(ctx.get(), &template_str.get());
        set_time.set(time);
        (err, render)
    });

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
                    <div class="flex flex-row justify-between items-end mb-2 w-[500px]">
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
                            <span class="px-4 pt-3 block">End Delimiter</span>
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
                    <div class="flex flex-row gap-2 items-end">
                        <button
                            class="bg-orange-300/50 hover:bg-orange-300/70 px-4 py-3"
                            on:click=move |_| set_time.update(|last| {
                                let (_, _, time) = render_time(ctx.get(), &template_str.get());
                                *last = time;
                            })
                        >
                            Rerender
                        </button>
                        <h3 class="text-2xl text-bold"> Render Result </h3>
                        <span class="text-xs mb-1">{time_str}</span>
                    </div>
                    {move || match has.get().0 {
                        Some(Error::BadSyntax(err)) => {
                            view! {
                                <div class="bg-red-100 border border-red-400 text-red-800 rounded-lg p-4 space-y-3">
                                    <div class="flex items-center">
                                        <span class="font-semibold text-red-700">Syntax Error:</span>
                                        <code class="ml-2 px-2 py-1 bg-red-200 text-red-800 rounded text-sm">{err.description}</code>
                                    </div>
                                    <div class="bg-red-200 rounded p-3 text-sm font-mono space-y-1">
                                        <div class="flex">
                                            <code class="text-red-600">{err.line + 1}</code>
                                            <code class="mx-2 text-gray-600">|</code>
                                            <code>{err.context}</code>
                                        </div>
                                        <div class="flex">
                                            <code class="text-red-600">{".".repeat((err.line + 1).to_string().len())}</code>
                                            <code class="mx-2 text-gray-600">|</code>
                                            <code class="text-red-500">{"-".repeat(err.column)}</code><span class="text-red-700 font-bold">^</span>
                                        </div>
                                    </div>
                                    <div class="text-sm text-gray-700">
                                        {"at line "}
                                        <span class="font-semibold text-gray-800">{err.line + 1}</span>
                                        {", column "}
                                        <span class="font-semibold text-gray-800">{err.column}</span>
                                    </div>
                                </div>
                            }.into_view()
                        }
                        Some(Error::VariableNotFound(var)) => {
                            view! {
                                <div class="bg-yellow-100 border border-yellow-400 text-yellow-800 rounded-lg p-4">
                                    <div class="flex items-center">
                                        <span class="font-semibold text-yellow-700">Variable Error:</span>
                                        <code class="ml-2 px-2 py-1 bg-yellow-200 text-yellow-800 rounded text-sm">
                                            {"Variable not found: "}{var}
                                        </code>
                                    </div>
                                </div>
                            }.into_view()
                        }
                        Some(Error::FunctionNotImplemented(func)) => {
                            view! {
                                <div class="bg-blue-100 border border-blue-400 text-blue-800 rounded-lg p-4">
                                    <div class="flex items-center">
                                        <span class="font-semibold text-blue-700">Function Error:</span>
                                        <code class="ml-2 px-2 py-1 bg-blue-200 text-blue-800 rounded text-sm">
                                            {"Function not implemented: "}{func}
                                        </code>
                                    </div>
                                </div>
                            }.into_view()
                        }
                        Some(Error::Function(err)) => {
                            view! {
                                <div class="bg-purple-100 border border-purple-400 text-purple-800 rounded-lg p-4">
                                    <div class="flex items-center">
                                        <span class="font-semibold text-purple-700">Internal Function Error:</span>
                                        <code class="ml-2 px-2 py-1 bg-purple-200 text-purple-800 rounded text-sm">{err.to_string()}</code>
                                    </div>
                                </div>
                            }.into_view()
                        }
                        None => {
                            if let Some(render) = has.get().1 {
                                view! {
                                    <textarea
                                        class="bg-orange-300/30 resize-none w-[500px] h-[354px] focus:outline-none p-4"
                                        prop:readonly={true}
                                        prop:defaultValue={render}
                                    />
                                }.into_view()
                            } else {
                                view! {
                                    <textarea
                                        class="bg-orange-300/30 resize-none w-[500px] h-[354px] focus:outline-none p-4"
                                        prop:readonly={true}
                                        prop:defaultValue="Nothing to render"
                                    />
                                }.into_view()
                            }
                        }
                    }}
                </div>
            </section>
        </div>
    }
}

fn main() {
    leptos::mount_to_body(|| view! { <App/> })
}
