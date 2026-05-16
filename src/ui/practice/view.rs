use crate::i18n::I18nManager;
use crate::models::cloze::ClozeSegment;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::practice::message::PracticeMessage;
use crate::ui::practice::state::PracticeState;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::widgets::container::card;
use crate::ui::widgets::text as txt;
use iced::widget::{Button, Column, Container, Row, Scrollable, Text, TextInput, rule};
use iced::{Alignment, Element, Length};

pub fn view<'a>(
    state: &'a PracticeState,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Element<'a, PracticeMessage, AppTheme> {
    if !state.is_active {
        setup_view(state, model, i18n)
    } else if state.is_session_complete() {
        complete_view(state, i18n)
    } else {
        practice_view(state, model, i18n)
    }
}

fn setup_view<'a>(
    state: &'a PracticeState,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Element<'a, PracticeMessage, AppTheme> {
    let tag_name = state
        .tag_filter
        .and_then(|id| model.tag_registry.get(id))
        .map(|t| t.name.clone())
        .unwrap_or_else(|| i18n.tr("practice-all-clozes").to_string());

    let clozes_available = count_available_clozes(state, model);

    let tag_button = Button::new(
        Row::new()
            .push(Text::new(i18n.tr("practice-filter")).size(FontSize::Body.px()))
            .push(
                Text::new(tag_name.clone())
                    .size(FontSize::Body.px())
                    .style(txt::primary),
            )
            .spacing(Spacing::DEFAULT.xs),
    )
    .style(button::secondary)
    .padding(ButtonSize::Standard.to_iced_padding())
    .on_press(PracticeMessage::ToggleTagPicker);

    let mut content = Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(Text::new(i18n.tr("practice-title")).size(FontSize::Display.px()))
        .push(
            Text::new(i18n.tr("practice-description"))
                .size(FontSize::Body.px())
                .style(txt::secondary),
        )
        .push(tag_button);

    if state.show_tag_picker {
        content = content.push(build_tag_picker(state, model, i18n));
    }

    if state.tag_filter.is_some() {
        content = content.push(
            Button::new(Text::new(i18n.tr("practice-clear-filter")).size(FontSize::Footnote.px()))
                .style(button::tertiary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(PracticeMessage::TagFilterCleared),
        );
    }

    content = content.push(
        Text::new(i18n.tr_with(
            "practice-clozes-available",
            &[&clozes_available.to_string()],
        ))
        .size(FontSize::Footnote.px())
        .style(txt::secondary),
    );

    let start_label = i18n.tr("practice-start-session");
    let start_btn = if clozes_available > 0 {
        Button::new(Text::new(start_label).size(FontSize::Body.px()))
            .style(button::primary)
            .padding(ButtonSize::Standard.to_iced_padding())
            .on_press(PracticeMessage::StartSession)
    } else {
        Button::new(Text::new(start_label).size(FontSize::Body.px()))
            .style(button::secondary)
            .padding(ButtonSize::Standard.to_iced_padding())
    };

    content = content.push(start_btn);

    Container::new(content)
        .padding(Spacing::DEFAULT.l)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(card)
        .into()
}

fn build_tag_picker<'a>(
    state: &'a PracticeState,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Element<'a, PracticeMessage, AppTheme> {
    let search_input = TextInput::new(&i18n.tr("tags-search-placeholder"), &state.tag_search)
        .on_input(PracticeMessage::TagSearchChanged)
        .padding(Spacing::DEFAULT.xs);

    let all_tag_label = i18n.tr("practice-all-clozes");
    let all_tags_btn = Button::new(Text::new(all_tag_label).size(FontSize::Body.px()))
        .style(if state.tag_filter.is_none() {
            button::primary
        } else {
            button::secondary
        })
        .padding(ButtonSize::Small.to_iced_padding())
        .width(Length::Fill)
        .on_press(PracticeMessage::TagFilterCleared);

    let mut tag_items: Vec<Element<'a, PracticeMessage, AppTheme>> = vec![all_tags_btn.into()];

    let search_lower = state.tag_search.to_lowercase();
    for (id, tag) in model.tag_registry.iter() {
        if !state.tag_search.is_empty() && !tag.name.to_lowercase().contains(&search_lower) {
            continue;
        }
        let is_selected = state.tag_filter == Some(*id);
        let meaning_count = model.meaning_registry.iter_by_tag(*id).count();

        let btn = Button::new(
            Row::new()
                .push(Text::new(&tag.name).size(FontSize::Body.px()))
                .push(
                    Text::new(format!(" ({})", meaning_count))
                        .size(FontSize::Footnote.px())
                        .style(txt::secondary),
                )
                .spacing(Spacing::DEFAULT.xs),
        )
        .style(if is_selected {
            button::primary
        } else {
            button::secondary
        })
        .padding(ButtonSize::Small.to_iced_padding())
        .width(Length::Fill)
        .on_press(PracticeMessage::TagFilterSelected(*id));

        tag_items.push(btn.into());
    }

    let list = Column::with_children(tag_items).spacing(Spacing::DEFAULT.xxs);

    Column::new()
        .spacing(Spacing::DEFAULT.s)
        .push(search_input)
        .push(Scrollable::new(list).height(Length::Fixed(200.0)))
        .into()
}

fn practice_view<'a>(
    state: &'a PracticeState,
    model: &'a Model,
    i18n: &'a I18nManager,
) -> Element<'a, PracticeMessage, AppTheme> {
    let cloze = state.current_cloze(model);
    let blank_count = state.blank_segments(model).len();
    let total = state.session_clozes.len();
    let current = state.current_index + 1;

    let header = Row::new()
        .push(
            Text::new(i18n.tr_with(
                "practice-progress",
                &[&current.to_string(), &total.to_string()],
            ))
            .size(FontSize::Heading.px()),
        )
        .push(
            Text::new(i18n.tr_with(
                "practice-score",
                &[
                    &state.correct_count.to_string(),
                    &state.total_blanks.to_string(),
                    &format!("{:.0}", state.score_percent()),
                ],
            ))
            .size(FontSize::Body.px())
            .style(txt::secondary),
        )
        .push(iced::widget::Space::new().width(Length::Fill))
        .push(
            Button::new(Text::new(i18n.tr("practice-end-session")).size(FontSize::Footnote.px()))
                .style(button::secondary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(PracticeMessage::EndSession),
        )
        .spacing(Spacing::DEFAULT.l)
        .align_y(Alignment::Center);

    let sentence_text = state.render_sentence_with_numbers(model);

    let sentence_display = Container::new(Text::new(sentence_text).size(FontSize::Title.px()))
        .padding(Spacing::DEFAULT.l)
        .style(card)
        .width(Length::Fill);

    let mut blanks_section = Column::new().spacing(Spacing::DEFAULT.s);

    if let Some(cloze) = cloze {
        let mut blank_idx = 0;
        for seg in &cloze.segments {
            if let ClozeSegment::Blank(answer) = seg {
                let user_value = state.answers.get(&blank_idx).cloned().unwrap_or_default();
                let result = state.results.get(&blank_idx);

                let input_style: fn(
                    &AppTheme,
                    iced::widget::text_input::Status,
                ) -> iced::widget::text_input::Style = match result {
                    Some(true) => correct_input_style,
                    Some(false) => incorrect_input_style,
                    None => default_input_style,
                };

                let blank_placeholder = i18n.tr_with(
                    "practice-blank-placeholder",
                    &[&(blank_idx + 1).to_string()],
                );

                let input: Element<'a, PracticeMessage, AppTheme> =
                    if state.submitted {
                        TextInput::new(&blank_placeholder, &user_value)
                            .on_input(move |_| PracticeMessage::AnswerChanged {
                                blank_index: blank_idx,
                                value: String::new(),
                            })
                            .padding(Spacing::DEFAULT.s)
                            .style(input_style)
                            .into()
                    } else {
                        TextInput::new(&blank_placeholder, &user_value)
                        .on_input(move |s| PracticeMessage::AnswerChanged {
                            blank_index: blank_idx,
                            value: s,
                        })
                        .padding(Spacing::DEFAULT.s)
                        .style(move |theme: &AppTheme, _status: iced::widget::text_input::Status| {
                            let colors = theme.colors();
                            iced::widget::text_input::Style {
                                background: colors.semantic.surface.raised.into(),
                                border: iced::Border {
                                    color: colors.semantic.interactive.primary.into(),
                                    width: 1.0,
                                    radius: Spacing::DEFAULT.xs.into(),
                                },
                                icon: iced::Color::default(),
                                placeholder: colors.semantic.text.tertiary,
                                value: colors.semantic.text.primary,
                                selection: colors.semantic.text.primary,
                            }
                        })
                        .into()
                    };

                let mut row = Row::new()
                    .push(Container::new(input).width(Length::Fixed(200.0)))
                    .spacing(Spacing::DEFAULT.s)
                    .align_y(Alignment::Center);

                if let Some(is_correct) = result {
                    let feedback: Element<'a, PracticeMessage, AppTheme> = if *is_correct {
                        Text::new(i18n.tr("practice-correct"))
                            .size(FontSize::Footnote.px())
                            .style(txt::success)
                            .into()
                    } else {
                        Text::new(i18n.tr_with("practice-correct-answer", &[answer]))
                            .size(FontSize::Footnote.px())
                            .style(txt::error)
                            .into()
                    };
                    row = row.push(feedback);
                }

                blanks_section = blanks_section.push(row);
                blank_idx += 1;
            }
        }
    }

    let mut actions = Row::new()
        .spacing(Spacing::DEFAULT.s)
        .align_y(Alignment::Center);

    if !state.submitted && blank_count > 0 {
        actions = actions.push(
            Button::new(Text::new(i18n.tr("practice-check-answer")).size(FontSize::Body.px()))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(PracticeMessage::SubmitAnswers),
        );

        let all_filled = (0..blank_count).all(|i| {
            state
                .answers
                .get(&i)
                .map(|s| !s.trim().is_empty())
                .unwrap_or(false)
        });

        if !all_filled {
            actions = actions.push(
                Text::new(i18n.tr("practice-fill-blanks"))
                    .size(FontSize::Footnote.px())
                    .style(txt::secondary),
            );
        }
    }

    if !state.submitted {
        actions = actions.push(
            Button::new(Text::new(i18n.tr("practice-skip")).size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(PracticeMessage::SkipCloze),
        );
    }

    if state.submitted {
        actions = actions.push(
            Button::new(Text::new(i18n.tr("practice-previous")).size(FontSize::Body.px()))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press_maybe(
                    (state.current_index > 0).then_some(PracticeMessage::PreviousCloze),
                ),
        );
        actions = actions.push(
            Button::new(Text::new(i18n.tr("practice-next")).size(FontSize::Body.px()))
                .style(button::primary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press_maybe(
                    (state.current_index + 1 < state.session_clozes.len())
                        .then_some(PracticeMessage::NextCloze),
                ),
        );
    }

    let scrollable_content = Column::new()
        .push(header)
        .push(rule::horizontal(1))
        .push(sentence_display)
        .push(Text::new(i18n.tr("practice-your-answers")).size(FontSize::Body.px()))
        .push(blanks_section)
        .push(rule::horizontal(1))
        .push(actions)
        .spacing(Spacing::DEFAULT.l);

    Container::new(Scrollable::new(scrollable_content))
        .padding(Spacing::DEFAULT.l)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(card)
        .into()
}

fn complete_view<'a>(
    state: &'a PracticeState,
    i18n: &I18nManager,
) -> Element<'a, PracticeMessage, AppTheme> {
    let percent = state.score_percent();

    Column::new()
        .spacing(Spacing::DEFAULT.l)
        .push(Text::new(i18n.tr("practice-session-complete")).size(FontSize::Display.px()))
        .push(
            Text::new(i18n.tr_with(
                "practice-final-score",
                &[
                    &state.correct_count.to_string(),
                    &state.total_blanks.to_string(),
                    &format!("{:.0}", percent),
                ],
            ))
            .size(FontSize::Title.px()),
        )
        .push(
            Row::new()
                .spacing(Spacing::DEFAULT.s)
                .push(
                    Button::new(Text::new(i18n.tr("practice-again")).size(FontSize::Body.px()))
                        .style(button::primary)
                        .padding(ButtonSize::Standard.to_iced_padding())
                        .on_press(PracticeMessage::StartSession),
                )
                .push(
                    Button::new(
                        Text::new(i18n.tr("practice-back-to-setup")).size(FontSize::Body.px()),
                    )
                    .style(button::secondary)
                    .padding(ButtonSize::Standard.to_iced_padding())
                    .on_press(PracticeMessage::EndSession),
                ),
        )
        .align_x(Alignment::Center)
        .into()
}

