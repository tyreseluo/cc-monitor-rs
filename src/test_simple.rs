use makepad_widgets::*;

live_design! {
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    TestApp = {{TestApp}} {
        ui: <Root>{
            main_window = <Window>{
                window: {
                    title: "Test Window",
                    position: vec2(100, 100),
                    inner_size: vec2(400, 300)
                },
                show_bg: true,
                
                body = <View>{
                    width: Fill,
                    height: Fill,
                    flow: Down,
                    spacing: 20,
                    padding: 20,
                    
                    <Label>{
                        text: "Hello World!",
                        draw_text: {
                            color: #ffffff,
                            text_style: {font_size: 24.0}
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook)]
pub struct TestApp {
    #[live] ui: WidgetRef,
}

impl LiveRegister for TestApp {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for TestApp {
    fn handle_startup(&mut self, cx: &mut Cx) {
        log!("Test app started!");
    }
}

impl AppMain for TestApp {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(TestApp);