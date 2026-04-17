use std::{future::Future, pin::Pin};

use crate::runtime::scope::ScopeStack;

use super::super::{output::Output, ExecutionError};
use super::FunctionLibrary;

pub trait BuiltIn: Sync + Send {
    fn call<'env, 'stack>(
        &'stack self,
        lib: &'stack FunctionLibrary,
        scope: &'stack mut ScopeStack<'env>,
        args: Vec<Output>,
    ) -> Pin<Box<dyn Future<Output = Result<Output, ExecutionError>> + Send + 'stack>>;
}

impl<F> BuiltIn for F
where
    F: Sync
        + Send
        + for<'env, 'stack> Fn(
            &'stack FunctionLibrary,
            &'stack mut ScopeStack<'env>,
            Vec<Output>,
        ) -> Pin<
            Box<dyn Future<Output = Result<Output, ExecutionError>> + Send + 'stack>,
        >,
{
    fn call<'env, 'stack>(
        &'stack self,
        lib: &'stack FunctionLibrary,
        scope: &'stack mut ScopeStack<'env>,
        args: Vec<Output>,
    ) -> Pin<Box<dyn Future<Output = Result<Output, ExecutionError>> + Send + 'stack>> {
        (self)(lib, scope, args)
    }
}

pub fn builtin_state<Ctx, F>(ctx: Ctx, f: F) -> Box<dyn BuiltIn>
where
    Ctx: Clone + Send + Sync + 'static,
    F: Sync
        + Send
        + for<'env, 'stack> Fn(
            Ctx,
            &'stack FunctionLibrary,
            &'stack mut ScopeStack<'env>,
            Vec<Output>,
        ) -> Pin<
            Box<dyn Future<Output = Result<Output, ExecutionError>> + Send + 'stack>,
        > + 'static,
{
    struct Wrapper<Ctx, F> {
        ctx: Ctx,
        f: F,
    }

    impl<Ctx, F> BuiltIn for Wrapper<Ctx, F>
    where
        Ctx: Clone + Send + Sync + 'static,
        F: Sync
            + Send
            + for<'env, 'stack> Fn(
                Ctx,
                &'stack FunctionLibrary,
                &'stack mut ScopeStack<'env>,
                Vec<Output>,
            ) -> Pin<
                Box<dyn Future<Output = Result<Output, ExecutionError>> + Send + 'stack>,
            > + 'static,
    {
        fn call<'env, 'stack>(
            &'stack self,
            lib: &'stack FunctionLibrary,
            scope: &'stack mut ScopeStack<'env>,
            args: Vec<Output>,
        ) -> Pin<Box<dyn Future<Output = Result<Output, ExecutionError>> + Send + 'stack>> {
            (self.f)(self.ctx.clone(), lib, scope, args)
        }
    }

    Box::new(Wrapper { ctx, f })
}
