use crate::enums::ir_cmd::IrCmd;
use crate::records::ir_value_location_tracking::IrValueLocationTracking;

impl IrValueLocationTracking {
    pub fn can_be_rematerialized(&self, cmd: IrCmd) -> bool {
        cmd == IrCmd::UINT_TO_NUM || cmd == IrCmd::INT_TO_NUM
    }
}
