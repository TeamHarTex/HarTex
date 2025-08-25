/*
 * SPDX-License-Identifier: AGPL-3.0-only
 *
 * This file is part of HarTex.
 *
 * HarTex
 * Copyright (c) 2021-2025 HarTex Project Developers
 *
 * HarTex is free software; you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as published by
 * the Free Software Foundation; either version 3 of the License, or
 * (at your option) any later version.
 *
 * HarTex is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License along
 * with HarTex. If not, see <https://www.gnu.org/licenses/>.
 */

#![allow(clippy::items_after_statements)]

use std::{
    cell::UnsafeCell,
    marker::PhantomData,
    mem::ManuallyDrop,
    panic::{RefUnwindSafe, UnwindSafe},
    pin::Pin,
    ptr::drop_in_place,
    sync::atomic::{AtomicBool, Ordering},
};

use tokio::sync::{Semaphore, SemaphorePermit};

enum LazyResultUninitializedData<R, F> {
    Function(ManuallyDrop<F>),
    Future(ManuallyDrop<R>),
}

union LazyResultData<T, R, F> {
    initialized: ManuallyDrop<Option<T>>,
    uninitialized: ManuallyDrop<LazyResultUninitializedData<R, F>>,
}

pub struct LazyResult<T, E, R = Pin<Box<dyn Future<Output = Result<T, E>> + Send>>, F = fn() -> R> {
    is_set: AtomicBool,
    val: UnsafeCell<LazyResultData<T, R, F>>,
    semaphore: Semaphore,
    phantom: PhantomData<E>,
}

impl<T, E, R, F> LazyResult<T, E, R, F> {
    #[inline]
    #[must_use]
    pub const fn new(initializer: F) -> Self {
        Self {
            is_set: AtomicBool::new(false),
            val: UnsafeCell::new(LazyResultData {
                uninitialized: ManuallyDrop::new(LazyResultUninitializedData::Function(
                    ManuallyDrop::new(initializer),
                )),
            }),
            semaphore: Semaphore::const_new(1),
            phantom: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    fn initialized(&self) -> bool {
        self.is_set.load(Ordering::Acquire)
    }

    #[inline]
    #[must_use]
    unsafe fn get_unchecked(&self) -> Option<&T> {
        unsafe { (*self.val.get()).initialized.as_ref() }
    }

    pub fn get(&self) -> Option<&T> {
        if self.initialized() {
            unsafe { self.get_unchecked() }
        } else {
            None
        }
    }
}

impl<T, E, R, F> LazyResult<T, E, R, F>
where
    R: Future<Output = Result<T, E>> + Unpin,
    F: Fn() -> R,
{
    #[allow(clippy::missing_panics_doc)]
    pub async fn force(&self) -> &T {
        if !self.initialized() {
            unsafe {
                Pin::new_unchecked(self).initialize().await;
            }
        }

        (unsafe { self.get_unchecked() })
            .unwrap_or_else(|| panic!("LazyResult has been poisoned due to initializer panic"))
    }
}

impl<T, E, R, F> LazyResult<T, E, R, F>
where
    R: Future<Output = Result<T, E>> + Unpin,
    F: Fn() -> R,
{
    #[cold]
    async fn initialize(self: Pin<&Self>) {
        let Ok(permit) = self.semaphore.acquire().await else {
            return;
        };

        enum InitializationStage<T> {
            Initializing,
            Initialized(ManuallyDrop<Option<T>>),
            Errored,
        }

        struct InitializationOnDrop<'a, 'permit, T, E, R, F> {
            lazy: Pin<&'a LazyResult<T, E, R, F>>,
            val: InitializationStage<T>,
            permit: ManuallyDrop<SemaphorePermit<'permit>>,
        }

        impl<T, E, R, F> Drop for InitializationOnDrop<'_, '_, T, E, R, F> {
            fn drop(&mut self) {
                match self.val {
                    InitializationStage::Initializing | InitializationStage::Errored => unsafe {
                        drop(ManuallyDrop::take(&mut self.permit));
                    },
                    InitializationStage::Initialized(ref mut value) => {
                        unsafe {
                            (*self.lazy.val.get()).initialized =
                                ManuallyDrop::new(ManuallyDrop::take(value));
                        }

                        self.lazy.is_set.store(true, Ordering::Release);
                        self.lazy.semaphore.close();
                    }
                }
            }
        }

        let uninit_data = unsafe {
            let ptr = self.val.get();
            &mut (*ptr).uninitialized
        };

        let mut iod = InitializationOnDrop {
            lazy: self,
            val: InitializationStage::Initialized(ManuallyDrop::new(None)),
            permit: ManuallyDrop::new(permit),
        };

        struct FutureMutPtr<R>(*mut R);
        unsafe impl<R> Send for FutureMutPtr<R> {}
        unsafe impl<R> Sync for FutureMutPtr<R> {}

        let futptr = {
            let mutptr: *mut ManuallyDrop<R> = match &mut **uninit_data {
                LazyResultUninitializedData::Function(f) => {
                    let f = unsafe { ManuallyDrop::take(f) };
                    let fut = f();

                    **uninit_data = LazyResultUninitializedData::Future(ManuallyDrop::new(fut));

                    let LazyResultUninitializedData::Future(fut) = &mut **uninit_data else {
                        unreachable!()
                    };
                    fut
                }
                LazyResultUninitializedData::Future(fut) => fut,
            };

            FutureMutPtr(mutptr.cast())
        };

        let fut: Pin<&mut R> = unsafe { Pin::new_unchecked(&mut *futptr.0) };
        iod.val = InitializationStage::Initializing;

        let result = fut.await;

        if result.is_err() {
            iod.val = InitializationStage::Errored;
        } else {
            iod.val = InitializationStage::Initialized(ManuallyDrop::new(result.ok()));
        }

        unsafe { drop_in_place(futptr.0) };
        drop(iod);
    }
}

impl<T, E, R, F> Drop for LazyResult<T, E, R, F> {
    fn drop(&mut self) {
        if self.initialized() {
            unsafe { ManuallyDrop::drop(&mut self.val.get_mut().initialized) };
        } else {
            unsafe {
                match &mut *self.val.get_mut().uninitialized {
                    LazyResultUninitializedData::Future(fut) => ManuallyDrop::drop(fut),
                    LazyResultUninitializedData::Function(func) => ManuallyDrop::drop(func),
                }
            }
        }
    }
}

unsafe impl<T, E, R, F> Send for LazyResult<T, E, R, F> {}
unsafe impl<T, E, R, F> Sync for LazyResult<T, E, R, F> {}

impl<T: UnwindSafe, E: UnwindSafe, R: UnwindSafe, F: UnwindSafe> UnwindSafe
    for LazyResult<T, E, R, F>
{
}

impl<T: UnwindSafe + RefUnwindSafe, E: UnwindSafe, R: UnwindSafe, F: UnwindSafe> RefUnwindSafe
    for LazyResult<T, E, R, F>
{
}

impl<T: Unpin, E: Unpin, R: Unpin, F> Unpin for LazyResult<T, E, R, F> {}

impl<'a, T, E, R, F> IntoFuture for &'a LazyResult<T, E, R, F>
where
    R: Future<Output = Result<T, E>> + Unpin,
    F: Fn() -> R,
{
    type Output = &'a T;

    type IntoFuture = impl Future<Output = Self::Output>;

    #[inline]
    fn into_future(self) -> Self::IntoFuture {
        self.force()
    }
}
