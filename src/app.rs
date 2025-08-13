use iced::{
    Element, Length, Task,
    widget::{
        Button, Column, Container, button, center_x, column, container, row, text, text_editor,
        text_input,
    },
};
use tokio::{
    fs::OpenOptions,
    io::{AsyncReadExt, AsyncWriteExt},
};

use crate::{
    airport::Airport,
    flightplan::{FlightPlan, User},
    styles,
    weather::Weather,
};

const USER_SAVE_PATH: &str = "user.json";

#[derive(Debug, Clone)]
pub enum UserEvent {
    LoadUserId,
    UserIdLoaded(Option<String>),
    SetUserId(String),
    SaveUserId,
    UserIdSaved(Result<(), String>),
}

#[derive(Debug, Clone)]
pub enum Event {
    UserEvent(UserEvent),
    FetchSimbrief,
    FlightPlanFetched(Box<Option<FlightPlan>>),
    RefreshWeather,
    EditDepartureIcao(String),
    EditDepartureWeather(Option<Weather>),
    EditArrivalIcao(String),
    EditArrivalWeather(Option<Weather>),
    EditDepartureNotes(text_editor::Action),
    EditArrivalNotes(text_editor::Action),
    DepartureMetarAction(text_editor::Action),
    ArrivalMetarAction(text_editor::Action),
    RouteAction(text_editor::Action),
}

pub struct App {
    user_id: String,
    departure_airport: Airport,
    arrival_airport: Airport,
    departure_notes: text_editor::Content,
    arrival_notes: text_editor::Content,
    departure_metar: text_editor::Content,
    arrival_metar: text_editor::Content,
    flightplan: Option<FlightPlan>,
    route: text_editor::Content,
}

impl App {
    pub fn new() -> (Self, Task<Event>) {
        (
            Self {
                user_id: String::new(),
                departure_airport: Airport::default(),
                arrival_airport: Airport::default(),
                departure_notes: text_editor::Content::new(),
                arrival_notes: text_editor::Content::new(),
                departure_metar: text_editor::Content::new(),
                arrival_metar: text_editor::Content::new(),
                flightplan: None,
                route: text_editor::Content::new(),
            },
            Task::done(Event::UserEvent(UserEvent::LoadUserId)),
        )
    }

