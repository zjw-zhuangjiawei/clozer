mod actions;
mod meaning;
mod search;
mod tags;
mod tree;

pub(super) use self::actions::build_action_bar;
pub(super) use self::search::build_search_bar;
pub(super) use self::tree::build_word_tree;

use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::layout::breakpoint::Breakpoint;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::widgets::container::card;
use crate::ui::words::message::{DeleteTarget, WordsMessage};
use crate::ui::words::state::WordsState;
use iced::Element;
use iced::widget::{Button, Column, Container, Row, Text};

pub fn view<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    breakpoint: Breakpoint,
) -> Element<'a, WordsMessage, AppTheme> {
    let (left_ratio, right_ratio) = breakpoint.column_ratio();

    let search_bar = build_search_bar(words_state, model, breakpoint);

    let word_tree = build_word_tree(words_state, model);

    if breakpoint.is_single_column() {
        if let Some(ref target) = words_state.pending_delete {
            return build_delete_confirmation(target, model);
        }
        Column::new()
            .push(search_bar)
            .push(iced::widget::rule::horizontal(1))
            .push(iced::widget::scrollable(word_tree).height(iced::Length::Fill))
            .push(build_action_bar(words_state, model))
            .spacing(Spacing::DEFAULT.s2)
            .padding(Spacing::DEFAULT.s2)
            .height(iced::Length::Fill)
            .into()
    } else {
        let left_panel = Column::new()
            .push(search_bar)
            .push(iced::widget::rule::horizontal(1))
            .push(iced::widget::scrollable(word_tree).height(iced::Length::Fill))
            .push(build_action_bar(words_state, model))
            .spacing(Spacing::DEFAULT.s2)
            .padding(Spacing::DEFAULT.s2)
            .width(iced::Length::FillPortion((left_ratio * 10.0) as u16));

        let right_panel: Element<'a, WordsMessage, AppTheme> =
            if let Some(ref target) = words_state.pending_delete {
                Container::new(build_delete_confirmation(target, model))
                    .width(iced::Length::FillPortion((right_ratio * 10.0) as u16))
                    .height(iced::Length::Fill)
                    .style(card)
                    .into()
            } else {
                Container::new(crate::ui::words::detail::view(
                    words_state.panel.state(),
                    &words_state.panel.word_buffer,
                    &words_state.panel.meaning_buffer,
                    words_state.panel.dictionary_loading,
                    &words_state.panel.dictionary_result,
                    model,
                ))
                .width(iced::Length::FillPortion((right_ratio * 10.0) as u16))
                .height(iced::Length::Fill)
                .style(card)
                .into()
            };

        Row::new()
            .push(left_panel)
            .push(right_panel)
            .spacing(Spacing::DEFAULT.xs2)
            .height(iced::Length::Fill)
            .into()
    }
}

fn build_delete_confirmation<'a>(
    target: &'a DeleteTarget,
    model: &'a Model,
) -> Element<'a, WordsMessage, AppTheme> {
    let (title, warning) = match target {
        DeleteTarget::Word(word_id) => {
            let word = model.word_registry.get(*word_id);
            let name = word.map(|w| w.content.as_str()).unwrap_or("unknown word");
            let meaning_count = word.map(|w| w.meaning_ids.len()).unwrap_or(0);
            (
                "Delete Word",
                format!("Delete \"{}\" and its {} meaning(s)?", name, meaning_count),
            )
        }
        DeleteTarget::Meanings(ids) => (
            "Delete Meanings",
            format!("Delete {} selected meaning(s)?", ids.len()),
        ),
        DeleteTarget::Clozes(ids) => (
            "Delete Clozes",
            format!("Delete {} selected cloze(s)?", ids.len()),
        ),
    };

    let content = Column::new()
        .push(Text::new(title).size(FontSize::Heading.px()))
        .push(
            Container::new(Text::new(warning).size(FontSize::Body.px()))
                .padding(Spacing::DEFAULT.m)
                .style(|theme: &AppTheme| {
                    let colors = theme.colors();
                    iced::widget::container::Style {
                        background: Some(colors.functional.danger.w50().into()),
                        border: iced::Border {
                            color: colors.functional.danger.w200(),
                            width: 1.0,
                            radius: Spacing::DEFAULT.xs.into(),
                        },
                        ..Default::default()
                    }
                }),
        )
        .push(
            Row::new()
                .push(
                    Button::new(Text::new("Delete").size(FontSize::Body.px()))
                        .style(button::danger)
                        .padding(ButtonSize::Standard.to_iced_padding())
                        .on_press(WordsMessage::DeleteConfirmed(target.clone())),
                )
                .push(
                    Button::new(Text::new("Cancel").size(FontSize::Body.px()))
                        .style(button::secondary)
                        .padding(ButtonSize::Standard.to_iced_padding())
                        .on_press(WordsMessage::DeleteCancelled),
                )
                .spacing(Spacing::DEFAULT.s),
        )
        .spacing(Spacing::DEFAULT.l)
        .padding(Spacing::DEFAULT.l);

    Container::new(content)
        .padding(Spacing::DEFAULT.l2)
        .width(iced::Length::Fill)
        .height(iced::Length::Fill)
        .style(card)
        .center_x(iced::Length::Fill)
        .center_y(iced::Length::Fill)
        .into()
}
