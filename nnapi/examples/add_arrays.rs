use nnapi::{Model, Operand};
use nnapi_sys::{OperandCode, OperationCode};

fn main() -> nnapi::Result<()> {
    let tensor9x_type = Operand::tensor(OperandCode::ANEURALNETWORKS_TENSOR_FLOAT32, &[9], 0., 0);

    let mut model = Model::from_operands([
        tensor9x_type,
        tensor9x_type,
        Operand::activation(),
        tensor9x_type,
    ])?;

    model.set_activation_operand_value(2)?;
    model.add_operation(OperationCode::ANEURALNETWORKS_ADD, &[0, 1, 2], &[3])?;
    model.identify_inputs_and_outputs(&[0, 1], &[3])?;

    model.finish()?;

    let mut compilation = model.compile()?;
    let execution = compilation.create_execution()?;

    Ok(())
}
