use std::sync::Arc;

use anyhow_ext::{Context, Result};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use uk_settings::SETTINGS;

use crate::{deploy, mods};

#[derive(Debug, Clone)]
pub struct Manager {
    mod_manager: Arc<RwLock<mods::Manager>>,
    deploy_manager: Arc<RwLock<deploy::Manager>>,
}

impl std::panic::RefUnwindSafe for Manager {}

impl Manager {
    pub fn init() -> Result<Self> {
        let mod_manager = Arc::new(RwLock::new(
            mods::Manager::init().context("Failed to initialize mod manager")?,
        ));
        Ok(Self {
            deploy_manager: Arc::new(RwLock::new(
                deploy::Manager::init(&mod_manager)
                    .context("Failed to initialize deployment manager")?,
            )),
            mod_manager,
        })
    }

    pub fn reload(&self) -> Result<()> {
        SETTINGS.write().reload();
        *self.mod_manager.write() =
            mods::Manager::init().context("Failed to initialize mod manager")?;
        *self.deploy_manager.write() = deploy::Manager::init(&self.mod_manager)
            .context("Failed to initialize deployment manager")?;
        Ok(())
    }

    pub fn change_profile(&self, profile: impl AsRef<str>) -> Result<()> {
        self.mod_manager.write().set_profile(profile.as_ref())?;
        if let Some(config) = SETTINGS.write().platform_config_mut() {
            config.profile = profile.as_ref().into();
        }
        Ok(())
    }

    #[inline(always)]
    pub fn mod_manager(&self) -> RwLockReadGuard<'_, mods::Manager> {
        self.mod_manager.read()
    }

    #[inline(always)]
    pub fn mod_manager_mut(&self) -> RwLockWriteGuard<'_, mods::Manager> {
        self.mod_manager.write()
    }

    #[inline(always)]
    pub fn deploy_manager(&self) -> RwLockReadGuard<'_, deploy::Manager> {
        self.deploy_manager.read()
    }
}
