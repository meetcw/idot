use std::fs;
use std::io;
use std::path::Path;

use colored::*;
use log::{debug, error, info};

use crate::configuration::*;
use crate::error::*;
use crate::path_extension::PathExtension;

pub struct Linker {}

impl Linker {
    pub fn create<P: AsRef<Path>>(workspace: P, configuration: &GroupConfiguration, simulate: bool) -> Result<()> {
        let workspace = workspace.as_ref().absolutize().unwrap();
        match configuration.links {
            Some(ref links) => {
                for (symbolic_link, link_configuration) in links {
                    let symbolic_link_file_path = Path::new(symbolic_link)
                        .absolutize()
                        .map_err(|e| Error::new("Invalid symbol link.").with_inner_error(&e))?;
                    let target_file_path = workspace
                        .join(&link_configuration.target)
                        .absolutize()
                        .map_err(|e| Error::new("Invalid symbol link.").with_inner_error(&e))?;
                    let relative = link_configuration
                        .relative
                        .unwrap_or(configuration.relative.unwrap_or(false));
                    let force = link_configuration
                        .force
                        .unwrap_or(configuration.force.unwrap_or(false));

                    let result = Linker::create_symbolic_link(
                        &symbolic_link_file_path,
                        &target_file_path,
                        relative,
                        force,
                        simulate,
                    );
                    if let Err(error) = result {
                        error!("Failed to create symbolic link: {}. {}", &symbolic_link, &error);
                    } else {
                        info!(
                            "Create symbolic link: `{}` -> `{}`.",
                            symbolic_link_file_path.to_str().unwrap(),
                            target_file_path.to_str().unwrap()
                        );
                    }
                }
            }
            None => {}
        }
        return Ok(());
    }

    fn create_symbolic_link<P: AsRef<Path>>(
        symbolic_link_file_path: P,
        target_file_path: P,
        relative: bool,
        force: bool,
        simulate: bool,
    ) -> io::Result<()> {
        let symbolic_link_file_path = symbolic_link_file_path.as_ref();
        let target_file_path = target_file_path.as_ref();
        let mut link_content = target_file_path.clone().to_path_buf();

        if let Some(symbolic_link_parent_path) = symbolic_link_file_path.parent() {
            if !symbolic_link_parent_path.exists() {
                if !symbolic_link_parent_path.is_dir(){
                    debug!("The parent path is exists, but it's not a directory: {}.", symbolic_link_parent_path.to_str().unwrap());
                    if force{
                        debug!("Force clean the parent path: {}.", symbolic_link_parent_path.to_str().unwrap());
                        if !simulate { fs::remove_file(&symbolic_link_parent_path)? }
                    }
                }
                debug!("Create parent directory: {}.", symbolic_link_parent_path.to_str().unwrap());
                if !simulate { fs::create_dir_all(&symbolic_link_parent_path)? }
            }
            if relative {
                link_content = target_file_path
                    .relative_to(&symbolic_link_parent_path.canonicalize()?).unwrap();
            }
        }

        if symbolic_link_file_path.actually_exists() && force {
            if symbolic_link_file_path.is_file() || symbolic_link_file_path.is_symbolic() {
                debug!("Force to delete file: {}.", symbolic_link_file_path.to_str().unwrap());
                if !simulate { fs::remove_file(&symbolic_link_file_path)? }
            } else {
                debug!("Force to delete directory: {}.", symbolic_link_file_path.to_str().unwrap());
                if !simulate { fs::remove_dir_all(&symbolic_link_file_path)? }
            }
        }
        debug!("Create symbolic link : {} target {}.", symbolic_link_file_path.to_str().unwrap(),link_content.to_str().unwrap());
        if !simulate {
            return std::os::unix::fs::symlink(&link_content, &symbolic_link_file_path);
        } else {
            return Ok(());
        }
    }

    pub fn delete<P: AsRef<Path>>(workspace: P, configuration: &GroupConfiguration, simulate: bool) -> Result<()> {
        let workspace = workspace.as_ref();
        match configuration.links {
            Some(ref links) => {
                for (symbolic_link, link_configuration) in links {
                    let symbolic_link_file_path = Path::new(symbolic_link)
                        .absolutize()
                        .map_err(|e| Error::new("Invalid symbolic link.").with_inner_error(&e))?;
                    let target_file_path = workspace
                        .join(&link_configuration.target)
                        .absolutize()
                        .map_err(|e| Error::new("Invalid symbol link.").with_inner_error(&e))?;
                    let relative = link_configuration
                        .relative
                        .unwrap_or(configuration.relative.unwrap_or(false));
                    match Linker::symbolic_link_status(&symbolic_link_file_path, &target_file_path, relative) {
                        LinkStatus::Active => {
                            let result =
                                Linker::delete_symbolic_link(workspace, &symbolic_link_file_path, simulate);
                            if let Err(error) = result {
                                error!("Failed to delete symbolic link: `{}`. {}", &symbolic_link, &error);
                            } else {
                                info!("Delete symbolic link: `{}`.", &symbolic_link_file_path.to_str().unwrap());
                            }
                        }
                        _ => {
                            debug!("Don't need delete: `{}`.", &symbolic_link_file_path.to_str().unwrap());
                        }
                    }
                }
            }
            None => {}
        }
        return Ok(());
    }

