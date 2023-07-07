use gst::prelude::*;

use gtk::prelude::*;
use gtk::{gdk, gio, glib};

fn create_ui(app: &gtk::Application) {
    let builder = gtk::Builder::from_file("window.ui");
    let window = builder.object::<gtk::Window>("window").unwrap();

    let pipeline_thumbnail = gst::parse_launch(
        "videotestsrc pattern=ball ! video/x-raw,width=1000,height=1000 ! gtk4paintablesink name=paintable",
    )
    .unwrap()
    .downcast::<gst::Pipeline>()
    .unwrap();
    let paintable_sink_thumbnail = pipeline_thumbnail.by_name("paintable").unwrap();
    let paintable_thumbnail = paintable_sink_thumbnail.property::<gdk::Paintable>("paintable");

    let pipeline_main = gst::parse_launch(
        "videotestsrc ! video/x-raw,width=1000,height=1000 ! gtk4paintablesink name=paintable",
    )
    .unwrap()
    .downcast::<gst::Pipeline>()
    .unwrap();
    let paintable_sink_main = pipeline_main.by_name("paintable").unwrap();
    let paintable_main = paintable_sink_main.property::<gdk::Paintable>("paintable");

    window.set_application(Some(app));
    let css = gtk::CssProvider::default();
    css.load_from_path("window.css");
    gtk::style_context_add_provider_for_display(
        &gdk::Display::default().unwrap(),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let stack: gtk::Stack = builder.object("stack").unwrap();

    let thumbnail: gtk::Picture = builder.object("thumbnail").unwrap();
    thumbnail.set_paintable(Some(&paintable_thumbnail));

    let main: gtk::Picture = builder.object("main").unwrap();
    main.set_paintable(Some(&paintable_main));

    let placeholder_stack = stack.downgrade();
    let show_placeholder: gtk::Button = builder.object("show-placeholder").unwrap();
    show_placeholder.connect_clicked(move |_| {
        placeholder_stack
            .upgrade()
            .unwrap()
            .set_visible_child_name("placeholder")
    });

    let thumbnail_stack = stack.downgrade();
    let show_thumbnail: gtk::Button = builder.object("show-thumbnail").unwrap();
    show_thumbnail.connect_clicked(move |_| {
        thumbnail_stack
            .upgrade()
            .unwrap()
            .set_visible_child_name("thumbnail")
    });

    let start_pipelines: gtk::Button = builder.object("start-pipelines").unwrap();

    start_pipelines.connect_clicked(move |_| {
        pipeline_thumbnail.set_state(gst::State::Playing).unwrap();
        pipeline_main.set_state(gst::State::Playing).unwrap();
    });

    window.present();
}

fn main() -> glib::ExitCode {
    gst::init().unwrap();

    gstgtk4::plugin_register_static().expect("Failed to register gstgtk4 plugin");

    let app = gtk::Application::new(None::<&str>, gio::ApplicationFlags::FLAGS_NONE);

    app.connect_activate(create_ui);
    dbg!(app.run())
}
