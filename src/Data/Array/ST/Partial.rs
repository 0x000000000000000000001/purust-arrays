pub fn Data_Array_ST_Partial_peekImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|index, array| {
        let index = usize::try_from(index.unwrap_int()).expect("Partial STArray.peek: negative index");
        Purs_Data_Array_ST::purust_st_array(&array).lock().get(index)
            .expect("Partial STArray.peek: index out of bounds").clone()
    }))
}

pub fn Data_Array_ST_Partial_pokeImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|index, value, array| {
        let index = usize::try_from(index.unwrap_int()).expect("Partial STArray.poke: negative index");
        let cell = Purs_Data_Array_ST::purust_st_array(&array);
        let mut values = cell.lock();
        *std::rc::Rc::make_mut(&mut values).get_mut(index)
            .expect("Partial STArray.poke: index out of bounds") = value;
        crate::Value::Unit
    }))
}
