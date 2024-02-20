use dioxus::prelude::*;

#[derive(Props)]
pub struct IconProps<'a> {
    pub class: Option<&'a str>,
}

#[component]
pub fn OreIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 48 48",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M0.00527625 42.1406C-0.010802 42.0669 0.00987002 41.9866 0.0672362 41.9293L4.91651 37.0801C4.9946 37.002 5.0059 36.8794 4.94428 36.7878C2.48729 33.1337 1.0535 28.7342 1.0535 23.9999C1.0535 11.3269 11.327 1.05343 24 1.05343C28.7343 1.05343 33.1338 2.48717 36.7878 4.94421C36.8794 5.00584 37.002 4.99454 37.0801 4.91645L41.9293 0.0672259C42.0189 -0.0224086 42.1642 -0.0224086 42.2538 0.0672259L47.9327 5.74625C48.0224 5.83588 48.0224 5.98109 47.9327 6.07073L43.0835 10.9199C43.0054 10.998 42.9941 11.1206 43.0557 11.2122C45.5127 14.8661 46.9464 19.2655 46.9464 23.9999C46.9464 36.6729 36.6729 46.9463 24 46.9463C19.2657 46.9463 14.8663 45.5126 11.2122 43.0556C11.1206 42.9939 10.998 43.0052 10.92 43.0833L6.07068 47.9328C5.9811 48.0224 5.83578 48.0224 5.7462 47.9328L0.0672362 42.2537C0.0349677 42.2215 0.0143518 42.182 0.00527625 42.1406ZM24 38.4562C21.5532 38.4562 19.2483 37.8482 17.2282 36.7753C14.6769 35.4202 12.5798 33.323 11.2247 30.7718C10.1516 28.7516 9.54369 26.4467 9.54369 23.9999C9.54369 16.0159 16.016 9.54362 24 9.54362C26.4469 9.54362 28.7518 10.1516 30.7719 11.2245C33.3232 12.5798 35.4202 14.6768 36.7753 17.228C37.8483 19.2481 38.4562 21.5531 38.4562 23.9999C38.4562 31.9839 31.9839 38.4562 24 38.4562Z"
            }
        }
    }
}

#[component]
pub fn QrCodeIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            fill: "none",
            view_box: "0 0 24 24",
            stroke_width: "1.5",
            stroke: "currentColor",
            class: "{class}",
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M3.75 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0 1 3.75 9.375v-4.5ZM3.75 14.625c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5a1.125 1.125 0 0 1-1.125-1.125v-4.5ZM13.5 4.875c0-.621.504-1.125 1.125-1.125h4.5c.621 0 1.125.504 1.125 1.125v4.5c0 .621-.504 1.125-1.125 1.125h-4.5A1.125 1.125 0 0 1 13.5 9.375v-4.5Z"
            }
            path {
                stroke_linecap: "round",
                stroke_linejoin: "round",
                d: "M6.75 6.75h.75v.75h-.75v-.75ZM6.75 16.5h.75v.75h-.75v-.75ZM16.5 6.75h.75v.75h-.75v-.75ZM13.5 13.5h.75v.75h-.75v-.75ZM13.5 19.5h.75v.75h-.75v-.75ZM19.5 13.5h.75v.75h-.75v-.75ZM19.5 19.5h.75v.75h-.75v-.75ZM16.5 16.5h.75v.75h-.75v-.75Z"
            }
        }
    }
}

#[component]
pub fn SearchIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            fill: "currentColor",
            view_box: "0 0 24 24",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M10.5 3.75a6.75 6.75 0 1 0 0 13.5 6.75 6.75 0 0 0 0-13.5ZM2.25 10.5a8.25 8.25 0 1 1 14.59 5.28l4.69 4.69a.75.75 0 1 1-1.06 1.06l-4.69-4.69A8.25 8.25 0 0 1 2.25 10.5Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn CubeIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M12.378 1.602a.75.75 0 0 0-.756 0L3 6.632l9 5.25 9-5.25-8.622-5.03ZM21.75 7.93l-9 5.25v9l8.628-5.032a.75.75 0 0 0 .372-.648V7.93ZM11.25 22.18v-9l-9-5.25v8.57a.75.75 0 0 0 .372.648l8.628 5.033Z"
            }
        }
    }
}

