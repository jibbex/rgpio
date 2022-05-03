# rgpio
Die Bibliothek wiringPi wird schon einige Zeit nicht mehr gepflegt. Sie liegt auch dem RaspberryPi OS längst nicht mehr standardmäßig bei. Die heutzutage empfohlene Variante für erfahrenere Entwickler um die GPIO zu steuern sieht vor mittels SysFS die entsprechenden File Descriptors zu nutzen. Das ist demnach nicht mehr als Dateien öffnen, lesen, schreiben und wieder schließen. 

Ich hatte für Azubinen und Azubis eine kleine Aufgabe erstellt, die vorsah eine entsprechende Library zu programmieren und diese dann in einem kleinen Konsolenprogramm zu verwenden. Dabei sollten sie die Lösung in C entwickeln.

Meine Musterlösung habe ich in Rust übersetzt und diese möchte ich hier teilen.

## Funktionen
| Name          | Argumente                                  | Rückgabewert | Beschreibung                               |
|---------------|--------------------------------------------|--------------|--------------------------------------------|
| export        | `gpio_num (i32)`                           |     ----     | Aktiviert GPIO *[n]*                       |
| unexport      | `gpio_num (i32)`                           |     ----     | Deaktiviert GPIO *[n]*                     |
| write         | `gpio_num (i32)`, `signal (bool)`          |     ----     | Schaltet den GPIO *[n]* an/aus             | 
| read          | `gpio_num (i32)`                           | `bool`       | Liest den aktuellen Zustand von GPIO *[n]* |
| set_direction | `gpio_num (i32)`, `direction (Directions)` |     ----     | Konfiguriert den GPIO *[n]* als In-/Output |