? um zu offenen Punkten zu navigieren.

# 1. Zielbestimmung
Bei dem Produkt handelt es sich um ein elektronisches Spiel, welches auf dem
Brettspiel Quoridor basiert. Das Spiel soll bis zu vier Spielern erlauben lokal
auf demselben Gerät zu spielen. Ein Online Multiplayer ist nicht vorgesehen.
Ein Vorteil gegenüber einem herkömmlichen Brettspiel ist die Verfügbarkeit, da
es downloadbar für den PC überall zur Verfügung steht.
# 2. Einsatz 
## Zielgruppen:
Spieler alt
Spieler jung
Admin / Developer gehört der hier hin ??? 
## Einsatzbereiche:
Das Spiel soll hauptsächlich für den freizeitlichen Gebrauch entworfen werden.
Es soll kompetitiv gestaltet sein, sodass Spieler ihre Fähigkeiten verglichen
können, sowie den Spieler eine angenehme Möglichkeit bieten sich die Zeit zu
vertreiben. Es soll die Konzentration und strategisches Denken verbessern.
## Szenarien: ????
# 3. Umgebung
Das Spiel wird für Desktop-PCs und Laptops unter den gängigen Betriebssystemen
Windows und MacOS entwickelt. Es handelt sich dabei um eine eigenständige
Anwendung. (Bibliotheken müssen gegebenenfalls mit installiert werden???)
# 4. Funktionalität
User-Story-Map
# 5. Daten
(muss ein Impressum oder ähnliches erstellt werden, wenn das Produkt nur lokal genutzt wird
und der Download über Github möglich ist?)

Daten, die Verarbeitet werden sind ein vom Nutzer freiwillig erstellbares
Spielerprofil, sowie die Informationen über das aktuelle Spiel. Ein
Spielerprofil enthält dabei nur einen vom Nutzer ausgedachten Nutzernamen und
Informationen zu mit diesem Profil gespielten Spielen, jedoch keine
personenbezogenen Daten wie E-Mail, Vorname, Nachname oder Alter. Die
Informationen sind nicht gesondert durch ein Passwort oder ähnliches geschützt
und die Spieldaten werden auch nicht verschlüsselt gespeichert. Die Speicherung
der Daten wird mit einer lokalen SQLite Datenbank in einer Datei auf der
Festplatte des Computers umgesetzt.

ER-Modell ???
# 6. Leistungen
Um eine angenehme Nutzererfahrung zu bieten werden folgende
Leistungsanforderungen aufgestellt:
- Skalierbarkeit: Die Spieleranzahl soll nicht über 4 Erhöht werden können. Es
  soll
  möglich sein in Zukunft die Spielfeldgröße sowie die Zugmöglichkeiten zu
  erweitern, ohne dass andere Leistungsanforderungen nicht mehr eingehalten
  werden. Die maximale Anzahl an Spielerprofilen soll dabei mindestens 100
  betragen, wobei der benötigte Speicherplatz nicht (Wert X ???) überschritten
  werden darf.
- Durchsatz: was genau ist durchsatz???? 
- Antwortzeiten: Die Latenz, wenn eine Aktion durch den Nutzer ausgeführt
  wurde, darf maximal eine halbe Sekunde betragen. Dies vom gilt vom Klick des
  Nutzers bis zum Anzeigen auf dem Bildschirm, wobei die Latenz des
  Bildschirms, der Maus und der Tastatur nicht beachtet werden. 
  (müssen wir das abhängig von System Voraussetzungen festlegen?)
- Ladezeiten: Die Initiale Ladezeit des Programms bis zum Anzeigen auf dem
  Bildschirm darf maximal eine Sekunde dauern, wobei die Latenz des
  Bildschirms, der Maus und der Tastatur nicht beachtet werden.
  (müssen wir das abhängig von System Voraussetzungen festlegen?)
# 7. Benutzeroberfläche
Das System sollte den Spielern jederzeit den aktuellen Status des Spiels
mitteilen, dies betrifft den Status des Spielfelds und die Position der Figuren
als auch bei längeren Wartezeiten den Spieler über diese zu informieren.

Der Spieler sollte das Spiel jederzeit speichern und verlassen können, um später an diesem Punkt weiter zuspielen.

Es soll eine einheitliche Auswahl an Farben und Symbolen genutzt werden, um dem
Spieler Informationen zu vermitteln. Das User-Interface soll fehlerhafte
Aktionen verhindern, oder den Spieler auf diese hinweisen.
Das Spiel soll für Einsteiger und erfahrene Nutzer gleichermaßen intuitiv
nutzbar sein. Um dies um zusetzen soll die Benutzeroberfläche minimalistisch
und frei von unnötigen Elementen sein.
Um es Anfängern einfacher zu machen, sollte das Spiel ein simples Tutorial
enthalten.
# 8. Qualitätsziele
- Wartbarkeit: Die Wartbarkeit und Erweiterbarkeit der Software soll einfach
möglich sein.
- Zuverlässigkeit: Das Spiel soll Zuverlässig und ohne Fehler laufen.
- Sicherheit: Was ist Sicherheit in unserem Kontext???
Um die Einhaltung der Qualitätsziele sicherzustellen, wird die Software mithilfe von Unit-Test hinreichend getestet.
# 9. Entwicklungsumgebung
- Softwaretools: Für das Schreiben des Codes wird Neovim und Visual Studio Code
  genutzt. Für die Versionierung Git, Github und Github Desktop.
- Programmierrichtlinien: Für die Qualität des Codes ist es essenziell, dass
  sich an gemeinsame Code-Standards gehalten wird. Für die Naming-Conventions
  werden die Standard Rust-Naming-Conventions für Rust und die camelCase
  Naming-Conventions für Svelte genutzt.
  (Was gehört sonst noch hier hin???)
- Versionskontrolle: Die Versionierung wird mit Git und Github umgesetzt.
  Dabei wird jedes Feature auf einem eigenen Branch entwickelt, welcher nach
  dem Merge mit dem Main-Branch gelöscht wieder gelöscht wird. Der Merge wird
  mit Github Pull Requests durchgeführt wobei vor dem Merge ein anderer
  Entwickler den Code geprüft haben muss.
# 10. Erweiterungen
Hier werden die Regel definiert. ??
