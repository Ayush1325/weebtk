use ki18n::klocalizedcontext::KLocalizedContext;
use qmetaobject::prelude::*;
use qmetaobject::QUrl;

qrc!(root_qml,
    "qml" as "" {
        "main.qml",
    }
);

qrc!(vndb_qml,
    "qml/vndb" as "vndb" {
        "login.qml",
    }
);

fn main() {
    qmetaobject::log::init_qt_to_rust();
    env_logger::init();

    let mut engine = QmlEngine::new();

    KLocalizedContext::init_from_engine(&engine);

    root_qml();
    vndb_qml();

    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));
    engine.exec();
}
