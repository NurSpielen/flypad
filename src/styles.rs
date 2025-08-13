use iced::{
    Element, Length,
    widget::{Container, Row, container, row, text},
};

use crate::app::Event;

pub fn label_container<'a>(input_text: impl Into<String>) -> Container<'a, Event> {
    container(text(input_text.into())).width(Length::FillPortion(1))
}

pub fn value_row(children: Element<'_, Event>) -> Row<'_, Event> {
    row![children].width(Length::FillPortion(3))
}

pub fn bordered_text_container<'a>(input_text: impl Into<String>) -> Container<'a, Event> {
    container(text(input_text.into()))
        .width(Length::FillPortion(1))
        .style(container::bordered_box)
        .padding(5)
}
