use tokenizers::tokenizer::Tokenizer;
// use onnxruntime::{
//     environment::Environment, ndarray::Array, tensor::OrtOwnedTensor, GraphOptimizationLevel,
//     LoggingLevel,
// };

fn main() {

    let tokenizer = Tokenizer::from_file("./tokenizer/tokenizers.json").unwrap();
    let output = tokenizer.encode("My name is Philipp", true).unwrap();
    // println!("{:?}", output.get_attention_mask().to_vec());
    // println!("{:?}", output.get_ids().to_vec());

    let input = "My name is Philipp";

    let input_ids = InferInputTensorBuilder::default()
            .name("input_ids".to_string())
            .dtype(TensorType::Int64)
            .length(encoding.get_ids().len())
            .with_u32_content(encoding.get_ids())
            .build()
            .unwrap();

        let attention_mask = InferInputTensorBuilder::default()
            .name("attention_mask".to_string())
            .dtype(TensorType::Int64)
            .length(encoding.get_attention_mask().len())
            .with_u32_content(encoding.get_attention_mask())
            .build()
            .unwrap();

    // match tokenizer.encode(input, true) {
    //     Ok(encoding) => ,
    //     Err(e) => format!("{:?}", e),
    // }
    // Ok(())
}
