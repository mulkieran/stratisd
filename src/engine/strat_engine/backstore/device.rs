// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Functions for dealing with devices.

use std::collections::HashMap;
use std::fmt::{self, Display};
use std::fs::{File, OpenOptions};
use std::os::unix::prelude::AsRawFd;
use std::path::Path;

use libudev;

use devicemapper::{devnode_to_devno, Bytes, Device};

use stratis::{ErrorEnum, StratisError, StratisResult};

use super::super::super::types::{DevUuid, PoolUuid};

use super::metadata::device_identifiers;
use super::udev::{get_udev_property, udev_block_device_apply, unclaimed};

ioctl!(read blkgetsize64 with 0x12, 114; u64);

pub fn blkdev_size(file: &File) -> StratisResult<Bytes> {
    let mut val: u64 = 0;

    match unsafe { blkgetsize64(file.as_raw_fd(), &mut val) } {
        Err(x) => Err(StratisError::Nix(x)),
        Ok(_) => Ok(Bytes(val)),
    }
}

/// Resolve a list of Paths of some sort to a set of unique Devices.
/// Return an IOError if there was a problem resolving any particular device.
/// The set of devices maps each device to one of the paths passed.
/// Returns an error if any path does not correspond to a block device.
pub fn resolve_devices<'a>(paths: &'a [&Path]) -> StratisResult<HashMap<Device, &'a Path>> {
    let mut map = HashMap::new();
    for path in paths {
        match devnode_to_devno(path)? {
            Some(devno) => {
                let _ = map.insert(Device::from(devno), *path);
            }
            None => {
                let err_msg = format!("path {} does not refer to a block device", path.display());
                return Err(StratisError::Engine(ErrorEnum::Invalid, err_msg));
            }
        }
    }
    Ok(map)
}

/// Something to identify a device that is not a StratisDevice.
/// This type is extremely rudimentary and may have to be modified as Stratis's
/// notion of how to obtain a signature for a deivce that is not Stratis's
/// changes.
#[derive(Debug, PartialEq, Eq)]
pub enum TheirsReason {
    /// Udev identifies device as belonging to another.
    Udev {
        id_part_table_type: Option<String>,
        id_fs_type: Option<String>,
    },
}

impl Display for TheirsReason {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TheirsReason::Udev {
                id_part_table_type,
                id_fs_type,
            } => write!(
                f,
                "ID_PART_TABLE_TYPE: {}, ID_FS_TYPE: {}",
                match id_part_table_type {
                    Some(val) => val,
                    None => "not available",
                },
                match id_fs_type {
                    Some(val) => val,
                    None => "not found",
                }
            ),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum DevOwnership {
    Contradiction,
    Ours(PoolUuid, DevUuid),
    Unowned,
    Theirs(TheirsReason),
}

/// Identify a device node using a combination of udev information and
/// Stratis signature information.
/// Return an error if the device is not in the udev database.
/// Return an error if the necessary udev information can not be read.
pub fn identify(devnode: &Path) -> StratisResult<DevOwnership> {
    /// A helper function. None if the device is unclaimed, the value of
    /// ID_FS_TYPE, which may yet be None, if it is.
    #[allow(option_option)]
    fn udev_info(device: &libudev::Device) -> StratisResult<Option<Option<String>>> {
        if unclaimed(device) {
            Ok(None)
        } else {
            Ok(Some(get_udev_property(device, "ID_FS_TYPE")?))
        }
    }

    match udev_block_device_apply(devnode, udev_info)? {
        Some(Ok(Some(Some(value)))) => {
            if value == "stratis" {
                if let Some((pool_uuid, device_uuid)) =
                    device_identifiers(&mut OpenOptions::new().read(true).open(&devnode)?)?
                {
                    Ok(DevOwnership::Ours(pool_uuid, device_uuid))
                } else {
                    Ok(DevOwnership::Contradiction)
                }
            } else {
                Ok(DevOwnership::Theirs(TheirsReason::Udev {
                    id_part_table_type: None,
                    id_fs_type: None,
                }))
            }
        }
        Some(Ok(Some(None))) => Ok(DevOwnership::Theirs(TheirsReason::Udev {
            id_part_table_type: None,
            id_fs_type: None,
        })),
        Some(Ok(None)) => {
            // Not a Stratis device OR running an older version of libblkid
            // that does not interpret Stratis devices. Fall back on reading
            // Stratis header via Stratis.
            // NOTE: This is a bit kludgy. If, at any time during stratisd
            // execution, a device is identified as a Stratis device by libblkid
            // then it is clear that the version of libblkid being run is new
            // enough. But to track that information requires some kind of
            // stateful global variable. So, instead, fall back on the safe
            // approach of just always reading the Stratis header, regardless
            // of what has happened in the past.
            Ok(if let Some((pool_uuid, device_uuid)) =
                device_identifiers(&mut OpenOptions::new().read(true).open(&devnode)?)?
            {
                DevOwnership::Ours(pool_uuid, device_uuid)
            } else {
                DevOwnership::Unowned
            })
        }
        Some(Err(err)) => Err(err),
        None => Err(StratisError::Engine(
            ErrorEnum::NotFound,
            format!(
                "No device in udev database corresponding to device node {:?}",
                devnode
            ),
        )),
    }
}

#[cfg(test)]
mod test {

    use std::path::Path;

    use super::super::super::cmd;
    use super::super::super::tests::{loopbacked, real};

    use super::*;

    /// Verify that a device with an ext3 filesystem directly on it is
    /// identified as not a Stratis device.
    fn test_other_ownership(paths: &[&Path]) {
        cmd::create_ext3_fs(paths[0]).unwrap();

        cmd::udev_settle().unwrap();

        assert!(match identify(paths[0]).unwrap() {
            DevOwnership::Theirs(_) => true,
            _ => false,
        })
    }

    #[test]
    pub fn loop_test_other_ownership() {
        loopbacked::test_with_spec(
            loopbacked::DeviceLimits::Range(1, 3, None),
            test_other_ownership,
        );
    }

    #[test]
    pub fn travis_test_other_ownership() {
        loopbacked::test_with_spec(
            loopbacked::DeviceLimits::Range(1, 3, None),
            test_other_ownership,
        );
    }

    #[test]
    pub fn real_test_other_ownership() {
        real::test_with_spec(
            real::DeviceLimits::AtLeast(1, None, None),
            test_other_ownership,
        );
    }

    /// Verify that an empty device is unowned.
    fn test_empty(paths: &[&Path]) {
        cmd::udev_settle().unwrap();

        assert!(match identify(paths[0]).unwrap() {
            DevOwnership::Unowned => true,
            _ => false,
        });
    }

    #[test]
    pub fn loop_test_device_empty() {
        loopbacked::test_with_spec(loopbacked::DeviceLimits::Range(1, 3, None), test_empty);
    }

    #[test]
    pub fn travis_test_device_empty() {
        loopbacked::test_with_spec(loopbacked::DeviceLimits::Range(1, 3, None), test_empty);
    }

    #[test]
    pub fn real_test_device_empty() {
        real::test_with_spec(real::DeviceLimits::AtLeast(1, None, None), test_empty);
    }
}
