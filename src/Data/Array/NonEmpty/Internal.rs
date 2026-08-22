pub fn Data_Array_NonEmpty_Internal_foldl1Impl() -> crate::UnknownType {
    crate::Value::Func(std::rc::Rc::new(move |mut f: crate::UnknownType| -> crate::UnknownType {
        crate::Value::Func(std::rc::Rc::new(move |mut xs: crate::UnknownType| -> crate::UnknownType {
                    let arr = xs.unwrap_array();
                    let mut acc = arr[0].clone();
                    for i in 1..arr.len() {
                        acc = f.unwrap_func()(acc).unwrap_func()(arr[i].clone());
                    }
                    acc
                }))
    }))
}

pub fn Data_Array_NonEmpty_Internal_foldr1Impl() -> crate::UnknownType {
    crate::Value::Func(std::rc::Rc::new(move |mut f: crate::UnknownType| -> crate::UnknownType {
        crate::Value::Func(std::rc::Rc::new(move |mut xs: crate::UnknownType| -> crate::UnknownType {
                    let arr = xs.unwrap_array();
                    let mut acc = arr[arr.len() - 1].clone();
                    for i in (0..arr.len() - 1).rev() {
                        acc = f.unwrap_func()(arr[i].clone()).unwrap_func()(acc);
                    }
                    acc
                }))
    }))
}

