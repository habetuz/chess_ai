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
fn test (val: bool) -> String {
    for i in 0..10 { // Iteriert 0,1,2,3,4,5,6,7,8,9
        if i == 5 {
            break "break"; // <- wird von for loop zurückgegeben.
        }
    } // <- Rückgabewert des for loops wird von Funktion zurückgegeben.
}
```
Aus loops kann man mit dem `break` Keyword ausbrechen und auch einen Wert zurückgeben, ähnlich wie `return`