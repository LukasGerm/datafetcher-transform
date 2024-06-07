#![allow(clippy::not_unsafe_ptr_arg_deref)]

/**
 * https://astexplorer.net/#/gist/e931bbd1350f930ec594e101d6a9a2b7/c992bb3545f0749cf34e4a90e3fdd734894f674e outcome
 */

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
