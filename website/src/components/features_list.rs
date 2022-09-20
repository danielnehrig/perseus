use perseus::i18n::Translator;
use perseus::link;
use perseus::navigate;
use sycamore::prelude::*;

/// Turns the features of Perseus into an actual list.
pub fn get_features_list<G: GenericNode>(cx: Scope) -> View<G> {
    let features = get_features();
    View::new_fragment(
        features.iter().map(move |feat| {
            let Feature { id_base, link } = feat.clone();
            let name_id = format!("feature-{}.name", &id_base);
            let desc_id = format!("feature-{}.desc", &id_base);
            view! { cx,
                li(class = "inline-block align-top") {
                    div(
                        class = "text-left cursor-pointer rounded-xl shadow-md hover:shadow-2xl transition-shadow duration-100 p-8 max-w-sm",
                        on:click = move |_| {
                            navigate(&link!(&link, cx))
                        }
                    ) {
                        p(class = "text-2xl xs:text-3xl sm:text-4xl") { ({
                            let translator = use_context::<Signal<Translator>>(cx).get_untracked();
                            translator.translate(&name_id, None)
                        }) }
                        p(class = "text-gray-100") { ({
                            let translator = use_context::<Signal<Translator>>(cx).get_untracked();
                            translator.translate(&desc_id, None)
                        }) }
                    }
                }
            }
        }).collect()
    )
}

/// A representation of a Perseus feature for listing. This uses translation IDs
/// instead of verbatim text.
#[derive(Clone)]
pub struct Feature {
    /// The base of the translation ID, which is expected to have `.name` and
    /// `.desc` variants.
    pub id_base: String,
    pub link: String,
}

/// Gets the current features of Perseus.
pub fn get_features() -> Vec<Feature> {
    vec![
        Feature {
            id_base: "ssg".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "ssr".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "i18n".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "incremental".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "revalidation".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "cli".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "routing".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "shell".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "deployment".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
        Feature {
            id_base: "exporting".to_string(),
            link: "/docs/feature/static-generation".to_string(),
        },
    ]
}
