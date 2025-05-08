use leptos::leptos_dom::log;
use leptos::prelude::*;
use phosphor_leptos::{Icon, IconWeight, CUBE, HEART, HORSE};


fn main() {
    log!("Hello, Horse Icon");
    log!("{}", HORSE.get(IconWeight::Regular));

    leptos::mount::mount_to_body(|| view! {
        <div>
            <MyComponent />
        </div>
    })
}

#[component]
fn MyComponent() -> impl IntoView {

    let icons = vec![
        ("HORSE", HORSE),
        ("HEART", HEART),
        ("CUBE", CUBE)
    ];

    view! {
        {icons
            .into_iter()
            .map(|(label, icon)| {
                view! {
                    <div>
                        <p>{label}</p>
                        <p>{icon.get(IconWeight::Regular)}</p>
                        <Icon icon=icon color="red" weight=IconWeight::Regular size="32px" />
                    </div>
                }
            })
            .collect_view()
        }
    }
}
