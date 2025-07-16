use std::os::raw::c_int;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub struct Button {
    pub label: &'static str,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub hovered: bool,
    pub has_icon: bool,
    pub icon_type: Option<&'static str>,
}

pub struct ButtonRow {
    pub buttons: Vec<Button>,
}

pub struct Theme {
    pub bg: u32,
    pub panel: u32,
    pub wizard_bg: u32,
    pub wizard_border: u32,
    pub text: u32,
    pub text_secondary: u32,
    pub button_bg: u32,
    pub button_border: u32,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            bg: rgb(32, 34, 37),
            panel: rgb(54, 57, 63),
            wizard_bg: rgb(38, 40, 45),
            wizard_border: rgb(255, 255, 255),
            text: rgb(255, 255, 255),
            text_secondary: rgb(200, 200, 200),
            button_bg: rgb(44, 47, 51),
            button_border: rgb(255, 255, 255),
        }
    }
}

pub struct WizardBlock<'a> {
    pub step: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub info: &'a str,
    pub button_row: ButtonRow,
}

impl<'a> WizardBlock<'a> {
    pub fn new(
        step: &'a str,
        title: &'a str,
        description: &'a str,
        info: &'a str,
        button_row: ButtonRow,
    ) -> Self {
        Self { step, title, description, info, button_row }
    }
    pub fn hit_test(&self, x: i32, y: i32) -> Option<usize> {
        for (i, btn) in self.button_row.buttons.iter().enumerate() {
            if x >= btn.x && x <= btn.x + btn.w && y >= btn.y && y <= btn.y + btn.h {
                return Some(i);
            }
        }
        None
    }
}

pub fn rgb(r: u8, g: u8, b: u8) -> u32 {
    r as u32 | ((g as u32) << 8) | ((b as u32) << 16)
}

pub fn greetings() -> [&'static str; 13] {
    [
        "Welcome to Solka Engine — your path to new heights!",
        "Solka Engine inspires great projects and new ideas!",
        "May Solka Engine bring you luck and success in every code!",
        "Solka Engine — your cosmic companion in the world of creativity!",
        "With Solka Engine, your dreams of cool apps become reality!",
        "Solka Engine: where the best projects and boldest ideas are born!",
        "Inspiration is near — Solka Engine always supports your flight!",
        "Solka Engine — your portal to the world of innovation and creativity!",
        "With Solka Engine, you are always one step ahead!",
        "Solka Engine: your reliable friend in the world of code and ideas!",
        "Let Solka Engine open new horizons for you!",
        "Solka Engine — your source of inspiration and strength!",
        "With Solka Engine, the impossible becomes possible!",
    ]
}

pub struct WizardLayout {
    pub wizard_x: i32,
    pub wizard_y: i32,
    pub wizard_w: i32,
    pub wizard_h: i32,
    pub msg_y: i32,
    pub line_y: i32,
    pub buttons_y: i32,
    pub line_x: i32,
    pub line_right: i32,
}

pub fn layout_wizard_and_buttons(win_w: i32, win_h: i32) -> WizardLayout {
    let lower_block_w = 540;
    let lower_block_h = 140;
    let left_margin = 32;
    let greet_icon_size = 56;
    let greet_bg_size = 62;
    let greet_icon_y = 48 + 18; // PANEL_HEIGHT + 18
    let greet_bg_y = greet_icon_y - (greet_bg_size - greet_icon_size) / 2;
    let greet_bg_h = greet_bg_size;
    let wizard_x = (win_w - lower_block_w) / 2;
    let wizard_y = greet_bg_y + greet_bg_h + 18;
    let wizard_w = lower_block_w;
    let wizard_h = lower_block_h + 38;
    let msg_y = wizard_y + wizard_h + 18;
    let line_y = msg_y + 6;
    let buttons_y = line_y + 18;
    let line_x = left_margin;
    let line_right = win_w - left_margin;
    WizardLayout {
        wizard_x,
        wizard_y,
        wizard_w,
        wizard_h,
        msg_y,
        line_y,
        buttons_y,
        line_x,
        line_right,
    }
}

pub fn make_main_buttons(win_w: i32, win_h: i32) -> ButtonRow {
    let layout = layout_wizard_and_buttons(win_w, win_h);
    let button_labels = [
        "Create Project",
        "Open Project",
        "Import Project",
        "Guide",
    ];
    let button_w = 140;
    let button_h = 38;
    let button_gap = 14;
    let left_margin = 32;
    let button_count = button_labels.len() as i32;
    let start_x = left_margin;
    let y = layout.buttons_y;
    ButtonRow {
        buttons: button_labels.iter().enumerate().map(|(i, &label)| {
            let (has_icon, icon_type) = match i {
                0 => (true, Some("document")),
                1 => (true, Some("folder")),
                2 => (true, Some("import")),
                3 => (true, Some("guide")),
                _ => (false, None),
            };
            let x = start_x + i as i32 * (button_w + button_gap);
            Button { label, x, y, w: button_w, h: button_h, hovered: false, has_icon, icon_type }
        }).collect()
    }
}

pub fn make_wizard<'a>(win_w: i32, win_h: i32, greet_idx: usize) -> WizardBlock<'a> {
    WizardBlock::new(
        "Step 1 of 1",
        "Setup Wizard",
        "To create your first project, click the Create Project button below. Follow the steps in the wizard to complete setup",
        "",
        ButtonRow { buttons: vec![] },
    )
}
