use crate::assets;
use crate::models::types::MeaningId;
use crate::state::Model;
use crate::ui::AppTheme;
use crate::ui::theme::{ButtonSize, FontSize, Spacing};
use crate::ui::widgets::button;
use crate::ui::widgets::svg_checkbox;
use crate::ui::widgets::text as txt;
use crate::ui::words::message::WordsMessage;
use crate::ui::words::state::WordsState;
use iced::Element;
use iced::widget::{Button, Column, Row, Space, Text, svg};

use super::tags::build_tags_row;

pub fn build_meaning_node<'a>(
    words_state: &'a WordsState,
    model: &'a Model,
    meaning: &'a crate::models::Meaning,
) -> Element<'a, WordsMessage, AppTheme> {
    let is_selected = words_state.selection.is_meaning_selected(meaning.id);
    let cloze_count = model.cloze_registry.iter_by_meaning_id(meaning.id).count();

    let checkbox = svg_checkbox(is_selected, WordsMessage::MeaningToggled(meaning.id));

    let definition: Element<'a, WordsMessage, AppTheme> =
        Button::new(Text::new(&meaning.definition).size(FontSize::Body.px()))
            .style(button::tertiary)
            .padding(ButtonSize::Small.to_iced_padding())
            .on_press(WordsMessage::MeaningSelected(meaning.id))
            .into();

    let cloze_status_text = if cloze_count > 0 {
        format!("{} clozes", cloze_count)
    } else {
        "no clozes".to_string()
    };
    let cloze_status = Text::new(cloze_status_text)
        .size(FontSize::Caption.px())
        .style(txt::tertiary);

    let meaning_header = Row::new()
        .push(checkbox)
        .push(definition)
        .push(Space::new())
        .push(cloze_status)
        .push(build_meaning_actions(meaning.id))
        .spacing(Spacing::DEFAULT.xs)
        .align_y(iced::Alignment::Center);

    let tags_row = build_tags_row(words_state, model, meaning);

    let cloze_preview_items: Vec<Element<'a, WordsMessage, AppTheme>> = model
        .cloze_registry
        .iter_by_meaning_id(meaning.id)
        .take(2)
        .map(|(cloze_id, cloze)| {
            let text = cloze.render_blanks();
            Button::new(Text::new(text).size(FontSize::Caption.px()))
                .style(button::tertiary)
                .padding(ButtonSize::Small.to_iced_padding())
                .on_press(WordsMessage::ClozeSelected(*cloze_id))
                .into()
        })
        .collect();

    let mut column = Column::new()
        .push(meaning_header)
        .push(tags_row)
        .spacing(Spacing::DEFAULT.xs2)
        .padding([Spacing::DEFAULT.xs2, Spacing::DEFAULT.s2]);

    if !cloze_preview_items.is_empty() {
        column = column.push(
            Column::with_children(cloze_preview_items)
                .spacing(Spacing::DEFAULT.xxs)
                .padding([Spacing::DEFAULT.xxs, Spacing::DEFAULT.s2]),
        );
    }

    column.into()
}

pub fn build_meaning_actions<'a>(meaning_id: MeaningId) -> Element<'a, WordsMessage, AppTheme> {
    let delete_icon_handle = assets::get_svg("delete_24dp_000000_FILL0_wght400_GRAD0_opsz24.svg")
        .map(svg::Handle::from_memory)
        .unwrap_or_else(|| svg::Handle::from_memory(Vec::new()));
    let delete_icon = svg(delete_icon_handle)
        .width(iced::Length::Fixed(16.0))
        .height(iced::Length::Fixed(16.0));

    Button::new(delete_icon)
        .style(button::danger)
        .padding(ButtonSize::Small.to_iced_padding())
        .on_press(WordsMessage::MeaningDeleted(meaning_id))
        .into()
}