fn count_available_clozes(state: &PracticeState, model: &Model) -> usize {
    if let Some(tag_id) = state.tag_filter {
        let mut count = 0;
        for (meaning_id, _) in model.meaning_registry.iter_by_tag(tag_id) {
            count += model.cloze_registry.count_by_meaning(*meaning_id);
        }
        count
    } else {
        model.cloze_registry.count()
    }
}

fn default_input_style(
    theme: &AppTheme,
    _status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let colors = theme.colors();
    iced::widget::text_input::Style {
        background: colors.semantic.surface.raised.into(),
        border: iced::Border {
            color: colors.semantic.interactive.primary.into(),
            width: 1.0,
            radius: Spacing::DEFAULT.xs.into(),
        },
        icon: iced::Color::default(),
        placeholder: colors.semantic.text.tertiary,
        value: colors.semantic.text.primary,
        selection: colors.semantic.text.primary,
    }
}

fn correct_input_style(
    theme: &AppTheme,
    _status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let colors = theme.colors();
    iced::widget::text_input::Style {
        background: colors.functional.success.w50().into(),
        border: iced::Border {
            color: colors.functional.success.w200().into(),
            width: 1.0,
            radius: Spacing::DEFAULT.xs.into(),
        },
        icon: iced::Color::default(),
        placeholder: colors.semantic.text.tertiary,
        value: colors.semantic.text.primary,
        selection: colors.semantic.text.primary,
    }
}

fn incorrect_input_style(
    theme: &AppTheme,
    _status: iced::widget::text_input::Status,
) -> iced::widget::text_input::Style {
    let colors = theme.colors();
    iced::widget::text_input::Style {
        background: colors.functional.danger.w50().into(),
        border: iced::Border {
            color: colors.functional.danger.w200().into(),
            width: 1.0,
            radius: Spacing::DEFAULT.xs.into(),
        },
        icon: iced::Color::default(),
        placeholder: colors.semantic.text.tertiary,
        value: colors.semantic.text.primary,
        selection: colors.semantic.text.primary,
    }
}
