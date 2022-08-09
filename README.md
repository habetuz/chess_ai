# chess_ai - Ein Schachalgorithmus

## Installation

<img src="https://raw.githubusercontent.com/habetuz/chess_ai/master/gifs/start.gif" width="300" height="300"/>

Lade dir die [kompilierte Version](https://github.com/habetuz/chess_ai/releases/tag/1.0) herunter, oder kompiliere das Projekt selbst mit [Rust](https://www.rust-lang.org/tools/install).

Führe dannach einfach die Datei aus und das Spiel kann beginnen!

## Verwendung

### Ziehen

<img src="https://raw.githubusercontent.com/habetuz/chess_ai/master/gifs/move.gif" width="300" height="300"/>

Du spielst die farbe Weiß! Ziehe eine Figur, indem du die Koordination eingibst: `von zu` Beispiel: `a1 a2` um von `a1` nach `a2` zu gehen.

Nachdem du deinen Zug gemacht hast wird die KI anfangen ihren Zug zu errechnen. Das kann einen Moment dauern!

### Mögliche Züge anzeigen lassen

<img src="https://raw.githubusercontent.com/habetuz/chess_ai/master/gifs/possible_moves.gif" width="300" height="300"/>

Gib die Koordinate einer deiner Figuren ein, um zu sehen, welche Züge für sie möglich sind.

## Funktion

Die KI nutzt MiniMax zum finden des relativ optimalen Zuges. Außerdem wird Alpha-Beta Pruning eingesetzt, um den Suchprozess zu beschleunigen.

Zur Evaluierung einer Situation werden den Figuren und den Positionen der Figuren Werte zugeordnet.

Die [Chessprogramming Wiki](https://www.chessprogramming.org/Main_Page) habe ich oft als nützliche Quelle genutzt.
