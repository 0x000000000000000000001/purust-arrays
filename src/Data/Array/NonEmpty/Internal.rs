pub fn Data_Array_NonEmpty_Internal_foldl1Impl() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a {
        call: Some(std::rc::Rc::new(move |mut f: crate::UnknownType| -> crate::UnknownType {
            crate::UnknownType::new(crate::Record_a {
                call: Some(std::rc::Rc::new(move |mut xs: crate::UnknownType| -> crate::UnknownType {
                    let arr = xs.init_array.as_ref().unwrap();
                    let mut acc = arr[0].clone();
                    for i in 1..arr.len() {
                        acc = f.call.as_ref().unwrap()(acc).call.as_ref().unwrap()(arr[i].clone());
                    }
                    acc
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}

pub fn Data_Array_NonEmpty_Internal_foldr1Impl() -> crate::UnknownType {
    crate::UnknownType::new(crate::Record_a {
        call: Some(std::rc::Rc::new(move |mut f: crate::UnknownType| -> crate::UnknownType {
            crate::UnknownType::new(crate::Record_a {
                call: Some(std::rc::Rc::new(move |mut xs: crate::UnknownType| -> crate::UnknownType {
                    let arr = xs.init_array.as_ref().unwrap();
                    let mut acc = arr[arr.len() - 1].clone();
                    for i in (0..arr.len() - 1).rev() {
                        acc = f.call.as_ref().unwrap()(arr[i].clone()).call.as_ref().unwrap()(acc);
                    }
                    acc
                })),
                ..Default::default()
            })
        })),
        ..Default::default()
    })
}

