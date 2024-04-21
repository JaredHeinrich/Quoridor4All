## CI-CD-Pipeline 5b
Die CI-CD Pipeline wird mithilfe von Github Actions/Workflows umgesetzt und
besteht aus 2 Teilen.

Der CI-Teil ist bei uns tests.yml und führt einen Test Build sowie alle
Unit-Tests aus. Dieser wird beim Erstellen einer Pull-Request auf main
ausgeführt und durchläuft die folgenden Schritte:

- Starte den Workflow mit dem neusten Ubuntu. 
- Setzt das Standart Working-Directory auf ./quoridor4all/src-tauri/ (da liegt der Rust Source-Code).
- Führt die Checkout-Action aus.
- Installiert die Notwendigen Dependencies für den Build.
- Installiert neuste Rust/Cargo Versionen.
- Führt einen Test-Build aus (überprüft, ob sich die Applikation bauen lässt).
- Lässt alle Unit-Tests laufen.

Der CD-Teil ist bei uns publish.yml. Dieser wird beim Erstellen einer
Pull-Request auf main ausgeführt und durchläuft die folgenden Schritte.

- Es wird ein Job für jede definierte Plattform gestartet.
- Führt die Checkout-Action aus.
- Setup-Node Action, um Node zu installieren.
- Rust-Toolchain Action, um Rust und Cargo zu installieren. 
- rust-cache Action cached bereits kompilierte Crates, um später Kompilierzeit zu sparen.
- npm-Dependencies installieren.
- tauri-action, um Build durchzuführen und ein Github release zu erstellen.
