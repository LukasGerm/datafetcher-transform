/**
 * https://astexplorer.net/#/gist/e931bbd1350f930ec594e101d6a9a2b7/c992bb3545f0749cf34e4a90e3fdd734894f674e outcome
 */

#![allow(clippy::not_unsafe_ptr_arg_deref)]
use swc_core::{
    ecma::{ast::Program, visit::FoldWith},
    plugin::{plugin_transform, proxies::TransformPluginProgramMetadata},
};

#[plugin_transform]
fn swc_plugin(program: Program, data: TransformPluginProgramMetadata) -> Program {
    let config = serde_json::from_str::<Option<datafetcher_transform::Config>>(
        &data
            .get_transform_plugin_config()
            .expect("failed to get plugin config for react-remove-properties"),
    )
    .expect("invalid packages")
    .unwrap_or(datafetcher_transform::Config::All(true));

    program.fold_with(&mut datafetcher_transform::datafetcher_transform(
        config,
    ))
}