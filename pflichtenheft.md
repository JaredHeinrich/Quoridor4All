? um zu offenen Punkten zu navigieren.

# 1. Zielbestimmung
Bei dem Produkt handelt es sich um ein Spiel, welches man offline im
Multiplayer mit bis zu vier Spielern spielen kann. Ein Vorteil gegenüber einem
herkömmlichen Brettspiel ist die Verfügbarkeit, da es auf jedem PC kostenlos
nutzbar ist. (kann es kostenlos sein, wenn wir es speziell für einen Kunden machen? / soll das hier rein)
# 2. Einsatz 
## Zielgruppen:
Spieler alt
Spieler jung
Admin / Developer gehört der hier hin ??? 
## Einsatzbereiche:
Mögliche Einsatzbereiche sind Spieleabende, Geburtstage, Familienfeiern.
## Szenarien:
Ein Szenario, in dem das Produkt genutzt werden könnte, wäre:
Man trifft sich mit seinen Freunden und möchte sich die Zeit mit einem Spiel
vertreiben.
# 3. Umgebung
Das Produkt ist auf Mac-OS sowie Windows nutzbar.
# 4. Funktionalität
User-Story-Map
# 5. Daten
(muss ein Impressum erstellt werden, wenn das Produkt nur lokal genutzt wird
und der Download über Github möglich ist?)

Daten, die Verarbeitet werden sind zum einen ein vom Nutzer freiwillig
anlegbares Spielerprofil, ohne E-Mail, Password oder Daten wie Alter, Name,
Nachname. Das Spielerprofil enthält einen vom Nutzer ausgedachter Nutzernamen,
sowie Information zu den mit diesem Nutzernamen gespielten Spielen. Die
Informationen sind nicht gesondert durch ein Passwort oder ähnliches geschützt,
da sie lediglich lokal gespeichert werden. Aus selbigem Grund werden die
Spieldaten auch nicht verschlüsselt gespeichert.
# 6. Leistungen
Um eine angenehme Nutzererfahrung zu bieten werden folgende
Leistungsanforderungen aufgestellt:
- Skalierbarkeit: Die Spieleranzahl wird nicht über 4 erhöhbar sein. Es soll
  möglich sein in Zukunft die Spielfeldgröße sowie die Zugmöglichkeiten zu
  erweitern, ohne dass andere Leistungsanforderungen nicht mehr eingehalten
  werden. Die maximale Anzahl an Spielerprofilen soll 100
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
# 8. Qualitätsziele
- Wartbarkeit: Die Wartbarkeit und Erweiterbarkeit der Software soll einfach
möglich sein.
- Zuverlässigkeit: Die Software soll Zuverlässig und ohne Fehler laufen.
- Sicherheit: Was ist Sicherheit in unserem Kontext???
Um die Einhaltung der Qualitätsziele sicherzustellen, wird die Software mithilfe von Unit-Test hinreichend getestet.
# 9. Entwicklungsumgebung
- Softwaretools:
Neo Vim,
Visual Studio Code,
Github Desktop,
Git
Auch Programmiersprachen??
- Programmierrichtlinien: Für die Naming-Conventions werden die Standard
  Rust-Naming-Conventions übernommen. (Was gehört sonst noch hier hin???)
- Versionskontrolle: Für die Versionierung wird Git und Github genutzt.
  Einzelne Features werden auf seperaten Branches entwickelt, welche nach dem
  merge mit dem main-branch gelöscht werden.
# 10. Erweiterungen
Hier werden die Regel definiert. ??
