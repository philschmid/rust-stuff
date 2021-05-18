use tokenizers::tokenizer::{Result, Tokenizer};
use tokenizers::Encoding;
// use onnxruntime::{
//     environment::Environment, ndarray::Array, tensor::OrtOwnedTensor, GraphOptimizationLevel,
//     LoggingLevel,
// };

fn main() {
    let input = "My name is Philipp";

    let output = match tokenize(input) {
        Ok(output) => output,
        Err(e) => panic!(e),
    };
    println!("{:?}", output.get_tokens());

    let my_tokenizer = match Tokenizer::from_file("./tokenizer/tokenizers.json") {
        Ok(tk) => tk,
        Err(e) => panic!(e),
    };

    let output2 = ref_tokenize(&my_tokenizer, input).unwrap();

    println!("{:?}", output2.get_tokens());
}

fn tokenize(input: &str) -> Result<Encoding> {
    let tokenizer = Tokenizer::from_file("./tokenizer/tokenizers.json")?;
    let output = tokenizer.encode(input, true)?;

    Ok(output)
}

fn ref_tokenize(tokenizer: &Tokenizer, input: &str) -> Result<Encoding> {
    let output = tokenizer.encode(input, true)?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tokenize() {
        let test_input = "My name is Philipp";
        let test_output = tokenize(test_input).unwrap();
        assert_eq!(
            ["[CLS]", "my", "name", "is", "philipp", "[SEP]"],
            test_output.get_tokens()
        );
    }
    #[test]
    #[should_panic]
    fn another() {
        panic!("Make this test fail");
    }
}
