use proc_macro2::TokenStream;
use proc_macro_error::{abort, abort_if_dirty, emit_error, ResultExt};
use quote::quote;

pub fn derive_enum(input: proc_macro::TokenStream) -> TokenStream {
  let input = TokenStream::from(input);
  let input: syn::DeriveInput = syn::parse2(input).unwrap_or_abort();

  let ident = &input.ident;

  // check if #[repr(i32)] is set
  if input.attrs.is_empty()
    || !input.attrs.iter().any(|a| {
      if let syn::AttrStyle::Outer = a.style {
        if let Some(ty) = a.parse_args::<syn::Ident>().ok() {
          if ty == "i32" {
            return true;
          } else {
            emit_error!(ty, "#[derive(BSDieselEnum)]: expected i32, found {}.", ty);
          }
        }
      }
      false
    })
  {
    emit_error!(
      input,
      "#[derive(BSDieselEnum)]: add attribute #[repr(i32)]."
    )
  }
  abort_if_dirty();

  let data = if let syn::Data::Enum(ref data) = input.data {
    data
  } else {
    abort!(input, "#[derive(BSDieselEnum)]: not an enum type.")
  };

  if data.variants.is_empty() {
    abort!(
      input,
      "#[derive(BSDieselEnum)]: unsupported zero-variant enum."
    )
  }

  let variants: Vec<_> = data
    .variants
    .iter()
    .filter_map(VariantInfo::from_variant)
    .collect();

  abort_if_dirty();

  let from_sql_arms: Vec<_> = variants.iter().map(|v| v.as_from_sql_arm(ident)).collect();
  let to_sql_arms: Vec<_> = variants.iter().map(|v| v.as_to_sql_arm(ident)).collect();
  let err_fmt = format!("Unrecognized {} variant {{}}", ident);

  quote! {
    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Integer, DB> for #ident
    where
      DB: diesel::backend::Backend,
      i32: diesel::deserialize::FromSql<diesel::sql_types::Integer, DB>,
    {
      fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        match i32::from_sql(bytes)? {
          #(#from_sql_arms,)*
          x => Err(format!(#err_fmt, x).into()),
        }
      }
    }

    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Integer, DB> for #ident
    where
      DB: diesel::backend::Backend,
      i32: diesel::serialize::ToSql<diesel::sql_types::Integer, DB>,
    {
      fn to_sql<W: std::io::Write>(
        &self,
        out: &mut diesel::serialize::Output<W, DB>,
      ) -> diesel::serialize::Result {
        let discriminant = match *self {
          #(#to_sql_arms,)*
        };
        discriminant.to_sql(out)
      }
    }

    impl diesel::expression::AsExpression<diesel::sql_types::Integer> for #ident {
      type Expression = <i32 as diesel::expression::AsExpression<diesel::sql_types::Integer>>::Expression;

      fn as_expression(self) -> Self::Expression {
        <i32 as diesel::expression::AsExpression<diesel::sql_types::Integer>>::as_expression(self as i32)
      }
    }

    impl<'a> diesel::expression::AsExpression<diesel::sql_types::Integer> for &'a #ident {
      type Expression = <i32 as diesel::expression::AsExpression<diesel::sql_types::Integer>>::Expression;

      fn as_expression(self) -> Self::Expression {
        let discriminant = match *self {
          #(#to_sql_arms,)*
        };
        <i32 as diesel::expression::AsExpression<diesel::sql_types::Integer>>::as_expression(discriminant)
      }
    }

    impl<__ST, __DB> diesel::deserialize::FromSqlRow<__ST, __DB> for #ident
    where
        __DB: diesel::backend::Backend,
        Self: diesel::deserialize::FromSql<__ST, __DB>,
    {
        fn build_from_row<R: diesel::row::Row<__DB>>(row: &mut R) -> diesel::deserialize::Result<Self> {
          diesel::deserialize::FromSql::<__ST, __DB>::from_sql(row.take())
        }
    }
    impl<__ST, __DB> diesel::deserialize::Queryable<__ST, __DB> for #ident
    where
        __DB: diesel::backend::Backend,
        Self: diesel::deserialize::FromSql<__ST, __DB>,
    {
        type Row = Self;
        fn build(row: Self::Row) -> Self {
            row
        }
    }
  }
}

struct VariantInfo<'a> {
  ident: &'a syn::Ident,
  discriminant: i32,
}

impl<'a> VariantInfo<'a> {
  fn from_variant(variant: &'a syn::Variant) -> Option<Self> {
    if let syn::Variant {
      ref ident,
      discriminant:
        Some((
          _,
          syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Int(ref lit_int),
            ..
          }),
        )),
      ..
    } = variant
    {
      let discriminant = lit_int.base10_parse::<i32>().unwrap_or_abort();
      return Some(VariantInfo {
        ident,
        discriminant,
      });
    } else {
      emit_error!(variant, "BSDieselEnum: Variant must have discriminant.");
    }
    None
  }

  fn as_from_sql_arm(&self, enum_ident: &syn::Ident) -> TokenStream {
    let VariantInfo {
      ident,
      discriminant,
    } = self;
    quote! {
      #discriminant => Ok(#enum_ident::#ident)
    }
  }

  fn as_to_sql_arm(&self, enum_ident: &syn::Ident) -> TokenStream {
    let VariantInfo {
      ident,
      discriminant,
    } = self;
    quote! {
      #enum_ident::#ident => #discriminant
    }
  }
}
