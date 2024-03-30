# A basic Rust to C++ transpiler  written in Rust
## Features 
- Variables (int, float, string, bool)
- Basic math  (+, -, *, /, %)
- If else statements
- For loops
- While loops
- Functions
## Usage
Run the executable in the terminal  with arguments for the path/name of the source file and the output file in that order
The executable can be renamed freely
Example:

    $basic-transpiler source.rs output.js
## Building
To build the project simply install rust via rustup, copy the repo and run:

    $cargo build --release
## Example
Input file:

    fn main () {
      let a = 5;
      let b = 10;
      while a <= b{
        if a < 0{
          a = a + 1;
        }
        else {
          a = a + 1;
        }
      } 
    }
Output file:

    function main(){                                                                              
    let a=5;                                                                                      
    let b=10;                                                                                     
    while (a<=b){                                                                                 
    if (a<0){                                                                                     
    a=a+1;                                                                                        
    }                                                                                             
    else{                                                                                         
    a=a+1;                                                                                        
    }                                                                                             
    }                                                                                             
    }
