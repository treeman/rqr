Good tutorial:

<https://www.thonky.com/qr-code-tutorial/introduction>

Errors in the tutorial:
* Bitwise repr of dec 16
* Dark module coordinates have x and y swapped
* Dark module is marked to be added in both module placement and format & version information sections.

# TODO

* Masking
* Add format & version info
* Generate svg
* Generate bitmap
* Additional options
  1. Specify chars in string output
  2. Toggle quiet zone
  3. Set module width
* Calculate the minimal applicable version
  Move it after encoding data? To avoid having to include a big table.
* Add doc comments
* Instead of `from_str` methods implement into trait?

