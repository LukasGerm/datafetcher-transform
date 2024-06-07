#![allow(clippy::not_unsafe_ptr_arg_deref)]

use swc_common::{SourceMapper, Spanned};
use swc_core::{
    common::FileName,
    ecma::{ast::Program, visit::VisitMutWith},
    plugin::{
        metadata::TransformPluginMetadataContextKind,
        plugin_transform,
        proxies::{PluginCommentsProxy, TransformPluginProgramMetadata},
    },
};

#[plugin_transform]
fn datafetcher_transform_ast(mut program: Program, data: TransformPluginProgramMetadata) -> Program {
    println!("datafetcher_transform_ast");

    program

}
