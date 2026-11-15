// grants.rs
// author: Ben Kyd
// created: 2026
//
// Here we will map other clients to their grants, this can be encoded in the filesystem
// to keep the application stateless.
//
// <peerid>.<blobseq>.friend
//
// There is a few things we need to do here, and a few operations that we might need
// to carry out.
// - a peer might ask what BLOBs we are keeping on their behalf
// - a peer might ask how much of their allocation is being used
//
// This all needs to be towards our overarching goal of keeping the application
// stateless therefore we will syscall to find size, read the config to
// understand grants and  use the name of the file to understand allocations.
//
// All of the methods here should be able to take an immutable reference to
// the settings object, or orchestrated by another module, a fs::Path

use crate::friends::BlobInfo;
use anyhow::{Context, Result};
use std::fs::{File, OpenOptions, read_dir};

// Extra meta to make it easier to handle locally
// this is never the blobinfo to send over the net
pub struct LocalBlobInfo<'a> {
    internal: BlobInfo,
    path: &'a std::path::Path,
}

pub fn blobs_by_peer(peerid: &str, storagepath: &str) -> Result<Vec<LocalBlobInfo>> {
    let it = read_dir(&storagepath).context("failed to read storagepath")?;

    let name = peerid.to_owned() + ".";

    let ret = Vec::new();

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

    Ok(ret)
}

pub fn used_grant_of_peer(peerid: &str, storagepath: &str) -> u32 {
    let blobs = blobs_by_peer(peerid, storagepath);

    


    0
}
