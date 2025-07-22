use makepad_widgets::*;
use crate::monitor::MonitorData;

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
        flow: Down,

        padding: 20
        spacing: 20

        show_bg: true,
        draw_bg: {
            color: #1a1a1a
        }

        // Title
        title = <View> {
            width: Fill,
            height: Fit,
            align: {x: 0.5, y: 0.5}

            <Label> {
                text: "Claude Code 网络监测器 v1.0"
                draw_text: {
                    text_style: {
                        font_size: 20.0
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

            <Label> {
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

            <Label> {
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

            <Label> {
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

            <Label> {
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

#[derive(Live, LiveHook, Widget)]
pub struct MainScreen {
    #[deref] view: View,
    #[rust] monitor_data: MonitorData,
}

impl Widget for MainScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl MainScreen {
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
            label.set_text(cx, &format!("🕐 最后更新: {}", time_str));
        }
    }

    pub fn update_time_display(&mut self, cx: &mut Cx) {
        // Update only the time display with current time
        if let Some(mut label) = self.view.label(id!(last_update)).borrow_mut() {
            let time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
            label.set_text(cx, &format!("🕐 最后更新: {}", time_str));
        }
    }
}