#[component]
pub fn CubeTransparentIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M11.622 1.602a.75.75 0 0 1 .756 0l2.25 1.313a.75.75 0 0 1-.756 1.295L12 3.118 10.128 4.21a.75.75 0 1 1-.756-1.295l2.25-1.313ZM5.898 5.81a.75.75 0 0 1-.27 1.025l-1.14.665 1.14.665a.75.75 0 1 1-.756 1.295L3.75 8.806v.944a.75.75 0 0 1-1.5 0V7.5a.75.75 0 0 1 .372-.648l2.25-1.312a.75.75 0 0 1 1.026.27Zm12.204 0a.75.75 0 0 1 1.026-.27l2.25 1.312a.75.75 0 0 1 .372.648v2.25a.75.75 0 0 1-1.5 0v-.944l-1.122.654a.75.75 0 1 1-.756-1.295l1.14-.665-1.14-.665a.75.75 0 0 1-.27-1.025Zm-9 5.25a.75.75 0 0 1 1.026-.27L12 11.882l1.872-1.092a.75.75 0 1 1 .756 1.295l-1.878 1.096V15a.75.75 0 0 1-1.5 0v-1.82l-1.878-1.095a.75.75 0 0 1-.27-1.025ZM3 13.5a.75.75 0 0 1 .75.75v1.82l1.878 1.095a.75.75 0 1 1-.756 1.295l-2.25-1.312a.75.75 0 0 1-.372-.648v-2.25A.75.75 0 0 1 3 13.5Zm18 0a.75.75 0 0 1 .75.75v2.25a.75.75 0 0 1-.372.648l-2.25 1.312a.75.75 0 1 1-.756-1.295l1.878-1.096V14.25a.75.75 0 0 1 .75-.75Zm-9 5.25a.75.75 0 0 1 .75.75v.944l1.122-.654a.75.75 0 1 1 .756 1.295l-2.25 1.313a.75.75 0 0 1-.756 0l-2.25-1.313a.75.75 0 1 1 .756-1.295l1.122.654V19.5a.75.75 0 0 1 .75-.75Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn AdjustmentsHorizontalIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            stroke_width: 0,
            path {
                d: "M18.75 12.75h1.5a.75.75 0 0 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM12 6a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 6ZM12 18a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 12 18ZM3.75 6.75h1.5a.75.75 0 1 0 0-1.5h-1.5a.75.75 0 0 0 0 1.5ZM5.25 18.75h-1.5a.75.75 0 0 1 0-1.5h1.5a.75.75 0 0 1 0 1.5ZM3 12a.75.75 0 0 1 .75-.75h7.5a.75.75 0 0 1 0 1.5h-7.5A.75.75 0 0 1 3 12ZM9 3.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5ZM12.75 12a2.25 2.25 0 1 1 4.5 0 2.25 2.25 0 0 1-4.5 0ZM9 15.75a2.25 2.25 0 1 0 0 4.5 2.25 2.25 0 0 0 0-4.5Z"
            }
        }
    }
}

#[component]
pub fn PauseIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M6.75 5.25a.75.75 0 0 1 .75-.75H9a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H7.5a.75.75 0 0 1-.75-.75V5.25Zm7.5 0A.75.75 0 0 1 15 4.5h1.5a.75.75 0 0 1 .75.75v13.5a.75.75 0 0 1-.75.75H15a.75.75 0 0 1-.75-.75V5.25Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn PlayIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M4.5 5.653c0-1.427 1.529-2.33 2.779-1.643l11.54 6.347c1.295.712 1.295 2.573 0 3.286L7.28 19.99c-1.25.687-2.779-.217-2.779-1.643V5.653Z",
                clip_rule: "evenodd"
            }
        }
    }
}

#[component]
pub fn PlusIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 20 20",
            fill: "currentColor",
            class: "{class}",
            path {
                d: "M10.75 4.75a.75.75 0 0 0-1.5 0v4.5h-4.5a.75.75 0 0 0 0 1.5h4.5v4.5a.75.75 0 0 0 1.5 0v-4.5h4.5a.75.75 0 0 0 0-1.5h-4.5v-4.5Z"
            }
        }
    }
}

#[component]
pub fn InfoIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 16 16",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M15 8A7 7 0 1 1 1 8a7 7 0 0 1 14 0ZM9 5a1 1 0 1 1-2 0 1 1 0 0 1 2 0ZM6.75 8a.75.75 0 0 0 0 1.5h.75v1.75a.75.75 0 0 0 1.5 0v-2.5A.75.75 0 0 0 8.25 8h-1.5Z",
            }
        }
    }
}

#[component]
pub fn WarningIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M9.401 3.003c1.155-2 4.043-2 5.197 0l7.355 12.748c1.154 2-.29 4.5-2.599 4.5H4.645c-2.309 0-3.752-2.5-2.598-4.5L9.4 3.003ZM12 8.25a.75.75 0 0 1 .75.75v3.75a.75.75 0 0 1-1.5 0V9a.75.75 0 0 1 .75-.75Zm0 8.25a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5Z"
            }
        }
    }
}

#[component]
pub fn CheckIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M2.25 12c0-5.385 4.365-9.75 9.75-9.75s9.75 4.365 9.75 9.75-4.365 9.75-9.75 9.75S2.25 17.385 2.25 12Zm13.36-1.814a.75.75 0 1 0-1.22-.872l-3.236 4.53L9.53 12.22a.75.75 0 0 0-1.06 1.06l2.25 2.25a.75.75 0 0 0 1.14-.094l3.75-5.25Z"
            }
        }
    }
}

