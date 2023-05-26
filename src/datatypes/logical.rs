use std::{marker::PhantomData, sync::Arc};

use crate::{
    datatypes::{DaftLogicalType, DateType, Field},
    error::DaftResult,
};

use super::{DataArray, DataType, EmbeddingType, FixedShapeImageType, ImageType};

pub struct LogicalArray<L: DaftLogicalType> {
    pub field: Arc<Field>,
    pub physical: DataArray<L::PhysicalType>,
    marker_: PhantomData<L>,
}

impl<L: DaftLogicalType + 'static> LogicalArray<L> {
    pub fn new<F: Into<Arc<Field>>>(field: F, physical: DataArray<L::PhysicalType>) -> Self {
        let field = field.into();
        assert!(
            field.dtype.is_logical(),
            "Can only construct Logical Arrays on Logical Types, got {}",
            field.dtype
        );
        assert_eq!(
            physical.data_type(),
            &field.dtype.to_physical(),
            "Expected {} for Physical Array, got {}",
            &field.dtype.to_physical(),
            physical.data_type()
        );

        LogicalArray {
            physical,
            field,
            marker_: PhantomData,
        }
    }

    pub fn empty(name: &str, dtype: &DataType) -> Self {
        let field = Field::new(name, dtype.clone());
        Self::new(field, DataArray::empty(name, &dtype.to_physical()))
    }

    pub fn name(&self) -> &str {
        self.field.name.as_ref()
    }

    pub fn rename(&self, name: &str) -> Self {
        let new_field = self.field.rename(name);
        let new_array = self.physical.rename(name);
        Self::new(new_field, new_array)
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn logical_type(&self) -> &DataType {
        &self.field.dtype
    }

    pub fn physical_type(&self) -> &DataType {
        self.physical.data_type()
    }

    pub fn len(&self) -> usize {
        self.physical.len()
    }

    pub fn size_bytes(&self) -> DaftResult<usize> {
        self.physical.size_bytes()
    }

    pub fn slice(&self, start: usize, end: usize) -> DaftResult<Self> {
        let new_array = self.physical.slice(start, end)?;
        Ok(Self::new(self.field.clone(), new_array))
    }

    pub fn head(&self, num: usize) -> DaftResult<Self> {
        self.slice(0, num)
    }

    pub fn concat(arrays: &[&Self]) -> DaftResult<Self> {
        if arrays.is_empty() {
            return Err(crate::error::DaftError::ValueError(
                "Need at least 1 logical array to concat".to_string(),
            ));
        }
        let physicals: Vec<_> = arrays.iter().map(|a| &a.physical).collect();
        let concatd = DataArray::<L::PhysicalType>::concat(physicals.as_slice())?;
        Ok(Self::new(arrays.first().unwrap().field.clone(), concatd))
    }
}

pub type DateArray = LogicalArray<DateType>;
pub type EmbeddingArray = LogicalArray<EmbeddingType>;
pub type ImageArray = LogicalArray<ImageType>;
pub type FixedShapeImageArray = LogicalArray<FixedShapeImageType>;
