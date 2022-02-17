# Technical test for JellySmack
![example workflow](https://github.com/Aeradriel/jellysmack_technical_test/actions/workflows/stick_path.yml/badge.svg)
![example workflow](https://github.com/Aeradriel/jellysmack_technical_test/actions/workflows/star_light.yml/badge.svg)

## Stick Path
### How to run
- Checkout the repository
- Navigate to the stick_path folder
- Use `cargo run` to compile and launch the program.

/!\ The program read from the standard input so you can either cat a file and redirect the standard ouput to the standard input of the program (`cat map.txt | cargo run`) or simply launch the program and then type your map. /!\
### Time spent
- Map generation, error handling, tests => 1h
- Refactor of map type, coordinates checks, tests => 1h30m
- Exit finding, manual tests => 30m
- Unit tests => 20m

TOTAL => ~3h30m

### Feedbacks
- The second example misses the first line (that states the size of the map.
- In the PDF, it looks that there is 2 spaces in between columns when there is no link but when you copy paste there is only one which can lead to questions about how to handle it. I have chosen to simply ignore spaces (and all other random characters and only retain the "|" and "-" characters in the map.

## Star Light
### How to run
- Checkout the repository
- Navigate to the star_light folder
- Use `cargo run` to compile and launch the program.

/!\ The program read from the standard input so you can either cat a file and redirect the standard ouput to the standard input of the program (`echo "11001001000\n10000110011" | cargo run`) or simply launch the program and then type your two patterns. /!\
### Time spent
- Input parsing & validation => 15m
- Algo thinking/implementation => 2h15m
- Unit tests/documentation => 20m

TOTAL => ~3h

### Feedbacks
- Strangely it took less time than the first assignment, but it was for sure more complicated (time felt long when thinking about the algorithm without being able to test a lot).
- A lot more abstract but a lot more rewarding when you find how to make it works.

### Warning
- Due to the high number of iterations needed, I have not been able to test a lot of cases so there might be some tricky case that are not handled correctly but the examples works perfectly.
