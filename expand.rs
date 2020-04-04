#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
#[macro_use]
extern crate diesel;
use bs_diesel_utils_codegen::BSDieselEnum;
extern crate test;
#[cfg(test)]
#[rustc_test_marker]
pub const test_derive_enum: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("test_derive_enum"),
        ignore: false,
        allow_fail: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(test_derive_enum())),
};
fn test_derive_enum() {
    #[repr(i32)]
    enum Enum {
        A = 1,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Enum {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Enum::A,) => {
                    let mut debug_trait_builder = f.debug_tuple("A");
                    debug_trait_builder.finish()
                }
            }
        }
    }
    impl<DB> diesel::deserialize::FromSql<diesel::sql_types::Integer, DB> for Enum
    where
        DB: diesel::backend::Backend,
        i32: diesel::deserialize::FromSql<diesel::sql_types::Integer, DB>,
    {
        fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
            match i32::from_sql(bytes)? {
                1i32 => Ok(Enum::A),
                x => Err({
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["Unrecognized Enum variant "],
                        &match (&x,) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                    res
                }
                .into()),
            }
        }
    }
    impl<DB> diesel::serialize::ToSql<diesel::sql_types::Integer, DB> for Enum
    where
        DB: diesel::backend::Backend,
        i32: diesel::serialize::ToSql<diesel::sql_types::Integer, DB>,
    {
        fn to_sql<W: std::io::Write>(
            &self,
            out: &mut diesel::serialize::Output<W, DB>,
        ) -> diesel::serialize::Result {
            let discriminant = match *self {
                Enum::A => 1i32,
            };
            discriminant.to_sql(out)
        }
    }
    impl diesel::expression::AsExpression<diesel::sql_types::Integer> for Enum {
        type Expression =
            <i32 as diesel::expression::AsExpression<diesel::sql_types::Integer>>::Expression;
        fn as_expression(self) -> Self::Expression {
            <i32 as diesel::expression::AsExpression<diesel::sql_types::Integer>>::as_expression(
                self as i32,
            )
        }
    }
    #[allow(non_snake_case, unused_extern_crates, unused_imports)]
    fn _impl_from_sql_row_for_enum() {
        extern crate std;
        use diesel;
        use diesel::deserialize::{self, FromSql, FromSqlRow, Queryable};
        impl<__ST, __DB> FromSqlRow<__ST, __DB> for Enum
        where
            __DB: diesel::backend::Backend,
            Self: FromSql<__ST, __DB>,
        {
            fn build_from_row<R: diesel::row::Row<__DB>>(row: &mut R) -> deserialize::Result<Self> {
                FromSql::<__ST, __DB>::from_sql(row.take())
            }
        }
        impl<__ST, __DB> Queryable<__ST, __DB> for Enum
        where
            __DB: diesel::backend::Backend,
            Self: FromSql<__ST, __DB>,
        {
            type Row = Self;
            fn build(row: Self::Row) -> Self {
                row
            }
        }
    }
}
#[main]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&test_derive_enum])
}
