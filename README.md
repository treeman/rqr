Good tutorial:

<https://www.thonky.com/qr-code-tutorial/introduction>

Errors in the tutorial:
* Bitwise repr of dec 16
* Dark module coordinates have x and y swapped
* Dark module is marked to be added in both module placement and format & version information sections.

Minor improvements suggestions:
* Note which QR is the mask evaluation based on?
* Maybe I did miss this somewhere, but I first implemented dark as 0,
  but it's supposed to be dark as 1.

# TODO

* Qr API, hide builder.
* Calculate minimal version
* Create CLI
* Calculate the minimal applicable version
  Move it after encoding data? To avoid having to include a big table.
* Add doc comments
* Instead of `from_str` methods implement into trait?

