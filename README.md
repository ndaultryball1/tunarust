# tunarust
Options pricing in rust - spare time project

This project is my first in rust, and a way for me to learn the language and numerical options pricing methods. 

I also will implement an interface in python as python FFI is something I have been curious about for a while.

I plan to implement finite difference and monte-carlo methods for european options.

Project organisation:

interface - python source code
options pricing - rust library to implement numerical methods, manipulate assets etc
pricing_interface - will provide wrappers around functions and objects from "options_pricing" in order to re-export them and provide a clean API.

Status:

[options_pricing]
* Explicit forward difference for Europeans - done
* Implicit methods for Europeans - Implemented, debugging necessary
* Monte Carlo methods - TBC

[pricing_interface]
*proof of concept rust function called from python - done
*Extern pricing functions - TBC