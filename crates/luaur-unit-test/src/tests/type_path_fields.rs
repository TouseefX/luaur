use luaur_analysis::enums::pack_field::PackField;
use luaur_analysis::enums::type_field::TypeField;
use luaur_analysis::methods::path_builder_args::PathBuilderArgs;
use luaur_analysis::methods::path_builder_build::PathBuilderBuild;
use luaur_analysis::methods::path_builder_lb::PathBuilderLb;
use luaur_analysis::methods::path_builder_mt::PathBuilderMt;
use luaur_analysis::methods::path_builder_negated::PathBuilderNegated;
use luaur_analysis::methods::path_builder_rets::PathBuilderRets;
use luaur_analysis::methods::path_builder_tail::PathBuilderTail;
use luaur_analysis::methods::path_builder_ub::PathBuilderUb;
use luaur_analysis::methods::path_builder_variadic::PathBuilderVariadic;
use luaur_analysis::records::path::Path;
use luaur_analysis::records::path_builder::PathBuilder;
use luaur_analysis::type_aliases::component::Component;

#[test]
fn type_path_fields() {
    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.mt().build(),
        Path::from_component(Component::TypeField(TypeField::Metatable))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.lb().build(),
        Path::from_component(Component::TypeField(TypeField::LowerBound))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.ub().build(),
        Path::from_component(Component::TypeField(TypeField::UpperBound))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.index_key().build(),
        Path::from_component(Component::TypeField(TypeField::IndexLookup))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.index_value().build(),
        Path::from_component(Component::TypeField(TypeField::IndexResult))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.negated().build(),
        Path::from_component(Component::TypeField(TypeField::Negated))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.variadic().build(),
        Path::from_component(Component::TypeField(TypeField::Variadic))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.args().build(),
        Path::from_component(Component::PackField(PackField::Arguments))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.rets().build(),
        Path::from_component(Component::PackField(PackField::Returns))
    );

    let mut builder = PathBuilder::new();
    assert_eq!(
        builder.tail().build(),
        Path::from_component(Component::PackField(PackField::Tail))
    );
}
