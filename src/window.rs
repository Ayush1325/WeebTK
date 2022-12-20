use crate::APP_ID;
use adw::subclass::prelude::*;
use gio::Settings;
use gtk::prelude::*;
use gtk::{gio, glib};

mod imp {
    use super::*;
    use gtk::Inhibit;
    use once_cell::sync::OnceCell;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/weebtk/window.ui")]
    pub struct WeebtkWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<gtk::HeaderBar>,
        #[template_child]
        pub main_stack: TemplateChild<gtk::Stack>,
        pub settings: OnceCell<Settings>,
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

    impl ObjectImpl for WeebtkWindow {
        fn constructed(&self) {
            self.parent_constructed();
            // Load latest window state
            let obj = self.obj();
            obj.setup_settings();
            obj.load_window_size();
            let mal_secret = obj.settings().string("mal-secret");
            if mal_secret.is_empty() {
                self.main_stack.set_visible_child_name("login");
            } else {
                self.main_stack.set_visible_child_name("mylist");
            }
        }
    }
    impl WidgetImpl for WeebtkWindow {}
    impl WindowImpl for WeebtkWindow {
        // Save window state right before the window will be closed
        fn close_request(&self) -> Inhibit {
            // Save window size
            self.obj()
                .save_window_size()
                .expect("Failed to save window state");

            // Don't inhibit the default handler
            Inhibit(false)
        }
    }
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

    fn setup_settings(&self) {
        let settings = Settings::new(APP_ID);
        self.imp()
            .settings
            .set(settings)
            .expect("`settings` should not be set before calling `setup_settings`.");
    }

    fn settings(&self) -> &Settings {
        self.imp()
            .settings
            .get()
            .expect("`settings` should be set in `setup_settings`.")
    }

    pub fn save_window_size(&self) -> Result<(), glib::BoolError> {
        // Get the size of the window
        let size = self.default_size();

        // Set the window state in `settings`
        self.settings().set_int("window-width", size.0)?;
        self.settings().set_int("window-height", size.1)?;
        self.settings()
            .set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    fn load_window_size(&self) {
        // Get the window state from `settings`
        let width = self.settings().int("window-width");
        let height = self.settings().int("window-height");
        let is_maximized = self.settings().boolean("is-maximized");

        // Set the size of the window
        self.set_default_size(width, height);

        // If the window was maximized when it was closed, maximize it again
        if is_maximized {
            self.maximize();
        }
    }
}

