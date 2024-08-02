use std::collections::HashMap;

use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};
/**
 * https://astexplorer.net/#/gist/e931bbd1350f930ec594e101d6a9a2b7/c992bb3545f0749cf34e4a90e3fdd734894f674e outcome
 */
use swc_core::{
    common::Spanned,
    ecma::{
        ast::*,
        transforms::testing::test_inline,
        visit::{as_folder, FoldWith, VisitMut, VisitMutWith},
    },
};

const ALLOWED_METHODS: [&str; 4] = [
    "useGrpcRequest",
    "useLegacyGrpcRequest",
    "useUpdateGrpcRequestCache",
    "useLegacyGrpcRequestCallback",
];

const NEEDS_UNIQUE_IDENTIFIER: [&str; 3] = [
    "useGrpcRequest",
    "useLegacyGrpcRequest",
    "useUpdateGrpcRequestCache",
];

pub struct TransformVisitor {
    imports: HashMap<String, ImportDecl>,
}

impl VisitMut for TransformVisitor {
    fn visit_mut_import_decl(&mut self, n: &mut ImportDecl) {
        for specifier in &n.specifiers {
            if let ImportSpecifier::Named(named_specifier) = specifier {
                let import_property = named_specifier.local.sym.to_string();
                println!("import_property: {}", import_property);
                self.imports.insert(import_property.clone(), n.clone());
            }
        }
    }
}
/// An example plugin function with macro support.
/// `plugin_transform` macro interop pointers into deserialized structs, as well
/// as returning ptr back to host.
///
/// It is possible to opt out from macro by writing transform fn manually
/// if plugin need to handle low-level ptr directly via
/// `__transform_plugin_process_impl(
///     ast_ptr: *const u8, ast_ptr_len: i32,
///     unresolved_mark: u32, should_enable_comments_proxy: i32) ->
///     i32 /*  0 for success, fail otherwise.
///             Note this is only for internal pointer interop result,
///             not actual transform result */`
///
/// This requires manual handling of serialization / deserialization from ptrs.
/// Refer swc_plugin_macro to see how does it work internally.
#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor {
        imports: HashMap::new(),
    }))
}

// An example to test plugin transform.
// Recommended strategy to test plugin's transform is verify
// the Visitor's behavior, instead of trying to run `process_transform` with mocks
// unless explicitly required to do so.
test_inline!(
    Default::default(),
    |_| as_folder(TransformVisitor {
        imports: HashMap::new()
    }),
    boo,
    // Input codes
    r#"console.log("transform");"#,
    // Output codes after transformed with plugin
    r#"console.log("transform");"#
);
