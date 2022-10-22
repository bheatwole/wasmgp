#[cfg(test)]
mod tests {
    use std::vec;

    use wasm_ast::{
        emit_binary, Export, Expression, Function, FunctionType, ModuleBuilder, NumberType,
        NumericInstruction, ResultType, ValueType, VariableInstruction, IntegerType, FloatType,
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

        // Create an instance of the module and get a pointer to the exported function by its name
        let (mut store, instance) = instanciate_binary(&buffer[..]);
        let typed_func = instance
            .get_typed_func::<i32, i32, _>(&mut store, &name)
            .unwrap();

        // Call the function and confirm we get the same value as what we passed in
        let result = typed_func.call(&mut store, 1).unwrap();
        assert_eq!(1, result);
        let result = typed_func.call(&mut store, 42).unwrap();
        assert_eq!(42, result);
    }

    #[test]
    fn what_happens_to_extra_stack_values() {
        // This test determines if we have to use every stack value in a function, or if there can be extra values.
        let mut builder = ModuleBuilder::new();

        // Create a main function that takes one int and returns the double of it. However it loads the parameter to the
        // stack three times instead of the two that are necessary.
        let one_i32 = ResultType::from(vec![ValueType::I32]);
        let func_type_index = builder
            .add_function_type(FunctionType::new(one_i32.clone(), one_i32))
            .unwrap();
        let body: Expression = vec![
            VariableInstruction::LocalGet(0).into(), // load param once
            VariableInstruction::LocalGet(0).into(), // load param twice
            VariableInstruction::LocalGet(0).into(), // load param thrice (not necessary)
            NumericInstruction::Add(NumberType::I32).into(), // double (pops twice and pushes back one answer)
        ]
        .into();
        let main_func = Function::new(func_type_index, ResultType::empty(), body);
        let main_func_index = builder.add_function(main_func).unwrap();

        // Export the function so that we can call it
        let name = "double";
        let export = Export::function(name.into(), main_func_index);
        builder.add_export(export);
        let module = builder.build();

        // Attempt to create an instance of the module. This will fail because the stack is not empty after the return
        // values have been popped
        let mut buffer = Vec::new();
        emit_binary(&module, &mut buffer).unwrap();
        let engine = Engine::default();
        let module_err = wasmtime::Module::new(&engine, &buffer[..]);
        assert!(module_err.is_err());

        // This test shows that Wasm expects very well-mannered and well-formed code from our genetic algorithm.
    }

    #[test]
    fn what_happens_to_extra_local_values() {
        // This test determines if a program compiles that has local values that aren't used
        let mut builder = ModuleBuilder::new();

        // Create a main function that takes one int and returns the double of it. However it loads the parameter to the
        // stack three times instead of the two that are necessary.
        let one_i32 = ResultType::from(vec![ValueType::I32]);
        let func_type_index = builder
            .add_function_type(FunctionType::new(one_i32.clone(), one_i32))
            .unwrap();
        let body: Expression = vec![
            VariableInstruction::LocalGet(0).into(), // load param once
            VariableInstruction::LocalGet(0).into(), // load param twice
            VariableInstruction::LocalGet(0).into(), // load param thrice
            VariableInstruction::LocalSet(1).into(), // put the param we just loaded into in local
            NumericInstruction::Add(NumberType::I32).into(), // double (pops twice and pushes back one answer)
        ]
        .into();
        // Define three local values in the main function. We will only use one
        let main_func = Function::new(func_type_index, ResultType::new(vec![
            IntegerType::I32.into(),
            IntegerType::I32.into(),
            FloatType::F64.into(),
        ]), body);
        let main_func_index = builder.add_function(main_func).unwrap();

        // Export the function so that we can call it
        let name = "double";
        let export = Export::function(name.into(), main_func_index);
        builder.add_export(export);
        let module = builder.build();

        let mut buffer = Vec::new();
        emit_binary(&module, &mut buffer).unwrap();

        // Attempt to create an instance of the module.
        let (mut store, instance) = instanciate_binary(&buffer[..]);
        let typed_func = instance
            .get_typed_func::<i32, i32, _>(&mut store, &name)
            .unwrap();

        // Call the function and confirm we get the same value as what we passed in
        let result = typed_func.call(&mut store, 1).unwrap();
        assert_eq!(2, result);
        let result = typed_func.call(&mut store, 42).unwrap();
        assert_eq!(84, result);

        // This test confirms that we can use local variables as a way to write generic code and have it work even after
        // a crossover operation.
    }
}
