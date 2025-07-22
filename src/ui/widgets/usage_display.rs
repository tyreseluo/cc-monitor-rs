use makepad_widgets::*;
use crate::monitor::CcusageData;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    use crate::ui::styles::*;
    
    pub UsageDisplay = {{UsageDisplay}} {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 8
        
        // Session start time
        session_row = <View> {
            width: Fill,
            height: Fit,
            
            session_label = <Label> {
                text: "📅 对话开始: --"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }
        
        // Time info
        time_row = <View> {
            width: Fill,
            height: Fit,
            
            time_label = <Label> {
                text: "⏱️  时间: -- → -- (重置)"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }
        
        // Remaining time
        remaining_row = <View> {
            width: Fill,
            height: Fit,
            
            remaining_label = <Label> {
                text: "⏰ 剩余: --"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }
        
        // Tokens
        tokens_row = <View> {
            width: Fill,
            height: Fit,
            
            tokens_label = <Label> {
                text: "🎫 Tokens: --"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }
        
        // Cost
        cost_row = <View> {
            width: Fill,
            height: Fit,
            
            cost_label = <Label> {
                text: "💰 费用: --"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }
        
        // Model
        model_row = <View> {
            width: Fill,
            height: Fit,
            
            model_label = <Label> {
                text: "🤖 模型: --"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }
        
        // Status
        status_row = <View> {
            width: Fill,
            height: Fit,
            
            status_label = <Label> {
                text: "📍 状态: ⏸️  未活动"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct UsageDisplay {
    #[deref] view: View,
    #[rust] ccusage_data: CcusageData,
}

impl Widget for UsageDisplay {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.apply_data_updates(cx);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl UsageDisplay {
    pub fn update_data(&mut self, cx: &mut Cx, data: CcusageData) {
        self.ccusage_data = data;
        self.apply_data_updates(cx);
        cx.redraw_all();
    }
    
    fn apply_data_updates(&mut self, cx: &mut Cx) {
        // Update session start
        if let Some(mut label) = self.view.label(id!(session_label)).borrow_mut() {
            let text = if self.ccusage_data.latest_session != "--" {
                format!("📅 对话开始: {}", self.ccusage_data.latest_session)
            } else {
                "📅 对话开始: --".to_string()
            };
            label.set_text(cx, &text);
        }
        
        // Update time info
        if let Some(mut label) = self.view.label(id!(time_label)).borrow_mut() {
            if self.ccusage_data.session_start != "--" {
                let text = format!("⏱️  时间: {} → {} (重置)", 
                    self.ccusage_data.session_start, 
                    self.ccusage_data.session_end
                );
                label.set_text(cx, &text);
            }
        }
        
        // Update remaining time
        if let Some(mut label) = self.view.label(id!(remaining_label)).borrow_mut() {
            label.set_text(cx, &format!("⏰ 剩余: {}", self.ccusage_data.remaining_time));
        }
        
        // Update tokens
        if let Some(mut label) = self.view.label(id!(tokens_label)).borrow_mut() {
            if self.ccusage_data.tokens != "--" {
                label.set_text(cx, &format!("🎫 Tokens: {}", self.ccusage_data.tokens));
            }
        }
        
        // Update cost
        if let Some(mut label) = self.view.label(id!(cost_label)).borrow_mut() {
            label.set_text(cx, &format!("💰 费用: {}", self.ccusage_data.cost));
        }
        
        // Update model
        if let Some(mut label) = self.view.label(id!(model_label)).borrow_mut() {
            label.set_text(cx, &format!("🤖 模型: {}", self.ccusage_data.model));
        }
        
        // Update status
        if let Some(mut label) = self.view.label(id!(status_label)).borrow_mut() {
            let status_text = match self.ccusage_data.status.as_str() {
                "ACTIVE" => "⚡ 活跃中",
                "COMPLETED" => "✅ 已完成",
                "RUNNING" => "🔄 运行中",
                _ => "⏸️  未活动",
            };
            label.set_text(cx, &format!("📍 状态: {}", status_text));
        }
    }
}