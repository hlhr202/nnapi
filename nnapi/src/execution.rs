use std::{
    ffi::c_void,
    ops::{Deref, DerefMut},
    ptr::{null_mut, NonNull},
};

use nnapi_sys::{
    ANeuralNetworksEvent, ANeuralNetworksExecution, ANeuralNetworksExecution_burstCompute,
    ANeuralNetworksExecution_create, ANeuralNetworksExecution_free,
    ANeuralNetworksExecution_setInput, ANeuralNetworksExecution_setOutput,
    ANeuralNetworksExecution_startCompute, ResultCode,
};

use crate::{Burst, Compilation, Event, IntoResult};

pub struct Execution {
    inner: NonNull<ANeuralNetworksExecution>,
}

impl Execution {
    pub fn new(compilation: &mut Compilation) -> crate::Result<Self> {
        let mut execution = null_mut();

        unsafe { ANeuralNetworksExecution_create(&mut **compilation, &mut execution) }
            .into_result()?;

        Ok(Execution {
            inner: NonNull::new(execution).ok_or(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL)?,
        })
    }

    #[inline]
    pub fn set_input<T>(&mut self, input_list_idx: i32, input: &[T]) -> crate::Result<()> {
        unsafe {
            self.set_input_raw(
                input_list_idx,
                input.as_ptr().cast(),
                std::mem::size_of_val(input),
            )
        }
    }

    /// Sets the input from a raw pointer for the neural network execution.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it takes a raw pointer as an argument.
    /// The caller must ensure that the `buffer` is valid and points to a memory
    /// region of at least `length` bytes that must remain valid for the duration
    /// of the execution.
    ///
    /// The `input_list_idx` must be a valid index in the input list of the neural
    /// network execution.
    ///
    /// Improper use of this function can lead to undefined behavior, so extra care
    /// must be taken when calling it.
    #[inline]
    pub unsafe fn set_input_raw(
        &mut self,
        input_list_idx: i32,
        buffer: *const c_void,
        length: usize,
    ) -> crate::Result<()> {
        ANeuralNetworksExecution_setInput(&mut **self, input_list_idx, null_mut(), buffer, length)
            .into_result()
    }

    #[inline]
    pub fn set_output<T>(&mut self, output_list_idx: i32, output: &mut [T]) -> crate::Result<()> {
        unsafe {
            self.set_output_raw(
                output_list_idx,
                output.as_mut_ptr().cast(),
                std::mem::size_of_val(output),
            )
        }
    }

    /// Sets the output from a raw pointer for the neural network execution.
    ///
    /// # Safety
    ///
    /// This function is unsafe because it takes a raw pointer as an argument.
    /// The caller must ensure that the `buffer` is valid and points to a memory
    /// region of at least `length` bytes that must remain valid for the duration
    /// of the execution.
    ///
    /// The `output_list_idx` must be a valid index in the output list of the neural
    /// network execution.
    ///
    /// Improper use of this function can lead to undefined behavior, so extra care
    /// must be taken when calling it.
    #[inline]
    pub unsafe fn set_output_raw(
        &mut self,
        output_list_idx: i32,
        buffer: *mut c_void,
        length: usize,
    ) -> crate::Result<()> {
        unsafe {
            ANeuralNetworksExecution_setOutput(
                &mut **self,
                output_list_idx,
                null_mut(),
                buffer,
                length,
            )
        }
        .into_result()
    }

    #[inline]
    pub fn compute(&mut self) -> crate::Result<Event> {
        let mut end_event: *mut ANeuralNetworksEvent = null_mut();
        unsafe { ANeuralNetworksExecution_startCompute(&mut **self, &mut end_event) }
            .into_result()?;

        Ok(Event {
            inner: NonNull::new(end_event).ok_or(ResultCode::ANEURALNETWORKS_UNEXPECTED_NULL)?,
        })
    }

    #[inline]
    pub fn burst_compute(&mut self, burst: &mut Burst) -> crate::Result<()> {
        unsafe { ANeuralNetworksExecution_burstCompute(&mut **self, &mut **burst) }.into_result()
    }
}

impl Deref for Execution {
    type Target = ANeuralNetworksExecution;

    #[inline]
    fn deref(&self) -> &Self::Target {
        unsafe { self.inner.as_ref() }
    }
}

impl DerefMut for Execution {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { self.inner.as_mut() }
    }
}

impl Drop for Execution {
    fn drop(&mut self) {
        unsafe {
            ANeuralNetworksExecution_free(self.inner.as_mut());
        }
    }
}
