# Rust Erklärung

## Rückgabe von Werten
In Rust können Werte nicht nur von Funktionen zurückgegeben werden, sondern im Prinzip von allem was von `{}` umgeben ist.

### Rückgabe aus Funktion durch `return`
```rust
fn test (val: bool) -> bool {
    loop {
        if val {
            return val;
        }
    }
}
```

Hier wird `val` durch das `return` keyword von der Funktion zurückgegeben. `return` sogt immer dafür, dass aus der Methode returned wird, egal in wie tief wir gerade im scope sind.

### Rückgabe durch Weglassen des Semikolons
```rust
fn test (val: bool) -> String {
    if val {
        "No";
        "Yes" // <- wird von `if-else`-Scope zurückgegeben (kein Semikolon)
    } else {
        "Yes";
        "No" // <- wird von `if-else`-Scope zurückgegeben (kein Semikolon)
    } // <- Rückgabewert des `if-else`-Scopes wird von Funktion zurückgegeben (kein Semikolon)
}
```

Bei der letzten Zeile eines Scopes kann man das Semikolon weglassen, um diesen Wert aus dem Scope zurückzugeben.

### Rückgabe aus loop durch `break`
```rust
fn test () -> String {
    loop {
        if 4 == 5 {
            break "Mathe ist kaputt!"; // <- wird von for loop zurückgegeben.
        } else {
            break "Mathe funktioniert!";
        }
    } // <- Rückgabewert des for loops wird von Funktion zurückgegeben.
}
```
Aus loops kann man mit dem `break` Keyword ausbrechen und auch einen Wert zurückgeben, ähnlich wie `return`

## Modules
Ein `module` ist vergleichbar mit einem `package` in Java oder `namespace` in C#.

Der name eines `modules` wird vom Dateinamen, oder bei `mod.rs` vom Ordnernamen, festgelegt.

Ein `Module` kann durch das `use` Keyword importiert werden. Importiert man aus dem eigenen Projekt ist das top layer `module` `crate`.

### Beispiel
```text
src
| main.rs
|
| engine
| | mod.rs
| | figures.rs
|
| io.rs
```

In `io.rs` kann ich nun auf Funktionen etc. von `mod.rs` zugreifen, wenn ich das `module` folgendermaßen importiere:

```rust
use crate::engine;
```