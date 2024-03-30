# A basic Rust to C++ transpiler  written in Rust
## Features 
- Variables (int, float, string, bool)
- Basic math  (+, -, *, /, %)
- If else statements
- For loops
- While loops
- Functions
## Usage
    $basic-transpiler sourece-file output-file
## Example
Input file

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
Output file 

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
