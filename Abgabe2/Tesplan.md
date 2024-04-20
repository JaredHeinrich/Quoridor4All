## Verantwortliche Person
Valentin Czekalla

## Größte Risiken
Spiellogik falsch

Nutererfahrung (Uninituitiv)

Kompatibilität

## Vorgehensweise
Die Backend-Logik-Funktionalität ist größtenteils durch Unit-Tests abgedeckt. Dies bietet sich an, da die Anforderungen an die Logik durch die festen Spielregeln nicht ändern und die Logik gut auf logische Fehler automatisch überprüfbar.
Der Frontend-Code lässt sich dagegen schlecht mit Unit-Tests überprüfen, da Fehler schwerer definiert werden können und es eher veränderte Anforderungen ohne absolute Wahrheit gibt. Daher wird die Funktionalität in manuellen End-to-End-Tests mitgetestet.

1.Spielstart
    1. Eingabe von Spielernamen
    1. Spielstart funktioniert
2. Spiel Spielen, Mit jedem Spieler
    1. Anzeige der möglichen Züge überprüfen
    2. Wand setzen (nicht in Konflikt mit anderer Wand)
    3. Spielfigur bewegen
    4. Spielfigur zurück
    5. Wand zurück
    6. Edge Case: Wand blockiert Spielzug --> Keine Anzeige als möglicher Zug
    7. Edge Case: anderer Spieler kann übersprungen werden
    8. Edge Case, anderer Spieler kann übersprungen werden und dahinter ist eine Wand
    9. Edge Case, alle 4 Spieler nebeneinander --> Potenziell endlose Schleife
    10. Edge Case: Path Checker grafisch überprüfen durch Versuch, Pfad zu verbauen
    11. Regeln am Anfang anschauen
    12. Regeln während des Spiels anschauen und Spiel fortfahren
    13. Regeln während des Spiels anschauen und Spiel abbrechen
    14. Spiel abbrechen und neues Spiel starten
    15. Wall Zug abbrechen
    16. Player Zug abbrechen
    17. keinen ausgewählten Zug versuchen, abzubrechen
3. Sieger anzeigen
    1. richtiger Name angezeigt
    2. Leaderboard überprüfen
    3. Navigation zurück zu Start überprüfen
4. Allgemein Usability durch unvertraute Person
   
## Test Touren
1. Couch Potato Tour/Garbage Collector Tour: möglichst schnell alles durchklicken (immer geradeauslaufen, bis zum Sieg)
   
## Protokoll
befindet sich in extra Datei Testprotokoll.xlsx
