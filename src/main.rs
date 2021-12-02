use cstr::cstr;
use ki18n_rs::KLocalizedContext;
use qmetaobject::prelude::*;
use qmetaobject::QUrl;

// The `QObject` custom derive macro allows to expose a class to Qt and QML
#[derive(QObject, Default)]
struct Greeter {
    // Specify the base class with the qt_base_class macro
    base: qt_base_class!(trait QObject),
    // Declare `name` as a property usable from Qt
    name: qt_property!(QString; NOTIFY name_changed),
    // Declare a signal
    name_changed: qt_signal!(),
    // And even a slot
    compute_greetings: qt_method!(
        fn compute_greetings(&self, verb: String) -> QString {
            format!("{} {}", verb, self.name.to_string()).into()
        }
    ),
}

qrc!(root_qml,
    "" {
        "qml/main.qml" as "main.qml",
    }
);

fn main() {
    qmetaobject::log::init_qt_to_rust();
    env_logger::init();

    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));
    let mut engine = QmlEngine::new();

    KLocalizedContext::init_from_engine(&engine);

    root_qml();
    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));
    engine.exec();
}
