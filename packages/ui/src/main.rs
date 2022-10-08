use std::{
    collections::HashMap,
    iter,
    rc::Rc,
    sync::atomic::{AtomicU64, Ordering},
};

use futures_signals::signal::{Mutable, SignalExt};
use indoc::indoc;
use serpent_ci_executor::syntax_tree::{parse, Expression, Function, Statement};
use silkenweb::{
    elements::{
        html::{a, button, div, i, li, ul, DivBuilder, LiBuilder},
        AriaElement, ElementEvents,
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

type State = Mutable<bool>;

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

fn end() -> Element {
    dropdown("end", [bs::SHADOW])
}

fn horizontal_line() -> Element {
    div().class([css::HORIZONTAL_LINE]).into()
}

fn arrow_right() -> Element {
    div().class([css::ARROW]).into()
}

fn render_call(
    name: &str,
    args: &[Expression],
    library: &Rc<HashMap<String, Function<State>>>,
) -> Vec<Element> {
    args.iter()
        .flat_map(|arg| render_expression(arg, library))
        .chain(iter::once(render_function(
            library.get(name).unwrap(),
            library,
        )))
        .collect()
}

fn render_function(f: &Function<State>, library: &Rc<HashMap<String, Function<State>>>) -> Element {
    // TODO: Icon
    let name = f.name();
    let expanded = f.state().clone();
    let function = button_group([bs::SHADOW])
        .aria_label(format!("Function {name}"))
        .child(dropdown(name, []))
        .child(
            button()
                .on_click(move |_, _| {
                    expanded.replace_with(|e| !*e);
                })
                .r#type("button")
                .class([bs::BTN, BUTTON_STYLE])
                .child(i().class_signal(f.state().signal().map(|expanded| {
                    [if expanded {
                        icon::BI_ZOOM_OUT
                    } else {
                        icon::BI_ZOOM_IN
                    }]
                }))),
        );
    let main = row([bs::ALIGN_ITEMS_CENTER])
        .child(function)
        .child(horizontal_line())
        .child(arrow_right());

    let library = library.clone();
    let body = f.body().clone();

    column([bs::ALIGN_ITEMS_STRETCH])
        .child(main)
        .optional_child_signal(f.state().signal().map(move |expanded| {
            expanded.then(|| {
                row([
                    // TODO: We can probably get rid of some `row`s and `column`s using
                    // align_self_*
                    bs::ALIGN_SELF_START,
                    css::SPEECH_BUBBLE_TOP,
                    bs::ALIGN_ITEMS_START,
                    bs::SHADOW,
                    bs::MT_3,
                    bs::P_3,
                    bs::BORDER,
                    bs::BORDER_SECONDARY,
                    bs::ROUNDED,
                    bs::ME_3,
                ])
                .children(body.iter().flat_map(|statement| match statement {
                    Statement::Pass => Vec::new(),
                    Statement::Expression(expr) => render_expression(expr, &library),
                }))
                .child(end())
            })
        }))
        .into()
}

fn render_expression(
    expr: &Expression,
    library: &Rc<HashMap<String, Function<State>>>,
) -> Vec<Element> {
    match expr {
        Expression::Variable { .. } => Vec::new(),
        Expression::Call { name, args } => render_call(name, args, library),
    }
}

fn main() {
    // TODO: We shouldn't put `state` on the funtion. It belongs on an instance of
    // the function reference.
    let module = parse(CODE).unwrap();
    let library: Rc<HashMap<String, Function<State>>> = Rc::new(
        module
            .functions()
            .into_iter()
            .map(|f| (f.name().to_owned(), f))
            .collect(),
    );

    let app = row([bs::M_3, bs::ALIGN_ITEMS_START, bs::OVERFLOW_AUTO])
        .children([render_function(&library["main"], &library), end()]);

    mount("app", app);
}
