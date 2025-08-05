use iced::{
    Element,
    Length::{self},
    Task,
    widget::{
        Button, button, center, column, container, row, scrollable, text, text_editor, text_input,
    },
};

use flypad::{airport::Airport, weather::Weather};

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view).run()
}

#[derive(Debug, Clone)]
enum Event {
    FetchSimbrief,
    RefreshWeather,
    EditDepartureIcao(String),
    EditDepartureWeather(Option<Weather>),
    EditArrivalIcao(String),
    EditArrivalWeather(Option<Weather>),
    EditDepartureNotes(text_editor::Action),
    EditArrivalNotes(text_editor::Action),
}

struct App {
    departure_airport: Airport,
    arrival_airport: Airport,
    departure_notes: text_editor::Content,
    arrival_notes: text_editor::Content,
}

impl App {
    pub fn new() -> (Self, Task<Event>) {
        (
            Self {
                departure_airport: Airport::default(),
                arrival_airport: Airport::default(),
                departure_notes: text_editor::Content::new(),
                arrival_notes: text_editor::Content::new(),
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
        );

        let arrival_column = Self::create_column(
            weather_button,
            &self.arrival_airport,
            Event::EditArrivalIcao,
            &self.arrival_notes,
            Event::EditArrivalNotes,
        );

        row![departure_column, arrival_column].spacing(20).into()
    }

    fn create_column<'a>(
        btn: Button<'a, Event>,
        airport: &'a Airport,
        icao_action: impl Fn(String) -> Event + 'a,
        editor_content: &'a text_editor::Content,
        editor_action: impl Fn(text_editor::Action) -> Event + 'a,
    ) -> Element<'a, Event> {
        let icao_row = row![
            text("ICAO"),
            text_input("ICAO", &airport.icao).on_input(icao_action)
        ]
        .spacing(15);

        let wind_row = row![
            container(text("Wind")).width(Length::FillPortion(2)),
            container(text(format!("{} °", &airport.weather.wind_direction)))
                .width(Length::FillPortion(1)),
            container(text(format!("{} kts", &airport.weather.wind_speed)))
                .width(Length::FillPortion(1))
        ];

        let temperature_row = row![
            container(text(format!(
                "Temperature: {}",
                &airport.weather.temperature
            )))
            .width(Length::FillPortion(1)),
            container(text(format!("Dew Point: {}", &airport.weather.dew_point)))
                .width(Length::FillPortion(1)),
        ];
        let airport_data = column![icao_row, wind_row, temperature_row].spacing(15);

        column![
            center(btn),
            airport_data,
            scrollable(
                text_editor(&editor_content)
                    .height(125)
                    .on_action(editor_action)
                    .wrapping(text::Wrapping::WordOrGlyph)
            )
        ]
        .width(Length::FillPortion(1))
        .into()
    }

    async fn refresh_airport_weather(icao: String) -> Option<Weather> {
        Weather::fetch(&icao, true).await.ok()
    }
}
