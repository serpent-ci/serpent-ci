use silkenweb::{
    elements::{
        html::{a, button, div, i, li, ul, DivBuilder, LiBuilder},
        svg::{
            attributes::Global,
            content_type::{Length::Px, Percentage},
            r#use, svg,
        },
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

fn function(name: &str, icon: &str, is_last: bool) -> Element {
    // TODO: Dropdown id
    let function = button_group()
        .aria_label(format!("Function {name}"))
        .child(
            button_group()
                .child(
                    button()
                        .class([bs::BTN, bs::BTN_SECONDARY, bs::DROPDOWN_TOGGLE])
                        .id("TODO")
                        .attribute("data-bs-toggle", "dropdown")
                        .r#type("button")
                        .aria_expanded("false")
                        .text(name),
                )
                .child(
                    ul().class([bs::DROPDOWN_MENU])
                        .aria_labelledby("TODO")
                        .children([dropdown_item("Run"), dropdown_item("Pause")]),
                ),
        )
        .child(
            button()
                .r#type("button")
                .class([bs::BTN, bs::BTN_SECONDARY])
                .child(i().class([icon])),
        );

    if is_last {
        column([bs::ALIGN_ITEMS_START]).child(function)
    } else {
        column([bs::ALIGN_ITEMS_STRETCH]).child(
            row([bs::ALIGN_ITEMS_CENTER])
                .child(function)
                .child(horizontal_line())
                .child(arrow_right()),
        )
    }
    .into()
}

fn collapsed_function(name: &str, is_last: bool) -> Element {
    function(name, "bi-zoom-in", is_last)
}

fn horizontal_line() -> Element {
    svg()
        .class([css::LINE_HORIZONTAL])
        .width(Percentage(100.0))
        .height(Px(20.0))
        .view_box("0 0 100 100")
        .preserve_aspect_ratio("none")
        .child(r#use().href("#horizontal-line"))
        .into()
}

fn arrow_right() -> Element {
    svg()
        .class([css::ARROW])
        .child(r#use().href("#arrow-right"))
        .into()
}

fn expanded_function(
    name: &str,
    body: impl IntoIterator<Item = Element>,
    is_last: bool,
) -> Element {
    let body = row([
        css::FUNCTION_BODY,
        bs::BORDER,
        bs::BORDER_SECONDARY,
        bs::ROUNDED,
    ])
    .children(body);
    let main = function(name, "bi-zoom-out", is_last);

    column([bs::ALIGN_ITEMS_STRETCH])
        .child(main)
        .child(body)
        .into()
}

fn main() {
    mount(
        "app",
        column([css::MARGIN, bs::ALIGN_ITEMS_START, bs::OVERFLOW_AUTO]).child(expanded_function(
            "main_function",
            [
                collapsed_function("function1", false),
                expanded_function(
                    "another_function",
                    [
                        collapsed_function("child_function1", false),
                        collapsed_function("child_function2", true),
                    ],
                    false,
                ),
                collapsed_function("function2", true),
            ],
            true,
        )),
    );
}
