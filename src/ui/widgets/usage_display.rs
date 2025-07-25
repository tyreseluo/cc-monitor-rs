use makepad_widgets::*;
use crate::monitor::CcusageData;
use crate::i18n;
use crate::utils::model_pricing;

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
        
        // Model Pricing Comparison
        model_pricing_section = <View> {
            width: Fill,
            height: Fit,
            flow: Down,
            spacing: 4
            
            pricing_title = <Label> {
                text: "📊 模型价格对比:"
                draw_text: {
                    text_style: {
                        font_size: 14.0
                    }
                    color: #ffffff
                }
            }
            
            pricing_list = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 2
                
                // Model pricing items will be dynamically created
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
    #[rust] model_prices: Vec<(String, String, f64)>,
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
        self.update_model_prices();
        self.apply_data_updates(cx);
        cx.redraw_all();
    }
    
    fn update_model_prices(&mut self) {
        // Calculate prices for all models based on current tokens
        // For simplicity, we'll assume 30% input tokens and 70% output tokens
        let total_tokens = self.ccusage_data.tokens_num;
        if total_tokens > 0 {
            let input_tokens = (total_tokens as f64 * 0.3) as u64;
            let output_tokens = (total_tokens as f64 * 0.7) as u64;
            self.model_prices = model_pricing::calculate_all_model_costs(input_tokens, output_tokens);
        } else {
            self.model_prices.clear();
        }
    }
    
    fn apply_data_updates(&mut self, cx: &mut Cx) {
        // Update session start
        if let Some(mut label) = self.view.label(id!(session_label)).borrow_mut() {
            let text = if self.ccusage_data.latest_session != "--" {
                format!("{}: {}", i18n::get(i18n::keys::USAGE_SESSION_START), self.ccusage_data.latest_session)
            } else {
                format!("{}: --", i18n::get(i18n::keys::USAGE_SESSION_START))
            };
            label.set_text(cx, &text);
        }
        
        // Update time info
        if let Some(mut label) = self.view.label(id!(time_label)).borrow_mut() {
            if self.ccusage_data.session_start != "--" {
                let text = format!("{}: {} → {} ({})", 
                    i18n::get(i18n::keys::USAGE_TIME),
                    self.ccusage_data.session_start, 
                    self.ccusage_data.session_end,
                    i18n::get(i18n::keys::USAGE_RESET)
                );
                label.set_text(cx, &text);
            }
        }
        
        // Update remaining time
        if let Some(mut label) = self.view.label(id!(remaining_label)).borrow_mut() {
            label.set_text(cx, &format!("{}: {}", i18n::get(i18n::keys::USAGE_REMAINING), self.ccusage_data.remaining_time));
        }
        
        // Update tokens
        if let Some(mut label) = self.view.label(id!(tokens_label)).borrow_mut() {
            if self.ccusage_data.tokens != "--" {
                label.set_text(cx, &format!("🎫 Tokens: {}", self.ccusage_data.tokens));
            }
        }
        
        // Update cost
        if let Some(mut label) = self.view.label(id!(cost_label)).borrow_mut() {
            label.set_text(cx, &format!("{}: {}", i18n::get(i18n::keys::USAGE_COST), self.ccusage_data.cost));
        }
        
        // Update model
        if let Some(mut label) = self.view.label(id!(model_label)).borrow_mut() {
            label.set_text(cx, &format!("{}: {}", i18n::get(i18n::keys::USAGE_MODEL), self.ccusage_data.model));
        }
        
        // Update status
        if let Some(mut label) = self.view.label(id!(status_label)).borrow_mut() {
            let status_text = match self.ccusage_data.status.as_str() {
                "ACTIVE" => i18n::get(i18n::keys::USAGE_ACTIVE),
                "COMPLETED" => i18n::get(i18n::keys::USAGE_COMPLETED),
                "RUNNING" => i18n::get(i18n::keys::USAGE_RUNNING),
                _ => i18n::get(i18n::keys::USAGE_INACTIVE),
            };
            label.set_text(cx, &format!("{}: {}", i18n::get(i18n::keys::USAGE_STATUS), status_text));
        }
        
        // Update model pricing comparison
        self.update_pricing_display(cx);
    }
    
    fn update_pricing_display(&mut self, cx: &mut Cx) {
        if self.model_prices.is_empty() {
            return;
        }
        
        // Update pricing title
        if let Some(mut label) = self.view.label(id!(pricing_title)).borrow_mut() {
            label.set_text(cx, &format!("📊 {}", i18n::get(i18n::keys::MODEL_PRICING_COMPARISON)));
        }
        
        // For now, we'll show the top 5 models with their prices in the cost label
        // In a more advanced implementation, we would dynamically create labels
        let mut pricing_text = String::new();
        for (i, (_model_id, display_name, cost)) in self.model_prices.iter().enumerate() {
            if i >= 5 {
                break;
            }
            if i > 0 {
                pricing_text.push_str(" | ");
            }
            pricing_text.push_str(&format!("{}: ${:.2}", display_name, cost));
        }
        
        // Update the pricing list content as part of the cost label for now
        if let Some(mut label) = self.view.label(id!(cost_label)).borrow_mut() {
            let current_cost = &self.ccusage_data.cost;
            label.set_text(cx, &format!("{}: {}\n{}", i18n::get(i18n::keys::USAGE_COST), current_cost, pricing_text));
        }
    }
    
    pub fn refresh_translations(&mut self, cx: &mut Cx) {
        self.apply_data_updates(cx);
    }
}