use makepad_widgets::*;
use crate::monitor::MonitorData;
use crate::i18n;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    use crate::ui::widgets::status_indicator::StatusIndicator;
    use crate::ui::widgets::usage_display::UsageDisplay;
    use crate::ui::widgets::chart::Chart;

    pub MainScreen = {{MainScreen}} {
        width: Fill,
        height: Fill,
        
        <View> {
            width: Fill,
            height: Fill,
            flow: Down,

            padding: 20
            spacing: 20

            show_bg: true,
            draw_bg: {
                color: #1a1a1a
            }

        // Title and language selector section
        header = <View> {
            width: Fill,
            height: Fit,
            flow: Right,
            align: {y: 0.5}
            spacing: 10
            
            // Title centered with flex
            <View> {
                width: Fill,
                height: Fit,
                align: {x: 0.5, y: 0.5}
                
                title_label = <Label> {
                    text: "Claude Code 网络监测器 v1.0"
                    draw_text: {
                        text_style: {
                            font_size: 20.0
                        }
                        color: #ffffff
                    }
                }
            }
            
            // Language dropdown on the right
            lang_dropdown = <DropDown> {
                width: 120,
                height: 30,
                labels: ["🇨🇳 中文", "🇺🇸 English", "🇯🇵 日本語"],
                values: [Chinese, English, Japanese]
                selected_item: 0
                popup_menu_position: BelowInput
                
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }

        // Divider
        <RectView> {
            width: Fill,
            height: 2,
            draw_bg: {
                color: #2a2a2a
            }
        }

        // Network Status Section
        network_section = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 10

            network_section_label = <Label> {
                text: "[🌐 网络连接状态]"
                draw_text: {
                    text_style: {
                        font_size: 16.0
                    }
                    color: #ffffff
                }
            }

            status_indicator = <StatusIndicator> {}
        }

        // Claude Usage Section
        usage_section = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 10
            margin: {top: 20}

            usage_section_label = <Label> {
                text: "[🤖 Claude Code 使用状态]"
                draw_text: {
                    text_style: {
                        font_size: 16.0
                    }
                    color: #ffffff
                }
            }

            usage_display = <UsageDisplay> {}
        }

        // Historical Stats Section
        stats_section = <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            spacing: 10
            margin: {top: 20}

            stats_section_label = <Label> {
                text: "[📊 历史账单统计 (基于 Token 计算)]"
                draw_text: {
                    text_style: {
                        font_size: 16.0
                    }
                    color: #ffffff
                }
            }

            chart = <Chart> {}
        }

        // Footer
        footer = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 5
            margin: {top: 20}
            align: {x: 0.5, y: 0.5}

            last_update = <Label> {
                text: "🕐 最后更新: --"
                draw_text: {
                    text_style: {
                        font_size: 12.0
                    }
                    color: #cccccc
                }
            }

            <Label> {
                text: "🔗 GitHub: https://github.com/zhanghandong/cc-monitor-rs"
                draw_text: {
                    text_style: {
                        font_size: 12.0
                    }
                    color: #4a9eff
                }
            }

            quit_label = <Label> {
                text: "按 Cmd+Q 停止监控"
                draw_text: {
                    text_style: {
                        font_size: 12.0
                    }
                    color: #cccccc
                }
            }
        }
    }
}
}

#[derive(Live, LiveHook, Widget)]
pub struct MainScreen {
    #[deref] view: View,
    #[rust] monitor_data: MonitorData,
}

