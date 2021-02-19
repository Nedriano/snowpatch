//
// snowpatch - continuous integration for patch-based workflows
//
// Copyright (C) 2016 IBM Corporation
// Authors:
//     Russell Currey <ruscur@russell.cc>
//     Andrew Donnellan <andrew.donnellan@au1.ibm.com>
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//
// config.rs - handle snowpatch config parsing from RON
//

// standard library
use std::fs::File;

// third party dependencies
use anyhow::{Context, Result};
use ron::de::from_reader;
use serde::Deserialize;
use url::Url;

/// Defines the full set of information snowpatch needs in order to do anything useful.
#[derive(Debug, Deserialize)]
pub struct Config {
    name: String,
    git: Git,
    patchwork: Patchwork,
}

/// Defines the git details snowpatch needs to push to remotes.
/// snowpatch uses libgit2 in order to communicate with remotes through SSH.
#[derive(Debug, Deserialize)]
pub struct Git {
    /// The user on the remote, typically `git` for most services, as in `git@github.com:ruscur/snowpatch.git`
    user: String,
    /// Full path to the public key, for example `/home/ruscur/.ssh/id_rsa.pub`
    public_key: String,
    /// Full path to the private key, for example `/home/ruscur/.ssh/id_rsa`
    private_key: String,
}

/// Defines the Patchwork server you wish to work with.
/// Credentials are not necessary unless you wish to push results.
/// snowpatch only supports API token authentication and not Basic Auth.
#[derive(Debug, Deserialize)]
pub struct Patchwork {
    /// URL of the Patchwork server, without port i.e. `https://patchwork.ozlabs.org`
    url: String,
    /// Port of the Patchwork server, defaults to `443`
    port: Option<u16>,
    /// API token you wish to use on the Patchwork server, only needed if pushing results
    token: Option<String>,
}

fn validate_config(config: &Config) -> Result<()> {
    // Validate paths
    File::open(&config.git.public_key).with_context(|| format!("Couldn't open public key file"))?;
    File::open(&config.git.private_key)
        .with_context(|| format!("Couldn't open private key file"))?;

    // Validate URLs
    Url::parse(&config.patchwork.url).with_context(|| format!("Couldn't parse Patchwork URL"))?;

    Ok(())
}

pub fn parse_config(filename: &str) -> Result<Config> {
    let file = File::open(filename)
        .with_context(|| format!("Failed to open config file at {}", filename))?;

    let config: Config =
        from_reader(file).with_context(|| format!("Failed to parse config file"))?;

    println!("Config: {:?}", &config);

    validate_config(&config)?;

    Ok(config)
}