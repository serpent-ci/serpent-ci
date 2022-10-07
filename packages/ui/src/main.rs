use std::sync::atomic::{AtomicU64, Ordering};

use silkenweb::{
    elements::{
        html::{a, button, div, i, li, ul, DivBuilder, LiBuilder},
        AriaElement,
    },
    mount,
    node::element::{Element, ElementBuilder},
    prelude::{HtmlElement, ParentBuilder},
};

mod bs {
    use silkenweb::css_classes;

    css_classes!(visibility: pub, path: "bootstrap.min.css");
}

mod css {
    use silkenweb::css_classes;

    css_classes!(visibility: pub, path: "serpent-ci.scss");
}

mod icon {
    use silkenweb::css_classes;

    css_classes!(visibility: pub, path: "bootstrap-icons.css");
}

fn dropdown(name: &str) -> Element {
    static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let id = format!("dropdown-{id}");

    button_group()
        .child(
            button()
                .class([bs::BTN, bs::BTN_SECONDARY, bs::DROPDOWN_TOGGLE])
                .id(&id)
                .attribute("data-bs-toggle", "dropdown")
                .r#type("button")
                .aria_expanded("false")
                .text(name),
        )
        .child(
            ul().class([bs::DROPDOWN_MENU])
                .aria_labelledby(id)
                .children([dropdown_item("Run"), dropdown_item("Pause")]),
        )
        .into()
}

fn button_group() -> DivBuilder {
    div().class([bs::BTN_GROUP]).role("group")
}

fn dropdown_item(name: &str) -> LiBuilder {
    li().child(a().class([bs::DROPDOWN_ITEM]).href("#").text(name))
}

fn row<'a>(classes: impl IntoIterator<Item = &'a str>) -> DivBuilder {
    div().class(classes.into_iter().chain([bs::D_FLEX, bs::FLEX_ROW]))
}

fn column<'a>(classes: impl IntoIterator<Item = &'a str>) -> DivBuilder {
    div().class(classes.into_iter().chain([bs::D_FLEX, bs::FLEX_COLUMN]))
}

fn function(name: &str, icon: &str) -> Element {
    let function = button_group()
        .aria_label(format!("Function {name}"))
        .child(dropdown(name))
        .child(
            button()
                .r#type("button")
                .class([bs::BTN, bs::BTN_SECONDARY])
                .child(i().class([icon])),
        );

    row([bs::ALIGN_ITEMS_CENTER])
        .child(function)
        .child(horizontal_line())
        .child(arrow_right())
        .into()
}

fn end() -> Element {
    dropdown("end")
}

fn collapsed_function(name: &str) -> Element {
    function(name, icon::BI_ZOOM_IN)
}

fn horizontal_line() -> Element {
    div().class([css::HORIZONTAL_LINE]).into()
}

fn arrow_right() -> Element {
    div().class([css::ARROW]).into()
}

fn expanded_function(name: &str, body: impl IntoIterator<Item = Element>) -> Element {
    let body = row([
        css::SPEECH_BUBBLE_TOP,
        bs::ALIGN_ITEMS_START,
        bs::SHADOW,
        bs::MT_3,
        bs::P_3,
        bs::BORDER,
        bs::BORDER_SECONDARY,
        bs::ROUNDED,
    ])
    .children(body)
    .child(end());
    let main = function(name, icon::BI_ZOOM_OUT);

    column([bs::ALIGN_ITEMS_STRETCH])
        .child(main)
        .child(body)
        .into()
}

fn main() {
    mount(
        "app",
        row([bs::M_3, bs::ALIGN_ITEMS_START, bs::OVERFLOW_AUTO]).children([
            expanded_function(
                "main_function",
                [
                    collapsed_function("function1"),
                    expanded_function(
                        "another_function",
                        [
                            collapsed_function("child_function1"),
                            collapsed_function("child_function2"),
                        ],
                    ),
                    collapsed_function("function2"),
                ],
            ),
            end(),
        ]),
    );
}
