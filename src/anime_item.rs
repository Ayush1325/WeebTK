use adw::subclass::prelude::*;
use gtk::glib;
use gtk::prelude::*;

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/org/gnome/weebtk/anime/anime-item.ui")]
    pub struct AnimeItem {}

    #[glib::object_subclass]
    impl ObjectSubclass for AnimeItem {
        const NAME: &'static str = "AnimeItem";
        type Type = super::AnimeItem;
        type ParentType = gtk::Box;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl WidgetImpl for AnimeItem {}
    impl ObjectImpl for AnimeItem {}
    impl BoxImpl for AnimeItem {}
}

glib::wrapper! {
    pub struct AnimeItem(ObjectSubclass<imp::AnimeItem>)
        @extends gtk::Widget, gtk::Box;
}

impl AnimeItem {}