    fn delete_symbolic_link<P: AsRef<Path>>(
        workspace: P,
        symbolic_link_file_path: P,
        simulate: bool,
    ) -> io::Result<()> {
        let workspace = workspace.as_ref().canonicalize().unwrap();
        let symbolic_link_file_path = symbolic_link_file_path.as_ref();

        if !symbolic_link_file_path.actually_exists() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("`{}` is not exists.",
                                                                       symbolic_link_file_path.to_str().unwrap())));
        }
        if !symbolic_link_file_path.is_symbolic() {
            return Err(io::Error::new(io::ErrorKind::NotFound, format!("`{}` is not a symbolic link.",
                                                                       symbolic_link_file_path.to_str().unwrap())));
        }
        let mut link_content = fs::read_link(&symbolic_link_file_path).unwrap();
        if link_content.is_relative() {
            link_content = symbolic_link_file_path
                .parent().unwrap()
                .join(&link_content)
                .absolutize()?;
        }
        if link_content.strip_prefix(&workspace).is_ok() {
            if !simulate {
                return fs::remove_file(symbolic_link_file_path);
            }else{
                return Ok(());
            }
        } else {
            return Err(io::Error::new(io::ErrorKind::Other, format!("`{}` is not belong to current workspace.",
                                                                    symbolic_link_file_path.to_str().unwrap())));
        }
    }

    pub fn status<P: AsRef<Path>>(workspace: P, configuration: &GroupConfiguration) -> Result<()> {
        let workspace = workspace.as_ref();
        match configuration.links {
            Some(ref links) => {
                for (symbolic_link, link_configuration) in links {
                    let symbolic_link_file_path = Path::new(symbolic_link)
                        .absolutize()
                        .map_err(|e| Error::new("Invalid symbol link.").with_inner_error(&e))?;
                    let target_file_path = workspace
                        .join(&link_configuration.target)
                        .absolutize()
                        .map_err(|e| Error::new("Invalid symbol link.").with_inner_error(&e))?;
                    let relative = link_configuration
                        .relative
                        .unwrap_or(configuration.relative.unwrap_or(false));
                    match Linker::symbolic_link_status(
                        &symbolic_link_file_path,
                        &target_file_path,
                        relative,
                    ) {
                        LinkStatus::Active => {
                            info!(
                                "`{}` -> `{}`.",
                                symbolic_link_file_path.to_str().unwrap().green(),
                                target_file_path.to_str().unwrap(),
                            );
                        }
                        LinkStatus::Inactive => {
                            info!(
                                "`{}` -> `{}`.",
                                symbolic_link_file_path.to_str().unwrap().red(),
                                target_file_path.to_str().unwrap(),
                            );
                        }
                    };
                }
            }
            None => {}
        }
        return Ok(());
    }

    fn symbolic_link_status<P: AsRef<Path>>(
        symbolic_link_file_path: P,
        target_file_path: P,
        relative: bool,
    ) -> LinkStatus {
        let symbolic_link_file_path = symbolic_link_file_path.as_ref();
        let target_file_path = target_file_path.as_ref();
        if !symbolic_link_file_path.actually_exists() {
            debug!(
                "`{}` {} ",
                symbolic_link_file_path.to_str().unwrap(),
                "is not exists."
            );
            return LinkStatus::Inactive;
        }
        if !symbolic_link_file_path.is_symbolic() {
            debug!(
                "`{}` {}",
                symbolic_link_file_path.to_str().unwrap(),
                "is not a symbolic link."
            );
            return LinkStatus::Inactive;
        }

        let link_content = symbolic_link_file_path.read_link().unwrap();
        let mut link_content_file_path = link_content.clone();
        if link_content.is_relative() {
            link_content_file_path = symbolic_link_file_path
                .parent()
                .unwrap()
                .join(&link_content)
                .absolutize()
                .unwrap();
        }
        if link_content_file_path == target_file_path {
            if relative == link_content.is_relative() {
                debug!(
                    "`{}` -> `{}`.",
                    symbolic_link_file_path.to_str().unwrap(),
                    link_content.to_str().unwrap()
                );
                return LinkStatus::Active;
            } else {
                debug!(
                    "`{}` -> `{}`, need update!",
                    symbolic_link_file_path.to_str().unwrap(),
                    link_content.to_str().unwrap()
                );
                return LinkStatus::Active;
            }
        } else {
            debug!(
                "`{}` -> `{}`.",
                symbolic_link_file_path.to_str().unwrap(),
                link_content.to_str().unwrap()
            );
            return LinkStatus::Inactive;
        }
    }
}


enum LinkStatus {
    Inactive,
    Active,
}
