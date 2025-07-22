use makepad_widgets::*;
use std::collections::HashMap;
use crate::monitor::DailyCost;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    use crate::ui::styles::*;
    
    pub Chart = {{Chart}} {
        width: Fill,
        height: 300,
        
        chart_view = <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            spacing: 10
            
            // Chart drawing area
            chart_area = <RectView> {
                width: Fill,
                height: 200,
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        return #2a2a2a;
                    }
                }
            }
            
            // Summary
            summary_area = <View> {
                width: Fill,
                height: Fit,
                flow: Down,
                spacing: 5
                margin: {top: 10}
                
                total_label = <Label> {
                    text: "💳 总计: $0.00"
                    draw_text: {
                        text_style: {
                            font_size: 14.0
                        }
                        color: #ffffff
                    }
                }
                
                average_label = <Label> {
                    text: "📊 平均: $0.00/天"
                    draw_text: {
                        text_style: {
                            font_size: 14.0
                        }
                        color: #ffffff
                    }
                }
                
                sessions_label = <Label> {
                    text: ""
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
}

#[derive(Live, LiveHook, Widget)]
pub struct Chart {
    #[deref] view: View,
    #[rust] daily_costs: HashMap<String, DailyCost>,
    #[rust] total_cost: f64,
    #[rust] session_count: u32,
    #[rust] active_sessions: u32,
}

impl Widget for Chart {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.apply_chart_updates(cx);
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl Chart {
    pub fn update_data(&mut self, cx: &mut Cx, daily_costs: HashMap<String, DailyCost>, 
                      total_cost: f64, session_count: u32, active_sessions: u32) {
        self.daily_costs = daily_costs;
        self.total_cost = total_cost;
        self.session_count = session_count;
        self.active_sessions = active_sessions;
        self.apply_chart_updates(cx);
        cx.redraw_all();
    }
    
    fn apply_chart_updates(&mut self, cx: &mut Cx) {
        // Update summary
        if let Some(mut label) = self.view.label(id!(total_label)).borrow_mut() {
            label.set_text(cx, &format!("💳 总计: ${:.2}", self.total_cost));
        }
        
        if let Some(mut label) = self.view.label(id!(average_label)).borrow_mut() {
            let avg = if !self.daily_costs.is_empty() {
                self.total_cost / self.daily_costs.len() as f64
            } else {
                0.0
            };
            label.set_text(cx, &format!("📊 平均: ${:.2}/天", avg));
        }
        
        if self.session_count > 0 {
            if let Some(mut label) = self.view.label(id!(sessions_label)).borrow_mut() {
                label.set_text(cx, &format!("🔢 总会话数: {}  |  ⚡ 活跃会话: {}", 
                    self.session_count, self.active_sessions));
            }
        }
    }
}