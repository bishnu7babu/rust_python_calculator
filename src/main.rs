use pyo3::prelude::*;
use pyo3::types::PyModule;
use std::ffi::CString;
use std::io::{self, Write};

fn main() {

    pyo3::prepare_freethreaded_python();

    println!("Welcome to rust and Python Calculator!");
    println!("------------------------------------------------");
    print!("Enter Your Number:: ");
    io::stdout().flush().unwrap();
    let mut num_buf = String::new();
    io::stdin().read_line(&mut num_buf).expect("Failed to read the line");
    let num_buf: i32 = num_buf.trim().parse().unwrap();

    print!("Enter Your Operation('+','-','*','/') :: ");
    io::stdout().flush().unwrap();
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Failed to read the line");
    let operation = operation.trim();

    print!("Enter Your Number:: ");
    io::stdout().flush().unwrap();
    let mut num_buf2 = String::new();
    io::stdin().read_line(&mut num_buf2).expect("Failed to read the line");
    let num_buf2: i32 = num_buf2.trim().parse().unwrap();

    Python::with_gil(|py|{
        let script = CString::new(include_str!("../py_calculator.py")).expect("Failed to create CString");
        let module_name = CString::new("py_calculator").expect("Failed to create the CString");

        let filename = CString::new("py_calculator.py").expect("Failed to create CString");

        let module = PyModule::from_code(py, script.as_c_str(), filename.as_c_str(), module_name.as_c_str()).expect("Failed to create python module");

        let result: f32 = module
            .getattr("calculator")
            .expect("Function calculator not found")
            .call1((num_buf, num_buf2, operation))
            .expect("Function call failed")
            .extract()
            .expect("Failed to extract python function result");

        println!("Calculated result = {}",result);
    });
}
