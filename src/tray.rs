use tray_icon::{
    menu::{Menu, MenuItem, PredefinedMenuItem},
    TrayIcon, TrayIconBuilder,
};

// Platform-specific requirements:
// - Windows: Requires win32 event loop
// - macOS: Must be created on main thread
// - Linux: Requires GTK event loop, libappindicator
use std::sync::{Arc, Mutex};
use anyhow::Result;
use crate::monitor::MonitorData;
use crate::i18n;

pub struct TrayManager {
    tray: TrayIcon,
    menu: Menu,
    status_item: MenuItem,
    network_item: MenuItem,
    usage_item: MenuItem,
    cost_item: MenuItem,
    model_item: MenuItem,
    remaining_item: MenuItem,
    status_active_item: MenuItem,
    quit_item: MenuItem,
}

impl TrayManager {
    pub fn new() -> Result<Self> {
        // Create menu items
        let status_item = MenuItem::new(&i18n::get(i18n::keys::APP_VERSION), true, None);
        let separator1 = PredefinedMenuItem::separator();

        let network_item = MenuItem::new(
            &format!("{}: {}", i18n::get(i18n::keys::TRAY_NETWORK), i18n::get(i18n::keys::TRAY_CHECKING)),
            false,
            None
        );
        let usage_item = MenuItem::new(
            &format!("{}: {}", i18n::get(i18n::keys::TRAY_USAGE), i18n::get(i18n::keys::TRAY_CHECKING)),
            false,
            None
        );
        let cost_item = MenuItem::new(
            &format!("{}: {}", i18n::get(i18n::keys::TRAY_COST), i18n::get(i18n::keys::TRAY_CHECKING)),
            false,
            None
        );
        let model_item = MenuItem::new(
            &format!("{}: {}", i18n::get(i18n::keys::TRAY_MODEL), i18n::get(i18n::keys::TRAY_CHECKING)),
            false,
            None
        );
        let remaining_item = MenuItem::new(
            &format!("{}: {}", i18n::get(i18n::keys::TRAY_REMAINING), i18n::get(i18n::keys::TRAY_CHECKING)),
            false,
            None
        );
        let status_active_item = MenuItem::new(
            &format!("{}: {}", i18n::get(i18n::keys::TRAY_STATUS), i18n::get(i18n::keys::TRAY_CHECKING)),
            false,
            None
        );

        let separator2 = PredefinedMenuItem::separator();
        let quit_item = MenuItem::new(&i18n::get(i18n::keys::TRAY_QUIT), true, None);

        // Create menu
        let menu = Menu::new();
        menu.append(&status_item)?;
        menu.append(&separator1)?;
        menu.append(&network_item)?;
        menu.append(&usage_item)?;
        menu.append(&cost_item)?;
        menu.append(&model_item)?;
        menu.append(&remaining_item)?;
        menu.append(&status_active_item)?;
        menu.append(&separator2)?;
        menu.append(&quit_item)?;

        // Create tray icon using embedded icon
        let icon = Self::create_icon()?;

        // Build tray with platform considerations
        let builder = TrayIconBuilder::new()
            .with_menu(Box::new(menu.clone()))
            .with_tooltip(&i18n::get(i18n::keys::APP_NAME))
            .with_icon(icon);

        // Linux sometimes requires a menu to show the icon
        #[cfg(target_os = "linux")]
        {
            // Menu is already set above, which helps with Linux visibility
        }

        let tray = builder.build()?;

        Ok(Self {
            tray,
            menu,
            status_item,
            network_item,
            usage_item,
            cost_item,
            model_item,
            remaining_item,
            status_active_item,
            quit_item,
        })
    }

