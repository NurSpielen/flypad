use iced::{
    Element, Length, Task,
    widget::{
        Button, Column, button, center, column, container, row, text, text_editor, text_input,
    },
};

use crate::{airport::Airport, styles, weather::Weather};

#[derive(Debug, Clone)]
pub enum Event {
    FetchSimbrief,
    RefreshWeather,
    EditDepartureIcao(String),
    EditDepartureWeather(Option<Weather>),
    EditArrivalIcao(String),
    EditArrivalWeather(Option<Weather>),
    EditDepartureNotes(text_editor::Action),
    EditArrivalNotes(text_editor::Action),
    DepartureMetarAction(text_editor::Action),
    ArrivalMetarAction(text_editor::Action),
}

pub struct App {
    departure_airport: Airport,
    arrival_airport: Airport,
    departure_notes: text_editor::Content,
    arrival_notes: text_editor::Content,
    departure_metar: text_editor::Content,
    arrival_metar: text_editor::Content,
}

impl App {
    pub fn new() -> (Self, Task<Event>) {
        (
            Self {
                departure_airport: Airport::default(),
                arrival_airport: Airport::default(),
                departure_notes: text_editor::Content::new(),
                arrival_notes: text_editor::Content::new(),
                departure_metar: text_editor::Content::new(),
                arrival_metar: text_editor::Content::new(),
            },
            Task::none(),
        )
    }

    pub fn update(&mut self, event: Event) -> Task<Event> {
        match event {
            Event::FetchSimbrief => {
                // Fetch data from simbrief here
                Task::none()
            }
            Event::RefreshWeather => Task::batch([
                Task::perform(
                    Self::refresh_airport_weather(self.departure_airport.icao.clone()),
                    Event::EditDepartureWeather,
                ),
                Task::perform(
                    Self::refresh_airport_weather(self.arrival_airport.icao.clone()),
                    Event::EditArrivalWeather,
                ),
            ]),
            Event::EditDepartureIcao(icao) => {
                self.departure_airport.icao = icao;
                Task::none()
            }
            Event::EditDepartureWeather(weather) => {
                if let Some(weather) = weather {
                    self.departure_metar = text_editor::Content::with_text(&weather.metar);
                    self.departure_airport.weather = weather;
                }
                Task::none()
            }
            Event::EditArrivalIcao(icao) => {
                self.arrival_airport.icao = icao;
                Task::none()
            }
            Event::EditArrivalWeather(weather) => {
                if let Some(weather) = weather {
                    self.arrival_metar = text_editor::Content::with_text(&weather.metar);
                    self.arrival_airport.weather = weather;
                }
                Task::none()
            }
            Event::EditDepartureNotes(action) => {
                self.departure_notes.perform(action);
                Task::none()
            }
            Event::EditArrivalNotes(action) => {
                self.arrival_notes.perform(action);
                Task::none()
            }
            Event::DepartureMetarAction(action) => {
                if let text_editor::Action::Edit(_) = action {
                    return Task::none();
                }
                self.departure_metar.perform(action);
                Task::none()
            }
            Event::ArrivalMetarAction(action) => {
                if let text_editor::Action::Edit(_) = action {
                    return Task::none();
                }
                self.arrival_metar.perform(action);
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Event> {
        let simbrief_button = button("Fetch Simbrief")
            .on_press(Event::FetchSimbrief)
            .padding(10);
        let weather_button = button("Refresh Weather")
            .on_press(Event::RefreshWeather)
            .padding(10);

        let departure_column = Self::create_column(
            simbrief_button,
            &self.departure_airport,
            Event::EditDepartureIcao,
            &self.departure_notes,
            Event::EditDepartureNotes,
            &self.departure_metar,
            Event::DepartureMetarAction,
        );

        let arrival_column = Self::create_column(
            weather_button,
            &self.arrival_airport,
            Event::EditArrivalIcao,
            &self.arrival_notes,
            Event::EditArrivalNotes,
            &self.arrival_metar,
            Event::ArrivalMetarAction,
        );

        row![departure_column, arrival_column].spacing(20).into()
    }

    fn create_column<'a>(
        btn: Button<'a, Event>,
        airport: &'a Airport,
        icao_action: impl Fn(String) -> Event + 'a,
        editor_content: &'a text_editor::Content,
        editor_action: impl Fn(text_editor::Action) -> Event + 'a,
        metar_content: &'a text_editor::Content,
        metar_action: impl Fn(text_editor::Action) -> Event + 'a,
    ) -> Column<'a, Event> {
        let icao_row = row![
            styles::label_container("ICAO"),
            styles::value_row(
                container(text_input("ICAO", &airport.icao).on_input(icao_action)).into()
            )
        ];

        let wind_row = row![
            styles::label_container("Wind"),
            styles::value_row(
                row![
                    styles::bordered_text_container(format!("{}", &airport.weather.wind_direction),),
                    styles::label_container("  °"),
                    styles::bordered_text_container(format!("{}", &airport.weather.wind_speed),),
                    styles::label_container("  kts")
                ]
                .into()
            )
        ];

        let temperature_row = row![
            styles::label_container("Temperature"),
            styles::value_row(
                row![
                    styles::bordered_text_container(format!("{}", &airport.weather.temperature),),
                    styles::label_container(" Dew Point"),
                    styles::bordered_text_container(format!("{}", &airport.weather.dew_point),),
                ]
                .into()
            )
        ];

        let qnh_row = row![
            styles::label_container("QNH"),
            styles::value_row(
                styles::bordered_text_container(format!("{}", &airport.weather.altimeter)).into()
            )
        ];

        let visibility_row = row![
            styles::label_container("Visibility"),
            styles::value_row(
                styles::bordered_text_container(format!("{}", &airport.weather.visibility)).into()
            )
        ];

        let metar_column = column![
            container(text("Metar")).padding(5),
            container(
                text_editor(&metar_content)
                    .on_action(metar_action)
                    .height(80)
                    .wrapping(text::Wrapping::WordOrGlyph)
            )
            .style(container::bordered_box)
        ];

        let atc_notes = column![
            container(text("ATC Notes")).padding(5),
            container(
                text_editor(&editor_content)
                    .height(125)
                    .on_action(editor_action)
                    .wrapping(text::Wrapping::WordOrGlyph),
            )
            .style(container::bordered_box)
        ];

        let information_container = container(
            column![
                icao_row,
                wind_row,
                temperature_row,
                qnh_row,
                visibility_row,
                metar_column,
                atc_notes
            ]
            .spacing(5)
            .padding(10),
        )
        .style(container::bordered_box);

        column![center(btn), information_container].width(Length::FillPortion(1))
    }

    async fn refresh_airport_weather(icao: String) -> Option<Weather> {
        Weather::fetch(&icao, true).await.ok()
    }
}
