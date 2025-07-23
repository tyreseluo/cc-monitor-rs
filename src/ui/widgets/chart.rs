use makepad_widgets::*;
use std::collections::HashMap;
use crate::monitor::DailyCost;
use crate::i18n;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;
    
    use crate::ui::styles::*;
    
    // Bar component that will show different heights
    BarView = <RoundedView> {
        width: 30,
        height: 50,
        show_bg: true
    }
    
    pub Chart = {{Chart}} {
        width: Fill,
        height: 300,
        
        chart_view = <View> {
            width: Fill,
            height: Fill,
            flow: Down,
            spacing: 10
            
            // Chart drawing area
            chart_area = <View> {
                width: Fill,
                height: 200,
                padding: 10,
                
                show_bg: true,
                draw_bg: {
                    color: #2a2a2a
                }
                
                flow: Overlay
                
                // Bars container
                bars_area = <View> {
                    width: Fill,
                    height: Fill,
                    flow: Right,
                    align: {x: 0.5, y: 1.0}
                    spacing: 5
                    padding: {bottom: 30, left: 10, right: 10}
                    
                    // Bar 1
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount1 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar1 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                    }
                    
                    // Bar 2
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount2 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar2 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 3
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount3 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar3 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 4
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount4 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar4 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 5
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount5 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar5 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 6
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount6 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar6 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 7
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount7 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar7 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 8
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount8 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar8 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 9
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount9 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar9 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                    
                    // Bar 10
                    <View> {
                        width: 35,
                        height: Fill,
                        flow: Down,
                        align: {x: 0.5, y: 1.0}
                        
                        amount10 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #ffffff
                            }
                            margin: {bottom: 2}
                        }
                        
                        bar10 = <BarView> {
                            draw_bg: { color: #4db8ff }
                        }
                        
                    }
                }
                
                // Date labels overlay at bottom
                <View> {
                    width: Fill,
                    height: Fill,
                    flow: Right,
                    align: {x: 0.5, y: 1.0}
                    spacing: 5
                    padding: {left: 10, right: 10, bottom: 5}
                    
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date1 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date2 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date3 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date4 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date5 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date6 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date7 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date8 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date9 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
                    }
                    <View> {
                        width: 35,
                        height: Fit,
                        align: {x: 0.5, y: 0.5}
                        
                        date10 = <Label> {
                            text: ""
                            draw_text: {
                                text_style: { font_size: 7.0 }
                                color: #cccccc
                            }
                            align: {x: 0.5, y: 0.5}
                        }
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
        // Sort daily costs by date
        let mut sorted_costs: Vec<_> = self.daily_costs.iter().collect();
        sorted_costs.sort_by(|a, b| a.0.cmp(b.0));
        
        // Find max cost for scaling
        let max_cost = sorted_costs.iter()
            .map(|(_, cost)| cost.cost)
            .fold(0.0, f64::max);
        
        // Update bars (show last 10 entries)
        let recent_costs: Vec<_> = sorted_costs.iter()
            .rev()
            .take(10)
            .rev()
            .collect();
        
        // Update bars
        let bar_ids = [
            id!(bar1), id!(bar2), id!(bar3), id!(bar4), id!(bar5),
            id!(bar6), id!(bar7), id!(bar8), id!(bar9), id!(bar10)
        ];
        let amount_ids = [
            id!(amount1), id!(amount2), id!(amount3), id!(amount4), id!(amount5),
            id!(amount6), id!(amount7), id!(amount8), id!(amount9), id!(amount10)
        ];
        let date_ids = [
            id!(date1), id!(date2), id!(date3), id!(date4), id!(date5),
            id!(date6), id!(date7), id!(date8), id!(date9), id!(date10)
        ];
        
        for i in 0..10 {
            if i < recent_costs.len() {
                let (date, cost) = recent_costs[i];
                
                // Calculate bar height (minimum 5, maximum 120)
                let height = if max_cost > 0.0 {
                    5.0 + (cost.cost / max_cost) as f32 * 115.0
                } else {
                    5.0
                };
                
                println!("Bar {}: cost=${:.2}, height={:.1}", i, cost.cost, height);
                
                // Update bar height
                if let Some(mut bar) = self.view.view(bar_ids[i]).borrow_mut() {
                    bar.apply_over(cx, live!{
                        height: (height)
                    });
                    println!("Updated bar {} height to {}", i, height);
                } else {
                    println!("Failed to find bar {}", i);
                }
                
                // Update amount label
                if let Some(mut label) = self.view.label(amount_ids[i]).borrow_mut() {
                    label.set_text(cx, &format!("${:.0}", cost.cost));
                }
                
                // Update date label
                if let Some(mut label) = self.view.label(date_ids[i]).borrow_mut() {
                    let date_parts: Vec<&str> = date.split('-').collect();
                    let short_date = if date_parts.len() == 2 {
                        format!("{}/{}", date_parts[0], date_parts[1])
                    } else {
                        date.to_string()
                    };
                    label.set_text(cx, &short_date);
                }
            } else {
                // Hide unused bars
                if let Some(mut bar) = self.view.view(bar_ids[i]).borrow_mut() {
                    bar.apply_over(cx, live!{
                        height: 0.0
                    });
                }
                if let Some(mut label) = self.view.label(amount_ids[i]).borrow_mut() {
                    label.set_text(cx, "");
                }
                if let Some(mut label) = self.view.label(date_ids[i]).borrow_mut() {
                    label.set_text(cx, "");
                }
            }
        }
        
        // Update summary
        if let Some(mut label) = self.view.label(id!(total_label)).borrow_mut() {
            label.set_text(cx, &format!("{}: ${:.2}", i18n::get(i18n::keys::HISTORY_TOTAL), self.total_cost));
        }
        
        if let Some(mut label) = self.view.label(id!(average_label)).borrow_mut() {
            let avg = if !self.daily_costs.is_empty() {
                self.total_cost / self.daily_costs.len() as f64
            } else {
                0.0
            };
            label.set_text(cx, &format!("{}: ${:.2}/{}", 
                i18n::get(i18n::keys::HISTORY_AVERAGE), 
                avg,
                i18n::get(i18n::keys::COMMON_DAY)
            ));
        }
        
        if self.session_count > 0 {
            if let Some(mut label) = self.view.label(id!(sessions_label)).borrow_mut() {
                label.set_text(cx, &format!("{}: {}  |  {}: {}", 
                    i18n::get(i18n::keys::HISTORY_SESSIONS_TOTAL),
                    self.session_count,
                    i18n::get(i18n::keys::HISTORY_SESSIONS_ACTIVE),
                    self.active_sessions
                ));
            }
        }
    }
    
    pub fn refresh_translations(&mut self, cx: &mut Cx) {
        self.apply_chart_updates(cx);
    }
}