use std::error::Error;

fn colorsquare_normal(indent: u8) {
    // ANSI escape codes
    let esc = "\x1b";

    // Foreground colors
    let c01 = format!("{esc}[31m");
    let c02 = format!("{esc}[91m");
    let c03 = format!("{esc}[32m");
    let c04 = format!("{esc}[92m");
    let c05 = format!("{esc}[33m");
    let c06 = format!("{esc}[93m");
    let c07 = format!("{esc}[34m");
    let c08 = format!("{esc}[94m");
    let c09 = format!("{esc}[35m");
    let c10 = format!("{esc}[95m");
    let c11 = format!("{esc}[36m");
    let c12 = format!("{esc}[96m");

    // Reset
    let reset = format!("{esc}[0m");

    // Output colord_ text
    println!();
    println!(" \
{c01}▀ █ {c02}█ ▀{reset}   \
{c03}▀ █ {c04}█ ▀{reset}   \
{c05}▀ █ {c06}█ ▀{reset}   \
{c07}▀ █ {c08}█ ▀{reset}   \
{c09}▀ █ {c10}█ ▀{reset}   \
{c11}▀ █ {c12}█ ▀{reset}");

    println!(" \
{c01}██  {c02} ██{reset}   \
{c03}██  {c04} ██{reset}   \
{c05}██  {c06} ██{reset}   \
{c07}██  {c08} ██{reset}   \
{c09}██  {c10} ██{reset}   \
{c11}██  {c12} ██{reset}");

    println!(" \
{c01}▄ █ {c02}█ ▄{reset}   \
{c03}▄ █ {c04}█ ▄{reset}   \
{c05}▄ █ {c06}█ ▄{reset}   \
{c07}▄ █ {c08}█ ▄{reset}   \
{c09}▄ █ {c10}█ ▄{reset}   \
{c11}▄ █ {c12}█ ▄{reset}");
    println!();
}

fn colorsquare_rgb(colors: &[(u8, u8, u8)]) {
    // ANSI escape codes
    let esc = "\x1b";

    // Ensure there are exactly 12 colors provided
    if colors.len() != 12 {
        eprintln!("Error: Exactly 12 colors are required.");
        return;
    }

    // Create formatted color strings
    let formatted_colors: Vec<String> = colors.iter()
        .map(|&(r, g, b)| format!("{esc}[38;2;{r};{g};{b}m"))
        .collect();

    // Reset
    let reset = format!("{esc}[0m");

    // Output colored text
    println!();
    println!(" \
{}▀ █ {}█ ▀{}   \
{}▀ █ {}█ ▀{}   \
{}▀ █ {}█ ▀{}   \
{}▀ █ {}█ ▀{}   \
{}▀ █ {}█ ▀{}   \
{}▀ █ {}█ ▀{}",
        formatted_colors[0], formatted_colors[1], reset,
        formatted_colors[2], formatted_colors[3], reset,
        formatted_colors[4], formatted_colors[5], reset,
        formatted_colors[6], formatted_colors[7], reset,
        formatted_colors[8], formatted_colors[9], reset,
        formatted_colors[10], formatted_colors[11], reset
    );

    println!(" \
{}██  {} ██{}   \
{}██  {} ██{}   \
{}██  {} ██{}   \
{}██  {} ██{}   \
{}██  {} ██{}   \
{}██  {} ██{}",
        formatted_colors[0], formatted_colors[1], reset,
        formatted_colors[2], formatted_colors[3], reset,
        formatted_colors[4], formatted_colors[5], reset,
        formatted_colors[6], formatted_colors[7], reset,
        formatted_colors[8], formatted_colors[9], reset,
        formatted_colors[10], formatted_colors[11], reset
    );

    println!(" \
{}▄ █ {}█ ▄{}   \
{}▄ █ {}█ ▄{}   \
{}▄ █ {}█ ▄{}   \
{}▄ █ {}█ ▄{}   \
{}▄ █ {}█ ▄{}   \
{}▄ █ {}█ ▄{}",
        formatted_colors[0], formatted_colors[1], reset,
        formatted_colors[2], formatted_colors[3], reset,
        formatted_colors[4], formatted_colors[5], reset,
        formatted_colors[6], formatted_colors[7], reset,
        formatted_colors[8], formatted_colors[9], reset,
        formatted_colors[10], formatted_colors[11], reset
    );
    println!();
}


fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let args: &[String] = &args[1..];
    let mut colorscheme: String = String::new();
    for (i, arg) in args.iter().enumerate() {
        if arg.starts_with("-"){
            match arg.as_str() {
                "--colorscheme" | "-c" => {
                    if i + 1 < args.len() {
                        match args[i + 1].as_str() {
                            "rgb" => {
                                colorscheme = "rgb".to_string();
                            }
                            "normal" | _ => {
                                colorscheme = "normal".to_string();
                            }
                        }
                    } else {
                        println!("No colorscheme specified after {}", arg);
                    }}
                _ => {
                    println!("{i}, {arg}")
                }
            }
        }
    }
    match colorscheme.as_str() {
        "rgb" => {
            let colors = [
                (0, 128, 255),  // Light Blue
                (0, 144, 238),  // Sky Blue
                (0, 160, 221),  // Cerulean
                (0, 176, 204),  // Cyan
                (0, 192, 187),  // Turquoise
                (0, 208, 170),  // Medium Turquoise
                (0, 224, 153),  // Aquamarine
                (0, 240, 136),  // Medium Aquamarine
                (0, 255, 119),  // Spring Green
                (17, 255, 102), // Medium Spring Green
                (34, 255, 85),  // Lime Green
                (51, 255, 68),  // Lawn Green
            ];

            colorsquare_rgb(&colors);
        }
        "normal" | _ => {
            colorsquare_normal(8);
        }
    }
    Ok(())
}

