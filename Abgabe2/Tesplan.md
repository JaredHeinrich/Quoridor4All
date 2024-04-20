## Verantwortliche Person
Valentin Czekalla

## Größte Risiken
Spiellogik falsch

Nutzererfahrung (intuitiv)

Kompatibilität

## Vorgehensweise
Die Backend-Logik-Funktionalität ist größtenteils durch Unit-Tests abgedeckt. Dies bietet sich an, da die Anforderungen an die Logik durch die festen Spielregeln nicht ändern und die Logik gut auf logische Fehler automatisch überprüfbar.
Der Frontend-Code lässt sich dagegen schlecht mit Unit-Tests überprüfen, da Fehler schwerer definiert werden können und es eher veränderte Anforderungen ohne absolute Wahrheit gibt. Daher wird die Funktionalität in manuellen End-to-End-Tests mitgetestet. Dafür wurden Testszenarien entwickelt und Test Touren durchgeführt

### Testszenarien (siehe Excel)
1. Spielstart
2. Spiel spielen
3. Sieger
4. Kompatibilität
   
## Test Touren
1. Couch Potato Tour/Garbage Collector Tour: möglichst schnell alles durchklicken (immer geradeauslaufen, bis zum Sieg)

   
## Protokoll
befindet sich in extra Datei Testprotokoll.xlsx