    pub fn update(&mut self, event: Event) -> Task<Event> {
        match event {
            Event::UserEvent(event) => self.perform_user_event(event),
            Event::FetchSimbrief => Task::perform(
                Self::refresh_simbrief_flightplan(self.user_id.clone()),
                Event::FlightPlanFetched,
            ),
            Event::FlightPlanFetched(option) => {
                self.flightplan = *option;
                if let Some(flightplan) = &self.flightplan {
                    let departure_icao = flightplan.origin.icao_code.clone();
                    let arrival_icao = flightplan.destination.icao_code.clone();
                    self.route = text_editor::Content::with_text(
                        &flightplan.flight_information.route_navigraph,
                    );
                    Task::batch([
                        Task::done(Event::EditDepartureIcao(departure_icao)),
                        Task::done(Event::EditArrivalIcao(arrival_icao)),
                    ])
                } else {
                    println!("No flight plan fetched");
                    Task::none()
                }
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
                Self::set_icao(&mut self.departure_airport, icao);
                Task::none()
            }
            Event::EditDepartureWeather(weather) => {
                if let Some(weather) = weather {
                    Self::set_current_weather(
                        &mut self.departure_airport,
                        &mut self.departure_metar,
                        weather,
                    );
                }
                Task::none()
            }
            Event::EditArrivalIcao(icao) => {
                Self::set_icao(&mut self.arrival_airport, icao);
                Task::none()
            }
            Event::EditArrivalWeather(weather) => {
                if let Some(weather) = weather {
                    Self::set_current_weather(
                        &mut self.arrival_airport,
                        &mut self.arrival_metar,
                        weather,
                    );
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
                if !matches!(action, text_editor::Action::Edit(_)) {
                    self.departure_metar.perform(action);
                }
                Task::none()
            }
            Event::ArrivalMetarAction(action) => {
                if !matches!(action, text_editor::Action::Edit(_)) {
                    self.arrival_metar.perform(action);
                }
                Task::none()
            }
            Event::RouteAction(action) => {
                if !matches!(action, text_editor::Action::Edit(_)) {
                    self.route.perform(action);
                }
                Task::none()
            }
        }
    }

    fn set_icao(airport: &mut Airport, icao: String) {
        airport.icao = icao;
    }

    fn set_current_weather(
        airport: &mut Airport,
        metar: &mut text_editor::Content,
        weather: Weather,
    ) {
        *metar = text_editor::Content::with_text(&weather.metar);
        airport.weather = weather;
    }

    fn perform_user_event(&mut self, event: UserEvent) -> Task<Event> {
        match event {
            UserEvent::LoadUserId => Task::perform(Self::load_user_id(USER_SAVE_PATH), |user_id| {
                Event::UserEvent(UserEvent::UserIdLoaded(user_id))
            }),
            UserEvent::UserIdLoaded(user_id) => match user_id {
                Some(user_id) => Task::done(Event::UserEvent(UserEvent::SetUserId(user_id))),
                None => Task::none(),
            },
            UserEvent::SetUserId(user_id) => {
                self.user_id = user_id;
                Task::none()
            }
            UserEvent::SaveUserId => Task::perform(
                Self::save_user_id(USER_SAVE_PATH, self.user_id.clone()),
                |result| Event::UserEvent(UserEvent::UserIdSaved(result)),
            ),
            UserEvent::UserIdSaved(result) => {
                match result {
                    Ok(_) => (),
                    Err(e) => eprintln!("{e}"),
                }

                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Event> {
        let user_id_input_field = center_x(
            container(
                text_input::TextInput::new("Simbrief", &self.user_id)
                    .on_input(|input| Event::UserEvent(UserEvent::SetUserId(input)))
                    .width(Length::Fixed(70.0)),
            )
            .style(container::bordered_box),
        );

        let simbrief_button = button("Fetch Simbrief").on_press(Event::FetchSimbrief);
        let weather_button = button("Refresh Weather").on_press(Event::RefreshWeather);

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

        let weather_and_notes_row = row![departure_column, arrival_column].spacing(20);

        let flight_plan_section = Self::populate_flight_plan_information();

        let route_section = Self::create_route_container(&self.route);

        column![
            user_id_input_field,
            weather_and_notes_row,
            flight_plan_section,
            route_section
        ]
        .spacing(10)
        .padding(10)
        .into()
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
                    styles::bordered_text_container(airport.weather.wind_direction().to_string()),
                    styles::label_container("  °"),
                    styles::bordered_text_container(airport.weather.wind_speed().to_string()),
                    styles::label_container("  kts")
                ]
                .into()
            )
        ];

        let temperature_row = row![
            styles::label_container("Temperature"),
            styles::value_row(
                row![
                    styles::bordered_text_container(airport.weather.temperature().to_string()),
                    styles::label_container(" Dew Point"),
                    styles::bordered_text_container(airport.weather.dew_point().to_string()),
                ]
                .into()
            )
        ];

        let qnh_row = row![
            styles::label_container("QNH"),
            styles::value_row(
                styles::bordered_text_container(airport.weather.altimeter().to_string()).into()
            )
        ];

        let visibility_row = row![
            styles::label_container("Visibility"),
            styles::value_row(
                styles::bordered_text_container(airport.weather.visibility().to_string()).into()
            )
        ];

        let metar_column = column![
            container(text("Metar")).padding(5),
            container(
                text_editor(metar_content)
                    .on_action(metar_action)
                    .height(80)
                    .wrapping(text::Wrapping::WordOrGlyph)
            )
            .style(container::bordered_box)
        ];

        let atc_notes = column![
            container(text("ATC Notes")).padding(5),
            container(
                text_editor(editor_content)
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

        column![center_x(btn), information_container]
            .width(Length::FillPortion(1))
            .spacing(10)
    }

    fn populate_flight_plan_information<'a>() -> Element<'a, Event> {
        container(column![]).into()
    }

    async fn load_user_id(path: &str) -> Option<String> {
        let mut file = OpenOptions::new().read(true).open(path).await.ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await.ok()?;
        let user: Option<User> = serde_json::from_str(&contents).ok();

        user.map(|User(user_id)| user_id)
    }

    // TODO: Modify return type with a more descriptive result
    async fn save_user_id(path: &str, user_id: String) -> Result<(), String> {
        let Ok(mut file) = OpenOptions::new()
            .read(true)
            .write(true)
            .truncate(true)
            .open(path)
            .await
        else {
            return Err("failed to open file when attempting to save".to_string());
        };

        let user = User(user_id);

        match serde_json::to_string_pretty(&user) {
            Ok(json) => {
                if let Err(e) = file.write_all(json.as_bytes()).await {
                    return Err(e.to_string());
                }
            }
            Err(_) => return Err("failed to serialize user".to_string()),
        }

        Ok(())
    }

    fn create_route_container(route_content: &text_editor::Content) -> Container<'_, Event> {
        container(column![
            container(text("Route")).padding(5),
            container(
                text_editor(route_content)
                    .height(125)
                    .on_action(Event::RouteAction)
                    .wrapping(text::Wrapping::WordOrGlyph),
            )
        ])
        .padding(10)
        .style(container::bordered_box)
    }

    async fn refresh_simbrief_flightplan(user_id: String) -> Box<Option<FlightPlan>> {
        Box::new(FlightPlan::fetch(&user_id).await.ok())
    }

    async fn refresh_airport_weather(icao: String) -> Option<Weather> {
        let weather = Weather::fetch(&icao, true).await.ok();
        if weather.is_none() {
            println!("An error occurred when deserializing the weather response");
        }

        weather
    }
}