impl Widget for MainScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        
        // Handle dropdown selection
        if let Event::Actions(actions) = event {
            if let Some(index) = self.view.drop_down(id!(lang_dropdown)).selected(&actions) {
                match index {
                    0 => self.select_language(cx, i18n::Language::Chinese),
                    1 => self.select_language(cx, i18n::Language::English),
                    2 => self.select_language(cx, i18n::Language::Japanese),
                    _ => {}
                }
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MainScreen {
    pub fn initialize(&mut self, cx: &mut Cx) {
        // Set initial language selection in dropdown
        let lang = i18n::get_language();
        let selected_index = match lang {
            i18n::Language::Chinese => 0,
            i18n::Language::English => 1,
            i18n::Language::Japanese => 2,
        };
        
        self.view.drop_down(id!(lang_dropdown)).set_selected_item(cx, selected_index);
        
        // Refresh all translations
        self.refresh_translations(cx);
    }
    
    pub fn update_data(&mut self, cx: &mut Cx, data: MonitorData) {
        self.monitor_data = data;

        // Update status indicator
        if let Some(mut status) = self.view.widget(id!(status_indicator)).borrow_mut::<crate::ui::widgets::status_indicator::StatusIndicator>() {
            status.update_status(
                cx,
                self.monitor_data.network_connected,
                self.monitor_data.connection_speed.clone(),
                self.monitor_data.latency.clone()
            );
        }

        // Update usage display
        if let Some(mut usage) = self.view.widget(id!(usage_display)).borrow_mut::<crate::ui::widgets::usage_display::UsageDisplay>() {
            usage.update_data(cx, self.monitor_data.ccusage_data.clone());
        }

        // Update chart
        if let Some(mut chart) = self.view.widget(id!(chart)).borrow_mut::<crate::ui::widgets::chart::Chart>() {
            let total_cost: f64 = self.monitor_data.daily_costs.values()
                .map(|dc| dc.cost)
                .sum();
            let session_count: u32 = self.monitor_data.daily_costs.values()
                .map(|dc| dc.sessions)
                .sum();
            let active_sessions = self.monitor_data.daily_costs.values()
                .filter(|dc| dc.sessions > 0)
                .count() as u32;

            chart.update_data(
                cx,
                self.monitor_data.daily_costs.clone(),
                total_cost,
                session_count,
                active_sessions
            );
        }

        // Update last update time
        if let Some(mut label) = self.view.label(id!(last_update)).borrow_mut() {
            let time_str = self.monitor_data.last_update.format("%Y-%m-%d %H:%M:%S").to_string();
            label.set_text(cx, &format!("{}: {}", i18n::get(i18n::keys::COMMON_LAST_UPDATE), time_str));
        }
    }
    
    pub fn refresh_translations(&mut self, cx: &mut Cx) {
        // Update title
        if let Some(mut label) = self.view.label(id!(title_label)).borrow_mut() {
            label.set_text(cx, &i18n::get(i18n::keys::APP_VERSION));
        }
        
        // Update section headers
        if let Some(mut label) = self.view.label(id!(network_section_label)).borrow_mut() {
            label.set_text(cx, &format!("[{}]", i18n::get(i18n::keys::NETWORK_TITLE)));
        }
        
        if let Some(mut label) = self.view.label(id!(usage_section_label)).borrow_mut() {
            label.set_text(cx, &format!("[{}]", i18n::get(i18n::keys::USAGE_TITLE)));
        }
        
        if let Some(mut label) = self.view.label(id!(stats_section_label)).borrow_mut() {
            label.set_text(cx, &format!("[{}]", i18n::get(i18n::keys::HISTORY_TITLE)));
        }
        
        // Update footer
        if let Some(mut label) = self.view.label(id!(quit_label)).borrow_mut() {
            label.set_text(cx, &i18n::get(i18n::keys::COMMON_PRESS_TO_QUIT));
        }
        
        // Refresh child widgets
        if let Some(mut status) = self.view.widget(id!(status_indicator)).borrow_mut::<crate::ui::widgets::status_indicator::StatusIndicator>() {
            status.refresh_translations(cx);
        }
        
        if let Some(mut usage) = self.view.widget(id!(usage_display)).borrow_mut::<crate::ui::widgets::usage_display::UsageDisplay>() {
            usage.refresh_translations(cx);
        }
        
        if let Some(mut chart) = self.view.widget(id!(chart)).borrow_mut::<crate::ui::widgets::chart::Chart>() {
            chart.refresh_translations(cx);
        }
    }

    pub fn update_time_display(&mut self, cx: &mut Cx) {
        // Update only the time display with current time
        if let Some(mut label) = self.view.label(id!(last_update)).borrow_mut() {
            let time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            label.set_text(cx, &format!("{}: {}", i18n::get(i18n::keys::COMMON_LAST_UPDATE), time_str));
        }
    }
    
    fn select_language(&mut self, cx: &mut Cx, lang: i18n::Language) {
        // Set language
        i18n::set_language(lang);
        
        // Refresh all translations
        self.refresh_translations(cx);
        
        // Notify about language change to update tray
        crate::ui_updates::enqueue_monitor_update(crate::ui_updates::MonitorUpdate::LanguageChanged);
        
        cx.redraw_all();
    }
}
