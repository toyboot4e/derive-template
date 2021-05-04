use darling::*;
use syn::*;

#[derive(FromDeriveInput)]
#[darling(attributes(my_derive), supports(struct_any, enum_any))]
pub struct TypeArgs {
    pub ident: Ident,
    pub generics: Generics,
    pub data: ast::Data<VariantArgs, FieldArgs>,
}

/// Parsed from struct's or enum variant's field
#[derive(FromField, Clone)]
#[darling(attributes(my_derive))]
pub struct FieldArgs {
    pub ident: Option<Ident>,
    pub ty: Type,
    // ---
    /// `#[my_derive(skip)]`
    #[darling(default)]
    pub skip: bool,
}

#[derive(FromVariant)]
#[darling(attributes(my_derive))]
pub struct VariantArgs {
    pub ident: Ident,
    pub fields: ast::Fields<FieldArgs>,
}

impl TypeArgs {
    /// Enumerates all the fields of a struct or enum variants
    pub fn all_fields(&self) -> Vec<self::FieldArgs> {
        match &self.data {
            ast::Data::Struct(field_args) => field_args.fields.clone(),
            ast::Data::Enum(variants) => variants
                .iter()
                .flat_map(|variant| variant.fields.clone().into_iter())
                .collect::<Vec<_>>(),
        }
    }
}
