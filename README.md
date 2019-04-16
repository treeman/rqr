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

* Add doc comments

# Examples

## CLI

There's a simple cli you can use.

It uses optional dependencies to avoid arg parser dependencies to the library, so you need to build it with the flag `--features cli`.

For example to pretty print a QR in a terminal:

```
> cargo run --features cli -- "HELLO WORLD"




        ██████████████        ██    ██████████████        
        ██          ██  ████    ██  ██          ██        
        ██  ██████  ██    ██  ████  ██  ██████  ██        
        ██  ██████  ██  ██████████  ██  ██████  ██        
        ██  ██████  ██  ████  ██    ██  ██████  ██        
        ██          ██    ██    ██  ██          ██        
        ██████████████  ██  ██  ██  ██████████████        
                        ████  ████                        
          ██  ████████  ████    ██████  ████  ██          
        ██  ████████  ██        ████████  ██████          
            ██  ██  ████      ██    ████                  
        ██  ████  ██      ██  ████      ████              
        ████  ████████████████  ██████  ██████████        
                        ██      ██    ██  ██              
        ██████████████    ████    ████    ████████        
        ██          ██  ██  ██    ██    ██  ██████        
        ██  ██████  ██  ████  ██    ██      ██████        
        ██  ██████  ██  ██  ██████      ██  ██            
        ██  ██████  ██    ██        ██        ████        
        ██          ██  ██████    ██████    ████          
        ██████████████    ██  ██              ██          




```

Or to generate an svg:

```
> cargo run --features cli -- "HELLO WORLD" -t svg --bg '#e5bde3' \
    --fg '#700' --width 200 > hello_world.svg
```

![](src/test/hello_world.svg)

