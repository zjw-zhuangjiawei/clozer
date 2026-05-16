//! Search bar with query input, sort picker, and clear button.

use crate::i18n::I18nManager;
use crate::query::SortType;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::layout::breakpoint::Breakpoint;
use crate::ui::theme::{ButtonSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::Element;
use iced::widget::{Button, PickList, Row, Text};

pub fn build_search_bar<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    breakpoint: Breakpoint,
    i18n: &'a I18nManager,
) -> Element<'a, WordsMessage, AppTheme> {
    let query = &words_state.search.query;
    let suggestion = if !query.is_empty() {
        words_state.search.get_suggestion(&model.word_registry)
    } else {
        None
    };

    let mut search_input =
        crate::ui::widgets::advanced_input::AdvancedInput::new(i18n.tr("words-search-placeholder"))
            .value(query)
            .on_input(WordsMessage::SearchQueryChanged)
            .on_submit(WordsMessage::SuggestionAccepted)
            .width(iced::Length::Fill)
            .padding(Spacing::DEFAULT.s);

    if let Some(sug) = suggestion {
        search_input = search_input.ghost_text(sug);
    }

    let search_with_ghost: Element<'a, WordsMessage, AppTheme> = Element::new(search_input);

    let sort_width = if breakpoint.is_single_column() {
        iced::Length::Fixed(90.0)
    } else {
        iced::Length::Fixed(110.0)
    };
    let sort_picker = PickList::new(
        SortType::variants(),
        Some(words_state.search.sort),
        WordsMessage::SortTypeChanged,
    )
    .width(sort_width)
    .placeholder(i18n.tr("words-sort"));

    let mut row = Row::new()
        .push(search_with_ghost)
        .push(sort_picker)
        .spacing(Spacing::DEFAULT.s2)
        .align_y(iced::Alignment::Center);

    if words_state.search.has_active_filters() {
        row = row.push(
            Button::new(Text::new(i18n.tr("words-clear-filters")))
                .style(button::secondary)
                .padding(ButtonSize::Standard.to_iced_padding())
                .on_press(WordsMessage::FiltersCleared),
        );
    }

    row.into()
}
