use crate::enums::condition_x_64::ConditionX64;
use crate::enums::size_x_64::SizeX64;
use crate::macros::codegen_assert::CODEGEN_ASSERT;
use crate::records::assembly_builder_x_64::AssemblyBuilderX64;
use crate::records::label::Label;
use crate::records::register_x_64::RegisterX64;

pub fn convert_number_to_index_or_jump(
    build: &mut AssemblyBuilderX64,
    tmp: RegisterX64,
    numd: RegisterX64,
    numi: RegisterX64,
    label: &mut Label,
) {
    CODEGEN_ASSERT!(numi.size() == SizeX64::dword);

    build.vcvttsd2si(numi.into(), numd.into());
    build.vcvtsi2sd(tmp.into(), numd.into(), numi.into());
    build.vucomisd(tmp.into(), numd.into());
    build.jcc(ConditionX64::NotZero, label);
}
