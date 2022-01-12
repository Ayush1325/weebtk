import QtQuick 2.6
import QtQuick.Layouts 1.2
import QtQuick.Controls 2.2 as Controls
import org.kde.kirigami 2.13 as Kirigami
import VNDB 1.0

Kirigami.ScrollablePage {
    title: i18nc("@title", "VNDB Login")

    Kirigami.FormLayout {
        id: layout
        Layout.fillWidth: true

        Controls.TextField {
            id: username
            Kirigami.FormData.label: "Username:"
        }
        Controls.TextField {
            id: password
            echoMode: TextInput.Password
            Kirigami.FormData.label: "Password:"
        }
        Controls.Button {
            Layout.fillWidth: true
            text: i18n("Submit")
            onClicked: {
                VNDB.login(username.text, password.text)
            }
        }
    }
}
