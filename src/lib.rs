use crate::colors::Color;
use crate::queue::Queue;
use leptos::web_sys::js_sys::Math;
use leptos::{
    component, create_effect, create_node_ref, create_rw_signal, view, watch, Children, For,
    IntoView, RwSignal, SignalGet, SignalSet, SignalUpdate, SignalWith,
};
use leptos_use::{
    use_draggable_with_options, use_element_bounding, UseDraggableCallbackArgs,
    UseDraggableOptions, UseElementBoundingReturn,
};

mod colors;
mod queue;

#[derive(Clone)]
struct ColorPicker {
    pub width: RwSignal<i32>,
    pub height: RwSignal<i32>,
    pub top: RwSignal<i32>,
    pub left: RwSignal<i32>,
}

impl ColorPicker {
    fn new() -> Self {
        Self {
            width: create_rw_signal(0),
            height: create_rw_signal(0),
            top: create_rw_signal(0),
            left: create_rw_signal(0),
        }
    }
}

#[component]
pub fn ColorPicker() -> impl IntoView {
    let color_picker = create_rw_signal(ColorPicker::new());

    let current_color = create_rw_signal(Color::new(0x80, 0x80, 0x80, 50, 50, 0.0));

    let colors = create_rw_signal(Queue::new().with_default_values(current_color.get()));

    let _ = watch(
        move || color_picker.get(),
        move |_, _, _| {
            let width = color_picker.with(|p| p.width.get()) / 2;
            let height = color_picker.with(|p| p.height.get()) / 2;
            current_color.set(Color::new(0x80, 0x80, 0x80, width, height, 0.0));
            colors.set(
                Queue::new()
                    .with_capacity(5)
                    .with_default_values(current_color.get()),
            );
        },
        false,
    );

    view! {
        <div class="column">
            <Picker current_color color_picker/>
            <ul class="color-list">
                <For
                    each=move || colors.get().into_iter().rev().enumerate()
                    key=|(id, color)| (*id, color.clone())
                    let:color
                >
                    <ColorBlock color=color.1 current_color/>
                </For>
                <SaveColorButton color=current_color colors/>
            </ul>
        </div>
    }
}

#[component]
fn SaveColorButton(color: RwSignal<Color>, colors: RwSignal<Queue<Color>>) -> impl IntoView {
    view! {
        <li>
            <button
                class="save-color-button"
                on:click=move |_| {
                    colors
                        .update(|colors| {
                            colors.push(color.get());
                        });
                }
            >
                <i class="icondata::RiSaveDeviceFill" width="3rem" height="3rem"/>
            </button>
        </li>
    }
}
#[component]
fn Picker(current_color: RwSignal<Color>, color_picker: RwSignal<ColorPicker>) -> impl IntoView {
    let color = current_color.clone();

    view! {
        <div class="color-picker-container">
            <ColorSelector color>
                <ColorCircle color_picker>
                    <ColorHandle color color_picker/>
                </ColorCircle>
            </ColorSelector>
        </div>
    }
}

#[component]
fn ColorSelector(children: Children, color: RwSignal<Color>) -> impl IntoView {
    let node_ref = create_node_ref();

    view! {
        <div
            class="base-color-ring"
            style=move || {
                format!(
                    "background: linear-gradient(0deg, rgba(50,50,50,1) 0%, rgba({},{},{},1) 50%, rgba(200,200,200,1) 100%);)",
                    color.with(|c|c.r.get()),
                    color.with(|c|c.g.get()),
                    color.with(|c|c.b.get()),
                )
            }
        >

            <span
                ref=node_ref
                class="ring-handle"
                style=move || format!("top: {}%; left: {}%;", 1, 50)
            ></span>
            <span class="color-ring">
                <span class="color-ring-container">{children()}</span>
            </span>
        </div>
    }
}

#[component]
fn ColorCircle(children: Children, color_picker: RwSignal<ColorPicker>) -> impl IntoView {
    let node_ref = create_node_ref();
    let UseElementBoundingReturn {
        x,
        y,
        height,
        width,
        ..
    } = use_element_bounding(node_ref);

    create_effect(move |_| {
        color_picker.update(|picker| {
            picker.width.set(width.get() as i32);
            picker.height.set(height.get() as i32);
            picker.top.set(y.get() as i32);
            picker.left.set(x.get() as i32);
        });
    });
    view! {
        <div class="color-circle" ref=node_ref>
            {children()}
        </div>
    }
}

#[component]
fn ColorHandle(color: RwSignal<Color>, color_picker: RwSignal<ColorPicker>) -> impl IntoView {
    let node_ref = create_node_ref();

    let _ = use_draggable_with_options(
        node_ref,
        UseDraggableOptions::default()
            .prevent_default(false)
            .on_move(move |evt: UseDraggableCallbackArgs| {
                let x = evt.position.x as i32 - color_picker.with(|p| p.left.get());
                let y = evt.position.y as i32 - color_picker.with(|p| p.top.get());
                let mouse_x = 2.0 * x as f64 / (color_picker.with(|p| p.width.get()) as f64) - 1.0;
                let mouse_y = 1.0 - 2.0 * y as f64 / color_picker.with(|p| p.height.get()) as f64;
                let radius = Math::sqrt(mouse_x * mouse_x + mouse_y * mouse_y);

                if radius > 1.01 {
                    return;
                }

                let (r, g, b) = Color::calculate_color(color, mouse_x, mouse_y, radius);
                color.set(Color::new(r, g, b, x, y, color.with(|c| c.get_degree())));
            }),
    );

    view! {
        <div ref=node_ref class="color-handle" style=move || color.with(|c|c.get_position())>
            <div class="color-handle-inner" style=move || color.with(|c|c.get_style())></div>
        </div>
    }
}

#[component]
fn ColorBlock(color: Color, current_color: RwSignal<Color>) -> impl IntoView {
    let color = create_rw_signal(color);
    view! {
        <li class="color-element">
            <button
                class="color-block"
                style=move || color.with(|c|c.get_style())
                on:click=move |_| current_color.set(color.get())
            ></button>
        </li>
    }
}
