? um zu offenen Punkten zu navigieren.

# 1. Zielbestimmung
Bei dem Produkt handelt es sich um ein elektronisches Spiel, welches auf dem
Brettspiel Quoridor basiert. Das Spiel soll bis zu vier Spielern erlauben lokal
auf demselben Gerät zu spielen. Ein Online Multiplayer ist nicht vorgesehen.
Ein Vorteil gegenüber einem herkömmlichen Brettspiel ist die Verfügbarkeit, da
es downloadbar für den PC überall zur Verfügung steht.
# 2. Einsatz 
## Zielgruppen:
Brettspielspieler jeglicher Altersgruppen mit Affinität zu digitalen Spielformat
Admin / Developer gehört der hier hin ??? 
## Einsatzbereiche:
Das Spiel soll für den freizeitlichen Gebrauch entworfen werden.
Es soll kompetitiv gestaltet sein, sodass Spieler ihre Fähigkeiten vergleichen
können, sowie den Spieler eine angenehme Möglichkeit bieten sich die Zeit zu
vertreiben. Es soll die Konzentration und strategisches Denken verbessern.
## Szenarien:
Das Produkt könnte beispielsweise durch eine Familie genutzt werden, die zuhause oder auf einer Reise zusammen etwas spielen möchte. Ein weiteres Szenario ist ein Spiel von Studenten in der Hochschule. Auch Zuhause bei Spieleabenden kann das Spiel zum Einsatz kommen, da dann nicht die Brettspielversion gekauft werden muss und digitale Hilfen bereit gestellt werden.
# 3. Umgebung
Das Spiel wird für Desktop-PCs und Laptops unter den gängigen Betriebssystemen
Windows und MacOS entwickelt. Es handelt sich dabei um eine eigenständige
Anwendung. (Bibliotheken müssen gegebenenfalls mit installiert werden???)
# 4. Funktionalität
- Angabe Benutzername, um die Anzahl der Siege anzuzeigen
- Anzeige der Spielregeln in einer kurzen verständlichen Form
- Anzeige des Spielbretts mit unterschiedlich gefärbten Spielfiguren und Wänden, Anzeige der Startseite jeder Spielfigur
- Anzeige der Anzahl an noch verfügbaren Wände für einen Spieler
- Anzeige des Spielers, der am Spielzug ist
- Möglichkeit in einem Spielzug eine Wand mittels Mausklick zu setzen oder seine Figur mit Mausklick zu ziehen
- Sicherstellen, dass Wände richtig gesetzt werden
- Sicherstellen, dass Figuren richtig ziehen können und gegebenenfalls übereinander um die Ecke springen können
- Sicherstellen, dass Figuren immer einen Weg auf die gegenüberliegende Seite besitzen
- Möglichkeit, einen Zug rückgängig zu machen
- Benachrichtigung über den Sieger

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
## 10.1 Spielregeln
Quoridor ist ein Brettspiel, dass mit zwei oder vier Personen gespielt werden
kann. Klassischerweise ist das Spielbrett ein 9x9-Feld. Jeder Spieler startet
mit seiner Spielfigur auf dem Mittelfeld einer Randspalte. Zusätzlich erhält
jeder Spieler 10 Wände der länge 2 Spielfelder (bei 4 Spielern 6 Wände). Das
Ziel ist es, die gegenüberliegende Randseite zu erreichen. Dafür ziehen die
Spieler abwechselnd, beziehungsweise im Uhzeigersinn. Ein Zug beinhält entweder
eine Bewegung der eigenen Spielfigur um eins in einer geraden Spielfeldrichtung
(nicht schräg) oder das Setzen einer Wand  zwischen 4 Felder. Eine Wand kann
nicht durch eine Spielfigur passiert werden. Eine Wand kann nicht eine andere
Wand schneiden. Auch dürfen Wände nicht so gesetzt werden, dass einer
Spielfigur es nicht mehr möglich ist, die andere Seite zu erreichen. Es
gewinnt, wessen Spielfigur als erstes die andere Seite erreicht.
