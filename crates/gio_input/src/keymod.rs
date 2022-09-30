use crate::keycode::keymod::*;

#[derive(Default)]
pub struct Keymod {
    state: usize,
}

impl Keymod {
    pub fn set_shift(&mut self) -> &mut Self {
        self.state |= SHIFT;
        self
    }
    pub fn set_control(&mut self) -> &mut Self {
        self.state |= CONTROL;
        self
    }

    pub fn set_alt(&mut self) -> &mut Self {
        self.state |= ALT;
        self
    }
    pub fn set_capslock(&mut self) -> &mut Self {
        self.state |= CAPSLOCK;
        self
    }
    pub fn set_system(&mut self) -> &mut Self {
        self.state |= SYSTEM;
        self
    }
    pub fn set_lshift(&mut self) -> &mut Self {
        self.state |= LSHIFT;
        self
    }
    pub fn set_rshift(&mut self) -> &mut Self {
        self.state |= RSHIFT;
        self
    }
    pub fn set_lcontrol(&mut self) -> &mut Self {
        self.state |= LCONTROL;
        self
    }
    pub fn set_rcontrol(&mut self) -> &mut Self {
        self.state |= RCONTROL;
        self
    }
    pub fn set_lalt(&mut self) -> &mut Self {
        self.state |= LALT;
        self
    }
    pub fn set_ralt(&mut self) -> &mut Self {
        self.state |= RALT;
        self
    }
    pub fn set_lsystem(&mut self) -> &mut Self {
        self.state |= LSYSTEM;
        self
    }
    pub fn set_rsystem(&mut self) -> &mut Self {
        self.state |= RSYSTEM;
        self
    }

    pub fn unset_shift(&mut self) -> &mut Self {
        self.state &= SHIFT;
        self
    }
    pub fn unset_control(&mut self) -> &mut Self {
        self.state &= CONTROL;
        self
    }

    pub fn unset_alt(&mut self) -> &mut Self {
        self.state &= ALT;
        self
    }
    pub fn unset_capslock(&mut self) -> &mut Self {
        self.state &= CAPSLOCK;
        self
    }
    pub fn unset_system(&mut self) -> &mut Self {
        self.state &= SYSTEM;
        self
    }
    pub fn unset_lshift(&mut self) -> &mut Self {
        self.state &= LSHIFT;
        self
    }
    pub fn unset_rshift(&mut self) -> &mut Self {
        self.state &= RSHIFT;
        self
    }
    pub fn unset_lcontrol(&mut self) -> &mut Self {
        self.state &= LCONTROL;
        self
    }
    pub fn unset_rcontrol(&mut self) -> &mut Self {
        self.state &= RCONTROL;
        self
    }
    pub fn unset_lalt(&mut self) -> &mut Self {
        self.state &= LALT;
        self
    }
    pub fn unset_ralt(&mut self) -> &mut Self {
        self.state &= RALT;
        self
    }
    pub fn unset_lsystem(&mut self) -> &mut Self {
        self.state &= LSYSTEM;
        self
    }
    pub fn unset_rsystem(&mut self) -> &mut Self {
        self.state &= RSYSTEM;
        self
    }

    pub fn is_shift_set(&self) -> bool {
        (self.state & SHIFT) != 0
    }
    pub fn is_control_set(&self) -> bool {
        (self.state & CONTROL) != 0
    }

    pub fn is_alt_set(&self) -> bool {
        (self.state & ALT) != 0
    }
    pub fn is_capslock_set(&self) -> bool {
        (self.state & CAPSLOCK) != 0
    }
    pub fn is_system_set(&self) -> bool {
        (self.state & SYSTEM) != 0
    }
    pub fn is_lshift_set(&self) -> bool {
        (self.state & LSHIFT) != 0
    }
    pub fn is_rshift_set(&self) -> bool {
        (self.state & RSHIFT) != 0
    }
    pub fn is_lcontrol_set(&self) -> bool {
        (self.state & LCONTROL) != 0
    }
    pub fn is_rcontrol_set(&self) -> bool {
        (self.state & RCONTROL) != 0
    }
    pub fn is_lalt_set(&self) -> bool {
        (self.state & LALT) != 0
    }
    pub fn is_ralt_set(&self) -> bool {
        (self.state & RALT) != 0
    }
    pub fn is_lsystem_set(&self) -> bool {
        (self.state & LSYSTEM) != 0
    }
    pub fn is_rsystem_set(&self) -> bool {
        (self.state & RSYSTEM) != 0
    }
}
