use flypad::app::App;

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view).run()
}