    pub fn update_status(&self, data: &MonitorData) -> Result<()> {
        // Update network status
        let network_text = if data.network_connected {
            format!("{} {}",
                i18n::get(i18n::keys::TRAY_NETWORK_CONNECTED),
                data.latency.as_deref().unwrap_or(""))
        } else {
            i18n::get(i18n::keys::TRAY_NETWORK_DISCONNECTED)
        };
        self.network_item.set_text(network_text);

        // Update usage status
        let usage_text = format!("📊 Token: {} ({})",
            data.ccusage_data.tokens,
            data.ccusage_data.latest_session
        );
        self.usage_item.set_text(usage_text);

        // Update cost
        let cost_text = format!("💰 {}: {}", i18n::get(i18n::keys::TRAY_COST), data.ccusage_data.cost);
        self.cost_item.set_text(cost_text);

        // Update model
        let model_text = format!("🤖 {}: {}",
            i18n::get(i18n::keys::TRAY_MODEL),
            if data.ccusage_data.model.is_empty() { "--" } else { &data.ccusage_data.model }
        );
        self.model_item.set_text(model_text);

        // Update remaining time
        let remaining_text = format!("⏱️ {}: {}", i18n::get(i18n::keys::TRAY_REMAINING), data.ccusage_data.remaining_time);
        self.remaining_item.set_text(remaining_text);

        // Update active status
        let status_text = if data.ccusage_data.status == "ACTIVE" {
            i18n::get(i18n::keys::TRAY_STATUS_ACTIVE)
        } else {
            i18n::get(i18n::keys::TRAY_STATUS_COMPLETED)
        };
        self.status_active_item.set_text(status_text);

        // Update tray tooltip with current status
        let tooltip = format!(
            "{}\n{} | {} | {}",
            i18n::get(i18n::keys::APP_NAME),
            if data.network_connected { "🟢" } else { "🔴" },
            data.ccusage_data.tokens,
            data.ccusage_data.remaining_time
        );
        let _ = self.tray.set_tooltip(Some(tooltip));

        Ok(())
    }

    fn create_icon() -> Result<tray_icon::Icon> {
        // Embed the logo directly in the binary
        const ICON_DATA: &[u8] = include_bytes!("../assets/ccm-logo-128.png");
        
        // Load the embedded icon
        let img = image::load_from_memory(ICON_DATA)?;
        let rgba = img.to_rgba8();
        let (width, height) = (rgba.width(), rgba.height());
        let icon_data = rgba.into_raw();
        
        tray_icon::Icon::from_rgba(icon_data, width, height)
            .map_err(|e| anyhow::anyhow!("Failed to create icon: {}", e))
    }

    pub fn handle_menu_event(&self, event: &tray_icon::menu::MenuEvent) -> bool {
        // Check if quit was clicked
        event.id() == self.quit_item.id()
    }
    
    pub fn refresh_translations(&self) -> Result<()> {
        // Update all menu item texts with current language
        self.status_item.set_text(i18n::get(i18n::keys::APP_VERSION));
        self.network_item.set_text(format!("{}: {}", i18n::get(i18n::keys::TRAY_NETWORK), i18n::get(i18n::keys::TRAY_CHECKING)));
        self.usage_item.set_text(format!("{}: {}", i18n::get(i18n::keys::TRAY_USAGE), i18n::get(i18n::keys::TRAY_CHECKING)));
        self.cost_item.set_text(format!("{}: {}", i18n::get(i18n::keys::TRAY_COST), i18n::get(i18n::keys::TRAY_CHECKING)));
        self.model_item.set_text(format!("{}: {}", i18n::get(i18n::keys::TRAY_MODEL), i18n::get(i18n::keys::TRAY_CHECKING)));
        self.remaining_item.set_text(format!("{}: {}", i18n::get(i18n::keys::TRAY_REMAINING), i18n::get(i18n::keys::TRAY_CHECKING)));
        self.status_active_item.set_text(format!("{}: {}", i18n::get(i18n::keys::TRAY_STATUS), i18n::get(i18n::keys::TRAY_CHECKING)));
        self.quit_item.set_text(i18n::get(i18n::keys::TRAY_QUIT));
        
        // Update tooltip
        let _ = self.tray.set_tooltip(Some(i18n::get(i18n::keys::APP_NAME)));
        
        Ok(())
    }
}

// Thread-safe wrapper for the tray manager
pub struct TrayHandle {
    inner: Arc<Mutex<Option<TrayManager>>>,
}

impl TrayHandle {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(None)),
        }
    }

    pub fn init(&self) -> Result<()> {
        let tray = TrayManager::new()?;
        *self.inner.lock().unwrap() = Some(tray);
        Ok(())
    }

    pub fn update_status(&self, data: &MonitorData) -> Result<()> {
        if let Some(tray) = self.inner.lock().unwrap().as_ref() {
            tray.update_status(data)?;
        }
        Ok(())
    }

    pub fn handle_menu_event(&self, event: &tray_icon::menu::MenuEvent) -> bool {
        if let Some(tray) = self.inner.lock().unwrap().as_ref() {
            return tray.handle_menu_event(event);
        }
        false
    }
    
    pub fn refresh_translations(&self) -> Result<()> {
        if let Some(tray) = self.inner.lock().unwrap().as_ref() {
            tray.refresh_translations()?;
        }
        Ok(())
    }
}
