// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

// Handles invoking external binaries.
// This module assumes that, for a given machine, there is only one place
// where the desired executable might be installed. It expects the engine
// to identify that place at its initialization by invoking verify_binaries(),
// and to exit immediately if verify_binaries() return an error. If this
// protocol is followed then when any command is executed the unique absolute
// path of the binary for this machine will already have been identified.
// However stratisd may run for a while and it is possible for the binary
// to be caused to be uninstalled while stratisd is being run. Therefore,
// the existence of the file is checked before the command is invoked, and
// an explicit error is returned if the executable can not be found.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;

use uuid::Uuid;

use stratis::{ErrorEnum, StratisError, StratisResult};

/// Find the binary with the given name by looking in likely locations.
/// Return None if no binary was found.
fn find_binary(name: &str) -> Option<PathBuf> {
    ["/usr/sbin", "/sbin", "/usr/bin", "/bin"]
        .iter()
        .map(|pre| [pre, name].iter().collect::<PathBuf>())
        .find(|path| path.exists())
}

// These are the external binaries that stratisd relies on.
// Any change in this list requires a corresponding change to BINARIES,
// and vice-versa.
const MKFS_XFS: &str = "mkfs.xfs";
const THIN_CHECK: &str = "thin_check";
const THIN_REPAIR: &str = "thin_repair";
const XFS_ADMIN: &str = "xfs_admin";
const XFS_GROWFS: &str = "xfs_growfs";

lazy_static! {
    static ref BINARIES: HashMap<String, Option<PathBuf>> = [
        (MKFS_XFS.to_string(), find_binary(MKFS_XFS)),
        (THIN_CHECK.to_string(), find_binary(THIN_CHECK)),
        (THIN_REPAIR.to_string(), find_binary(THIN_REPAIR)),
        (XFS_ADMIN.to_string(), find_binary(XFS_ADMIN)),
        (XFS_GROWFS.to_string(), find_binary(XFS_GROWFS))
    ].iter()
        .cloned()
        .collect();
}

/// Verify that all binaries that the engine might invoke are available at some
/// path. Return an error if any are missing. Required to be called on engine
/// initialization.
pub fn verify_binaries() -> StratisResult<()> {
    // Would like to use find() here instead of loop, but compiler bug
    // https://github.com/rust-lang/rust/issues/51415 makes that impossible.
    for (ref name, ref path) in BINARIES.iter() {
        if path.is_none() {
            return Err(StratisError::Error(format!(
                "Unable to find absolute path for \"{}\"",
                name
            )));
        }
    }
    Ok(())
}

/// Common function to call a command line utility, returning an Result with an error message which
/// also includes stdout & stderr if it fails.
fn execute_cmd(cmd: &mut Command, error_msg: &str) -> StratisResult<()> {
    let result = cmd.output()?;
    if result.status.success() {
        Ok(())
    } else {
        let std_out_txt = String::from_utf8_lossy(&result.stdout);
        let std_err_txt = String::from_utf8_lossy(&result.stderr);
        let err_msg = format!(
            "{} stdout: {} stderr: {}",
            error_msg, std_out_txt, std_err_txt
        );
        Err(StratisError::Engine(ErrorEnum::Error, err_msg))
    }
}

/// Get an absolute path for the executable with the given name.
/// Precondition: verify_binaries() has already been invoked.
fn get_executable(name: &str) -> StratisResult<PathBuf> {
    let executable = BINARIES
        .get(name)
        .expect("name arguments are all constants defined with BINARIES, lookup can not fail")
        .as_ref()
        .expect("verify_binaries() was previously called and returned no error");
    if !executable.exists() {
        return Err(StratisError::Error(format!(
            "Unable to find absolute path for \"{}\"",
            name
        )));
    }
    Ok(executable.to_path_buf())
}

/// Create a filesystem on devnode.
pub fn create_fs(devnode: &Path, uuid: Uuid) -> StratisResult<()> {
    execute_cmd(
        Command::new(get_executable(MKFS_XFS)?.as_os_str())
            .arg("-f")
            .arg("-q")
            .arg(&devnode)
            .arg("-m")
            .arg(format!("uuid={}", uuid)),
        &format!("Failed to create new filesystem at {:?}", devnode),
    )
}

/// Use the xfs_growfs command to expand a filesystem mounted at the given
/// mount point.
pub fn xfs_growfs(mount_point: &Path) -> StratisResult<()> {
    execute_cmd(
        Command::new(get_executable(XFS_GROWFS)?.as_os_str())
            .arg(mount_point)
            .arg("-d"),
        &format!("Failed to expand filesystem {:?}", mount_point),
    )
}

/// Set a new UUID for filesystem on the devnode.
pub fn set_uuid(devnode: &Path, uuid: Uuid) -> StratisResult<()> {
    execute_cmd(
        Command::new(get_executable(XFS_ADMIN)?.as_os_str())
            .arg("-U")
            .arg(format!("{}", uuid))
            .arg(&devnode),
        &format!("Failed to set UUID for filesystem {:?}", devnode),
    )
}

/// Call thin_check on a thinpool
pub fn thin_check(devnode: &Path) -> StratisResult<()> {
    execute_cmd(
        Command::new(get_executable(THIN_CHECK)?.as_os_str())
            .arg("-q")
            .arg(devnode),
        &format!("thin_check for thin pool meta device {:?} failed", devnode),
    )
}

/// Call thin_repair on a thinpool
pub fn thin_repair(meta_dev: &Path, new_meta_dev: &Path) -> StratisResult<()> {
    execute_cmd(
        Command::new(get_executable(THIN_REPAIR)?.as_os_str())
            .arg("-i")
            .arg(meta_dev)
            .arg("-o")
            .arg(new_meta_dev),
        &format!("thin_repair of thin pool meta device {:?} failed", meta_dev),
    )
}
