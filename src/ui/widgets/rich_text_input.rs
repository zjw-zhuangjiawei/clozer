//! Rich text input widget with inline ghost text (auto-completion) support.

use crate::ui::widgets::text_input_local::{self as text_input, Editor, Value as TextValue};
use iced::advanced::Shell;
use iced::advanced::clipboard::{self, Clipboard};
use iced::advanced::layout::{self, Layout};
use iced::advanced::mouse;
use iced::advanced::renderer;
use iced::advanced::text::paragraph;
use iced::advanced::text::{self as core_text, Paragraph as ParagraphTrait};
use iced::advanced::widget::Widget;
use iced::advanced::widget::tree::{self, Tree};
use iced::alignment;
use iced::keyboard;
use iced::keyboard::key;
use iced::time::{Duration, Instant};
use iced::touch;
use iced::widget::text_input::Catalog;
use iced::{Color, Event, Length, Padding, Pixels, Point, Rectangle, Size, Vector};

pub struct RichTextInput<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::text::Renderer,
{
    id: Option<iced::widget::Id>,
    placeholder: String,
    value: String,
    ghost_text: Option<String>,
    is_secure: bool,
    font: Option<Renderer::Font>,
    width: Length,
    padding: Padding,
    size: Option<Pixels>,
    line_height: core_text::LineHeight,
    alignment: alignment::Horizontal,
    on_input: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_submit: Option<Message>,
    class: Theme::Class<'a>,
    last_status: Option<Status>,
}

