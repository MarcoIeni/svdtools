use crate::patch::yaml::yaml_parser as yml;
use svd_parser as svd;

pub fn modify_cpu(dest: &mut Option<svd::Cpu>, src: &yml::Cpu) {
    match dest {
        None => {
            unimplemented!("cannot instanciate a cpu struct at the moment, pending until https://github.com/rust-embedded/svd/pull/101/ is merged");

            // *dest = Some(svd::Cpu {
            //     name: src.name.clone().unwrap_or_default(),
            //     revision: src.revision.clone().unwrap_or_default(),
            //     endian: svd::Endian::Other,
            //     // endian: {
            //     //     match src.endian {
            //     //         Some(src_endian) => src_endian.to_svd(),
            //     //         None => svd::Endian::Other,
            //     //     }
            //     // },
            //     mpu_present: src.mpu_present.unwrap_or_default(),
            //     fpu_present: src.fpu_present.unwrap_or_default(),
            //     nvic_priority_bits: src.nvic_prio_bits.unwrap_or_default(),
            //     has_vendor_systick: src.vendor_systick_config.unwrap_or_default(),
            //     _extensible: (),
            // });
        }
        Some(dest) => {
            modify_if_some(&mut dest.name, &src.name);
            modify_if_some(&mut dest.revision, &src.revision);
            modify_if_some(&mut dest.endian, &src.endian);
            modify_if_some(&mut dest.mpu_present, &src.mpu_present);
            modify_if_some(&mut dest.fpu_present, &src.fpu_present);
            modify_if_some(&mut dest.nvic_priority_bits, &src.nvic_prio_bits);
            modify_if_some(&mut dest.has_vendor_systick, &src.vendor_systick_config);
        }
    };
}

impl yml::RegisterProperties {
    pub fn modify(&self, dest: &mut svd::RegisterProperties) {
        modify_option(&mut dest.size, &self.size);
        modify_option(&mut dest.reset_value, &self.reset_value);
        modify_option(&mut dest.reset_mask, &self.reset_mask);
        modify_access(&mut dest.access, &self.access);
    }
}

fn modify_access(dest: &mut Option<svd::Access>, src: &Option<yml::Access>) {
    if let Some(src) = src {
        *dest = Some(src.to_svd());
    }
}

impl yml::Device {
    pub fn modify(&self, dest: &mut svd::Device) {
        modify_if_some(&mut dest.name, &self.name);
        modify_option(&mut dest.version, &self.version);
        modify_option(&mut dest.description, &self.description);
        modify_option(&mut dest.address_unit_bits, &self.address_unit_bits);
        modify_option(&mut dest.width, &self.width);

        // edit cpu
        if let Some(new_cpu) = &self.cpu {
            modify_cpu(&mut dest.cpu, new_cpu);
        }

        self.default_register_properties
            .modify(&mut dest.default_register_properties);

        // edit peripherals
        for (periph_name, new_periph) in &self.peripherals {
            // TODO At the moment we ignore addressBlocks feature since it is
            //      never used in the stm32-rs repository. Is it ok?
            let mut old_periph = get_peripheral_mut(dest, periph_name)
                .expect("peripheral {} of _modify not found in svd");
            new_periph.modify(&mut old_periph);
        }
    }
}

fn get_peripheral_mut<'a>(
    svd: &'a mut svd::Device,
    peripheral_name: &str,
) -> Option<&'a mut svd::Peripheral> {
    svd.peripherals
        .iter_mut()
        .filter(|p| p.name == peripheral_name)
        .next()
}

impl yml::Peripheral {
    pub fn modify(&self, dest: &mut svd::Peripheral) {
        modify_if_some(&mut dest.name, &self.name);
        modify_option(&mut dest.version, &self.body.version);
        modify_option(&mut dest.display_name, &self.body.display_name);
        modify_option(&mut dest.description, &self.body.description);
        modify_option(&mut dest.group_name, &self.body.group_name);
        modify_if_some(&mut dest.base_address, &self.body.base_address);
        if let Some(addr_block) = &self.address_block {
            addr_block.modify(&mut dest.address_block);
        }

        // TODO should I use derived_from attribute?
    }
}

impl yml::OptAddressBlock {
    fn modify(&self, dest: &mut Option<svd::AddressBlock>) {
        match dest {
            Some(dest) => {
                modify_if_some(&mut dest.offset, &self.offset);
                modify_if_some(&mut dest.size, &self.size);
                modify_if_some(&mut dest.usage, &self.usage);
            }
            None => {
                *dest = Some(svd::AddressBlock {
                    offset: self.offset.unwrap_or_default(),
                    size: self.size.unwrap_or_default(),
                    usage: self.usage.clone().unwrap_or_default(),
                })
            }
        }
    }
}

fn modify_option<T: Clone>(dest: &mut Option<T>, src: &Option<T>) {
    if let Some(dest) = dest {
        modify_if_some(dest, src);
    } else {
        *dest = src.clone();
    }
}

fn modify_if_some<T: Clone>(dest: &mut T, src: &Option<T>) {
    if let Some(src) = src {
        *dest = src.clone();
    }
}
