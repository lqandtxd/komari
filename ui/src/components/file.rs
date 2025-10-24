use dioxus::prelude::*;
use tw_merge::tw_merge;

#[derive(Props, PartialEq, Clone)]
pub struct FileInputProps {
    #[props(default)]
    on_file: Callback<String>,
    #[props(default = ".png".to_string())]
    accept: String,
    #[props(default = "Image".to_string())]
    name: String,
    #[props(default)]
    class: String,
    children: Element,
}

#[component]
pub fn FileInput(props: FileInputProps) -> Element {
    let class = props.class;
    let accept = props.accept;
    let name = props.name;

    let handle_on_change = move |e: Event<FormData>| {
        if let Some(file) = e
            .data
            .files()
            .and_then(|engine| engine.files().into_iter().next())
        {
            props.on_file.call(file);
        }
    };

    rsx! {
        label { class: tw_merge!("inline-block relative", class),
            input {
                class: "sr-only",
                r#type: "file",
                accept,
                name,
                onchange: handle_on_change,
            }
            {props.children}
        }
    }
}
