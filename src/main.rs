use colored::*;

fn main() {
        let flag: Vec<ColoredString> = vec![
                FLAG_STRIP_ONE_OUT.truecolor(255,165,0).bold(),
                FLAG_STRIP_POLE.truecolor(153, 76, 0).bold(),
                FLAG_STRIP_ONE_IN.white().bold(),
                FLAG_STRIP_POLE.truecolor(153, 76, 0).bold(),
                FLAG_STRIP_TWO_PRE_WHEEL.white().bold(),
                FLAG_STRIP_TWO_WHEEL.blue().bold(),
                FLAG_STRIP_TWO_POST_WHEEL.bright_white().bold(),
                FLAG_STRIP_POLE.truecolor(153, 76, 0).bold(),
                FLAG_STRIP_THREE.green().bold(), 
                POLE.truecolor(153, 76, 0).bold(),
        ];

        for line in flag.iter() {
                print!("{}", line);
        }

        println!("{}", DOCKER_BANNER.blue().bold());
        println!("{}", MESSAGE.white().bold());
        println!("{}", WATER.blue().bold());
}


const FLAG_STRIP_ONE_OUT: &str = r#"
        ∆,____________________,"#;

const FLAG_STRIP_ONE_IN: &str = ",____________________|";

const FLAG_STRIP_POLE: &str = r#"
        |"#;

const FLAG_STRIP_TWO_PRE_WHEEL: &str = ",________";
const FLAG_STRIP_TWO_WHEEL: &str = "(*)";
const FLAG_STRIP_TWO_POST_WHEEL: &str = "_________|";
const FLAG_STRIP_THREE: &str = ",____________________|";

const POLE: &str = r#"
        ||
        ||
        ||
        ||
        ==
       ===="#;

const DOCKER_BANNER: &str = r#"
        ##                  .
        ## ## ##           ==
        ## ## ## ##       ===
     /""""""""""""""""\___/ ===
~~~ {~~ ~~~~ ~~~ ~~~~ ~~ ~ /  ===-- ~~~
     \______ o          __/
       \    \        __/        
        \____\______/
~~~ ~~ ~~~~ ~~~ ~~~~ ~~ ~~ /  ===-- ~~~"#;

const MESSAGE: &str = "   ~ HAPPY INDEPENDENCE DAY INDIA ~   ";
const WATER: &str = "~~~ ~~ ~~~~ ~~~ ~~~~ ~~ ~~ /  ===-- ~~~";
