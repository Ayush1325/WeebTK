use adw::subclass::prelude::*;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/weebtk/window.ui")]
    pub struct WeebtkWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub label: TemplateChild<gtk::Label>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for WeebtkWindow {
        const NAME: &'static str = "WeebtkWindow";
        type Type = super::WeebtkWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for WeebtkWindow {}
    impl WidgetImpl for WeebtkWindow {}
    impl WindowImpl for WeebtkWindow {}
    impl ApplicationWindowImpl for WeebtkWindow {}
    impl AdwApplicationWindowImpl for WeebtkWindow {}
}

glib::wrapper! {
    pub struct WeebtkWindow(ObjectSubclass<imp::WeebtkWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl WeebtkWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
    }
}