#[component]
pub fn UserIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                clip_rule: "evenodd",
                d: "M7.5 6a4.5 4.5 0 1 1 9 0 4.5 4.5 0 0 1-9 0ZM3.751 20.105a8.25 8.25 0 0 1 16.498 0 .75.75 0 0 1-.437.695A18.683 18.683 0 0 1 12 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 0 1-.437-.695Z"
            }
        }
    }
}

#[component]
pub fn GlobeIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M21.721 12.752a9.711 9.711 0 0 0-.945-5.003 12.754 12.754 0 0 1-4.339 2.708 18.991 18.991 0 0 1-.214 4.772 17.165 17.165 0 0 0 5.498-2.477ZM14.634 15.55a17.324 17.324 0 0 0 .332-4.647c-.952.227-1.945.347-2.966.347-1.021 0-2.014-.12-2.966-.347a17.515 17.515 0 0 0 .332 4.647 17.385 17.385 0 0 0 5.268 0ZM9.772 17.119a18.963 18.963 0 0 0 4.456 0A17.182 17.182 0 0 1 12 21.724a17.18 17.18 0 0 1-2.228-4.605ZM7.777 15.23a18.87 18.87 0 0 1-.214-4.774 12.753 12.753 0 0 1-4.34-2.708 9.711 9.711 0 0 0-.944 5.004 17.165 17.165 0 0 0 5.498 2.477ZM21.356 14.752a9.765 9.765 0 0 1-7.478 6.817 18.64 18.64 0 0 0 1.988-4.718 18.627 18.627 0 0 0 5.49-2.098ZM2.644 14.752c1.682.971 3.53 1.688 5.49 2.099a18.64 18.64 0 0 0 1.988 4.718 9.765 9.765 0 0 1-7.478-6.816ZM13.878 2.43a9.755 9.755 0 0 1 6.116 3.986 11.267 11.267 0 0 1-3.746 2.504 18.63 18.63 0 0 0-2.37-6.49ZM12 2.276a17.152 17.152 0 0 1 2.805 7.121c-.897.23-1.837.353-2.805.353-.968 0-1.908-.122-2.805-.353A17.151 17.151 0 0 1 12 2.276ZM10.122 2.43a18.629 18.629 0 0 0-2.37 6.49 11.266 11.266 0 0 1-3.746-2.504 9.754 9.754 0 0 1 6.116-3.985Z"
            }
        }
    }
}

#[component]
pub fn CircleStackIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M21 6.375c0 2.692-4.03 4.875-9 4.875S3 9.067 3 6.375 7.03 1.5 12 1.5s9 2.183 9 4.875Z",
            }
            path {
                fill_rule: "evenodd",
                d: "M12 12.75c2.685 0 5.19-.586 7.078-1.609a8.283 8.283 0 0 0 1.897-1.384c.016.121.025.244.025.368C21 12.817 16.97 15 12 15s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.285 8.285 0 0 0 1.897 1.384C6.809 12.164 9.315 12.75 12 12.75Z",
            }
            path {
                fill_rule: "evenodd",
                d: "M12 16.5c2.685 0 5.19-.586 7.078-1.609a8.282 8.282 0 0 0 1.897-1.384c.016.121.025.244.025.368 0 2.692-4.03 4.875-9 4.875s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.284 8.284 0 0 0 1.897 1.384C6.809 15.914 9.315 16.5 12 16.5Z"
            }
            path {
                fill_rule: "evenodd",
                d: "M12 20.25c2.685 0 5.19-.586 7.078-1.609a8.282 8.282 0 0 0 1.897-1.384c.016.121.025.244.025.368 0 2.692-4.03 4.875-9 4.875s-9-2.183-9-4.875c0-.124.009-.247.025-.368a8.284 8.284 0 0 0 1.897 1.384C6.809 19.664 9.315 20.25 12 20.25Z"
            }
        }
    }
}

#[component]
pub fn PaperAirplaneIcon<'a>(cx: Scope<'a, IconProps<'a>>) -> Element {
    let class = cx.props.class.unwrap_or("");
    render! {
        svg {
            view_box: "0 0 24 24",
            fill: "currentColor",
            class: "{class}",
            path {
                fill_rule: "evenodd",
                d: "M3.478 2.404a.75.75 0 0 0-.926.941l2.432 7.905H13.5a.75.75 0 0 1 0 1.5H4.984l-2.432 7.905a.75.75 0 0 0 .926.94 60.519 60.519 0 0 0 18.445-8.986.75.75 0 0 0 0-1.218A60.517 60.517 0 0 0 3.478 2.404Z"
            }
        }
    }
}
