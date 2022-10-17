#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_ast::{
        emit_binary, Export, Function, FunctionType, ModuleBuilder, ResultType, ValueType,
        VariableInstruction,
    };
    use wasmtime::{Engine, Instance, Store};

    fn instanciate_binary(bytes: impl AsRef<[u8]>) -> (Store<()>, Instance) {
        let engine = Engine::default();
        let module = wasmtime::Module::new(&engine, bytes).unwrap();
        let mut store = Store::new(&engine, ());
        let instance = Instance::new(&mut store, &module, &vec![]).unwrap();
        (store, instance)
    }

    #[test]
    fn nop_main_func() {
        let mut builder = ModuleBuilder::new();

        // Create a main function with one parameter and returning it
        let one_i32 = ResultType::from(vec![ValueType::I32]);
        let func_type_index = builder
            .add_function_type(FunctionType::new(one_i32.clone(), one_i32))
            .unwrap();
        let main_func = Function::new(
            func_type_index,
            ResultType::empty(),
            vec![VariableInstruction::LocalGet(0).into()].into(),
        );
        let main_func_index = builder.add_function(main_func).unwrap();

        // Export the function so that we can call it
        let name = "test";
        let export = Export::function(name.into(), main_func_index);
        builder.add_export(export);
        let module = builder.build();

        let mut buffer = Vec::new();
        emit_binary(&module, &mut buffer).unwrap();

        let (mut store, instance) = instanciate_binary(&buffer[..]);
        let typed_func = instance
            .get_typed_func::<i32, i32, _>(&mut store, &name)
            .unwrap();
        let result = typed_func.call(&mut store, 1).unwrap();
        assert_eq!(1, result);
    }
}
