use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: asset!("/assets/tailwind.css") }
        // background
        div { class: "relative w-screen h-screen overflow-hidden bg-gray-800 text-white select-none font-sans",
            div { class: "absolute inset-0  cursor-crosshair",
                svg { class: "w-full h-full pointer-events-none", // 青い四角形
                    rect {
                        x: "500",
                        y: "500",
                        width: "60",
                        height: "60",
                        fill: "#3b82f6",
                    }

                    // 赤い円
                    circle {
                        cx: "120",
                        cy: "120",
                        r: "30",
                        fill: "#ef4444",
                        opacity: "0.8",
                    }
                }
            }

            // menu
            div { class: "absolute  top-4 left-4 flex flex-col gap-2 z-10 ",
                // filename
                div { class: "border border-white/40 bg-black/60 backdrop-blur px-3 py-1.5 rounded text-sm w-fit min-w-[150px]",
                    span { class: "cursor-pointer hover:text-blue-400 transition-colors",
                        "filename.svg"
                    }
                }
                div { class: "flex gap-2 items-start mt-1",
                    ActionButton {
                        label: "クリップ\nボード",
                        shortcut: "Ctrl+C",
                        onclick: move |_| {},
                    }
                    ActionButton {
                        label: "markdown\n貼り付け\n用にコピ",
                        shortcut: "Ctrl+Shift+S",
                        onclick: move |_| {},
                    }
                    ActionButton {
                        label: "セーブ",
                        shortcut: "Ctrl+S",
                        onclick: move |_| {},
                    }
                }
            }
            div { class: "absolute top-4 left-1/2 -translate-x-1/2 border border-white/40 bg-black/70 backdrop-blur px-4 py-2 rounded flex items-center gap-4 z-10",
                div { class: "flex gap-2" }
            }
        }
    }
}

#[component]
fn ActionButton(
    label: &'static str,
    shortcut: &'static str,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex flex-col items-center gap-1",
            button {

                class: "border border-white/40 bg-black/60 hover:bg-white/15 w-16 h-16 rounded flex items-center justify-center text-center text-xs p-1 leading-tight whitespace-pre-line active:scale-95 transition",
                onclick: move |e| onclick.call(e),
                "{label}"
            }
            span { class: "text-[10px] text-white/60 tracking-tighter", "{shortcut}" }
        }

    }
}

fn ToolButton(icon: &'static str, shortcut: &'static str, active: bool) -> Element {
    rsx! {
        button { class: "relative border w-10 h-10 rounded flex items-center justify-center text-sm font-mono active:scale-95 transition border-blue-400 bg-white/20",

            span { "{icon}" }
            span { class: "absolute bottom-0.5 right-1 text-[9px] text-white/50 font-sans",
                "{shortcut}"
            }
        }
    }
}
