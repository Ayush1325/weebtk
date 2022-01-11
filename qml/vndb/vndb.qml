import QtQuick 2.6
import QtQuick.Layouts 1.2
import QtQuick.Controls 2.2 as Controls
import org.kde.kirigami 2.13 as Kirigami

import VisualNovelListModel 1.0

Kirigami.ScrollablePage {
    title: i18nc("@title", "VNDB")

    //    Controls.Button {
    //        text: i18n("Login")
    //        onClicked: {
    //            pageStack.push(Qt.resolvedUrl("qrc:///vndb/login.qml"))
    //        }
    //    }
    Kirigami.CardsGridView {
        id: view
        model: VisualNovelListModel {
            id: model
        }
        delegate: Kirigami.Card {
            id: card
            banner {
                title: model.name
                source: Qt.resolvedUrl(model.image)
            }
            contentItem: Controls.Label {
                wrapMode: Text.WordWrap
                text: model.description
            }
        }
    }
}
