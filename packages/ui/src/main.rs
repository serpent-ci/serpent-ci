use std::{
    collections::HashMap,
    sync::atomic::{AtomicU64, Ordering},
};

use indoc::indoc;
use serpent_ci_executor::syntax_tree::{parse, Expression, Function, Statement};
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
    silkenweb::css_classes!(visibility: pub, path: "bootstrap.min.css");
}

mod css {
    silkenweb::css_classes!(visibility: pub, path: "serpent-ci.scss");
}

mod icon {
    silkenweb::css_classes!(visibility: pub, path: "bootstrap-icons.css");
}

const CODE: &str = indoc! {"
    def main():
        function1()
        function2()

    def function1():
        function2(function3())

    def function2():
        pass

    def function3():
        pass
"};

const BUTTON_STYLE: &str = bs::BTN_OUTLINE_SECONDARY;

fn dropdown<'a>(name: &'a str, classes: impl IntoIterator<Item = &'a str>) -> Element {
    static ID_COUNTER: AtomicU64 = AtomicU64::new(0);

    let id = ID_COUNTER.fetch_add(1, Ordering::SeqCst);
    let id = format!("dropdown-{id}");

    button_group(classes)
        .child(
            button()
                .class([bs::BTN, BUTTON_STYLE, bs::DROPDOWN_TOGGLE])
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

fn button_group<'a>(classes: impl IntoIterator<Item = &'a str>) -> DivBuilder {
    div()
        .class(classes.into_iter().chain([bs::BTN_GROUP]))
        .role("group")
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
    let function = button_group([bs::SHADOW])
        .aria_label(format!("Function {name}"))
        .child(dropdown(name, []))
        .child(
            button()
                .r#type("button")
                .class([bs::BTN, BUTTON_STYLE])
                .child(i().class([icon])),
        );

    row([bs::ALIGN_ITEMS_CENTER])
        .child(function)
        .child(horizontal_line())
        .child(arrow_right())
        .into()
}

fn end() -> Element {
    dropdown("end", [bs::SHADOW])
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

fn render_call(name: &str, _args: &[Expression], library: &HashMap<&str, &Function>) -> Element {
    render_function(library.get(name).unwrap(), library)
}

fn render_function(f: &Function, library: &HashMap<&str, &Function>) -> Element {
    let name = f.name();
    let body: Vec<Element> = f
        .body()
        .iter()
        .filter_map(|statement| match statement {
            Statement::Pass => None,
            Statement::Expression(expr) => render_expression(expr, library),
        })
        .collect();

    if body.is_empty() {
        collapsed_function(name)
    } else {
        expanded_function(name, body)
    }
}

fn render_expression(expr: &Expression, library: &HashMap<&str, &Function>) -> Option<Element> {
    match expr {
        Expression::Variable { .. } => None,
        Expression::Call { name, args } => Some(render_call(name, args, library)),
    }
}

fn main() {
    let module = parse(CODE).unwrap();
    let library: HashMap<&str, &Function> =
        module.functions().iter().map(|f| (f.name(), f)).collect();

    let app = row([bs::M_3, bs::ALIGN_ITEMS_START, bs::OVERFLOW_AUTO])
        .children([render_function(&module.functions()[0], &library), end()]);

    mount("app", app);
}
