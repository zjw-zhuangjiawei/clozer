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
use crate::ui::theme::Spacing;
use crate::ui::widgets::container::card;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::Element;
use iced::widget::{Column, Container, Row};

pub fn view<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    breakpoint: Breakpoint,
) -> Element<'a, WordsMessage, AppTheme> {
    let (left_ratio, right_ratio) = breakpoint.column_ratio();

    let search_bar = build_search_bar(words_state, model, breakpoint);

    let word_tree = build_word_tree(words_state, model);

    if breakpoint.is_single_column() {
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

        let right_panel = Container::new(crate::ui::words::detail::view(
            words_state.panel.state(),
            &words_state.panel.word_buffer,
            &words_state.panel.meaning_buffer,
            model,
        ))
        .width(iced::Length::FillPortion((right_ratio * 10.0) as u16))
        .height(iced::Length::Fill)
        .style(card);

        Row::new()
            .push(left_panel)
            .push(right_panel)
            .spacing(Spacing::DEFAULT.xs2)
            .height(iced::Length::Fill)
            .into()
    }
}
