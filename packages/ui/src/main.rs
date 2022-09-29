use silkenweb::{
    elements::{
        html::{a, button, div, i, li, p, ul, Div, DivBuilder, LiBuilder},
        svg::svg,
    },
    mount,
    node::element::ElementBuilder,
    prelude::{HtmlElement, ParentBuilder},
};

mod bs {
    use silkenweb::css_classes;

    css_classes!(visibility: pub, path: "bootstrap.min.css");
}

fn button_group() -> DivBuilder {
    div().class([bs::BTN_GROUP]).attribute("role", "group")
}

fn dropdown_item(name: &str) -> LiBuilder {
    li().child(a().class([bs::DROPDOWN_ITEM]).href("#").text(name))
}

fn function(name: &str, icon: &str) -> DivBuilder {
    // TODO: Add `aria` stuff to silkenweb so we don't need to use `attribute`
    // TODO: `role("group")`
    // TODO: Dropdown id
    div().child(
        button_group()
            .attribute("aria-label", format!("Function {name}"))
            .child(
                button_group()
                    .child(
                        button()
                            .class([bs::BTN, bs::BTN_OUTLINE_SECONDARY, bs::DROPDOWN_TOGGLE])
                            .id("TODO")
                            .attribute("data-bs-toggle", "dropdown")
                            .r#type("button")
                            .attribute("aria-expanded", "false")
                            .text(name),
                    )
                    .child(
                        ul().class([bs::DROPDOWN_MENU])
                            .attribute("aria-labelledby", "TODO")
                            .children([dropdown_item("Run"), dropdown_item("Pause")]),
                    ),
            )
            .child(
                button()
                    .r#type("button")
                    .class([bs::BTN, bs::BTN_OUTLINE_SECONDARY])
                    .child(i().class([icon])),
            ),
    )
}

fn collapsed_function(name: &str) -> Div {
    function(name, "bi-zoom-in").build()
}

fn expanded_function(name: &str) -> DivBuilder {
    function(name, "bi-zoom-out").child(svg())
}

fn main() {
    mount("app", expanded_function("main_function"));
}
