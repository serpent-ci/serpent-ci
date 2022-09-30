use silkenweb::{
    elements::{
        html::{a, button, div, i, li, ul, Div, DivBuilder, LiBuilder},
        svg::{r#use, svg, Svg},
        SvgElement,
    },
    mount,
    node::element::ElementBuilder,
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
    div().class([bs::BTN_GROUP]).attribute("role", "group")
}

fn dropdown_item(name: &str) -> LiBuilder {
    li().child(a().class([bs::DROPDOWN_ITEM]).href("#").text(name))
}

fn function(name: &str, icon: &str) -> DivBuilder {
    // TODO: Add `aria` stuff to silkenweb so we don't need to use `attribute`
    // TODO: `role("group")`
    // TODO: Dropdown id
    div()
        .class([bs::D_FLEX, bs::FLEX_COLUMN, bs::ALIGN_ITEMS_START])
        .child(
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

fn arrow_down() -> Svg {
    svg()
        .class([css::ARROW_VERTICAL])
        .child(r#use().href("#arrow-down"))
        .build()
}

fn arrow_right() -> Svg {
    svg()
        .class([css::ARROW_HORIZONTAL])
        .child(r#use().href("#arrow-right"))
        .build()
}

fn expanded_function(name: &str, children: impl IntoIterator<Item = Div>) -> Div {
    let mut child_elems = div().class([bs::D_FLEX, bs::FLEX_ROW]);
    let mut children = children.into_iter();

    if let Some(child) = children.next() {
        child_elems = child_elems.child(child);
    }

    for child in children {
        child_elems = child_elems.child(arrow_right());
        child_elems = child_elems.child(child);
    }

    function(name, "bi-zoom-out")
        .child(arrow_down())
        .child(child_elems)
        .build()
}

fn main() {
    mount(
        "app",
        div().class([bs::OVERFLOW_AUTO]).child(expanded_function(
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
        )),
    );
}
