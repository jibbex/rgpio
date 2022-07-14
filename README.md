# rgpio
The wiringPi library has not been maintained for some time. It is also no longer included in the RaspberryPi OS by default. The nowadays recommended way for more experienced developers to control the GPIO is to use the corresponding file descriptors via SysFS. This is nothing more than opening, reading, writing and closing files. 

I had created a small task for trainees to program a corresponding library and then use it in a small console program. They had to develop the solution in C.

I translated my exemplary solution into Rust and I would like to share it here.

## Functions
| Name          | Argumente                                  | Rückgabewert | Beschreibung                               |
|---------------|--------------------------------------------|--------------|--------------------------------------------|
| export        | `gpio_num (i32)`                           |     ----     | Activates GPIO *[n]*                       |
| unexport      | `gpio_num (i32)`                           |     ----     | Deactivates GPIO *[n]*                     |
| write         | `gpio_num (i32)`, `signal (bool)`          |     ----     | Switches the GPIO *[n]* on/off             | 
| read          | `gpio_num (i32)`                           | `bool`       | Gets the current state of GPIO *[n]*       |
| set_direction | `gpio_num (i32)`, `direction (Directions)` |     ----     | Configures the GPIO *[n]* as input/output  |
