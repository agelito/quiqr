# quiqr
Quiqr is a tool for quickly generating a QR code from text and display then display the generated QR code on screen.

## Usage
Quiqr can display QR codes coming from various input sources. But it can only have one input source selected. The options are:
- String supplied as command line argument.
- String from current clipboard content.
- String from stdin.

Quiqr can also optionally save the generated QR code to an image file.

### Running
```
# Generates a QR code from the string `Hello, Quiqr!`.
./quiqr -e "Hello, Quiqr!"

# Generates a QR code from the current clipboard contents.
./quiqr -c 

# Generates a QR code from stdin.
./quiqr -s 

# 🖼 Generates a QR code and saves it to 'qr.png'.
./quiqr -c -w "qr.png"
```

### Keyboard Shortcuts
- `<ESC>` or `<Q>` Close the window.
