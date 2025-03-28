use owo_colors::OwoColorize;

pub struct McCliLogo {
    logo: &'static str,
}

impl McCliLogo {
    pub fn new() -> Self {
        McCliLogo {
            logo: r#"
                                            /$$$$$$    /$$       /$$$$$$
                                            /$$ __   $$| $$      |_  $$_/
                     /$$$$$$/$$$$   /$$$$$$$| $$  \__/| $$       | $$  
                    | $$_  $$_  $$ /$$_____/| $$      | $$       | $$  
                    | $$ \ $$ \ $$| $$      | $$      | $$       | $$
                    | $$ | $$ | $$| $$      | $$    $$| $$       | $$  
                    | $$ | $$ | $$|  $$$$$$$|  $$$$$$/| $$$$$$$$/$$$$$$
    ／l、           |__/ |__/ |__/ \_______/ \______/ |________/|______/
   （ﾟ､ ｡ ７         
    \_ じしf_,)ノ                                                       DestinySoul Studios
            "#,
        }
    }

    pub fn display(&self) {
        println!("{}", self.logo.bright_blue().bold());
    }
}