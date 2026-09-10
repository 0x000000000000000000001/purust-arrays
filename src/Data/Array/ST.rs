use std::rc::Rc;
use std::sync::{Mutex, MutexGuard};

// A mutable cell owns a shared immutable buffer. The unsafe conversions share
// that buffer in O(1); a subsequent mutation copies it only when still shared.
pub struct PurustSTArray(Mutex<Rc<Vec<crate::UnknownType>>>);

impl PurustSTArray {
    pub fn lock(&self) -> MutexGuard<'_, Rc<Vec<crate::UnknownType>>> {
        self.0.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

pub fn purust_st_array(value: &crate::UnknownType) -> &PurustSTArray {
    value.unwrap_class::<PurustSTArray>()
}

fn purust_st_array_box(values: Rc<Vec<crate::UnknownType>>) -> crate::UnknownType {
    crate::Value::Class(Rc::new(PurustSTArray(Mutex::new(values))))
}

fn purust_st_array_removed(
    just: crate::UnknownType,
    nothing: crate::UnknownType,
    value: Option<crate::UnknownType>,
) -> crate::UnknownType {
    match value {
        Some(value) => just.unwrap_func1()(value),
        None => nothing,
    }
}

pub fn Data_Array_ST_new() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|_| purust_st_array_box(Rc::new(Vec::new()))))
}

pub fn Data_Array_ST_unsafeFreezeImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|array| {
        crate::Value::Array(purust_st_array(&array).lock().clone())
    }))
}

pub fn Data_Array_ST_unsafeThawImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|array| purust_st_array_box(array.unwrap_array())))
}

pub fn Data_Array_ST_freezeImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|array| {
        crate::Value::Array(Rc::new(purust_st_array(&array).lock().as_ref().clone()))
    }))
}

pub fn Data_Array_ST_thawImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|array| {
        purust_st_array_box(Rc::new(array.unwrap_array().as_ref().clone()))
    }))
}

pub fn Data_Array_ST_cloneImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|array| {
        purust_st_array_box(Rc::new(purust_st_array(&array).lock().as_ref().clone()))
    }))
}

pub fn Data_Array_ST_lengthImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|array| {
        crate::mk_int(purust_st_array(&array).lock().len() as i64)
    }))
}

pub fn Data_Array_ST_peekImpl() -> crate::UnknownType {
    crate::Value::Func4(purust_core::Func4::Static(|just, nothing, index, array| {
        let value = usize::try_from(index.unwrap_int()).ok()
            .and_then(|index| purust_st_array(&array).lock().get(index).cloned());
        purust_st_array_removed(just, nothing, value)
    }))
}

pub fn Data_Array_ST_pokeImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|index, value, array| {
        let cell = purust_st_array(&array);
        let mut values = cell.lock();
        let index = usize::try_from(index.unwrap_int()).ok();
        if let Some(index) = index.filter(|&index| index < values.len()) {
            Rc::make_mut(&mut values)[index] = value;
            crate::Value::Bool(true)
        } else {
            crate::Value::Bool(false)
        }
    }))
}

pub fn Data_Array_ST_popImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|just, nothing, array| {
        let value = Rc::make_mut(&mut purust_st_array(&array).lock()).pop();
        purust_st_array_removed(just, nothing, value)
    }))
}

pub fn Data_Array_ST_shiftImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|just, nothing, array| {
        let cell = purust_st_array(&array);
        let value = {
            let mut values = cell.lock();
            if values.is_empty() { None } else { Some(Rc::make_mut(&mut values).remove(0)) }
        };
        purust_st_array_removed(just, nothing, value)
    }))
}

pub fn Data_Array_ST_pushImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|value, array| {
        let cell = purust_st_array(&array);
        let mut values = cell.lock();
        Rc::make_mut(&mut values).push(value);
        crate::mk_int(values.len() as i64)
    }))
}

pub fn Data_Array_ST_pushAllImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|inserted, array| {
        let inserted = inserted.unwrap_array();
        let cell = purust_st_array(&array);
        let mut values = cell.lock();
        Rc::make_mut(&mut values).extend(inserted.iter().cloned());
        crate::mk_int(values.len() as i64)
    }))
}

pub fn Data_Array_ST_unshiftAllImpl() -> crate::UnknownType {
    crate::Value::Func2(purust_core::Func2::Static(|inserted, array| {
        let inserted = inserted.unwrap_array();
        let cell = purust_st_array(&array);
        let mut values = cell.lock();
        Rc::make_mut(&mut values).splice(0..0, inserted.iter().cloned());
        crate::mk_int(values.len() as i64)
    }))
}

pub fn Data_Array_ST_spliceImpl() -> crate::UnknownType {
    crate::Value::Func4(purust_core::Func4::Static(|index, count, inserted, array| {
        let inserted = inserted.unwrap_array();
        let cell = purust_st_array(&array);
        let mut values = cell.lock();
        let length = values.len();
        let index = index.unwrap_int();
        let start = if index < 0 {
            length.saturating_sub(index.unsigned_abs() as usize)
        } else {
            (index as usize).min(length)
        };
        let removed = (count.unwrap_int().max(0) as usize).min(length - start);
        let result = Rc::make_mut(&mut values).splice(start..start + removed, inserted.iter().cloned()).collect();
        crate::Value::Array(Rc::new(result))
    }))
}

pub fn Data_Array_ST_sortByImpl() -> crate::UnknownType {
    crate::Value::Func3(purust_core::Func3::Static(|compare, from_ordering, array| {
        let cell = purust_st_array(&array);
        {
            let mut values = cell.lock();
            // Comparators are pure. Keep the sort atomic with respect to aliases;
            // the stable sort preserves the source order of equal elements.
            Rc::make_mut(&mut values).sort_by(|left, right| {
                from_ordering.unwrap_func1()(compare.unwrap_func2()(left.clone(), right.clone()))
                    .unwrap_int().cmp(&0)
            });
        }
        array
    }))
}

pub fn Data_Array_ST_toAssocArrayImpl() -> crate::UnknownType {
    crate::Value::Func1(purust_core::Func1::Static(|array| {
        let values = purust_st_array(&array).lock().clone();
        crate::Value::Array(Rc::new(values.iter().enumerate().map(|(index, value)| {
            crate::Value::Record_index_value(perceus_ptr::PerceusPtr::new(crate::Record_index_value {
                index: Some(crate::mk_int(index as i64)), value: Some(value.clone()), ..Default::default()
            }))
        }).collect()))
    }))
}
