mod args;

use darling::*;
use proc_macro2::TokenStream as TokenStream2;
use quote::*;
use syn::*;

// impl `MyTrait` for `struct` or `enum`
pub fn impl_my_trait(ast: DeriveInput) -> TokenStream2 {
    let args = args::TypeArgs::from_derive_input(&ast).unwrap();
    match &args.data {
        ast::Data::Struct(ref field_args) => self::impl_visit_struct(&args, field_args),
        ast::Data::Enum(ref variants) => self::impl_visit_enum(&args, variants),
    }
}

/// Create generics with additional boundaries for every field type
fn create_impl_generics(args: &args::TypeArgs) -> Generics {
    let mut generics = args.generics.clone();

    generics.make_where_clause().predicates.extend(
        args.all_fields()
            .iter()
            .filter(|f| !f.skip)
            .map(|f| &f.ty)
            .map::<WherePredicate, _>(|ty| parse_quote! { #ty: MyTrait }),
    );

    generics
}

fn impl_visit_struct(
    args: &args::TypeArgs,
    field_args: &ast::Fields<args::FieldArgs>,
) -> TokenStream2 {
    let ty_ident = &args.ident;

    let generics = self::create_impl_generics(args);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics MyTrait for #ty_ident #ty_generics #where_clause {
            fn my_func(&mut self) {
                // todo!
            }
        }
    }
}

/// impl `Visit` for `enum`
fn impl_visit_enum(args: &args::TypeArgs, variants: &[args::VariantArgs]) -> TokenStream2 {
    let ty_ident = &args.ident;

    let generics = self::create_impl_generics(args);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    quote! {
        impl #impl_generics MyTrait for #ty_ident #ty_generics #where_clause {
            fn my_func(&mut self) {
                // todo!
            }
        }
    }
}
