// grants.rs
// author: Ben Kyd
// created: 2026
//
// Here we will map other clients to their grants, this can be encoded in the filesystem
// to keep the application stateless.
//
// <peerid>.<blobseq>.friend

use crate::friends::BlobInfo;
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions, read_dir};

pub fn blobs_by_peer(peerid: &str, storagepath: &str) -> Result<Vec<BlobInfo>> {
    let it = read_dir(&storagepath).context("failed to read storagepath")?;

    let name = peerid.to_owned() + ".";

    for entry in it {
        let pathstr = entry
            .context("failed to read dir entry")?
            .file_name()
            .into_string()
            .unwrap();

        if !pathstr.ends_with(".friend") {
            continue;
        }

        if !pathstr.starts_with(&name) {
            continue;
        }

        let sequenceid = pathstr.split(".").nth(1).unwrap();

        println!("peerID {} owns {} sequence {}", peerid, pathstr, sequenceid);
    }

    Ok(Vec::new())
}
