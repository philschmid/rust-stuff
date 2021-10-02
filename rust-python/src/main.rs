use pyo3::prelude::*;
use std::time::{Duration, Instant};

fn load_python_module_from_file() {
    let now = Instant::now();
    const PYTHON_MODULE: &'static str = include_str!("../pyth/pipeline.py");
    println!("Loading python as string {}", now.elapsed().as_millis());

    Python::with_gil(|py| {
        println!("Loading python with gil {}", now.elapsed().as_millis());
        let pipeline_file = PyModule::from_code(py, PYTHON_MODULE, "pipeline.py", "pipeline").unwrap();
        println!("Loading python module {}", now.elapsed().as_millis());
        let handler = pipeline_file.getattr("Handler").unwrap();
        println!("Loading handler module {}", now.elapsed().as_millis());
        let handler = handler.call1(("text-classification",)).unwrap();
        println!("Loading init handler {}", now.elapsed().as_millis());

        let pred = handler.call_method1("__call__", ("i love you.",)).unwrap();
        println!("predict {}", now.elapsed().as_millis());
        println!("{}",pred)
    })
}

// fn load_python_module_in_rust() {

//     let now = Instant::now();

//     Python::with_gil(|py| {
//         println!("Loading python with gil {}", now.elapsed().as_millis());
//         let transformers = PyModule::import(py, "transformers").unwrap();;
//         println!("Loading python module {}", now.elapsed().as_millis());
//         let text_classification:Py<PyAny> = transformers.getattr("pipeline").unwrap().call1(("text-classification",)).unwrap().extract().unwrap();;    
//         println!("Loading handler module {}", now.elapsed().as_millis());   
//         let pred = text_classification.call1(py,("i love you.",)).unwrap();  
//         println!("predict {}", now.elapsed().as_millis());
//         println!("{}",pred)
//     })
// }

fn load_python_module_in_rust() {

    let now = Instant::now();

        let gil = Python::acquire_gil();
        let transformers = PyModule::import(gil.python(), "transformers").unwrap();
        let text_classification: Py<PyAny> = transformers
            .getattr("pipeline")
            .unwrap()
            .call1(("text-classification",))
            .unwrap()
            .extract()
            .unwrap();

        let pred = text_classification.call1(gil.python(),("i love you.",)).unwrap();  
        println!("predict {}", now.elapsed().as_millis());
        println!("{}", pred);
}



fn main() {
    // load_python_module_from_file()
    load_python_module_in_rust()
}