impl<'a, Message, Theme, Renderer> RichTextInput<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::text::Renderer,
    Message: Clone,
{
    pub fn new(placeholder: &'a str) -> Self {
        Self {
            id: None,
            placeholder: String::from(placeholder),
            value: String::new(),
            ghost_text: None,
            is_secure: false,
            font: None,
            width: Length::Fill,
            padding: iced::widget::text_input::DEFAULT_PADDING,
            size: None,
            line_height: core_text::LineHeight::default(),
            alignment: alignment::Horizontal::Left,
            on_input: None,
            on_submit: None,
            class: Theme::default(),
            last_status: None,
        }
    }

    pub fn id(mut self, id: iced::widget::Id) -> Self {
        self.id = Some(id);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    pub fn ghost_text(mut self, text: impl Into<String>) -> Self {
        self.ghost_text = Some(text.into());
        self
    }

    pub fn on_input(mut self, on_input: impl Fn(String) -> Message + 'a) -> Self {
        self.on_input = Some(Box::new(on_input));
        self
    }

    pub fn on_submit(mut self, message: Message) -> Self {
        self.on_submit = Some(message);
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(Pixels(size));
        self
    }

    pub fn font(mut self, font: Renderer::Font) -> Self {
        self.font = Some(font);
        self
    }

    pub fn class(mut self, class: Theme::Class<'a>) -> Self {
        self.class = class;
        self
    }
}

struct RichTextInputState<P: ParagraphTrait> {
    value: TextValue,
    cursor: text_input::Cursor,
    value_paragraph: paragraph::Plain<P>,
    placeholder_paragraph: paragraph::Plain<P>,
    ghost_paragraph: paragraph::Plain<P>,
    is_focused: Option<Focus>,
    is_dragging: Option<Drag>,
    is_pasting: Option<TextValue>,
    last_click: Option<mouse::Click>,
    keyboard_modifiers: keyboard::Modifiers,
}

impl<P: ParagraphTrait> RichTextInputState<P> {
    fn new() -> Self {
        Self {
            value: TextValue::new(""),
            cursor: text_input::Cursor::default(),
            value_paragraph: paragraph::Plain::default(),
            placeholder_paragraph: paragraph::Plain::default(),
            ghost_paragraph: paragraph::Plain::default(),
            is_focused: None,
            is_dragging: None,
            is_pasting: None,
            last_click: None,
            keyboard_modifiers: keyboard::Modifiers::default(),
        }
    }

    fn is_focused(&self) -> bool {
        self.is_focused.is_some()
    }
}

#[derive(Debug, Clone)]
struct Focus {
    updated_at: Instant,
    now: Instant,
    is_window_focused: bool,
}

impl Focus {
    #[allow(dead_code)]
    fn is_focused(&self) -> bool {
        self.is_window_focused
    }
}

#[derive(Debug, Clone)]
enum Drag {
    Select,
    SelectWords { anchor: usize },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Status {
    Active,
    Hovered,
    Focused { is_hovered: bool },
    Disabled,
}

const CURSOR_BLINK_INTERVAL_MILLIS: u128 = 500;

fn state<Renderer: iced::advanced::text::Renderer>(
    tree: &mut Tree,
) -> &mut RichTextInputState<Renderer::Paragraph> {
    tree.state
        .downcast_mut::<RichTextInputState<Renderer::Paragraph>>()
}

#[allow(clippy::too_many_arguments)]
fn replace_paragraph<Renderer>(
    renderer: &Renderer,
    state: &mut RichTextInputState<Renderer::Paragraph>,
    layout: Layout<'_>,
    secure: bool,
    font: Option<Renderer::Font>,
    text_size: Option<Pixels>,
    line_height: core_text::LineHeight,
    ghost_text: Option<&str>,
) where
    Renderer: iced::advanced::text::Renderer,
{
    let font = font.unwrap_or_else(|| renderer.default_font());
    let text_size = text_size.unwrap_or_else(|| renderer.default_size());
    let text_bounds = layout.children().next().unwrap().bounds();

    let display_value = if secure {
        state.value.secure()
    } else {
        state.value.clone()
    };

    state.value_paragraph = paragraph::Plain::new(core_text::Text {
        font,
        line_height,
        content: display_value.to_string(),
        bounds: Size::new(f32::INFINITY, text_bounds.height),
        size: text_size,
        align_x: core_text::Alignment::Default,
        align_y: alignment::Vertical::Center,
        shaping: core_text::Shaping::Advanced,
        wrapping: core_text::Wrapping::default(),
    });

    state.placeholder_paragraph = paragraph::Plain::new(core_text::Text {
        font,
        line_height,
        content: String::new(),
        bounds: Size::new(text_bounds.width, text_bounds.height),
        size: text_size,
        align_x: core_text::Alignment::Default,
        align_y: alignment::Vertical::Center,
        shaping: core_text::Shaping::Advanced,
        wrapping: core_text::Wrapping::default(),
    });

    let ghost_content = if secure {
        String::new()
    } else {
        ghost_text
            .and_then(|suggestion| {
                let lower_suggestion = suggestion.to_lowercase();
                let value_content = state.value.to_string();
                let lower_value = value_content.to_lowercase();
                if lower_suggestion.starts_with(&lower_value)
                    && lower_suggestion.len() > lower_value.len()
                {
                    Some(&suggestion[lower_value.len()..])
                } else {
                    None
                }
            })
            .unwrap_or("")
            .to_string()
    };

    state.ghost_paragraph = paragraph::Plain::new(core_text::Text {
        font,
        line_height,
        content: ghost_content,
        bounds: Size::new(f32::INFINITY, text_bounds.height),
        size: text_size,
        align_x: core_text::Alignment::Default,
        align_y: alignment::Vertical::Center,
        shaping: core_text::Shaping::Advanced,
        wrapping: core_text::Wrapping::default(),
    });
}

#[allow(clippy::too_many_arguments)]
fn update_placeholder_and_ghost<Renderer>(
    state: &mut RichTextInputState<Renderer::Paragraph>,
    placeholder: &str,
    ghost_text: Option<&str>,
    value: &str,
    font: Renderer::Font,
    text_size: Pixels,
    line_height: core_text::LineHeight,
    text_bounds: Rectangle,
    secure: bool,
) where
    Renderer: iced::advanced::text::Renderer,
{
    let _ = state.placeholder_paragraph.update(core_text::Text {
        font,
        line_height,
        content: placeholder,
        bounds: Size::new(text_bounds.width, text_bounds.height),
        size: text_size,
        align_x: core_text::Alignment::Default,
        align_y: alignment::Vertical::Center,
        shaping: core_text::Shaping::Advanced,
        wrapping: core_text::Wrapping::default(),
    });

    let ghost_content = if secure {
        String::new()
    } else {
        ghost_text
            .and_then(|suggestion| {
                let lower_suggestion = suggestion.to_lowercase();
                let lower_value = value.to_lowercase();
                if lower_suggestion.starts_with(&lower_value)
                    && lower_suggestion.len() > lower_value.len()
                {
                    Some(&suggestion[lower_value.len()..])
                } else {
                    None
                }
            })
            .unwrap_or("")
            .to_string()
    };

    let _ = state.ghost_paragraph.update(core_text::Text {
        font,
        line_height,
        content: &ghost_content,
        bounds: Size::new(f32::INFINITY, text_bounds.height),
        size: text_size,
        align_x: core_text::Alignment::Default,
        align_y: alignment::Vertical::Center,
        shaping: core_text::Shaping::Advanced,
        wrapping: core_text::Wrapping::default(),
    });
}

fn measure_cursor_and_scroll_offset(
    paragraph: &impl ParagraphTrait,
    text_bounds: Rectangle,
    cursor_index: usize,
) -> (f32, f32) {
    let grapheme_position = paragraph
        .grapheme_position(0, cursor_index)
        .unwrap_or(Point::ORIGIN);

    let offset = ((grapheme_position.x + 5.0) - text_bounds.width).max(0.0);

    (grapheme_position.x, offset)
}

fn scroll_offset<P: ParagraphTrait>(
    text_bounds: Rectangle,
    value: &TextValue,
    state: &RichTextInputState<P>,
) -> f32 {
    if state.is_focused() {
        let focus_position = match state.cursor.state(value) {
            text_input::State::Index(i) => i,
            text_input::State::Selection { end, .. } => end,
        };

        let (_, offset) = measure_cursor_and_scroll_offset(
            state.value_paragraph.raw(),
            text_bounds,
            focus_position,
        );

        offset
    } else {
        0.0
    }
}

fn find_cursor_position<P: ParagraphTrait>(
    text_bounds: Rectangle,
    value: &TextValue,
    state: &RichTextInputState<P>,
    x: f32,
) -> Option<usize> {
    let offset = scroll_offset(text_bounds, value, state);
    let value_string = value.to_string();

    let char_offset = state
        .value_paragraph
        .raw()
        .hit_test(Point::new(x + offset, text_bounds.height / 2.0))
        .map(core_text::Hit::cursor)?;

    Some(
        unicode_segmentation::UnicodeSegmentation::graphemes(
            &value_string[..char_offset.min(value_string.len())],
            true,
        )
        .count(),
    )
}

fn alignment_offset(
    text_bounds_width: f32,
    text_min_width: f32,
    alignment: alignment::Horizontal,
) -> f32 {
    if text_min_width > text_bounds_width {
        0.0
    } else {
        match alignment {
            alignment::Horizontal::Left => 0.0,
            alignment::Horizontal::Center => (text_bounds_width - text_min_width) / 2.0,
            alignment::Horizontal::Right => text_bounds_width - text_min_width,
        }
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for RichTextInput<'a, Message, Theme, Renderer>
where
    Theme: Catalog,
    Renderer: iced::advanced::text::Renderer,
    Message: Clone + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<RichTextInputState<Renderer::Paragraph>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(RichTextInputState::<Renderer::Paragraph>::new())
    }

    fn diff(&self, tree: &mut Tree) {
        let state = tree
            .state
            .downcast_mut::<RichTextInputState<Renderer::Paragraph>>();

        if self.on_input.is_none() {
            state.is_pasting = None;
        }
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let text_size = self.size.unwrap_or_else(|| renderer.default_size());
        let padding = self.padding.fit(Size::ZERO, limits.max());
        let height = self.line_height.to_absolute(text_size);

        let limits = limits.width(self.width).shrink(padding);
        let text_bounds = limits.resolve(self.width, height, Size::ZERO);

        let state = tree
            .state
            .downcast_mut::<RichTextInputState<Renderer::Paragraph>>();

        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let display_value = if self.is_secure {
            state.value.secure()
        } else {
            state.value.clone()
        };

        state.value_paragraph = paragraph::Plain::new(core_text::Text {
            font,
            line_height: self.line_height,
            content: display_value.to_string(),
            bounds: Size::new(f32::INFINITY, text_bounds.height),
            size: text_size,
            align_x: core_text::Alignment::Default,
            align_y: alignment::Vertical::Center,
            shaping: core_text::Shaping::Advanced,
            wrapping: core_text::Wrapping::default(),
        });

        layout::Node::with_children(
            text_bounds.expand(padding),
            vec![layout::Node::new(text_bounds).move_to(Point::new(padding.left, padding.top))],
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let state = tree
            .state
            .downcast_ref::<RichTextInputState<Renderer::Paragraph>>();

        let is_disabled = self.on_input.is_none();
        let current_status = if is_disabled {
            Status::Disabled
        } else if state.is_focused() {
            Status::Focused {
                is_hovered: cursor.is_over(layout.bounds()),
            }
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        let style = theme.style(
            &self.class,
            match current_status {
                Status::Active => iced::widget::text_input::Status::Active,
                Status::Hovered => iced::widget::text_input::Status::Hovered,
                Status::Focused { .. } => iced::widget::text_input::Status::Focused {
                    is_hovered: matches!(current_status, Status::Focused { is_hovered: true }),
                },
                Status::Disabled => iced::widget::text_input::Status::Disabled,
            },
        );

        let bounds = layout.bounds();
        let text_bounds = layout.children().next().unwrap().bounds();

        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: style.border,
                ..renderer::Quad::default()
            },
            style.background,
        );

        let display_value = if self.is_secure {
            state.value.secure()
        } else {
            state.value.clone()
        };
        let text = display_value.to_string();

        let (cursor_quad, offset, is_selecting) = if let Some(focus) = state
            .is_focused
            .as_ref()
            .filter(|focus| focus.is_window_focused)
        {
            match state.cursor.state(&display_value) {
                text_input::State::Index(position) => {
                    let (text_value_width, offset) = measure_cursor_and_scroll_offset(
                        state.value_paragraph.raw(),
                        text_bounds,
                        position,
                    );

                    let is_cursor_visible = !is_disabled
                        && ((focus.now - focus.updated_at).as_millis()
                            / CURSOR_BLINK_INTERVAL_MILLIS)
                            .is_multiple_of(2);

                    let cursor_quad = if is_cursor_visible {
                        Some((
                            renderer::Quad {
                                bounds: Rectangle {
                                    x: (text_bounds.x + text_value_width).floor(),
                                    y: text_bounds.y,
                                    width: 1.0,
                                    height: text_bounds.height,
                                },
                                ..renderer::Quad::default()
                            },
                            style.value,
                        ))
                    } else {
                        None
                    };

                    (cursor_quad, offset, false)
                }
                text_input::State::Selection { start, end } => {
                    let left = start.min(end);
                    let right = end.max(start);

                    let (left_position, left_offset) = measure_cursor_and_scroll_offset(
                        state.value_paragraph.raw(),
                        text_bounds,
                        left,
                    );

                    let (right_position, right_offset) = measure_cursor_and_scroll_offset(
                        state.value_paragraph.raw(),
                        text_bounds,
                        right,
                    );

                    let width = right_position - left_position;

                    (
                        Some((
                            renderer::Quad {
                                bounds: Rectangle {
                                    x: text_bounds.x + left_position,
                                    y: text_bounds.y,
                                    width,
                                    height: text_bounds.height,
                                },
                                ..renderer::Quad::default()
                            },
                            style.selection,
                        )),
                        if end == right {
                            right_offset
                        } else {
                            left_offset
                        },
                        true,
                    )
                }
            }
        } else {
            (None, 0.0, false)
        };

        let draw = |renderer: &mut Renderer, clip_bounds: Rectangle| {
            let paragraph = if text.is_empty() {
                state.placeholder_paragraph.raw()
            } else {
                state.value_paragraph.raw()
            };

            let align_offset =
                alignment_offset(text_bounds.width, paragraph.min_width(), self.alignment);

            if let Some((cursor_quad, color)) = cursor_quad {
                renderer.with_translation(Vector::new(align_offset - offset, 0.0), |renderer| {
                    renderer.fill_quad(cursor_quad, color);
                });
            }

            renderer.fill_paragraph(
                paragraph,
                text_bounds.anchor(
                    paragraph.min_bounds(),
                    alignment::Horizontal::Left,
                    alignment::Vertical::Center,
                ) + Vector::new(align_offset - offset, 0.0),
                if text.is_empty() {
                    style.placeholder
                } else {
                    style.value
                },
                clip_bounds,
            );

            if !text.is_empty() && !self.is_secure {
                let ghost = state.ghost_paragraph.raw();
                if ghost.min_width() > 0.0 {
                    let value_width = state.value_paragraph.raw().min_width();
                    let ghost_start_x = align_offset - offset + value_width;

                    if ghost_start_x < text_bounds.width {
                        renderer.fill_paragraph(
                            ghost,
                            text_bounds.anchor(
                                ghost.min_bounds(),
                                alignment::Horizontal::Left,
                                alignment::Vertical::Center,
                            ) + Vector::new(ghost_start_x, 0.0),
                            Color::from_rgba(0.5, 0.5, 0.5, 0.5),
                            clip_bounds,
                        );
                    }
                }
            }
        };

        if is_selecting {
            renderer.with_layer(text_bounds, |renderer| draw(renderer, text_bounds));
        } else {
            draw(renderer, text_bounds);
        }
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = state::<Renderer>(tree);

        if self.value != state.value.to_string() {
            state.value = TextValue::new(&self.value);
            state.cursor.move_to(state.value.len());
        }

        let font = self.font.unwrap_or_else(|| renderer.default_font());
        let text_size = self.size.unwrap_or_else(|| renderer.default_size());
        let text_bounds = layout.children().next().unwrap().bounds();

        update_placeholder_and_ghost::<Renderer>(
            state,
            &self.placeholder,
            self.ghost_text.as_deref(),
            &self.value,
            font,
            text_size,
            self.line_height,
            text_bounds,
            self.is_secure,
        );

        let sync_value =
            |widget: &mut Self, state: &mut RichTextInputState<Renderer::Paragraph>| {
                widget.value = state.value.to_string();
                replace_paragraph(
                    renderer,
                    state,
                    layout,
                    widget.is_secure,
                    widget.font,
                    widget.size,
                    widget.line_height,
                    widget.ghost_text.as_deref(),
                );
            };

        match &event {
            Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerPressed { .. }) => {
                let cursor_before = state.cursor;

                let click_position = cursor.position_over(layout.bounds());

                state.is_focused = if click_position.is_some() {
                    let now = Instant::now();
                    Some(Focus {
                        updated_at: now,
                        now,
                        is_window_focused: true,
                    })
                } else {
                    None
                };

                if let Some(cursor_position) = click_position {
                    let target = {
                        let align_offset = alignment_offset(
                            text_bounds.width,
                            state.value_paragraph.raw().min_width(),
                            self.alignment,
                        );
                        cursor_position.x - text_bounds.x - align_offset
                    };

                    let click =
                        mouse::Click::new(cursor_position, mouse::Button::Left, state.last_click);

                    let value = if self.is_secure {
                        state.value.secure()
                    } else {
                        state.value.clone()
                    };

                    match click.kind() {
                        mouse::click::Kind::Single => {
                            let position = if target > 0.0 {
                                find_cursor_position(text_bounds, &value, state, target)
                            } else {
                                None
                            }
                            .unwrap_or(0);

                            if state.keyboard_modifiers.shift() {
                                state
                                    .cursor
                                    .select_range(state.cursor.start(&state.value), position);
                            } else {
                                state.cursor.move_to(position);
                            }

                            state.is_dragging = Some(Drag::Select);
                        }
                        mouse::click::Kind::Double => {
                            if self.is_secure {
                                state.cursor.select_all(&state.value);
                                state.is_dragging = None;
                            } else {
                                let position =
                                    find_cursor_position(text_bounds, &state.value, state, target)
                                        .unwrap_or(0);

                                state.cursor.select_range(
                                    state.value.previous_start_of_word(position),
                                    state.value.next_end_of_word(position),
                                );

                                state.is_dragging = Some(Drag::SelectWords { anchor: position });
                            }
                        }
                        mouse::click::Kind::Triple => {
                            state.cursor.select_all(&state.value);
                            state.is_dragging = None;
                        }
                    }

                    state.last_click = Some(click);

                    if cursor_before != state.cursor {
                        shell.request_redraw();
                    }

                    shell.capture_event();
                }
            }
            Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
            | Event::Touch(touch::Event::FingerLifted { .. })
            | Event::Touch(touch::Event::FingerLost { .. }) => {
                state.is_dragging = None;
            }
            Event::Mouse(mouse::Event::CursorMoved { position })
            | Event::Touch(touch::Event::FingerMoved { position, .. }) => {
                if state.is_dragging.is_some() {
                    let target = {
                        let align_offset = alignment_offset(
                            text_bounds.width,
                            state.value_paragraph.raw().min_width(),
                            self.alignment,
                        );
                        position.x - text_bounds.x - align_offset
                    };

                    let value = if self.is_secure {
                        state.value.secure()
                    } else {
                        state.value.clone()
                    };

                    let cursor_position =
                        find_cursor_position(text_bounds, &value, state, target).unwrap_or(0);

                    let selection_before = state.cursor.selection(&value);

                    match state.is_dragging {
                        Some(Drag::Select) => {
                            state
                                .cursor
                                .select_range(state.cursor.start(&value), cursor_position);
                        }
                        Some(Drag::SelectWords { anchor }) => {
                            if cursor_position < anchor {
                                state.cursor.select_range(
                                    state.value.previous_start_of_word(cursor_position),
                                    state.value.next_end_of_word(anchor),
                                );
                            } else {
                                state.cursor.select_range(
                                    state.value.previous_start_of_word(anchor),
                                    state.value.next_end_of_word(cursor_position),
                                );
                            }
                        }
                        None => {}
                    }

                    if let Some(focus) = &mut state.is_focused {
                        focus.updated_at = Instant::now();
                    }

                    if selection_before != state.cursor.selection(&value) {
                        shell.request_redraw();
                    }

                    shell.capture_event();
                }
            }
            Event::Keyboard(keyboard::Event::KeyPressed {
                key,
                text,
                modified_key,
                physical_key,
                ..
            }) => {
                if let Some(focus) = &mut state.is_focused {
                    let modifiers = state.keyboard_modifiers;

                    match key.to_latin(*physical_key) {
                        Some('c') if state.keyboard_modifiers.command() && !self.is_secure => {
                            if let Some((start, end)) = state.cursor.selection(&state.value) {
                                clipboard.write(
                                    clipboard::Kind::Standard,
                                    state.value.select(start, end).to_string(),
                                );
                            }

                            shell.capture_event();
                            return;
                        }
                        Some('x') if state.keyboard_modifiers.command() && !self.is_secure => {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            if let Some((start, end)) = state.cursor.selection(&state.value) {
                                clipboard.write(
                                    clipboard::Kind::Standard,
                                    state.value.select(start, end).to_string(),
                                );
                            }

                            let mut editor = Editor::new(&mut state.value, &mut state.cursor);
                            editor.delete();

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = Instant::now();
                            sync_value(self, state);
                            return;
                        }
                        Some('v')
                            if state.keyboard_modifiers.command()
                                && !state.keyboard_modifiers.alt() =>
                        {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            let content = match state.is_pasting.take() {
                                Some(content) => content,
                                None => {
                                    let content: String = clipboard
                                        .read(clipboard::Kind::Standard)
                                        .unwrap_or_default()
                                        .chars()
                                        .filter(|c| !c.is_control())
                                        .collect();

                                    TextValue::new(&content)
                                }
                            };

                            let mut editor = Editor::new(&mut state.value, &mut state.cursor);
                            editor.paste(content.clone());

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            state.is_pasting = Some(content);
                            focus.updated_at = Instant::now();
                            sync_value(self, state);
                            return;
                        }
                        Some('a') if state.keyboard_modifiers.command() => {
                            let cursor_before = state.cursor;

                            state.cursor.select_all(&state.value);

                            if cursor_before != state.cursor {
                                focus.updated_at = Instant::now();
                                shell.request_redraw();
                            }

                            shell.capture_event();
                            return;
                        }
                        _ => {}
                    }

                    if let Some(text) = text {
                        let Some(on_input) = &self.on_input else {
                            return;
                        };

                        state.is_pasting = None;

                        if let Some(c) = text.chars().next().filter(|c| !c.is_control()) {
                            let mut editor = Editor::new(&mut state.value, &mut state.cursor);

                            editor.insert(c);

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = Instant::now();
                            sync_value(self, state);
                            return;
                        }
                    }

                    match modified_key.as_ref() {
                        keyboard::Key::Named(key::Named::Tab) => {
                            if self.ghost_text.is_some() {
                                if let Some(on_submit) = self.on_submit.clone() {
                                    shell.publish(on_submit);
                                }
                                shell.capture_event();
                            }
                        }
                        keyboard::Key::Named(key::Named::Enter) => {
                            if let Some(on_submit) = self.on_submit.clone() {
                                shell.publish(on_submit);
                                shell.capture_event();
                            }
                        }
                        keyboard::Key::Named(key::Named::Backspace) => {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            if state.cursor.selection(&state.value).is_none() {
                                if (self.is_secure && modifiers.jump()) || modifiers.macos_command()
                                {
                                    state
                                        .cursor
                                        .select_range(state.cursor.start(&state.value), 0);
                                } else if modifiers.jump() {
                                    state.cursor.select_left_by_words(&state.value);
                                }
                            }

                            let mut editor = Editor::new(&mut state.value, &mut state.cursor);
                            editor.backspace();

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = Instant::now();
                            sync_value(self, state);
                        }
                        keyboard::Key::Named(key::Named::Delete) => {
                            let Some(on_input) = &self.on_input else {
                                return;
                            };

                            if state.cursor.selection(&state.value).is_none() {
                                if (self.is_secure && modifiers.jump()) || modifiers.macos_command()
                                {
                                    state.cursor.select_range(
                                        state.cursor.start(&state.value),
                                        state.value.len(),
                                    );
                                } else if modifiers.jump() {
                                    state.cursor.select_right_by_words(&state.value);
                                }
                            }

                            let mut editor = Editor::new(&mut state.value, &mut state.cursor);
                            editor.delete();

                            let message = (on_input)(editor.contents());
                            shell.publish(message);
                            shell.capture_event();

                            focus.updated_at = Instant::now();
                            sync_value(self, state);
                        }
                        keyboard::Key::Named(key::Named::Home) => {
                            let cursor_before = state.cursor;

                            if modifiers.shift() {
                                state
                                    .cursor
                                    .select_range(state.cursor.start(&state.value), 0);
                            } else {
                                state.cursor.move_to(0);
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = Instant::now();
                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        keyboard::Key::Named(key::Named::End) => {
                            let cursor_before = state.cursor;

                            if modifiers.shift() {
                                state.cursor.select_range(
                                    state.cursor.start(&state.value),
                                    state.value.len(),
                                );
                            } else {
                                state.cursor.move_to(state.value.len());
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = Instant::now();
                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        keyboard::Key::Named(key::Named::ArrowLeft) => {
                            let cursor_before = state.cursor;

                            if (self.is_secure && modifiers.jump()) || modifiers.macos_command() {
                                if modifiers.shift() {
                                    state
                                        .cursor
                                        .select_range(state.cursor.start(&state.value), 0);
                                } else {
                                    state.cursor.move_to(0);
                                }
                            } else if modifiers.jump() {
                                if modifiers.shift() {
                                    state.cursor.select_left_by_words(&state.value);
                                } else {
                                    state.cursor.move_left_by_words(&state.value);
                                }
                            } else if modifiers.shift() {
                                state.cursor.select_left(&state.value);
                            } else {
                                state.cursor.move_left(&state.value);
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = Instant::now();
                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        keyboard::Key::Named(key::Named::ArrowRight) => {
                            let cursor_before = state.cursor;

                            if (self.is_secure && modifiers.jump()) || modifiers.macos_command() {
                                if modifiers.shift() {
                                    state.cursor.select_range(
                                        state.cursor.start(&state.value),
                                        state.value.len(),
                                    );
                                } else {
                                    state.cursor.move_to(state.value.len());
                                }
                            } else if modifiers.jump() {
                                if modifiers.shift() {
                                    state.cursor.select_right_by_words(&state.value);
                                } else {
                                    state.cursor.move_right_by_words(&state.value);
                                }
                            } else if modifiers.shift() {
                                state.cursor.select_right(&state.value);
                            } else {
                                state.cursor.move_right(&state.value);
                            }

                            if cursor_before != state.cursor {
                                focus.updated_at = Instant::now();
                                shell.request_redraw();
                            }

                            shell.capture_event();
                        }
                        keyboard::Key::Named(key::Named::Escape) => {
                            state.is_focused = None;
                            state.is_dragging = None;
                            state.is_pasting = None;
                            state.keyboard_modifiers = keyboard::Modifiers::default();

                            shell.capture_event();
                        }
                        _ => {}
                    }
                }
            }
            Event::Keyboard(keyboard::Event::KeyReleased { key, .. }) => {
                if state.is_focused.is_some()
                    && let keyboard::Key::Character("v") = key.as_ref()
                {
                    state.is_pasting = None;
                    shell.capture_event();
                }

                state.is_pasting = None;
            }
            Event::Keyboard(keyboard::Event::ModifiersChanged(modifiers)) => {
                state.keyboard_modifiers = *modifiers;
            }
            Event::Window(iced::window::Event::Unfocused) => {
                if let Some(focus) = &mut state.is_focused {
                    focus.is_window_focused = false;
                }
            }
            Event::Window(iced::window::Event::Focused) => {
                if let Some(focus) = &mut state.is_focused {
                    focus.is_window_focused = true;
                    focus.updated_at = Instant::now();
                    shell.request_redraw();
                }
            }
            Event::Window(iced::window::Event::RedrawRequested(now)) => {
                if let Some(focus) = &mut state.is_focused
                    && focus.is_window_focused
                    && matches!(
                        state.cursor.state(&state.value),
                        text_input::State::Index(_)
                    )
                {
                    focus.now = *now;

                    let millis_until_redraw = CURSOR_BLINK_INTERVAL_MILLIS
                        - (*now - focus.updated_at).as_millis() % CURSOR_BLINK_INTERVAL_MILLIS;

                    shell.request_redraw_at(
                        *now + Duration::from_millis(millis_until_redraw as u64),
                    );
                }
            }
            _ => {}
        }

        let current_status = if self.on_input.is_none() {
            Status::Disabled
        } else if state.is_focused() {
            Status::Focused {
                is_hovered: cursor.is_over(layout.bounds()),
            }
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        if let Event::Window(iced::window::Event::RedrawRequested(_now)) = event {
            self.last_status = Some(current_status);
        } else if self
            .last_status
            .is_some_and(|last_status| current_status != last_status)
        {
            shell.request_redraw();
        }
    }

    fn mouse_interaction(
        &self,
        _tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            if self.on_input.is_none() {
                mouse::Interaction::Idle
            } else {
                mouse::Interaction::Text
            }
        } else {
            mouse::Interaction::default()
        }
    }
}
